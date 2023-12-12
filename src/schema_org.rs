use std::collections::HashMap;

use webpage::HTML;

use crate::parser::{AttributeParser, MetadataKey};
use crate::attribute::{Attribute, AttributeType};


/// Mapping from generic [`InternalAttributeKey`] to Schema.org-specific
/// [`AttributeKey`] instances.
pub const fn keys(key: AttributeType) -> &'static [MetadataKey] {
    match key {
        AttributeType::Title    => &[MetadataKey{key: "headline"}, 
                                     MetadataKey{key: "alternativeHeadline"}],
        AttributeType::Author   => &[MetadataKey{key: "article:author"}], // TODO
        AttributeType::Language => &[MetadataKey{key: "inLanguage"}],
        AttributeType::Site     => &[MetadataKey{key: "site_name"}], // TODO
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
        let parsed_schema = HashMap::new(); 

        // TODO: don't assume that the first object is the correct entity
        let schema = &html.schema_org[0];
        let _schema_json = &schema.value; 

        
        parsed_schema
    }
}
