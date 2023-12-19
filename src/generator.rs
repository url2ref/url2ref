//! Generator responsible for producing a [`Reference`]

use std::result;

use deepl_api::{DeepL, TranslatableTextList, Error as DeepLError};
use thiserror::Error;
use webpage::HTML;

use crate::attribute::{Attribute, AttributeType, Translation};
use crate::parser::{parse_html_from_file, parse_html_from_url, AttributeCollection};
use crate::reference::Reference;
use crate::GenerationOptions;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Errors encountered during reference generation are
/// wrapped in this enum.
#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("URL failed to parse")]
    URLParseError(#[from] std::io::Error),
    #[error("DeepL translation failed")]
    DeepLError(#[from] DeepLError),
    #[error("Title translation procedure failed")]
    TranslationError,
    #[error("Environment variable not found")]
    VarError(#[from] std::env::VarError),
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

pub mod attribute_config {
    use derive_builder::Builder;
    use strum::EnumCount;

    use crate::{parser::MetadataType, attribute::AttributeType};

    #[derive(Clone)]
    pub struct AttributePriority {
        pub priority: [MetadataType; MetadataType::COUNT], // TODO: Re-think this type
        count: i32
    }
    impl Default for AttributePriority {
        fn default() -> Self {
            Self {
                priority: [MetadataType::SchemaOrg, MetadataType::OpenGraph],
                count: MetadataType::COUNT as i32
            }
        }
    }
    impl AttributePriority {
        pub fn and_then(mut self, metadata_type: MetadataType) -> Self {
            self.priority[self.count as usize] = metadata_type;
            self.count += 1;
             
            self
        }
    }

    #[derive(Default, Builder, Clone)]
    #[builder(setter(into, strip_option))]
    pub struct AttributeConfig {
        pub title: Option<AttributePriority>,
        pub authors: Option<AttributePriority>,
        pub date: Option<AttributePriority>,
        pub language: Option<AttributePriority>,
        pub locale: Option<AttributePriority>,
        pub site: Option<AttributePriority>,
        pub url: Option<AttributePriority>,
        pub journal: Option<AttributePriority>,
        pub publisher: Option<AttributePriority>,
    }
    
    type P = Option<AttributePriority>;
    impl AttributeConfig {
        pub fn new(title: P, authors: P, date: P, language: P, locale: P, site: P, url: P, journal: P, publisher: P) -> Self {
            Self {
                title, authors, date, language, locale, site, url, journal, publisher
            }
        }

        pub fn get(&self, attribute_type: AttributeType) -> &Option<AttributePriority> {
            match attribute_type {
                AttributeType::Title    => &self.title,
                AttributeType::Author   => &self.authors,
                AttributeType::Date     => &self.date,
                AttributeType::Language => &self.language,
                AttributeType::Locale   => &self.locale,
                AttributeType::Site     => &self.site,
                AttributeType::Url      => &self.url,
                AttributeType::Type     => &None // TODO: Decide future of AttributeType::Type
            }
        }
    }
}

/// Generates a [`Reference`] from a URL.
pub fn from_url(url: &str, options: &GenerationOptions) -> Result<Reference> {
    let html = parse_html_from_url(url)?;
    create_reference(&html, &options)
}

/// Generates a [`Reference`] from raw HTML as read from a file.
pub fn from_file(html_path: &str, options: &GenerationOptions) -> Result<Reference> {
    let html = parse_html_from_file(html_path)?;
    create_reference(&html, &options)
}

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn create_reference(html: &HTML, options: &GenerationOptions) -> Result<Reference> {
    // Build attribute collection based on configuration
    let attribute_collection = AttributeCollection::initialize(&options.config, html);

    let title = attribute_collection.get(AttributeType::Title);
    let author = attribute_collection.get(AttributeType::Author);
    let date = attribute_collection.get(AttributeType::Date);
    let language = attribute_collection.get(AttributeType::Locale);
    let site = attribute_collection.get(AttributeType::Site);
    let url = attribute_collection.get(AttributeType::Url);

    // Act according to translation options;
    // if translation fails, None will be the result.
    let translated_title = translate_title(&title, &options.translation_options).ok();

    let reference = Reference::NewsArticle {
        title: title.cloned(),
        translated_title,
        author: author.cloned(),
        date: date.cloned(),
        language: language.cloned(),
        url: url.cloned(),
        site: site.cloned(),
    };

    Ok(reference)
}

/// Attempts to translate the provided [`Attribute::Title`].
/// Returns Option<[`Attribute::TranslatedTitle`]> on if successful and None otherwise.
fn translate_title(title: &Option<&Attribute>, options: &TranslationOptions) -> Result<Attribute> {
    // If title parameter is actually an Attribute::Title,
    // proceed with translation. Otherwise, throw an error.
    if let Some(Attribute::Title(content)) = title {
        let text = translate(content, &options)?;
        let translation_attribute = Attribute::TranslatedTitle(
            Translation { 
                text,
                // We can safely unwrap here as the call to translate()
                // would've already failed if no target language was provided.
                language: options.target.clone().unwrap()
            }
        );
        Ok(translation_attribute)
    } else {
        Err(ReferenceGenerationError::TranslationError)
    }
}

/// Translates content according to the provided TranslationOptions.
fn translate<'a>(content: &'a str, options: &TranslationOptions) -> Result<String> {     
    let api_key = options.deepl_key.clone().ok_or(ReferenceGenerationError::TranslationError)?;   
    let deepl = DeepL::new(api_key);

    let texts = TranslatableTextList {
        source_language: options.source.clone(),
        target_language: options.target.clone().ok_or(ReferenceGenerationError::TranslationError)?,
        texts: vec!(content.to_string()),
    };

    let translated = deepl.translate(None, texts)?;
    Ok(translated[0].text.to_owned())
}