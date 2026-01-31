//! Attribute type definitions for citation metadata.
//!
//! This module defines the types used to represent extracted metadata values.
//! Attributes are the building blocks of a [`Reference`](crate::reference::Reference).
//!
//! # Overview
//!
//! - [`AttributeType`] - Identifies what kind of metadata (title, author, date, etc.)
//! - [`Attribute`] - The actual value with type-specific representation
//! - [`Author`] - Represents an author (person or organization)
//! - [`Date`] - Represents dates with varying precision
//! - [`Translation`] - A translated title with language code
//!
//! # Example
//!
//! ```rust
//! use url2ref::attribute::{Attribute, Author, Date};
//! use chrono::NaiveDate;
//!
//! // Create attributes
//! let title = Attribute::Title("Article Title".to_string());
//! let author = Attribute::Authors(vec![
//!     Author::Person("Jane Smith".to_string()),
//! ]);
//! let date = Attribute::Date(Date::YearMonthDay(
//!     NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
//! ));
//!
//! // Attributes implement Display
//! println!("{}", title);  // "Article Title"
//! println!("{}", date);   // "2024-01-15"
//! ```

use std::fmt;
use chrono::{NaiveDate, DateTime, Utc};
use strum::EnumIter;

/// Identifies the type of a metadata attribute.
///
/// Used to map between different metadata formats (Open Graph, Schema.org, etc.)
/// and the internal representation. Each source may use different key names
/// for the same attribute type.
///
/// # Example
///
/// ```rust
/// use url2ref::attribute::AttributeType;
///
/// // Title might come from og:title, schema:headline, or <title>
/// let attr_type = AttributeType::Title;
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter, Debug)]
pub enum AttributeType {
   /// Article/page title
   Title,
   /// Author name(s)
   Author,
   /// Locale code (e.g., "en_US")
   Locale,
   /// Language code (e.g., "en")
   Language,
   /// Website/publication name
   Site,
   /// Publication date
   Date,
   /// Date the page was archived
   ArchiveDate,
   /// Original URL
   Url,
   /// Archive (Wayback Machine) URL
   ArchiveUrl,
   /// Content type (article, news, etc.)
   Type,
   /// Journal name (for scholarly articles)
   Journal,
   /// Publisher organization
   Publisher,
   /// Institution (for academic content)
   Institution,
   /// Volume number (for journals)
   Volume
}

/// A metadata attribute value.
///
/// Wraps the various types of values that can be extracted from web pages.
/// Each variant holds the appropriate type for that kind of data.
///
/// Implements [`Display`](fmt::Display) for easy conversion to strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    /// Article or page title
    Title(String),
    /// Translated title with language information
    TranslatedTitle(Translation),
    /// One or more authors
    Authors(Vec<Author>),
    /// Publication date
    Date(Date),
    /// Archive date
    ArchiveDate(Date),
    /// Content language (ISO 639 code)
    Language(String),
    /// Locale (e.g., "en_US")
    Locale(String),
    /// Website name
    Site(String),
    /// Original URL
    Url(String),
    /// Archive URL
    ArchiveUrl(String),
    /// Content type
    Type(String),
    /// Journal name
    Journal(String),
    /// Publisher name
    Publisher(String),
    /// Institution name
    Institution(String),
    /// Volume number
    Volume(String)
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::Title(s) => write!(f, "{}", s),
            Attribute::TranslatedTitle(t) => write!(f, "{}", t.text),
            Attribute::Authors(authors) => {
                let names: Vec<String> = authors.iter().map(|a| a.to_string()).collect();
                write!(f, "{}", names.join(", "))
            },
            Attribute::Date(d) => write!(f, "{}", d),
            Attribute::ArchiveDate(d) => write!(f, "{}", d),
            Attribute::Language(s) => write!(f, "{}", s),
            Attribute::Locale(s) => write!(f, "{}", s),
            Attribute::Site(s) => write!(f, "{}", s),
            Attribute::Url(s) => write!(f, "{}", s),
            Attribute::ArchiveUrl(s) => write!(f, "{}", s),
            Attribute::Type(s) => write!(f, "{}", s),
            Attribute::Journal(s) => write!(f, "{}", s),
            Attribute::Publisher(s) => write!(f, "{}", s),
            Attribute::Institution(s) => write!(f, "{}", s),
            Attribute::Volume(s) => write!(f, "{}", s),
        }
    }
}

/// Represents an author of content.
///
/// Authors can be individual people, organizations, or generic names
/// when the type cannot be determined.
///
/// # Example
///
/// ```rust
/// use url2ref::attribute::Author;
///
/// let person = Author::Person("Jane Smith".to_string());
/// let org = Author::Organization("Reuters".to_string());
///
/// println!("{}", person);  // "Jane Smith"
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Author {
    /// An individual person's name
    Person(String),
    /// An organization or company name
    Organization(String),
    /// Generic author when type is unknown
    Generic(String)
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Author::Person(s) => write!(f, "{}", s),
            Author::Organization(s) => write!(f, "{}", s),
            Author::Generic(s) => write!(f, "{}", s),
        }
    }
}

/// A translated title with its target language.
///
/// Created when translation is enabled in [`GenerationOptions`](crate::GenerationOptions).
///
/// # Example
///
/// ```rust
/// use url2ref::attribute::Translation;
///
/// let trans = Translation {
///     text: "The Future of Technology".to_string(),
///     language: "EN".to_string(),
/// };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Translation {
    /// The translated text
    pub text: String,
    /// ISO 639 language code of the translation
    pub language: String,
}

/// Represents a date with varying levels of precision.
///
/// Web pages may provide dates with different levels of detail.
/// This enum captures dates ranging from just a year to full timestamps.
///
/// # Example
///
/// ```rust
/// use url2ref::attribute::Date;
/// use chrono::NaiveDate;
///
/// // Full date
/// let full = Date::YearMonthDay(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
/// println!("{}", full);  // "2024-01-15"
///
/// // Year only
/// let year = Date::Year(2024);
/// println!("{}", year);  // "2024"
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Date {
    /// Full datetime with timezone
    DateTime(DateTime<Utc>),
    /// Date with year, month, and day
    YearMonthDay(NaiveDate),
    /// Date with only year and month
    YearMonth {
        /// The year
        year: i32,
        /// The month (1-12)
        month: i32
    },
    /// Date with only the year
    Year(i32),
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Date::DateTime(dt) => write!(f, "{}", dt.format("%Y-%m-%d")),
            Date::YearMonthDay(nd) => write!(f, "{}", nd.format("%Y-%m-%d")),
            Date::YearMonth { year, month } => write!(f, "{}-{:02}", year, month),
            Date::Year(year) => write!(f, "{}", year),
        }
    }
}
