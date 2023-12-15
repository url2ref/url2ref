//! [`Reference`] definitions

use chrono::NaiveDate;

use citation::*;

use crate::attribute::Attribute;

/// Enum for types of references.
/// The names generally mirror the ones in the Schema.org vocabulary.
#[derive(Debug)]
pub enum Reference {
    NewsArticle {
        title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        site: Option<Attribute>,
        url: Option<Attribute>,
    },
    ScholarlyArticle {
        title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        url: Option<Attribute>,
        journal: Option<Attribute>,
        publisher: Option<Attribute>
    },
    GenericReference {
        title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        site: Option<Attribute>,
        url: Option<Attribute>,
    }
}
impl Reference {
    /// Returns a citation in BibTeX markup
    pub fn bibtex(&self) -> String {
        // Match on self...
        todo!();
    }

    /// Returns a citation in Wiki markup
    pub fn wiki(&self) -> String {
        match self {
            Reference::NewsArticle {title, author, date, language, site, url} => {
                let formatted_string = WikiCitation::new()
                    .try_add(title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(site)
                    .try_add(url)
                    .build();
                formatted_string
            }
            Reference::ScholarlyArticle {title, author, date, language, url, journal, publisher} => {
                let formatted_string = WikiCitation::new()
                    .try_add(title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(url)
                    .try_add(journal)
                    .try_add(publisher)
                    .build();
                formatted_string
            }
            Reference::GenericReference {title, author, date, language, site, url} => {
                let formatted_string = WikiCitation::new()
                    .try_add(title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(site)
                    .try_add(url)
                    .build();
                formatted_string
            }
        }
    }

    /// Returns a citation in APA style
    pub fn apa(&self) -> String {
        // Match on self...
        todo!();
    }
}

/// Module providing functionality for building up citations
/// in various formats using the Builder pattern.
mod citation {
    use super::Attribute;
    use super::NaiveDate;

    pub trait CitationBuilder {
        fn new() -> Self;
        fn try_add(self, value: &Option<Attribute>) -> Self;
        fn add(self, value: &Attribute) -> Self;
        fn build(self) -> String;
    }

    /// Builds a citation using the [{{cite web}} template] from the English Wikipedia
    ///
    /// [{{cite web}} template]: https://en.wikipedia.org/wiki/Template:Cite_web
    pub struct WikiCitation {
        formatted_string: String,
    }
    impl WikiCitation {
        fn handle_authors(&self, _authors: &Vec<String>) -> (&str, &String) {
            todo!();
        }

        fn handle_date(&self, _date: &NaiveDate) -> &String {
            todo!();
        }
    }
    impl CitationBuilder for WikiCitation {
        fn new() -> Self {
            WikiCitation { formatted_string: String::from("") }
        }

        fn try_add(self, value: &Option<Attribute>) -> Self {
            match value {
                Some(value) => self.add(&value),
                None => self,
            }
        }

        fn add(mut self,  value: &Attribute) -> Self {
            let result_option = match value {
                Attribute::Title(val) => Some(("title", val)),
                Attribute::Author(vals) => Some(self.handle_authors(vals)),
                Attribute::Date(val) => Some(("date", self.handle_date(val))),
                Attribute::Language(val) => Some(("language", val)),
                Attribute::Site(val) => Some(("site", val)),
                Attribute::Url(val) => Some(("url", val)),
                Attribute::Journal(val) => Some(("journal", val)),
                Attribute::Publisher(val) => Some(("publisher", val)),
                _ => None
            };
            
            if let Some((prefix, parsed_value)) = result_option {
                self.formatted_string.push_str(&format!(" |{}={}", prefix, parsed_value));
            }
            self
        }

        fn build(self) -> String {
            format!("{{{{cite web{} }}}}", self.formatted_string)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn wiki_citation_try_add() {
            let title = "Test title";
            let attribute = Attribute::Title(title.to_string());

            let wiki_citation = WikiCitation::new()
                .try_add(&Some(attribute))
                .build();
            let expected_result = format!("{{{{cite web |title={title} }}}}");

            assert_eq!(wiki_citation, expected_result)
        }
    }
}