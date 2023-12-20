//! Generator responsible for producing a [`Reference`]

use std::result;
use strum::{EnumIter, EnumCount};
use deepl_api::{DeepL, TranslatableTextList, Error as DeepLError};
use thiserror::Error;

use crate::attribute::{Attribute, AttributeType, Translation};
use crate::doi::DoiError;
use crate::parser::{AttributeCollection, ParseInfo};
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

    #[error("Retrieving DOI failed")]
    DoiError(#[from] DoiError),
}

#[derive(Default, Clone, Copy, PartialEq, EnumIter, EnumCount, Eq, Hash)]
pub enum MetadataType {
    #[default]
    OpenGraph,
    SchemaOrg,
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

pub mod attribute_config {
    use derive_builder::Builder;

    use super::MetadataType;
    use crate::attribute::AttributeType;

    #[derive(Clone)]
    pub struct AttributePriority {
        pub priority: Vec<MetadataType>
    }
    impl Default for AttributePriority {
        fn default() -> Self {
            Self {
                priority: vec!(MetadataType::SchemaOrg, MetadataType::OpenGraph),
            }
        }
    }
    impl AttributePriority {
        pub fn new(priority: &[MetadataType]) -> Self {
            Self {
                priority: priority.to_vec()
            }
        }
    }

    #[derive(Default, Builder, Clone)]
    #[builder(setter(into, strip_option), default)]
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
        pub institution: Option<AttributePriority>,
        pub volume: Option<AttributePriority>,
    }

    impl AttributeConfig {
        pub fn new(priority: AttributePriority) -> Self {
            AttributeConfigBuilder::default()
                .title(priority.clone())
                .authors(priority.clone())
                .date(priority.clone())
                .language(priority.clone())
                .locale(priority.clone())
                .site(priority.clone())
                .url(priority.clone())
                .journal(priority.clone())
                .publisher(priority.clone())
                .build()
                .unwrap()
        }

        pub fn get(&self, attribute_type: AttributeType) -> &Option<AttributePriority> {
            match attribute_type {
                AttributeType::Title       => &self.title,
                AttributeType::Author      => &self.authors,
                AttributeType::Date        => &self.date,
                AttributeType::Language    => &self.language,
                AttributeType::Locale      => &self.locale,
                AttributeType::Site        => &self.site,
                AttributeType::Url         => &self.url,
                AttributeType::Type        => &None, // TODO: Decide future of AttributeType::Type
                AttributeType::Journal     => &self.journal,
                AttributeType::Publisher   => &self.publisher,
                AttributeType::Volume      => &self.volume,
                AttributeType::Institution => &self.institution,
            }
        }
    }
}

/// Generates a [`Reference`] from a URL.
pub fn from_url(url: &str, options: &GenerationOptions) -> Result<Reference> {
    let parse_info = ParseInfo::from_url(url)?;
    create_reference(&parse_info, &options)
}

/// Generates a [`Reference`] from raw HTML as read from a file.
pub fn from_file(html_path: &str, options: &GenerationOptions) -> Result<Reference> {
    let parse_info = ParseInfo::from_file(html_path)?;
    create_reference(&parse_info, &options)
}

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn create_reference(parse_info: &ParseInfo, options: &GenerationOptions) -> Result<Reference> {
    // Build attribute collection based on configuration
    let attribute_collection = AttributeCollection::initialize(&options.attribute_config, parse_info);

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