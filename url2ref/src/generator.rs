//! Generator responsible for producing a [`Reference`]

use deepl_api::{DeepL, Error as DeepLError, TranslatableTextList};
use std::result;

use chrono::{NaiveDateTime, DateTime, Utc, ParseError};
use serde::Deserialize;
use serde_json::Value;
use strum::{EnumIter, EnumCount};
use thiserror::Error;

use crate::attribute::{Attribute, AttributeType, Date, Translation};

use serde::Serialize;

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
    Doi
}

/// User options for title translation.
#[derive(Clone, Default)]
pub struct TranslationOptions {
    /// Contains an ISO 639 language code. If None, source language is guessed
    pub source: Option<String>,
    /// Contains an ISO 639 language code. If None, no translation.
    pub target: Option<String>,
    /// DeepL API key
    pub deepl_key: Option<String>,
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

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn create_reference(parse_info: &ParseInfo, options: &GenerationOptions) -> GenerationResult<Reference> {
    // Build attribute collection based on configuration
    let attributes = AttributeCollection::initialize(&options.attribute_config, parse_info);

    let title = attributes.get(AttributeType::Title).cloned();
    let author = attributes.get(AttributeType::Author).cloned();
    let date = attributes.get(AttributeType::Date).cloned();
    let language = attributes.get(AttributeType::Locale).cloned();
    let site = attributes.get(AttributeType::Site).cloned();
    let url = attributes.get(AttributeType::Url).cloned()
        .or(parse_info.url.map(|x| Attribute::Url(x.to_string()))); // If no URL collected, attempt to use user-supplied URL
    let publisher = attributes.get(AttributeType::Publisher).cloned();

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
    // If title parameter is actually an Attribute::Title,
    // proceed with translation. Otherwise, throw an error.
    if let Some(Attribute::Title(content)) = title {
        let text = translate(content, &options)?;
        let translation_attribute = Attribute::TranslatedTitle(Translation {
            text,
            // We can safely unwrap here as the call to translate()
            // would've already failed if no target language was provided.
            language: options.target.clone().unwrap(),
        });
        Ok(translation_attribute)
    } else {
        Err(ReferenceGenerationError::TranslationError)
    }
}

/// Translates content according to the provided TranslationOptions.
fn translate<'a>(content: &'a str, options: &TranslationOptions) -> GenerationResult<String> {
    let api_key = options.deepl_key.clone().ok_or(ReferenceGenerationError::TranslationError)?;
    let deepl = DeepL::new(api_key);

    let texts = TranslatableTextList {
        source_language: options.source.clone(),
        target_language: options
            .target
            .clone()
            .ok_or(ReferenceGenerationError::TranslationError)?,
        texts: vec![content.to_string()],
    };

    let translated = deepl.translate(None, texts)?;
    Ok(translated[0].text.to_owned())
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
    
    // Make a request to save the page
    // The response will redirect to the archived page
    let _response = curl::get(&save_url, None, true)?;
    
    // After saving, fetch the latest snapshot which should be the one we just created
    // Wait a moment for the archive to be indexed, then query for it
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Query for the newly created archive
    call_wayback_api(url, &None)
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
        let expected = vec![MetadataType::OpenGraph, MetadataType::SchemaOrg];
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
        
        // Timestamp is difficult to test for, so it is not needed for now.
        let (url_result, _) = fetch_archive_info(&url_attribute, &archive_options);
        
        let expected_archive_url = "http://web.archive.org/web/20211026003805/https://www.information.dk/kultur/2018/01/casper-mandrilaftalen-burde-lade-goere-gjorde";
        let expected_archive_url_attribute = Some(Attribute::ArchiveUrl(expected_archive_url.to_string()));
        
        assert_eq!(url_result, expected_archive_url_attribute);
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
