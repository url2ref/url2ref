//! Generator responsible for producing a [`Reference`]

use chrono::{NaiveDate, DateTime};
use thiserror::Error;

use crate::attribute::{AttributeConfigList, Attribute, InternalAttributeKey};
use crate::parser::{AttributeCollection, parse_html};
use crate::reference::Reference;
use crate::GenerationOptions;

/// Errors encountered during reference generation are
/// wrapped in this enum.
#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("URL failed to parse")]
    URLParseError(#[from] std::io::Error),
}

fn parse_date(date_string: Option<String>) -> Option<NaiveDate> {
    // TODO: make this more clean (one liner?)

    let date_time = match date_string {
        Some(date) => DateTime::parse_from_rfc3339(&date as &str).ok(), // Option<DateTime>
        None => None
    };
   
    match date_time {
        Some(dt) => Some(dt.date_naive()),
        None => None
    }
}

fn value_from_attribute(attribute_option: Option<&Attribute>) -> Option<String> {
    match attribute_option {
        Some(attribute) => Some(attribute.value.clone()),
        None => None
    }
}

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
fn form_reference(url: &str, recipes: Vec<AttributeConfigList>) -> Result<Reference, ReferenceGenerationError> {
    let mut attribute_collection = AttributeCollection::new();
    let html = parse_html(url)?;
    
    for attribute_config_list in recipes.iter() {
        attribute_collection = attribute_collection.build(attribute_config_list, &html);
    }
    
    let title = attribute_collection.get(InternalAttributeKey::Title);
    let author = attribute_collection.get(InternalAttributeKey::Author);
    let date = attribute_collection.get(InternalAttributeKey::Date);
    let language = attribute_collection.get(InternalAttributeKey::Locale);
    let site = attribute_collection.get(InternalAttributeKey::Site);
    let url_attrib = attribute_collection.get(InternalAttributeKey::Url);

    let reference = Reference::NewsArticle { 
        title: value_from_attribute(title),
        author: value_from_attribute(author),
        date: parse_date(value_from_attribute(date)),
        language: value_from_attribute(language),
        url: value_from_attribute(url_attrib),
        site: value_from_attribute(site),
    };
    
    println!("{:?}", reference);
    Ok(reference)
}

/// Generate a [`Reference`] from a URL string.
pub fn generate(
    url: &str,
    options: GenerationOptions,
) -> Result<Reference, ReferenceGenerationError> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference(url, options.recipes);

    reference
}
