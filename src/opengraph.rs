use std::collections::HashMap;

use webpage::HTML;

use crate::attribute::{Attribute, AttributeType, Author};
use crate::parser::MetadataKey;
use crate::parser::{parse_date, AttributeParser};

/// Mapping from generic [`AttributeType`] to Open Graph-specific
/// [`MetadataKey`] instances.
#[rustfmt::skip]
pub const fn keys(key: AttributeType) -> &'static [MetadataKey] {
    match key {
        AttributeType::Title    => &[MetadataKey{key: "title"}],
        AttributeType::Author   => &[MetadataKey{key: "article:author"}],
        AttributeType::Locale   => &[MetadataKey{key: "locale"}],
        AttributeType::Site     => &[MetadataKey{key: "site_name"}],
        AttributeType::Url      => &[MetadataKey{key: "url"}],
        AttributeType::Date     => &[MetadataKey{key: "article:published_time"},
                                     MetadataKey{key: "article:modified_time"},
                                     MetadataKey{key: "updated_time"}],
        AttributeType::Type     => &[MetadataKey{key: "type"}],
        _                       => &[],
    }
}

pub struct OpenGraph;

fn try_find_attribute(
    og: &HashMap<String, String>,
    external_keys: &[MetadataKey],
) -> Option<String> {
    for external_key in external_keys.iter() {
        if og.contains_key(external_key.key) {
            return og.get(external_key.key).cloned();
        }
    }

    None
}

fn attribute_type_to_attribute(
    attribute_type: AttributeType,
    attribute_value: String,
) -> Option<Attribute> {
    match attribute_type {
        AttributeType::Title => Some(Attribute::Title(attribute_value)),
        AttributeType::Author => {
            let author = Author::Generic(attribute_value);
            Some(Attribute::Authors(vec![author]))
        }
        AttributeType::Date => {
            let date = parse_date(&attribute_value)?;
            Some(Attribute::Date(date))
        }
        AttributeType::Locale => Some(Attribute::Locale(attribute_value)),
        AttributeType::Language => Some(Attribute::Language(attribute_value)),
        AttributeType::Site => Some(Attribute::Site(attribute_value)),
        AttributeType::Url => Some(Attribute::Url(attribute_value)),
        AttributeType::Type => None,
    }
}

impl AttributeParser for OpenGraph {
    fn parse_attribute(html: &HTML, attribute_type: AttributeType) -> Option<Attribute> {
        let og = &html.opengraph.properties;
        let external_keys = keys(attribute_type);
        let attribute_value = try_find_attribute(&og, external_keys)?;

        attribute_type_to_attribute(attribute_type, attribute_value)
    }
}
