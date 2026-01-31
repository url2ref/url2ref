//! Zotero/Citoid integration for metadata extraction.
//!
//! Uses Wikipedia's Citoid service (which wraps Zotero's translation server)
//! to extract metadata from URLs. This provides a fallback for sites that
//! don't implement Open Graph or Schema.org properly.
//!
//! API Documentation: https://www.mediawiki.org/wiki/Citoid

use serde::Deserialize;
use thiserror::Error;
use chrono::Datelike;

use crate::attribute::{Attribute, AttributeType, Author, Date};
use crate::curl;

/// Citoid API endpoint (Wikipedia's Zotero wrapper)
const CITOID_ENDPOINT: &str = "https://en.wikipedia.org/api/rest_v1/data/citation/zotero/";

#[derive(Error, Debug)]
pub enum ZoteroError {
    #[error("HTTP request failed")]
    CurlError(#[from] curl::CurlError),

    #[error("Failed to parse Zotero response")]
    ParseError(#[from] serde_json::Error),

    #[error("No results returned from Zotero")]
    NoResults,

    #[error("Invalid response format")]
    InvalidFormat,
}

/// Creator/author information from Zotero
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoteroCreator {
    pub creator_type: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub name: Option<String>, // For organizations
}

impl ZoteroCreator {
    /// Convert to our Author type
    pub fn to_author(&self) -> Option<Author> {
        if let Some(name) = &self.name {
            // Organization or single-name entity
            return Some(Author::Organization(name.clone()));
        }

        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => {
                let full_name = format!("{} {}", first.trim(), last.trim());
                Some(Author::Person(full_name))
            }
            (None, Some(last)) => Some(Author::Person(last.clone())),
            (Some(first), None) => Some(Author::Person(first.clone())),
            (None, None) => None,
        }
    }

    /// Check if this is an author (not editor, translator, etc.)
    pub fn is_author(&self) -> bool {
        match &self.creator_type {
            Some(ct) => matches!(ct.as_str(), "author" | "contributor" | "artist"),
            None => true, // Default to author if not specified
        }
    }
}

/// Main Zotero response structure
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoteroResult {
    pub item_type: Option<String>,
    pub title: Option<String>,
    pub creators: Option<Vec<ZoteroCreator>>,
    pub date: Option<String>,
    pub publication_title: Option<String>, // Journal/newspaper name
    pub book_title: Option<String>,
    pub website_title: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    pub url: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub place: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<String>,
    #[serde(rename = "ISSN")]
    pub issn: Option<String>,
    pub abstract_note: Option<String>,
    pub extra: Option<String>, // Contains PMID, bibcode, etc.
    pub university: Option<String>, // For theses
    pub thesis_type: Option<String>,
}

impl ZoteroResult {
    /// Get authors only (filter out editors, translators, etc.)
    pub fn get_authors(&self) -> Vec<Author> {
        self.creators
            .as_ref()
            .map(|creators| {
                creators
                    .iter()
                    .filter(|c| c.is_author())
                    .filter_map(|c| c.to_author())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get the site/publication name
    pub fn get_site_name(&self) -> Option<String> {
        self.publication_title
            .clone()
            .or_else(|| self.website_title.clone())
            .or_else(|| self.book_title.clone())
    }

    /// Parse the date string into our Date type
    pub fn get_date(&self) -> Option<Date> {
        let date_str = self.date.as_ref()?;
        parse_zotero_date(date_str)
    }

    /// Extract PMID from the extra field if present
    pub fn get_pmid(&self) -> Option<String> {
        let extra = self.extra.as_ref()?;
        // Pattern: "PMID: 12345678"
        let re = regex::Regex::new(r"PMID:\s*(\d+)").ok()?;
        re.captures(extra)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Extract bibcode from the extra field if present
    pub fn get_bibcode(&self) -> Option<String> {
        let extra = self.extra.as_ref()?;
        // Pattern: "ADS Bibcode: 2020ApJ..."
        let re = regex::Regex::new(r"ADS Bibcode:\s*(\S+)").ok()?;
        re.captures(extra)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Check if this result looks valid (has meaningful data)
    pub fn is_valid(&self) -> bool {
        // Reject common garbage responses
        if let Some(title) = &self.title {
            let title_lower = title.to_lowercase();
            if title_lower.starts_with("not found")
                || title_lower == "404"
                || title_lower == "error"
                || title_lower == "access denied"
                || title.is_empty()
            {
                return false;
            }
        } else {
            return false; // No title = invalid
        }
        true
    }
}

/// Parse various date formats that Zotero might return
fn parse_zotero_date(date_str: &str) -> Option<Date> {
    let date_str = date_str.trim();

    // Try ISO 8601 format first (2024-01-15)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Some(Date::YearMonthDay(date));
    }

    // Try year-month (2024-01)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d") {
        let year = date.year();
        let month = date.month() as i32;
        return Some(Date::YearMonth { year, month });
    }

    // Try full datetime
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Some(Date::DateTime(dt.with_timezone(&chrono::Utc)));
    }

    // Try year only
    if let Ok(year) = date_str.parse::<i32>() {
        if (1000..=2100).contains(&year) {
            return Some(Date::Year(year));
        }
    }

    // Try common formats like "January 15, 2024"
    let formats = [
        "%B %d, %Y",  // January 15, 2024
        "%b %d, %Y",  // Jan 15, 2024
        "%d %B %Y",   // 15 January 2024
        "%d %b %Y",   // 15 Jan 2024
        "%Y/%m/%d",   // 2024/01/15
        "%m/%d/%Y",   // 01/15/2024
    ];

    for fmt in formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, fmt) {
            return Some(Date::YearMonthDay(date));
        }
    }

