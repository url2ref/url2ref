//! Parser which extracts the metadata to be combined into the final [`Reference`]

use std::collections::HashMap;

use crate::attribute::{
    keys_from_type, Attribute, AttributeConfigList, AttributeKey, InternalAttributeKey,
    MetaDataType,
};
use crate::generator::ReferenceGenerationError;
use webpage::{Webpage, WebpageOptions, HTML};

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html(url: &str) -> Result<HTML, ReferenceGenerationError> {
    let html = Webpage::from_url(url, WebpageOptions::default())?.html;
    Ok(html)
}

#[derive(Clone)]
pub struct AttributeCollection {
    pub attributes: HashMap<InternalAttributeKey, Attribute>,
}
impl AttributeCollection {
    pub fn new() -> AttributeCollection {
        AttributeCollection {
            attributes: HashMap::new(),
        }
    }

    fn insert(&mut self, internal_key: InternalAttributeKey, attribute: Attribute) {
        self.attributes.insert(internal_key, attribute);
    }

    pub fn get(&self, internal_key: InternalAttributeKey) -> Option<&Attribute> {
        self.attributes.get(&internal_key)
    }

    fn contains(&self, internal_key: InternalAttributeKey) -> bool {
        self.attributes.contains_key(&internal_key)
    }

    fn extract_attributes_opengraph(&self, html: &HTML) -> HashMap<String, String> {
        println!("{:?}", html.opengraph.properties);
        html.opengraph.properties.clone()
    }

    fn extract_attributes_schema_org(&self, _html: &HTML) -> HashMap<String, String> {
        todo!();
    }

    // TODO: Implement actual builder pattern without copy
    pub fn build(&self, config_list: &AttributeConfigList, html: &HTML) -> AttributeCollection {
        let attributes = match config_list.meta_data_type {
            MetaDataType::OpenGraph => self.extract_attributes_opengraph(html),
            MetaDataType::SchemaOrg => self.extract_attributes_schema_org(html),
        };

        let mut return_collection = self.clone();

        let meta_data_type = config_list.meta_data_type;

        for attribute_config in config_list.list.iter() {
            let internal_key = attribute_config.internal_key;
            let external_keys = keys_from_type(internal_key, meta_data_type);

            let attribute_option = self.try_find_attribute(external_keys, &attributes);
            let priority = attribute_config.priority;

            if let Some(attribute_value) = attribute_option {
                let attribute = Attribute {
                    value: attribute_value,
                    priority: priority,
                };

                return_collection.insert_if(internal_key, attribute);
            };
        }

        return_collection
    }

    fn try_find_attribute(
        &self,
        external_keys: &[AttributeKey],
        attributes: &HashMap<String, String>,
    ) -> Option<String> {
        for external_key in external_keys.iter() {
            if attributes.contains_key(external_key.key) {
                return attributes.get(external_key.key).cloned();
            }
        }

        None
    }

    fn insert_if(&mut self, internal_key: InternalAttributeKey, attribute: Attribute) {
        if !self.contains(internal_key) {
            self.insert(internal_key, attribute);
            return;
        }

        let found_attribute = self.get(internal_key).unwrap();
        if attribute.priority > found_attribute.priority {
            self.insert(internal_key, attribute);
        }
    }
}
