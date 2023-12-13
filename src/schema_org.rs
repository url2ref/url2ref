pub mod generic;
pub mod author;
pub mod site;

use generic::create_generic_attribute;
use author::create_author_attribute;
use site::create_site_attribute;


use std::collections::HashMap;

use strum::IntoEnumIterator;
use webpage::HTML;
use serde_json::Value;


use crate::parser::{AttributeParser, MetadataKey};
use crate::attribute::{Attribute, AttributeType};

/// Mapping from generic [`InternalAttributeKey`] to Schema.org-specific
/// [`AttributeKey`] instances.
pub const fn keys(key: AttributeType) -> &'static [MetadataKey] {
    match key {
        AttributeType::Title    => &[MetadataKey{key: "headline"}, 
                                     MetadataKey{key: "alternativeHeadline"}],
        AttributeType::Author   => &[MetadataKey{key: "author"}],
        AttributeType::Language => &[MetadataKey{key: "inLanguage"}],
        AttributeType::Site     => &[MetadataKey{key: "publisher"},
                                     MetadataKey{key: "sourceOrganization"}], 
        AttributeType::Url      => &[MetadataKey{key: "mainEntityOfPage"},  
                                     MetadataKey{key: "url"}],
        AttributeType::Date     => &[MetadataKey{key: "datePublished"}, 
                                     MetadataKey{key: "dateModified"}],
        AttributeType::Type     => &[MetadataKey{key: "@type"}],
        _                       => &[],
    }
}


pub struct SchemaOrg;

impl AttributeParser for SchemaOrg {

    fn parse_attributes(html: &HTML) -> HashMap<AttributeType, Attribute> {
        let mut parsed_schema = HashMap::new(); 

        // TODO: don't assume that the first object is the correct entity
        let schema = &html.schema_org[0];
        let schema_json: &Value = &schema.value; 
        println!("{:?}", schema_json);

        for attribute_type in AttributeType::iter() {
            let external_keys = keys(attribute_type);

            // Some fields require explicit handling because of nested structures.
            let attribute_option = match attribute_type {
                AttributeType::Author => create_author_attribute(&schema_json, external_keys),
                AttributeType::Site => create_site_attribute(&schema_json, external_keys),
                _ => create_generic_attribute(&schema_json, external_keys, attribute_type),
            };

            Self::insert_if_some(&mut parsed_schema, attribute_type, attribute_option);
        }

        println!("{:?}", parsed_schema);
        parsed_schema
    }
}
