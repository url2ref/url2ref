//! Parser which extracts the metadata to be combined into a [`crate::reference::Reference`].

use std::collections::HashMap;
use std::{fs, result};

use crate::attribute::{Attribute, AttributeType, Date};
use crate::curl::get_html;
use crate::doi::{self, Doi};
use crate::generator::attribute_config::{AttributeConfig, AttributePriority};
use crate::generator::{MetadataType, ReferenceGenerationError};
use crate::opengraph::OpenGraph;
use crate::schema_org::SchemaOrg;

use biblatex::Bibliography;
use chrono::{DateTime, TimeZone, Utc};
use strum::IntoEnumIterator;
use webpage::HTML;

type Result<T> = result::Result<T, ReferenceGenerationError>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct MetadataKey {
    pub key: &'static str,
}

pub struct ParseInfo {
    pub raw_html: String,
    pub html: Option<HTML>,
    pub bibliography: Option<Bibliography>,
}

impl ParseInfo {
    pub fn from_url(url: &str, parsers: &[MetadataType]) -> Result<ParseInfo> {
        use MetadataType::*;
        let raw_html = get_html(url)?;

        let schema_or_og = parsers.contains(&OpenGraph) || parsers.contains(&SchemaOrg);
        let doi = parsers.contains(&Doi);

        let html = parse_html_from_string(raw_html.clone(), &schema_or_og);
        let bib = doi::try_doi_to_bib(url, raw_html.as_str(), &doi);

        if (schema_or_og && html.is_err()) && (doi && bib.is_err()) {
            return Err(ReferenceGenerationError::ParseFailure);
        }

        Ok(ParseInfo {
            raw_html: raw_html,
            html: html.ok(),
            bibliography: bib.ok(),
        })
    }

    pub fn from_file(path: &str) -> Result<ParseInfo> {
        let raw_html = fs::read_to_string(path)?;

        // TODO: should we return ParseFailure here?
        let html = parse_html_from_string(raw_html.clone(), &true)?;

        Ok(ParseInfo {
            raw_html: raw_html,
            html: Some(html),
            bibliography: None,
        })
    }
}

/// Parses the web page into an HTML object using [`webpage`].
pub fn parse_html_from_string(raw_html: String, contained: &bool) -> Result<HTML> {
    if !contained {
        return Err(ReferenceGenerationError::ParseSkip);
    }

    let html = HTML::from_string(raw_html, None)?;
    Ok(html)
}

/// Parse a string into a [`NaiveDate`] object
pub fn parse_date(date_str: &str) -> Option<Date> {
    let dt = DateTime::parse_from_rfc3339(date_str).ok()?;
    let dt_utc = Utc.from_utc_datetime(&dt.naive_utc());

    Some(Date::DateTime(dt_utc))
}

/// Implemented by parsers of different metadata formats
/// (Schema.org, Open Graph, etc.)
pub trait AttributeParser {
    fn parse_attribute(parse_info: &ParseInfo, attribute_type: AttributeType) -> Option<Attribute>;
}

/// Attempt to parse a single attribute
fn parse(
    parse_info: &ParseInfo,
    attribute_type: AttributeType,
    formats: &AttributePriority,
) -> Option<Attribute> {
    for format in &formats.priority {
        let attribute = match format {
            MetadataType::OpenGraph => OpenGraph::parse_attribute(parse_info, attribute_type),
            MetadataType::SchemaOrg => SchemaOrg::parse_attribute(parse_info, attribute_type),
            MetadataType::Doi => Doi::parse_attribute(parse_info, attribute_type),
        };
        if attribute.is_some() {
            return attribute;
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
    pub fn initialize(config: &AttributeConfig, parse_info: &ParseInfo) -> Self {
        Self {
            attributes: HashMap::new(),
        }
        .add_all(config, parse_info)
    }

    /// Retrieves an [`Attribute`] reference from the collection.
    pub fn get(&self, attribute_type: AttributeType) -> Option<&Attribute> {
        self.attributes.get(&attribute_type)
    }

    /// Adds a single [`Attribute`] to the collection.
    fn add(
        mut self,
        attribute_type: AttributeType,
        config: &AttributeConfig,
        parse_info: &ParseInfo,
    ) -> Self {
        let priorities = config.get(attribute_type);
        let attribute = parse(
            parse_info,
            attribute_type,
            &priorities.clone().unwrap_or_default(),
        );
        self.insert_if(attribute_type, attribute);

        self
    }

    /// Adds the [`Attribute`]s corresponding to all [`AttributeType`] variants to
    /// the collection.
    fn add_all(mut self, config: &AttributeConfig, parse_info: &ParseInfo) -> Self {
        AttributeType::iter().for_each(|x| {
            self = self.clone().add(x, config, parse_info);
        });
        self
    }

    fn insert_if(&mut self, attribute_type: AttributeType, attribute: Option<Attribute>) {
        if attribute.is_some() {
            self.attributes.insert(attribute_type, attribute.unwrap());
        }
    }
}
