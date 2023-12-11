//! [`Reference`] definitions

use chrono::NaiveDate;

/// Enum for types of references.
/// The names mirror the ones in the Schema.org vocabulary.

#[derive(Debug)]
pub enum Reference {
    NewsArticle {
        title: Option<String>,
        author: Option<String>,
        date: Option<NaiveDate>,
        language: Option<String>,
        site: Option<String>,
        url: Option<String>,
    },
    ScholarlyArticle {
        title: Option<String>,
        author: Option<String>,
        date: Option<NaiveDate>,
        language: Option<String>,
        site: Option<String>,
        url: Option<String>,
        journal: Option<String>
    },
    GenericReference {
        title: Option<String>,
        author: Option<String>,
        date: Option<NaiveDate>,
        language: Option<String>,
        site: Option<String>,
        url: Option<String>,
    }
}

impl Reference {
    pub fn bibtex(&self) -> String {
        // Match on self...
        todo!();
    }

    pub fn wiki(&self) -> String {
        // Match on self...
        todo!();
    }

    pub fn apa(&self) -> String {
        // Match on self...
        todo!();
    }
}