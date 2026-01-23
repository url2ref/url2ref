//! Definitions for attributes and the types used for mapping them to
//! their corresponding keys in different metadata formats.

use std::fmt;
use chrono::{NaiveDate, DateTime, Utc};
use strum::EnumIter;

/// Types of attributes contained in a [`crate::reference::Reference`].
/// Allows for mapping to specific keys which denote the same
/// attribute types in various metadata formats.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter, Debug)]
pub enum AttributeType {
   Title,
   Author,
   Locale,
   Language,
   Site,
   Date,
   ArchiveDate,
   Url,
   ArchiveUrl,
   Type,
   Journal,
   Publisher,
   Institution,
   Volume
}

/// Wrapper for the internal representation for attributes
/// used in a [`crate::reference::Reference`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    Title(String),
    TranslatedTitle(Translation),
    Authors(Vec<Author>),
    Date(Date),
    ArchiveDate(Date),
    Language(String),
    Locale(String),
    Site(String),
    Url(String),
    ArchiveUrl(String),
    Type(String),
    Journal(String),
    Publisher(String),
    Institution(String),
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

/// Author enum to make handling of authors in [`crate::citation`] easier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Author {
    Person(String),
    Organization(String),
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

/// Translation containing translated text as well as
/// the language it's in as an ISO 639 language code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Translation {
    pub text: String,
    pub language: String,
}

/// Date enum that can hold both fully complete
/// DateTimes and partially complete dates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Date {
    DateTime(DateTime<Utc>),
    YearMonthDay(NaiveDate),
    YearMonth {
        year:  i32,
        month: i32
    },
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
