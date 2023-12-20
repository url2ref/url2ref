//! Strategies for parsing [`Attribute::Author`].


use crate::attribute::{Attribute, Author};
use crate::schema_org::MetadataKey;

use serde_json::Value;


fn match_author_type(author_type: &String, name: &String) -> Option<Author> {
    match author_type.as_str() {
        "Person" => Some(Author::Person(name.clone())),
        "Organization" => Some(Author::Organization(name.clone())),
        _ => None
    }
}


fn match_tuple(object_type: &Value, name_value: &Value) -> Option<Author> {
    match (object_type, name_value) {
        (Value::String(author_type), Value::String(name)) => match_author_type(author_type, name),
        (_, _) => None
    }
}


fn try_find_author_array_of_persons_stategy(value_list: &Vec<Value>) -> Option<Vec<Author>> {
    let mut ret = Vec::new();
    for value in value_list {
        match value {
            Value::Object(map) => {
                let object_type = &map["@type"];
                let name_value = &map["name"];

                let author_option = match_tuple(object_type, name_value);

                if let Some(author) = author_option {
                    ret.push(author);
                }

            },
            _ => todo!()
        }
    }

    if ret.is_empty() {
        return None
    }

    Some(ret)
}


fn try_find_author_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<Vec<Author>> {
    for external_key in external_keys.iter() {
        let value = &schema_value[external_key.key];
        let found_option = match value {
            Value::Array(value_list) => try_find_author_array_of_persons_stategy(&value_list),
            Value::Object(_) => None, // -> Person, Organization
            _ => None,
        };

        if found_option.is_some() {
            return found_option
        }
    }

    None
}

pub fn create_author_attribute(schema_value: &Value, external_keys: &[MetadataKey]) -> Option<Attribute> {
    let attribute_option = try_find_author_attribute(&schema_value, external_keys)?;
    Some(Attribute::Authors(attribute_option))
}