use crate::attribute::{Attribute, AttributeType};
use crate::parser::parse_date;
use crate::schema_org::MetadataKey;

use serde_json::Value;

fn try_find_generic_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
) -> Option<String> {
    for external_key in external_keys.iter() {
        let found_option = match &schema_value[external_key.key] {
            Value::String(string) => Some(string),
            _ => None,
        };

        if let Some(_) = found_option {
            return found_option.cloned();
        }
    }

    None
}

pub fn attribute_type_to_attribute(
    attribute_type: AttributeType,
    attribute_value: String,
) -> Option<Attribute> {
    match attribute_type {
        AttributeType::Title => Some(Attribute::Title(attribute_value)),
        AttributeType::Author => panic!("Author should have been handled by specialized method"),
        AttributeType::Date => {
            let date_option = parse_date(&attribute_value);
            match date_option {
                Some(date) => Some(Attribute::Date(date)),
                None => None,
            }
        }
        AttributeType::Locale => Some(Attribute::Locale(attribute_value)),
        AttributeType::Language => Some(Attribute::Language(attribute_value)),
        AttributeType::Site => panic!("Site should have been handled by specialized method"),
        AttributeType::Url => Some(Attribute::Url(attribute_value)),
        _ => None,
    }
}

pub fn create_generic_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey],
    attribute_type: AttributeType,
) -> Option<Attribute> {
    let attribute_option = try_find_generic_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return attribute_type_to_attribute(attribute_type, attribute_value);
    }

    None
}
