//! Generator responsible for producing a [`Reference`]

use strum::IntoEnumIterator;
use thiserror::Error;
use webpage::HTML;

use crate::attribute::AttributeType;
use crate::parser::{parse_html_from_file, parse_html_from_url, AttributeCollection, MetadataType};
use crate::reference::Reference;
use crate::GenerationOptions;

/// Errors encountered during reference generation are
/// wrapped in this enum.
#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("URL failed to parse")]
    URLParseError(#[from] std::io::Error),
}

pub struct AttributeConfig {
    pub attribute_type: AttributeType,
    pub priority: i32,
}

pub struct AttributeConfigList {
    pub list: Vec<AttributeConfig>,
    pub meta_data_type: MetadataType,
}

impl AttributeConfigList {
    fn default_list() -> Vec<AttributeConfig> {
        AttributeType::iter()
            .map(|at| AttributeConfig {
                attribute_type: at,
                priority: 1,
            })
            .collect()
    }

    pub fn default_opengraph() -> AttributeConfigList {
        AttributeConfigList {
            list: Self::default_list(),
            meta_data_type: MetadataType::OpenGraph,
        }
    }

    pub fn default_schema_org() -> AttributeConfigList {
        AttributeConfigList {
            list: Self::default_list(),
            meta_data_type: MetadataType::SchemaOrg,
        }
    }
}

fn form_reference_from_url(
    url: &str,
    recipes: Vec<AttributeConfigList>,
) -> Result<Reference, ReferenceGenerationError> {
    let html = parse_html_from_url(url)?;
    form_reference(&html, recipes)
}

fn form_reference_from_file(
    path: &str,
    recipes: Vec<AttributeConfigList>,
) -> Result<Reference, ReferenceGenerationError> {
    let html = parse_html_from_file(path)?;
    form_reference(&html, recipes)
}


/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn form_reference(
    html: &HTML,
    recipes: Vec<AttributeConfigList>,
) -> Result<Reference, ReferenceGenerationError> {
    let mut attribute_collection = AttributeCollection::new();

    for attribute_config_list in recipes.iter() {
        attribute_collection = attribute_collection.build(attribute_config_list, &html);
    }

    let title = attribute_collection.get_as_attribute(AttributeType::Title);
    let author = attribute_collection.get_as_attribute(AttributeType::Author);
    let date = attribute_collection.get_as_attribute(AttributeType::Date);
    let language = attribute_collection.get_as_attribute(AttributeType::Locale);
    let site = attribute_collection.get_as_attribute(AttributeType::Site);
    let url_attrib = attribute_collection.get_as_attribute(AttributeType::Url);

    let reference = Reference::NewsArticle {
        title: title.cloned(),
        author: author.cloned(),
        date: date.cloned(),
        language: language.cloned(),
        url: url_attrib.cloned(),
        site: site.cloned(),
    };

    Ok(reference)
}

/// Generate a [`Reference`] from a URL string.
pub fn generate(
    url: &str,
    options: GenerationOptions,
) -> Result<Reference, ReferenceGenerationError> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference_from_url(url, options.recipes);

    reference
}

/// Generate a [`Reference`] from a raw HTML string read from a file.
pub fn generate_from_file(
    path: &str,
    options: GenerationOptions,
) -> Result<Reference, ReferenceGenerationError> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference_from_file(path, options.recipes);

    reference
}