//! Definitions for attributes and the types used for mapping them to
//! their corresponding keys in different metadata formats.

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

/// Author enum to make handling of authors in [`crate::citation`] easier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Author {
    Person(String),
    Organization(String),
    Generic(String)
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
