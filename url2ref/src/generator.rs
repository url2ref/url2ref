//! Generator responsible for producing a [`Reference`]

use deepl_api::{DeepL, Error as DeepLError, TranslatableTextList};
use htmlescape::decode_html;
use std::result;

use chrono::{NaiveDateTime, DateTime, Utc, ParseError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumIter, EnumCount};
use thiserror::Error;

use crate::attribute::{Attribute, AttributeType, Date, Translation};
use crate::ai_extractor::{self, AiExtractionError};

use crate::curl::CurlError;
use crate::doi::DoiError;
use crate::parser::{AttributeCollection, ParseInfo};
use crate::reference::Reference;
use crate::GenerationOptions;
use crate::curl;

type GenerationResult<T> = result::Result<T, ReferenceGenerationError>;

/// Errors encountered during reference generation are
/// wrapped in this enum.
#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("curl GET failed")]
    CurlError(#[from] CurlError),

    #[error("All provided parsers failed")]
    ParseFailure,

    #[error("Parser was skipped")]
    ParseSkip,

    #[error("HTML failed to parse")]
    HTMLParseError(#[from] std::io::Error),

    #[error("DeepL translation failed")]
    DeepLError(#[from] DeepLError),

    #[error("Title translation procedure failed")]
    TranslationError,

    #[error("Retrieving DOI failed")]
    DoiError(#[from] DoiError),

    #[error("Retrieving DOI failed")]
    ArchiveError(#[from] ArchiveError),

    #[error("AI extraction failed")]
    AiExtractionError(#[from] AiExtractionError),
}

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("Wayback Machine API call failed")]
    CurlError(#[from] curl::CurlError),

    #[error("Couldn't deserialize JSON into WaybackSnapshot struct")]
    DeserializeError(#[from] serde_json::Error),

    #[error("JSON byte-to-String conversion failed")]
    ByteConversionError(#[from] std::string::FromUtf8Error)
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, EnumIter, EnumCount, Eq, Hash, Serialize, Deserialize,
)]
pub enum MetadataType {
    #[default]
    OpenGraph,
    SchemaOrg,
    HtmlMeta,
    Doi,
    Zotero,
    Ai,
}

// Re-export AI types for convenience
pub use crate::ai_extractor::{AiExtractionOptions, AiProvider};

/// Supported translation service providers.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranslationProvider {
    #[default]
    DeepL,
    Google,
}

/// User options for title translation.
#[derive(Clone, Default)]
pub struct TranslationOptions {
    /// The translation service provider to use
    pub provider: TranslationProvider,
    /// Contains an ISO 639 language code. If None, source language is guessed
    pub source: Option<String>,
    /// Contains an ISO 639 language code. If None, no translation.
    pub target: Option<String>,
    /// DeepL API key (required if provider is DeepL)
    pub deepl_key: Option<String>,
    /// Google Cloud Translation API key (required if provider is Google)
    pub google_key: Option<String>,
}

/// User options for fetching of archived URL and date.
#[derive(Clone)]
pub struct ArchiveOptions {
    /// Whether to attempt to fetch an archived URL and date
    pub include_archived: bool,
    /// Whether to attempt perform the archive operation if the site
    /// hasn't been archived yet.
    pub perform_archival: bool,
}
impl Default for ArchiveOptions {
    fn default() -> Self {
        Self {
            include_archived: true,
            perform_archival: true,
        }
    }
}

pub mod attribute_config {
    use std::collections::{HashMap, HashSet};

    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};

    use super::MetadataType;
    use crate::attribute::AttributeType;

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct AttributePriority {
        pub priority: Vec<MetadataType>,
    }

    impl Default for AttributePriority {
        fn default() -> Self {
            Self {
                priority: vec![MetadataType::OpenGraph, MetadataType::SchemaOrg, MetadataType::HtmlMeta],
            }
        }
    }
    impl AttributePriority {
        pub fn new(priority: &[MetadataType]) -> Self {
            Self {
                priority: priority.to_vec(),
            }
        }
    }

    #[derive(Default, Builder, Clone, Serialize, Deserialize, Debug)]
    #[builder(setter(into, strip_option), default)]
    pub struct AttributeConfig {
        pub title: Option<AttributePriority>,
        pub authors: Option<AttributePriority>,
        pub date: Option<AttributePriority>,
        pub archive_date: Option<AttributePriority>,
        pub language: Option<AttributePriority>,
        pub locale: Option<AttributePriority>,
        pub site: Option<AttributePriority>,
        pub url: Option<AttributePriority>,
        pub archive_url: Option<AttributePriority>,
        pub journal: Option<AttributePriority>,
        pub publisher: Option<AttributePriority>,
        pub institution: Option<AttributePriority>,
        pub volume: Option<AttributePriority>,
    }

    impl AttributeConfig {
        pub fn new(priority: AttributePriority) -> Self {
            AttributeConfigBuilder::default()
                .title(priority.clone())
                .authors(priority.clone())
                .date(priority.clone())
                .archive_date(priority.clone())
                .language(priority.clone())
                .locale(priority.clone())
                .site(priority.clone())
                .url(priority.clone())
                .archive_url(priority.clone())
                .journal(priority.clone())
                .publisher(priority.clone())
                .institution(priority.clone())
                .volume(priority.clone())
                .build()
                .unwrap()
        }

        pub fn get(&self, attribute_type: AttributeType) -> &Option<AttributePriority> {
            match attribute_type {
                AttributeType::Title       => &self.title,
                AttributeType::Author      => &self.authors,
                AttributeType::Date        => &self.date,
                AttributeType::ArchiveDate => &self.archive_date,
                AttributeType::Language    => &self.language,
                AttributeType::Locale      => &self.locale,
                AttributeType::Site        => &self.site,
                AttributeType::Url         => &self.url,
                AttributeType::ArchiveUrl  => &self.archive_url,
                AttributeType::Type        => &None, // TODO: Decide future of AttributeType::Type
                AttributeType::Journal     => &self.journal,
                AttributeType::Publisher   => &self.publisher,
                AttributeType::Volume      => &self.volume,
                AttributeType::Institution => &self.institution,
            }
        }

        /// Finds the parsers used.
        /// Serialize to JSON, deserialize back to a HashMap. This allows us to iterate over all fields.
        /// This is important because if additional fields of AttributeConfig are added, this function will
        /// still work.
        pub fn parsers_used(&self) -> Vec<MetadataType> {
            let json_string = serde_json::to_string(self).unwrap();
            let map: HashMap<String, Option<AttributePriority>> =
                serde_json::from_str(&json_string).unwrap();
            

            let flattened_map: Vec<MetadataType> = map
                .values()
                .into_iter()
                .map(|a| a.clone().unwrap_or_default().priority)
                .collect::<Vec<Vec<MetadataType>>>()
                .concat();

            flattened_map
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect()
        }
    }
}

/// Generates a [`Reference`] from a URL.
pub fn from_url(url: &str, options: &GenerationOptions) -> GenerationResult<Reference> {
    let parse_info = ParseInfo::from_url(url, &options.attribute_config.parsers_used())?;
    create_reference(&parse_info, &options)
}

/// Generates a [`Reference`] from raw HTML as read from a file.
pub fn from_file(html_path: &str, options: &GenerationOptions) -> GenerationResult<Reference> {
    let parse_info = ParseInfo::from_file(html_path)?;
    create_reference(&parse_info, &options)
}

/// Creates a [`Reference`] from pre-fetched [`ParseInfo`].
/// This is the public entry point for generating references from cached HTML.
pub fn create_reference_from_parse_info(parse_info: &ParseInfo, options: &GenerationOptions) -> GenerationResult<Reference> {
    create_reference(parse_info, options)
}

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn create_reference(parse_info: &ParseInfo, options: &GenerationOptions) -> GenerationResult<Reference> {
    // Build attribute collection based on configuration
    let attributes = AttributeCollection::initialize(&options.attribute_config, parse_info);

    let mut title = attributes.get(AttributeType::Title).cloned();
    let mut author = attributes.get(AttributeType::Author).cloned();
    let mut date = attributes.get(AttributeType::Date).cloned();
    let mut language = attributes.get(AttributeType::Language).cloned();
    let mut site = attributes.get(AttributeType::Site).cloned();
    let url = attributes.get(AttributeType::Url).cloned()
        .or(parse_info.url.map(|x| Attribute::Url(x.to_string()))); // If no URL collected, attempt to use user-supplied URL
    let mut publisher = attributes.get(AttributeType::Publisher).cloned();

    println!("[url2ref] create_reference: collected attributes");
    println!("[url2ref]   title: {:?}", title);
    println!("[url2ref]   author: {:?}", author);
    println!("[url2ref]   date: {:?}", date);
    println!("[url2ref]   site: {:?}", site);
    println!("[url2ref]   publisher: {:?}", publisher);
    println!("[url2ref]   language: {:?}", language);
    println!("[url2ref]   translation_options.target: {:?}", options.translation_options.target);

    // AI fallback: if AI extraction is enabled and we have missing fields, try AI
    if options.ai_options.enabled {
        println!("[url2ref] AI extraction is enabled");
        let missing_fields = title.is_none() || author.is_none() || date.is_none() 
            || site.is_none() || publisher.is_none() || language.is_none();
        
        println!("[url2ref] Missing fields check: title={}, author={}, date={}, site={}, publisher={}, language={}",
            title.is_none(), author.is_none(), date.is_none(), site.is_none(), publisher.is_none(), language.is_none());
        
        if missing_fields {
            println!("[url2ref] AI extraction enabled, attempting to fill missing fields");
            
            if let Some(url_str) = parse_info.url {
                match ai_extractor::extract_metadata(url_str, &parse_info.raw_html, &options.ai_options) {
                    Ok(ai_metadata) => {
                        println!("[url2ref] AI extraction succeeded:");
                        println!("[url2ref]   AI title: {:?}", ai_metadata.title);
                        println!("[url2ref]   AI authors: {:?}", ai_metadata.authors);
                        println!("[url2ref]   AI date: {:?}", ai_metadata.date);
                        println!("[url2ref]   AI site: {:?}", ai_metadata.site);
                        println!("[url2ref]   AI publisher: {:?}", ai_metadata.publisher);
                        println!("[url2ref]   AI language: {:?}", ai_metadata.language);
                        
                        // Only fill in missing fields from AI
                        if title.is_none() {
                            title = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Title);
                            println!("[url2ref]   -> Filled title from AI: {:?}", title);
                        }
                        if author.is_none() {
                            author = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Author);
                            println!("[url2ref]   -> Filled author from AI: {:?}", author);
                        }
                        if date.is_none() {
                            date = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Date);
                            println!("[url2ref]   -> Filled date from AI: {:?}", date);
                        }
                        if site.is_none() {
                            site = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Site);
                            println!("[url2ref]   -> Filled site from AI: {:?}", site);
                        }
                        if publisher.is_none() {
                            publisher = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Publisher);
                            println!("[url2ref]   -> Filled publisher from AI: {:?}", publisher);
                        }
                        if language.is_none() {
                            language = ai_extractor::get_attribute_from_ai(&ai_metadata, AttributeType::Language);
                            println!("[url2ref]   -> Filled language from AI: {:?}", language);
                        }
                    }
                    Err(e) => {
                        println!("[url2ref] AI extraction failed: {:?}", e);
                        // Continue without AI data - this is a fallback, not critical
                    }
                }
            } else {
                println!("[url2ref] No URL available for AI extraction");
            }
        } else {
            println!("[url2ref] No missing fields, skipping AI extraction");
        }
    } else {
        println!("[url2ref] AI extraction is disabled");
    }

    // Act according to translation options;
    // if translation fails, None will be the result.
    let translated_title = translate_title(&title, &options.translation_options).ok();

    // Include archived URL and date according to archive options.
    let (archive_url, archive_date) = fetch_archive_info(&url, &options.archive_options);

    let reference = Reference::NewsArticle {
        title,
        translated_title,
        author,
        date,
        language,
        url,
        site,
        publisher,
        archive_url,
        archive_date
    };

    Ok(reference)
}

