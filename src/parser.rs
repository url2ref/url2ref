//! Parser which extracts the metadata to be combined into a [`crate::reference::Reference`].

use std::collections::HashMap;
use std::result;

use crate::attribute::{Attribute, AttributeType};
use crate::generator::attribute_config::{AttributePriority, AttributeConfig};
use crate::generator::{MetadataType, ReferenceGenerationError};
use crate::opengraph::OpenGraph;
use crate::schema_org::SchemaOrg;

use chrono::{DateTime, NaiveDate};
use strum::IntoEnumIterator;
use webpage::{Webpage, WebpageOptions, HTML};

type Result<T> = result::Result<T, ReferenceGenerationError>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct MetadataKey {
    pub key: &'static str,
}

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html_from_url(url: &str) -> Result<HTML> {
    let html = Webpage::from_url(url, WebpageOptions::default())?.html;
    Ok(html)
}

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html_from_file(path: &str) -> Result<HTML> {
    let html = HTML::from_file(path, None)?;
    Ok(html)
}

/// Parse a string into a [`NaiveDate`] object
pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    DateTime::parse_from_rfc3339(date_str).ok().map(|v| v.date_naive())
}

/// Implemented by parsers of different metadata formats
/// (Schema.org, Open Graph, etc.)
pub trait AttributeParser {
    fn parse_attribute(html: &HTML, attribute_type: AttributeType) -> Option<Attribute>;
}

/// Attempt to parse a single attribute
fn parse(html: &HTML, attribute_type: AttributeType, formats: &AttributePriority) -> Option<Attribute> {
    for format in &formats.priority {
        let attribute = match format {
            MetadataType::OpenGraph => OpenGraph::parse_attribute(html, attribute_type),
            MetadataType::SchemaOrg => SchemaOrg::parse_attribute(html, attribute_type)
        };
        if attribute.is_some() {
            return attribute
        }
    }

    None
}

#[derive(Clone)]
pub struct AttributeCollection {
    pub attributes: HashMap<AttributeType, Attribute>,
}
impl AttributeCollection {
    /// Initialize an [`AttributeCollection`] from the supplied
    /// [`AttributeConfig`] and [`HTML`].
    pub fn initialize(config: &AttributeConfig, html: &HTML) -> Self {
        Self { attributes: HashMap::new() }.add_all(config, html)
    }

    /// Retrieves an [`Attribute`] reference from the collection.
    pub fn get(&self, attribute_type: AttributeType) -> Option<&Attribute> {
        self.attributes.get(&attribute_type)
    }

    /// Adds a single [`Attribute`] to the collection.
    fn add(mut self, attribute_type: AttributeType, config: &AttributeConfig, html: &HTML) -> Self {
        let priorities = config.get(attribute_type);
        let attribute = parse(html, attribute_type, &priorities.clone().unwrap_or_default());
        self.insert_if(attribute_type, attribute);

        self
    }

    /// Adds the [`Attribute`]s corresponding to all [`AttributeType`] variants to
    /// the collection.
    fn add_all(mut self, config: &AttributeConfig, html: &HTML) -> Self {
        AttributeType::iter().for_each(|x| {
            self = self.clone().add(x, config, html);
        });
        self
    }

    fn insert_if(&mut self, attribute_type: AttributeType, attribute: Option<Attribute>) {
        if attribute.is_some() {
            self.attributes.insert(attribute_type, attribute.unwrap());
        }
    }
}