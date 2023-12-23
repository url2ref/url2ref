pub mod generic;
pub mod author;
pub mod site;

use generic::create_generic_attribute;
use author::create_author_attribute;
use site::create_site_attribute;

use serde_json::Value;

use crate::parser::{AttributeParser, MetadataKey, ParseInfo};
use crate::attribute::{Attribute, AttributeType};


/// Mapping from generic [`AttributeType`] to Schema.org-specific
/// [`MetadataKey`] instances.
#[rustfmt::skip]
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

    fn parse_attribute(parse_info: &ParseInfo, attribute_type: AttributeType) -> Option<Attribute> {
        let html = parse_info.html.as_ref()?;
        let schema = html.schema_org.get(0)?;
        let schema_json: &Value = &schema.value;

        let external_keys = keys(attribute_type);

        // Some fields require explicit handling because of nested structures.
        match attribute_type {
            AttributeType::Author => create_author_attribute(&schema_json, external_keys),
            AttributeType::Site => create_site_attribute(&schema_json, external_keys),
            _ => create_generic_attribute(&schema_json, external_keys, attribute_type),
        }
    }
}
