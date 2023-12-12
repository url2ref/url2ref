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
                    .try_add(ReferenceField::Title, title)
                    .try_add(ReferenceField::Author, author)
                    .try_add(ReferenceField::Date, date)
                    .try_add(ReferenceField::Language, language)
                    .try_add(ReferenceField::Site, site)
                    .try_add(ReferenceField::Url, url)
                    .build();
                formatted_string
            }
            Reference::ScholarlyArticle {title, author, date,
                language, url, journal, publisher} => {
                let formatted_string = WikiCitation::new()
                    .try_add(ReferenceField::Title, title)
                    .try_add(ReferenceField::Author, author)
                    .try_add(ReferenceField::Date, date)
                    .try_add(ReferenceField::Language, language)
                    .try_add(ReferenceField::Url, url)
                    .try_add(ReferenceField::Journal, journal)
                    .try_add(ReferenceField::Publisher, publisher)
                    .build();
                formatted_string
            }
            Reference::GenericReference {title, author, date,
                language, site, url} => {
                let formatted_string = WikiCitation::new()
                    .try_add(ReferenceField::Title, title)
                    .try_add(ReferenceField::Author, author)
                    .try_add(ReferenceField::Date, date)
                    .try_add(ReferenceField::Language, language)
                    .try_add(ReferenceField::Site, site)
                    .try_add(ReferenceField::Url, url)
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

    pub enum ReferenceField {
        Title,
        Author,
        Date,
        Language,
        Site,
        Url,
        Journal,
        Publisher
    }

    pub trait CitationBuilder {
        fn new() -> Self;
        fn try_add<T: Display>(self, prefix: ReferenceField, value: &Option<T>) -> Self;
        fn add(self, prefix: ReferenceField, value: &str) -> Self;
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
            WikiCitation { formatted_string: String::from("{{cite web") }
        }

        fn try_add<T: Display>(self, prefix: ReferenceField, value: &Option<T>) -> Self {
            match value {
                Some(value) => self.add(prefix, &value.to_string()),
                None => self,
            }
        }

        fn add(mut self, prefix: ReferenceField, value: &str) -> Self {
            let internal_prefix = match prefix {
                ReferenceField::Title => "title",
                ReferenceField::Author => "author",
                ReferenceField::Date => "date",
                ReferenceField::Language => "language",
                ReferenceField::Site => "site",
                ReferenceField::Url => "url",
                ReferenceField::Journal => "journal",
                ReferenceField::Publisher => "publisher"
            };
            
            self.formatted_string.push_str(&format!(" |{}={}", internal_prefix, value));
            self
        }

        fn build(self) -> String {
            format!("{} }}}}", self.formatted_string)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn wiki_citation_try_add() {
            let prefix = ReferenceField::Title;
            let prefix_str = "title";
            let title = "Test title";

            let wiki_citation = WikiCitation::new().try_add(prefix, &Some(title)).build();
            let expected_result = format!("{{{{cite web |{prefix_str}={title} }}}}");

            assert_eq!(wiki_citation, expected_result)
        }
    }
}