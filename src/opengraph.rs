use std::collections::HashMap;

use strum::IntoEnumIterator;
use webpage::HTML;

use crate::attribute::{Attribute, AttributeType};
use crate::parser::MetadataKey;
use crate::parser::{parse_date, AttributeParser};


/// Mapping from generic [`InternalAttributeKey`] to OpenGraph-specific
/// [`AttributeKey`] instances.
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
        AttributeType::Title  => Some(Attribute::Title(attribute_value)),
        AttributeType::Author => Some(Attribute::Author(vec![attribute_value])),
        AttributeType::Date   => {
            let date_option = parse_date(attribute_value);
            match date_option {
                Some(date) => Some(Attribute::Date(date)),
                None => None,
            }
        }
        AttributeType::Locale   => Some(Attribute::Locale(attribute_value)),
        AttributeType::Language => Some(Attribute::Language(attribute_value)),
        AttributeType::Site     => Some(Attribute::Site(attribute_value)),
        AttributeType::Url      => Some(Attribute::Url(attribute_value)),
        AttributeType::Type     => None,
    }
}

impl AttributeParser for OpenGraph {
    fn parse_attributes(html: &HTML) -> HashMap<AttributeType, Attribute> {
        let mut parsed_opengraph = HashMap::new();
        //println!("{:?}", html.opengraph.properties);

        let og = &html.opengraph.properties;

        for attribute_type in AttributeType::iter() {
            let external_keys = keys(attribute_type);
            let attribute_option = try_find_attribute(&og, external_keys);

            if let Some(attribute_value) = attribute_option {
                let attribute_option = attribute_type_to_attribute(attribute_type, attribute_value);

                Self::insert_if_some(&mut parsed_opengraph, attribute_type, attribute_option);
            }
        }

        //println!("{:?}", parsed_opengraph);
        parsed_opengraph
    }
}
