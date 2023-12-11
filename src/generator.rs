//! Generator responsible for producing a [`Reference`]

use thiserror::Error;

use crate::attribute::AttributeConfigList;
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

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
/// TODO: Add support for prioritization
/// such that certain Open Graph or Schema.org values
/// can be prioritized over the other, allowing for flexibility
/// and giving the user options (also as a way to decide tie breaks).


fn form_reference(url: &str, recipes: Vec<AttributeConfigList>) -> Reference {
    // Title
    // look up in config: any prios?
    //      -> OpenGraph, Schema ...
    // is there OpenGraph in results?
    // if yes, get hashmap from OpenGraph struct
    // lookup title in the hashmap
    // else try next priority
    // ...
    // if no values, then empty / None

    let mut attribute_collection = AttributeCollection::new();
    let html = parse_html(url).unwrap();
    
    for attribute_config_list in recipes.iter() {
        attribute_collection = attribute_collection.build(attribute_config_list, &html);
    }

    println!("Title: {:?}", attribute_collection.get(crate::attribute::InternalAttributeKey::Title).unwrap().value);
    println!("Author: {:?}", attribute_collection.get(crate::attribute::InternalAttributeKey::Author).unwrap().value);
    todo!()
}

/*
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
*/

/// Generate a [`Reference`] from a URL string.
pub fn generate(
    url: &str,
    options: GenerationOptions,
) -> Result<Reference, ReferenceGenerationError> {
    // Parse the HTML to gain access to Schema.org and Open Graph metadata
    let reference = form_reference(url, options.recipes);

    Ok(reference)
}
