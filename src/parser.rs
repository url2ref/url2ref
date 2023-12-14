//! Parser which extracts the metadata to be combined into the final [`Reference`]
use std::collections::HashMap;

use crate::attribute::{Attribute, AttributeType};
use crate::generator::{AttributeConfigList, ReferenceGenerationError};
use crate::opengraph::OpenGraph;
use crate::schema_org::SchemaOrg;
use chrono::{DateTime, NaiveDate};
use webpage::{Webpage, WebpageOptions, HTML};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct MetadataKey {
    pub key: &'static str,
}

#[derive(Clone, Copy)]
pub enum MetadataType {
    OpenGraph,
    SchemaOrg,
}

#[derive(Clone)]
pub struct StoredAttribute {
    pub value: Attribute,
    pub priority: i32,
}

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html_from_url(url: &str) -> Result<HTML, ReferenceGenerationError> {
    let html = Webpage::from_url(url, WebpageOptions::default())?.html;
    Ok(html)
}

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html_from_file(path: &str) -> Result<HTML, ReferenceGenerationError> {
    let html = HTML::from_file(path, None)?;
    Ok(html)
}

pub fn parse_date(date_string: String) -> Option<NaiveDate> {
    // TODO: make this more clean (one liner?)
    let date_time = DateTime::parse_from_rfc3339(&date_string as &str).ok();

    match date_time {
        Some(dt) => Some(dt.date_naive()),
        None => None,
    }
}

pub trait AttributeParser {
    fn parse_attributes(html: &HTML) -> HashMap<AttributeType, Attribute>;

    fn insert_if_some(
        map: &mut HashMap<AttributeType, Attribute>,
        attribute_type: AttributeType,
        attribute_option: Option<Attribute>,
    ) {
        if let Some(attribute) = attribute_option {
            map.insert(attribute_type, attribute);
        }
    }
}

#[derive(Clone)]
pub struct AttributeCollection {
    pub attributes: HashMap<AttributeType, StoredAttribute>,
}
impl AttributeCollection {
    pub fn new() -> AttributeCollection {
        AttributeCollection {
            attributes: HashMap::new(),
        }
    }

    fn insert(&mut self, internal_key: AttributeType, attribute: StoredAttribute) {
        self.attributes.insert(internal_key, attribute);
    }

    pub fn get(&self, internal_key: AttributeType) -> Option<&StoredAttribute> {
        self.attributes.get(&internal_key)
    }

    fn contains(&self, internal_key: AttributeType) -> bool {
        self.attributes.contains_key(&internal_key)
    }

    fn parse_attributes(
        &self,
        html: &HTML,
        meta_data_type: MetadataType,
    ) -> HashMap<AttributeType, Attribute> {
        match meta_data_type {
            MetadataType::OpenGraph => OpenGraph::parse_attributes(html),
            MetadataType::SchemaOrg => SchemaOrg::parse_attributes(html),
        }
    }

    // TODO: Implement actual builder pattern without copy
    pub fn build(&self, config_list: &AttributeConfigList, html: &HTML) -> AttributeCollection {
        let attributes = self.parse_attributes(html, config_list.meta_data_type);

        let mut return_collection = self.clone();

        for attribute_config in config_list.list.iter() {
            let attribute_type = attribute_config.attribute_type;

            let attribute_option = attributes.get(&attribute_type);
            let priority = attribute_config.priority;

            if let Some(attribute) = attribute_option {
                let attribute = StoredAttribute {
                    value: attribute.clone(),
                    priority: priority,
                };

                return_collection.insert_if(attribute_type, attribute);
            };
        }

        return_collection
    }

    fn insert_if(&mut self, internal_key: AttributeType, attribute: StoredAttribute) {
        if !self.contains(internal_key) {
            self.insert(internal_key, attribute);
            return;
        }

        let found_attribute = self.get(internal_key).unwrap();
        if attribute.priority > found_attribute.priority {
            self.insert(internal_key, attribute);
        }
    }

    pub fn get_as_attribute(&self, attribute_type: AttributeType) -> Option<&Attribute> {
        let attribute_option = self.get(attribute_type);
        match attribute_option {
            Some(attribute) => Some(&attribute.value),
            None => None,
        }
    }
}
