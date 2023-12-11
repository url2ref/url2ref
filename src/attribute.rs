#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct AttributeKey {
    pub key: &'static str,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum InternalAttributeKey {
   Title, 
   Author, 
   Locale,
   Site,
   Date,
   Type
}

/// Mapping from generic [`InternalAttributeKey`] to OpenGraph-specific
/// [`AttributeKey`] instances.
pub mod opengraph {
    use super::AttributeKey;
    use super::InternalAttributeKey;

    pub const fn keys(key: InternalAttributeKey) -> &'static [AttributeKey] {
        match key {
            InternalAttributeKey::Title  => &[AttributeKey{key: "title"}],
            InternalAttributeKey::Author => &[AttributeKey{key: "article:author"}],
            InternalAttributeKey::Locale => &[AttributeKey{key: "locale"}],
            InternalAttributeKey::Site   => &[AttributeKey{key: "site_name"}],
            InternalAttributeKey::Date   => &[AttributeKey{key: "article:published_time"}, 
                                              AttributeKey{key: "article:modified_time"}],
            InternalAttributeKey::Type   => &[AttributeKey{key: "type"}],
        }
    }
}

/// Mapping from generic [`InternalAttributeKey`] to Schema.org-specific
/// [`AttributeKey`] instances.
pub mod schema_org {
    use super::AttributeKey;
    use super::InternalAttributeKey;
    
    pub const fn keys(_key: InternalAttributeKey) -> &'static [AttributeKey] {
        todo!();
    }}

pub fn keys_from_type(key: InternalAttributeKey, meta_data_type: MetaDataType) -> &'static [AttributeKey] {
    match meta_data_type {
        MetaDataType::OpenGraph => opengraph::keys(key),
        MetaDataType::SchemaOrg => schema_org::keys(key),
    }
}

#[derive(Clone, Copy)]
pub enum MetaDataType {
    OpenGraph,
    SchemaOrg
}

#[derive(Clone)]
pub struct Attribute {
    pub value: String,
    pub priority: i32
}

pub struct AttributeConfig {
   pub internal_key: InternalAttributeKey,
   pub priority: i32,
}

pub struct AttributeConfigList {
   pub list: Vec<AttributeConfig>,
   pub meta_data_type: MetaDataType
}