/// Attempts to translate the provided [`Attribute::Title`].
/// Returns Option<[`Attribute::TranslatedTitle`]> on if successful and None otherwise.
fn translate_title(title: &Option<Attribute>, options: &TranslationOptions) -> GenerationResult<Attribute> {
    println!("[url2ref] translate_title called");
    println!("[url2ref]   title: {:?}", title);
    println!("[url2ref]   options.target: {:?}", options.target);
    println!("[url2ref]   options.provider: {:?}", options.provider);
    println!("[url2ref]   options.deepl_key: {}", options.deepl_key.as_ref().map(|k| format!("SET ({} chars)", k.len())).unwrap_or("NOT SET".to_string()));
    println!("[url2ref]   options.google_key: {}", options.google_key.as_ref().map(|k| format!("SET ({} chars)", k.len())).unwrap_or("NOT SET".to_string()));

    // If title parameter is actually an Attribute::Title,
    // proceed with translation. Otherwise, throw an error.
    if let Some(Attribute::Title(content)) = title {
        println!("[url2ref]   Proceeding with translation of: {}", content);
        match translate(content, &options) {
            Ok(text) => {
                println!("[url2ref]   Translation successful: {}", text);
                let translation_attribute = Attribute::TranslatedTitle(Translation {
                    text,
                    // We can safely unwrap here as the call to translate()
                    // would've already failed if no target language was provided.
                    language: options.target.clone().unwrap(),
                });
                Ok(translation_attribute)
            }
            Err(e) => {
                eprintln!("[url2ref]   Translation FAILED: {:?}", e);
                Err(e)
            }
        }
    } else {
        eprintln!("[url2ref]   No title attribute to translate");
        Err(ReferenceGenerationError::TranslationError)
    }
}

