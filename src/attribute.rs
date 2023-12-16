//! Definitions for attributes and the types used for mapping them to
//! their corresponding keys in different metadata formats.

use chrono::NaiveDate;
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
   Url,
   Type
}

/// Wrapper for the internal representation for attributes
/// used in a [`crate::reference::Reference`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    Title(String),
    Authors(Vec<Author>),
    Date(NaiveDate),
    Language(String),
    Locale(String),
    Site(String),
    Url(String),
    Journal(String),
    Publisher(String)
}

/// Author enum to make handling of authors in [`crate::citation`] easier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Author {
    Person(String),
    Organization(String),
    Generic(String)
}