/// Strategies for parsing [`Attribute`]::Author.


use crate::attribute::Attribute;
use crate::schema_org::MetadataKey;

use serde_json::{Map, Value};


fn try_find_author_array_of_persons_stategy(value_list: &Vec<Value>) -> Option<Vec<String>> {
    let mut ret = Vec::new();
    //println!("{:?}", value_list);
    for value in value_list {
        let name_option = match value {
            Value::Object(person) => match &person["name"] {
                Value::String(name) => Some(name.clone()),
                _ => None,
            },
            _ => None,
        };

        if let Some(name) = name_option {
            ret.push(name);
        }
    }

    //println!("{:?}", ret);

    if ret.is_empty() {
        return None;
    }

    Some(ret)
}

fn try_find_author_map_of_persons(value_map: &Map<String, Value>) -> Option<Vec<String>> {
    let person_array = &value_map["name"];

    let ret: Option<Vec<String>> = match person_array {
        Value::Array(value_list) => {
            let mut inner: Vec<String> = Vec::new();

            for value in value_list {
                let name_option = match value {
                    Value::String(name) => Some(name.clone()),
                    _ => None,
                };

                if let Some(name) = name_option {
                    inner.push(name);
                }
            }

            return Some(inner);
        }
        _ => None,
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
            _ => None,
        };

        if let Some(_) = found_option {
            return found_option;
        }
    }

    None
}

pub fn create_author_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<Attribute> {
    let attribute_option = try_find_author_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return Some(Attribute::Author(attribute_value));
    }

    None
}