/// Translates content according to the provided TranslationOptions.
/// Dispatches to the appropriate translation provider based on options.
fn translate<'a>(content: &'a str, options: &TranslationOptions) -> GenerationResult<String> {
    println!("[url2ref] translate() called with provider: {:?}", options.provider);
    
    let target_lang = match options.target.clone() {
        Some(lang) => {
            println!("[url2ref]   target_lang: {}", lang);
            lang
        }
        None => {
            eprintln!("[url2ref]   ERROR: No target language specified!");
            return Err(ReferenceGenerationError::TranslationError);
        }
    };

    match options.provider {
        TranslationProvider::DeepL => {
            println!("[url2ref]   Using DeepL provider");
            translate_with_deepl(content, options, &target_lang)
        }
        TranslationProvider::Google => {
            println!("[url2ref]   Using Google provider");
            translate_with_google(content, options, &target_lang)
        }
    }
}

/// Translates content using the DeepL API.
fn translate_with_deepl(content: &str, options: &TranslationOptions, target_lang: &str) -> GenerationResult<String> {
    let api_key = options.deepl_key.clone().ok_or(ReferenceGenerationError::TranslationError)?;
    let deepl = DeepL::new(api_key);

    let texts = TranslatableTextList {
        source_language: options.source.clone(),
        target_language: target_lang.to_string(),
        texts: vec![content.to_string()],
    };

    let translated = deepl.translate(None, texts)?;
    let text = &translated[0].text;
    // Decode any HTML entities that might be present
    Ok(decode_html(text).unwrap_or_else(|_| text.to_owned()))
}

