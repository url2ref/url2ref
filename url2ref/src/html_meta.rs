//! Parser for standard HTML meta tags and microdata.
//! This serves as a fallback when OpenGraph and Schema.org don't provide all needed data.

use scraper::{Html, Selector};

use crate::attribute::{Attribute, AttributeType, Author, Date};
use crate::parser::{parse_date, AttributeParser, ParseInfo, MetadataKey};

/// Mapping from generic [`AttributeType`] to HTML meta tag name attributes.
#[rustfmt::skip]
pub const fn keys(key: AttributeType) -> &'static [MetadataKey] {
    match key {
        AttributeType::Author    => &[MetadataKey{key: "author"},
                                      MetadataKey{key: "article:author"},
                                      MetadataKey{key: "byl"}],
        AttributeType::Publisher => &[MetadataKey{key: "publisher"},
                                      MetadataKey{key: "article:publisher"}],
        AttributeType::Date      => &[MetadataKey{key: "date"},
                                      MetadataKey{key: "article:published_time"},
                                      MetadataKey{key: "pubdate"},
                                      MetadataKey{key: "publishdate"},
                                      MetadataKey{key: "DC.date.issued"}],
        AttributeType::Site      => &[MetadataKey{key: "application-name"},
                                      MetadataKey{key: "apple-mobile-web-app-title"}],
        AttributeType::Language  => &[MetadataKey{key: "language"},
                                      MetadataKey{key: "DC.language"}],
        _                        => &[],
    }
}

pub struct HtmlMeta;

impl HtmlMeta {
    /// Try to find a meta tag with name or property attribute matching one of the keys
    fn try_find_meta_content(raw_html: &str, external_keys: &[MetadataKey]) -> Option<String> {
        let document = Html::parse_document(raw_html);
        
        for external_key in external_keys.iter() {
            // Try name attribute
            let name_selector = Selector::parse(&format!(r#"meta[name="{}"]"#, external_key.key)).ok()?;
            if let Some(element) = document.select(&name_selector).next() {
                if let Some(content) = element.value().attr("content") {
                    if !content.trim().is_empty() {
                        return Some(content.to_string());
                    }
                }
            }
            
            // Try property attribute (for some OG-style meta tags)
            let prop_selector = Selector::parse(&format!(r#"meta[property="{}"]"#, external_key.key)).ok()?;
            if let Some(element) = document.select(&prop_selector).next() {
                if let Some(content) = element.value().attr("content") {
                    if !content.trim().is_empty() {
                        return Some(content.to_string());
                    }
                }
            }
        }
        
        None
    }

    /// Try to find date from HTML5 time elements with itemprop
    fn try_find_microdata_date(raw_html: &str) -> Option<String> {
        let document = Html::parse_document(raw_html);
        
        // Look for time elements with datetime attribute
        let selectors = [
            r#"time[itemprop="datePublished"]"#,
            r#"time[itemprop="published"]"#,
            r#"time[itemprop="dateCreated"]"#,
            r#"time[datetime]"#,
        ];
        
        for selector_str in selectors.iter() {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(datetime) = element.value().attr("datetime") {
                        if !datetime.trim().is_empty() {
                            return Some(datetime.to_string());
                        }
                    }
                }
            }
        }
        
        None
    }

    /// Try to find author from various HTML patterns
    fn try_find_author(raw_html: &str) -> Option<Vec<Author>> {
        let document = Html::parse_document(raw_html);
        
        // Common author selectors
        let selectors = [
            r#"[itemprop="author"] [itemprop="name"]"#,
            r#"[itemprop="author"]"#,
            r#"[rel="author"]"#,
            r#".author"#,
            r#".byline"#,
            r#"[class*="author-name"]"#,
            r#"[class*="byline"]"#,
        ];
        
        for selector_str in selectors.iter() {
            if let Ok(selector) = Selector::parse(selector_str) {
                let authors: Vec<Author> = document
                    .select(&selector)
                    .filter_map(|element| {
                        let text = element.text().collect::<String>().trim().to_string();
                        if !text.is_empty() && text.len() < 200 {
                            // Clean up common prefixes
                            let cleaned = text
                                .trim_start_matches("Af ")
                                .trim_start_matches("By ")
                                .trim_start_matches("af ")
                                .trim_start_matches("by ")
                                .trim()
                                .to_string();
                            if !cleaned.is_empty() {
                                Some(Author::Generic(cleaned))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if !authors.is_empty() {
                    return Some(authors);
                }
            }
        }
        
        None
    }
}

fn attribute_type_to_attribute(
    attribute_type: AttributeType,
    attribute_value: String,
) -> Option<Attribute> {
    match attribute_type {
        AttributeType::Author => {
            let author = Author::Generic(attribute_value);
            Some(Attribute::Authors(vec![author]))
        }
        AttributeType::Date => {
            // Try RFC3339 format first
            if let Some(date) = parse_date(&attribute_value) {
                return Some(Attribute::Date(date));
            }
            // Try parsing ISO 8601 without timezone
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&attribute_value, "%Y-%m-%dT%H:%M:%S") {
                return Some(Attribute::Date(Date::DateTime(chrono::Utc.from_utc_datetime(&dt))));
            }
            // Try date only format
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&attribute_value, "%Y-%m-%d") {
                return Some(Attribute::Date(Date::YearMonthDay(date)));
            }
            None
        }
        AttributeType::Publisher => Some(Attribute::Publisher(attribute_value)),
        AttributeType::Site => Some(Attribute::Site(attribute_value)),
        AttributeType::Language => Some(Attribute::Language(attribute_value)),
        _ => None,
    }
}

impl AttributeParser for HtmlMeta {
    fn parse_attribute(parse_info: &ParseInfo, attribute_type: AttributeType) -> Option<Attribute> {
        let raw_html = &parse_info.raw_html;
        
        match attribute_type {
            AttributeType::Author => {
                // First try meta tags
                let external_keys = keys(attribute_type);
                if let Some(value) = HtmlMeta::try_find_meta_content(raw_html, external_keys) {
                    return attribute_type_to_attribute(attribute_type, value);
                }
                // Then try microdata/HTML patterns
                if let Some(authors) = HtmlMeta::try_find_author(raw_html) {
                    return Some(Attribute::Authors(authors));
                }
                None
            }
            AttributeType::Date => {
                // First try meta tags
                let external_keys = keys(attribute_type);
                if let Some(value) = HtmlMeta::try_find_meta_content(raw_html, external_keys) {
                    return attribute_type_to_attribute(attribute_type, value);
                }
                // Then try microdata time elements
                if let Some(datetime) = HtmlMeta::try_find_microdata_date(raw_html) {
                    return attribute_type_to_attribute(attribute_type, datetime);
                }
                None
            }
            _ => {
                let external_keys = keys(attribute_type);
                let attribute_value = HtmlMeta::try_find_meta_content(raw_html, external_keys)?;
                attribute_type_to_attribute(attribute_type, attribute_value)
            }
        }
    }
}

use chrono::TimeZone;
