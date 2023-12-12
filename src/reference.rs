//! [`Reference`] definitions

use chrono::NaiveDate;

use citation::*;

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
        url: Option<String>,
        journal: Option<String>,
        publisher: Option<String>
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
        match self {
            Reference::NewsArticle {title, author, date,
                language, site, url}=> {
                let formatted_string = WikiCitation::new()
                    .try_add("title", title)
                    .try_add("author", author)
                    .try_add("date", date)
                    .try_add("language", language)
                    .try_add("site", site)
                    .try_add("url", url)
                    .build();
                formatted_string
            }
            Reference::ScholarlyArticle {title, author, date,
                language, url, journal, publisher} => {
                let formatted_string = WikiCitation::new()
                    .try_add("title", title)
                    .try_add("author", author)
                    .try_add("date", date)
                    .try_add("language", language)
                    .try_add("url", url)
                    .try_add("journal", journal)
                    .try_add("publisher", publisher)
                    .build();
                formatted_string
            }
            Reference::GenericReference {title, author, date,
                language, site, url} => {
                let formatted_string = WikiCitation::new()
                    .try_add("title", title)
                    .try_add("author", author)
                    .try_add("date", date)
                    .try_add("language", language)
                    .try_add("site", site)
                    .try_add("url", url)
                    .build();
                formatted_string
            }
        }
    }

    pub fn apa(&self) -> String {
        // Match on self...
        todo!();
    }
}

/// Module providing functionality for building up citations
/// in various formats using the Builder pattern.
mod citation {
    use std::fmt::Display;

    pub trait CitationBuilder {
        fn new() -> Self;
        fn try_add<T: Display>(self, prefix: &str, value: &Option<T>) -> Self;
        fn add(self, prefix: &str, value: &str) -> Self;
        fn build(self) -> String;
    }

    /// Builds a citation using the [{{cite web}} template] from the English Wikipedia
    ///
    /// [{{cite web}} template]: https://en.wikipedia.org/wiki/Template:Cite_web
    pub struct WikiCitation {
        formatted_string: String,
    }
    impl CitationBuilder for WikiCitation {
        fn new() -> Self {
            WikiCitation { formatted_string: String::from("{{cite") }
        }

        fn try_add<T: Display>(self, prefix: &str, value: &Option<T>) -> Self {
            match value {
                Some(value) => self.add(prefix, &value.to_string()),
                None => self,
            }
        }

        fn add(mut self, prefix: &str, value: &str) -> Self {
            self.formatted_string.push_str(&format!(" |{}={}", prefix, value));
            self
        }

        fn build(self) -> String {
            format!("{} }}", self.formatted_string)
        }
    }
}