/// Response structure for Google Cloud Translation API v2.
#[derive(Debug, Deserialize)]
struct GoogleTranslateResponse {
    data: GoogleTranslateData,
}

#[derive(Debug, Deserialize)]
struct GoogleTranslateData {
    translations: Vec<GoogleTranslation>,
}

#[derive(Debug, Deserialize)]
struct GoogleTranslation {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

/// Translates content using the Google Cloud Translation API v2.
fn translate_with_google(content: &str, options: &TranslationOptions, target_lang: &str) -> GenerationResult<String> {
    println!("[url2ref] translate_with_google() called");
    
    let api_key = match options.google_key.clone() {
        Some(key) => {
            println!("[url2ref]   Google API key found ({} chars)", key.len());
            key
        }
        None => {
            eprintln!("[url2ref]   ERROR: Google API key is NOT SET!");
            return Err(ReferenceGenerationError::TranslationError);
        }
    };
    
    // Build the API URL with query parameters
    let encoded_text = urlencoding::encode(content);
    let mut url = format!(
        "https://translation.googleapis.com/language/translate/v2?key={}&q={}&target={}",
        api_key, encoded_text, target_lang.to_lowercase()
    );
    
    // Add source language if specified
    if let Some(ref source_lang) = options.source {
        url.push_str(&format!("&source={}", source_lang.to_lowercase()));
    }

    println!("[url2ref]   Making request to Google Translate API (key redacted in URL)");
    println!("[url2ref]   Target language: {}", target_lang);

    // Make the API request
    let response_str = match curl::get(&url, None, false) {
        Ok(resp) => {
            println!("[url2ref]   API response received ({} bytes)", resp.len());
            println!("[url2ref]   Response: {}", &resp[..resp.len().min(500)]);
            resp
        }
        Err(e) => {
            eprintln!("[url2ref]   ERROR: curl request failed: {:?}", e);
            return Err(ReferenceGenerationError::CurlError(e));
        }
    };
    
    // Parse the JSON response
    let response: GoogleTranslateResponse = match serde_json::from_str(&response_str) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[url2ref]   ERROR: Failed to parse JSON response: {:?}", e);
            eprintln!("[url2ref]   Raw response: {}", response_str);
            return Err(ReferenceGenerationError::TranslationError);
        }
    };
    
    // Extract the translated text
    match response.data.translations.first() {
        Some(t) => {
            // Google Translate API returns HTML-encoded text, so we need to decode it
            let decoded_text = decode_html(&t.translated_text)
                .unwrap_or_else(|_| t.translated_text.clone());
            println!("[url2ref]   Translation result: {}", decoded_text);
            Ok(decoded_text)
        }
        None => {
            eprintln!("[url2ref]   ERROR: No translations in response");
            Err(ReferenceGenerationError::TranslationError)
        }
    }
}

