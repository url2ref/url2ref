use std::collections::HashMap;

use strum::IntoEnumIterator;
use webpage::HTML;
use serde_json::{Value, Map};

use crate::parser::{AttributeParser, MetadataKey, parse_date};
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



fn try_find_generic_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<String> {
    for external_key in external_keys.iter() {
        
        let found_option = match &schema_value[external_key.key] {
            Value::String(string) => Some(string),
            _ => None
        };

        if let Some(_) = found_option {
            return found_option.cloned()
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


fn create_generic_attribute(schema_value: &Value, external_keys: &[MetadataKey], attribute_type: AttributeType) -> Option<Attribute> {
    let attribute_option = try_find_generic_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return attribute_type_to_attribute(attribute_type, attribute_value);
    }
    
    None
}

fn try_find_author_array_of_persons_stategy(value_list: &Vec<Value>) -> Option<Vec<String>> {
    let mut ret = Vec::new();
    println!("{:?}", value_list);
    for value in value_list {
        let name_option = match value {
            Value::Object(person) => {
                match &person["name"] {
                    Value::String(name) => Some(name.clone()),
                    _ => None
                }
            },
            _ => None
        };

        if let Some(name) = name_option {
            ret.push(name);
        }
        
    }

    println!("{:?}", ret);

    if ret.is_empty() {
        return None
    }
    
    Some(ret)
}

fn try_find_author_map_of_persons(value_map: &Map<String, Value>) -> Option<Vec<String>> {
    let person_array = &value_map["name"];
    
    let ret : Option<Vec<String>> = match person_array {
        Value::Array(value_list) => {
            let mut inner: Vec<String> = Vec::new();         
            
            for value in value_list {
                let name_option = match value {
                    Value::String(name) => Some(name.clone()),
                    _ => None
                };
                
                if let Some(name) = name_option {
                    inner.push(name);
                }

            }
            
            
            return Some(inner)
        },
        _ => None
    };
    
    println!("{:?}", ret);
    
    ret
}

fn try_find_author_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<Vec<String>> {
    
    for external_key in external_keys.iter() {
        let value = &schema_value[external_key.key];
        let found_option = match value {
            Value::Array(value_list) => try_find_author_array_of_persons_stategy(&value_list),
            Value::Object(value_map) => try_find_author_map_of_persons(&value_map),
            _ => None
        };

        if let Some(_) = found_option {
            return found_option
        }
    }

    None
    
}

fn create_author_attribute(schema_value: &Value, external_keys: &[MetadataKey]) -> Option<Attribute> {
    let attribute_option = try_find_author_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return Some(Attribute::Author(attribute_value))
    }
    
    None
}

fn try_find_site_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<String> {
    for external_key in external_keys.iter() {
        let value = &schema_value[external_key.key];
        let found_option = match value {
            Value::Object(value_map) => {
                let name_value = &value_map["name"];
                match name_value {
                    Value::String(name) => Some(name.clone()),
                    _ => None
                }
            }
            _ => None
        };
        
        if let Some(_) = found_option {
            return found_option
        }
        
    }
    None
}


fn create_site_attribute(schema_value: &Value, external_keys: &[MetadataKey], attribute_type: AttributeType) -> Option<Attribute> {
    let attribute_option = try_find_site_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return attribute_type_to_attribute(attribute_type, attribute_value);
    }
    
    None
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

            let attribute_option = match attribute_type {
                AttributeType::Author => create_author_attribute(&schema_json, external_keys),
                AttributeType::Site => create_site_attribute(&schema_json, external_keys, attribute_type),
                _ => create_generic_attribute(&schema_json, external_keys, attribute_type),
            };

            if let Some(attribute) = attribute_option {
                parsed_schema.insert(attribute_type, attribute);
            }
        }

        println!("{:?}", parsed_schema);
        parsed_schema
    }
}
