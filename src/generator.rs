//! Generator responsible for producing a [`Reference`]

use std::result;

use deepl_api::{DeepL, TranslatableTextList, Error as DeepLError};
use strum::IntoEnumIterator;
use thiserror::Error;
use webpage::HTML;

use crate::attribute::{Attribute, AttributeType, Translation};
use crate::parser::{parse_html_from_file, parse_html_from_url, AttributeCollection, MetadataType};
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
#[derive(Default)]
pub struct TranslationOptions {
    /// Contains an ISO 639 language code. If None, source language is guessed
    pub source: Option<String>,
    /// Contains an ISO 639 language code. If None, no translation.
    pub target: Option<String>,
    /// DeepL API key
    pub deepl_key: Option<String>,
}

pub struct AttributeConfig {
    pub attribute_type: AttributeType,
    pub priority: i32,
}

pub struct RecipeOptions {
    pub list: Vec<AttributeConfig>,
    pub meta_data_type: MetadataType,
}

impl RecipeOptions {
    fn default_list() -> Vec<AttributeConfig> {
        AttributeType::iter()
            .map(|attribute_type| AttributeConfig { attribute_type, priority: 1 })
            .collect()
    }

    pub fn default_opengraph() -> RecipeOptions {
        RecipeOptions {
            list: Self::default_list(),
            meta_data_type: MetadataType::OpenGraph,
        }
    }

    pub fn default_schema_org() -> RecipeOptions {
        RecipeOptions {
            list: Self::default_list(),
            meta_data_type: MetadataType::SchemaOrg,
        }
    }
}

fn form_reference_from_url(url: &str, options: &GenerationOptions) -> Result<Reference> {
    let html = parse_html_from_url(url)?;
    form_reference(&html, options)
}

fn form_reference_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    let html = parse_html_from_file(path)?;
    form_reference(&html, options)
}


/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn form_reference(html: &HTML, options: &GenerationOptions) -> Result<Reference> {
    // Generate attributes
    let mut attribute_collection = AttributeCollection::new();

    for attribute_config_list in options.recipes.iter() {
        attribute_collection = attribute_collection.build(attribute_config_list, &html);
    }

    let title = attribute_collection.get_as_attribute(AttributeType::Title);
    let author = attribute_collection.get_as_attribute(AttributeType::Author);
    let date = attribute_collection.get_as_attribute(AttributeType::Date);
    let language = attribute_collection.get_as_attribute(AttributeType::Locale);
    let site = attribute_collection.get_as_attribute(AttributeType::Site);
    let url_attrib = attribute_collection.get_as_attribute(AttributeType::Url);

    // Act according to translation options;
    // if translation fails, None will be the result.
    let translated_title = translate_title(&title, &options.translation_options).ok();

    let reference = Reference::NewsArticle {
        title: title.cloned(),
        translated_title,
        author: author.cloned(),
        date: date.cloned(),
        language: language.cloned(),
        url: url_attrib.cloned(),
        site: site.cloned(),
    };

    Ok(reference)
}

/// Generate a [`Reference`] from a URL string.
pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference_from_url(url, options);

    reference
}

/// Generate a [`Reference`] from a raw HTML string read from a file.
pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference_from_file(path, options);

    reference
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