/// Struct denoting a snapshot returned by the Wayback Machine API.
/// For more information, see the [`Wayback Machine API documentation`].
/// 
/// [`Wayback Machine API documentation`]: https://archive.org/help/wayback_api.php
#[derive(Debug, Deserialize)]
struct WaybackSnapshot {
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "available")]
    _available: bool,
    url: String,
    timestamp: String,
}

/// Attempt to fetch archive information from the Wayback Machine and
/// construct an archive URL and date.
/// If no archive exists and `perform_archival` is true, attempt to create one.
fn fetch_archive_info(url: &Option<Attribute>, options: &ArchiveOptions) -> (Option<Attribute>, Option<Attribute>) {
    if !options.include_archived {
        return (None, None)
    }

    // If URL specified, attempt to fetch archived URL.
    if let Some(Attribute::Url(url_str)) = url {
        // First, try to get an existing archive
        let wayback_snapshot = call_wayback_api(url_str, &None).ok();

        // If no archive exists and perform_archival is enabled, try to create one
        let wayback_snapshot = match wayback_snapshot {
            Some(snapshot) => Some(snapshot),
            None if options.perform_archival => {
                // Try to save the page to the Wayback Machine
                save_to_wayback(url_str).ok()
            }
            None => None,
        };

        let url_attribute  = wayback_snapshot.as_ref().map(|snapshot| Attribute::ArchiveUrl(snapshot.url.clone()));
        let date_attribute = wayback_snapshot.as_ref().map(|snapshot| {
            Attribute::ArchiveDate(
                Date::DateTime(
                    parse_wayback_timestamp(&snapshot.timestamp).unwrap() // TODO: Get rid of this unwrap()
                )
            )
        });

        return (url_attribute, date_attribute)
    } 
    
    (None, None)
}

/// Save a URL to the Wayback Machine using the Save Page Now (SPN) API.
/// Returns a WaybackSnapshot if successful.
fn save_to_wayback(url: &str) -> Result<WaybackSnapshot, ArchiveError> {
    // Use the Wayback Machine's Save Page Now endpoint
    let save_url = format!("https://web.archive.org/save/{}", url);
    
    // Make a request to save the page - capture the redirect Location header
    // The response will be a 302 redirect to the archived page
    let location = curl::get_redirect_location(&save_url)?;
    
    // Parse the archive URL and timestamp from the Location header
    // Format: https://web.archive.org/web/20260124121505/https://example.com/...
    if let Some(timestamp) = parse_wayback_location(&location) {
        Ok(WaybackSnapshot {
            _status: "200".to_string(),
            _available: true,
            url: location,
            timestamp,
        })
    } else {
        // Fallback: query the API to get the snapshot info
        std::thread::sleep(std::time::Duration::from_secs(2));
        call_wayback_api(url, &None)
    }
}

/// Parse a Wayback Machine archive URL to extract the timestamp.
/// Expected format: https://web.archive.org/web/TIMESTAMP/original_url
fn parse_wayback_location(location: &str) -> Option<String> {
    // Look for the pattern /web/TIMESTAMP/ where TIMESTAMP is 14 digits
    let parts: Vec<&str> = location.split("/web/").collect();
    if parts.len() >= 2 {
        // Get the part after /web/ and extract the timestamp (first 14 chars or until /)
        let after_web = parts[1];
        let timestamp: String = after_web.chars().take_while(|c| c.is_ascii_digit()).collect();
        if timestamp.len() == 14 {
            return Some(timestamp);
        }
    }
    None
}