    None
}

/// Fetch metadata from Citoid/Zotero for a URL
pub fn fetch_zotero_metadata(url: &str) -> Result<ZoteroResult, ZoteroError> {
    let encoded_url = urlencoding::encode(url);
    let api_url = format!("{}{}", CITOID_ENDPOINT, encoded_url);

    // Use JSON accept header and a proper User-Agent (required by Wikipedia API)
    let headers = [
        "Accept: application/json",
        "User-Agent: url2ref/0.2.0 (https://github.com/url2ref; citation tool)",
    ];
    let response = curl::get_with_headers(&api_url, &headers, true)?;

    println!("[url2ref] Citoid API response (first 500 chars): {}", 
        &response.chars().take(500).collect::<String>());

    // Citoid returns an array, we want the first result
    let results: Vec<ZoteroResult> = serde_json::from_str(&response)
        .map_err(|e| {
            println!("[url2ref] JSON parse error: {} for response: {}", e, 
                &response.chars().take(200).collect::<String>());
            e
        })?;

    results.into_iter().next().ok_or(ZoteroError::NoResults)
}

/// Cached Zotero result for use with AttributeParser
pub struct ZoteroCache {
    pub result: Option<ZoteroResult>,
}

impl ZoteroCache {
    pub fn new() -> Self {
        Self { result: None }
    }

    pub fn fetch(&mut self, url: &str) -> Option<&ZoteroResult> {
        if self.result.is_none() {
            match fetch_zotero_metadata(url) {
                Ok(result) if result.is_valid() => {
                    self.result = Some(result);
                }
                Ok(_) => {
                    // Invalid result, leave as None
                }
                Err(e) => {
                    log::debug!("Zotero fetch failed for {}: {}", url, e);
                }
            }
        }
        self.result.as_ref()
    }
}

impl Default for ZoteroCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Zotero parser implementation
pub struct Zotero;

impl Zotero {
    /// Parse an attribute from a cached Zotero result
    pub fn parse_from_result(
        result: &ZoteroResult,
        attribute_type: AttributeType,
    ) -> Option<Attribute> {
        match attribute_type {
            AttributeType::Title => result.title.clone().map(Attribute::Title),

            AttributeType::Author => {
                let authors = result.get_authors();
                if authors.is_empty() {
                    None
                } else {
                    Some(Attribute::Authors(authors))
                }
            }

            AttributeType::Date => result.get_date().map(Attribute::Date),

            AttributeType::Language => result.language.clone().map(Attribute::Language),

            AttributeType::Site => result.get_site_name().map(Attribute::Site),

            AttributeType::Url => result.url.clone().map(Attribute::Url),

            AttributeType::Journal => result.publication_title.clone().map(Attribute::Journal),

            AttributeType::Publisher => result
                .publisher
                .clone()
                .or_else(|| result.university.clone())
                .map(Attribute::Publisher),

            AttributeType::Volume => result.volume.clone().map(Attribute::Volume),

            _ => None,
        }
    }
}

/// URLs that Zotero typically handles poorly or shouldn't be sent
const ZOTERO_BLACKLIST: &[&str] = &[
    "facebook.com",
    "twitter.com",
    "x.com",
    "instagram.com",
    "tiktok.com",
    "archive.org/web", // Wayback URLs confuse it
    "youtube.com",
    "youtu.be",
    "linkedin.com",
    "reddit.com",
];

/// Check if a URL should be skipped for Zotero
pub fn should_skip_zotero(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    ZOTERO_BLACKLIST
        .iter()
        .any(|blocked| url_lower.contains(blocked))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_zotero_date() {
        assert!(matches!(
            parse_zotero_date("2024-01-15"),
            Some(Date::YearMonthDay(_))
        ));
        assert!(matches!(
            parse_zotero_date("2024"),
            Some(Date::Year(2024))
        ));
        assert!(matches!(
            parse_zotero_date("2024-01"),
            Some(Date::YearMonth { year: 2024, month: 1 })
        ));
    }

    #[test]
    fn test_creator_to_author() {
        let creator = ZoteroCreator {
            creator_type: Some("author".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Smith".to_string()),
            name: None,
        };
        assert!(matches!(creator.to_author(), Some(Author::Person(_))));

        let org = ZoteroCreator {
            creator_type: Some("author".to_string()),
            first_name: None,
            last_name: None,
            name: Some("Associated Press".to_string()),
        };
        assert!(matches!(org.to_author(), Some(Author::Organization(_))));
    }

    #[test]
    fn test_should_skip_zotero() {
        assert!(should_skip_zotero("https://twitter.com/user/status/123"));
        assert!(should_skip_zotero("https://www.facebook.com/page"));
        assert!(!should_skip_zotero("https://www.nytimes.com/article"));
    }
}
