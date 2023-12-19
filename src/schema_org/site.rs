use crate::attribute::Attribute;
use crate::schema_org::MetadataKey;

use serde_json::Value;

fn try_find_site_attribute(schema_value: &Value, external_keys: &[MetadataKey]) -> Option<String> {
    for external_key in external_keys.iter() {
        let value = &schema_value[external_key.key];
        let found_option = match value {
            Value::Object(value_map) => {
                let name_value = &value_map["name"];
                match name_value {
                    Value::String(name) => Some(name.clone()),
                    _ => None,
                }
            }
            _ => None,
        };

        if let Some(_) = found_option {
            return found_option;
        }
    }
    None
}

pub fn create_site_attribute(
    schema_value: &Value,
    external_keys: &[MetadataKey]
) -> Option<Attribute> {
    let attribute_option = try_find_site_attribute(&schema_value, external_keys);
    if let Some(attribute_value) = attribute_option {
        return Some(Attribute::Site(attribute_value))
    }

    None
}