/// Send a query for a URL to the Wayback Machine API and return the closest snapshot.
fn call_wayback_api(url: &str, timestamp_option: &Option<&str>) -> Result<WaybackSnapshot, ArchiveError> {
    // URL-encode the URL for the query parameter (handles special chars like ø → %C3%B8)
    let encoded_url = urlencoding::encode(url);
    
    // Build the request URL, only including timestamp if provided
    let request_url = match timestamp_option {
        Some(ts) if !ts.is_empty() => format!("https://archive.org/wayback/available?url={encoded_url}&timestamp={ts}"),
        _ => format!("https://archive.org/wayback/available?url={encoded_url}"),
    };
    
    let response = curl::get(&request_url, None, false)?;
    
    // Extract snapshot information for the closest retrieved snapshot.
    let json_value = serde_json::from_str::<Value>(&response)?;
    let snapshot_info = &json_value["archived_snapshots"]["closest"];
    
    // Check if closest snapshot exists (not null)
    if snapshot_info.is_null() {
        return Err(ArchiveError::DeserializeError(
            serde_json::from_str::<WaybackSnapshot>("{}").unwrap_err()
        ));
    }

    // Attempt to deserialize the snapshot information to a [`WaybackSnapshot`] struct.
    serde_json::from_value(snapshot_info.clone())
        .map_err(|err| ArchiveError::DeserializeError(err))
}

/// Utility function to parse a timestamp from snapshots 
/// returned by the Wayback Machine API.
fn parse_wayback_timestamp(timestamp: &str) -> Result<DateTime<Utc>, ParseError> {
    let timestamp_format = "%Y%m%d%H%M%S";

    let naive_datetime = NaiveDateTime::parse_from_str(&timestamp, &timestamp_format)?;
    let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    Ok(datetime_utc)
}
#[cfg(test)]
mod test {
    use crate::attribute::Attribute;

    use super::{
        attribute_config::{AttributeConfig, AttributePriority}, fetch_archive_info, ArchiveOptions, MetadataType
    };

    #[test]
    fn test_get_unique_parsers() {
        let expected = vec![MetadataType::OpenGraph, MetadataType::Doi];
        let config = AttributeConfig::new(AttributePriority {
            priority: expected.clone(),
        });
        let result = config.parsers_used();

        assert_eq!(expected.len(), result.len());
        assert!(expected.iter().all(|item| result.contains(item)));
    }

    // Tests that the default implementation is used and is functional. If default parsers are changed,
    // this test must be changed to match.
    #[test]
    fn test_attribute_config_default() {
        let expected = vec![MetadataType::OpenGraph, MetadataType::SchemaOrg, MetadataType::HtmlMeta];
        let config = AttributeConfig::default();
        let result = config.parsers_used();

        assert_eq!(expected.len(), result.len());
        assert!(expected.iter().all(|item| result.contains(item)));
    }

    #[test]
    fn test_archive_url() {
        let url = "https://www.information.dk/kultur/2018/01/casper-mandrilaftalen-burde-lade-goere-gjorde";
        let url_attribute = Some(Attribute::Url(url.to_string()));
        let archive_options = ArchiveOptions::default();
        
        // Fetch archive info - we just verify we get an archive URL, not a specific one
        // since the Wayback Machine may have newer snapshots
        let (url_result, _) = fetch_archive_info(&url_attribute, &archive_options);
        
        // Verify we got an archive URL that contains the expected base pattern
        assert!(url_result.is_some());
        if let Some(Attribute::ArchiveUrl(archive_url)) = url_result {
            assert!(archive_url.starts_with("http://web.archive.org/web/"));
            assert!(archive_url.contains("information.dk/kultur/2018/01/casper-mandrilaftalen"));
        } else {
            panic!("Expected ArchiveUrl attribute");
        }
    }

    #[test]
    fn test_archive_url_disabled() {
        let url = "https://www.information.dk/kultur/2018/01/casper-mandrilaftalen-burde-lade-goere-gjorde";
        let url_attribute = Some(Attribute::Url(url.to_string()));
        let archive_options = ArchiveOptions {
            include_archived: false,
            perform_archival: false
        };
        
        // Timestamp is difficult to test for, so it is not needed for now.
        let (url_result, _) = fetch_archive_info(&url_attribute, &archive_options);
        assert_eq!(url_result, None);
    }
}
