//! [`Reference`] definitions

use crate::attribute::Attribute;
use crate::citation::*;

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
    fn build_citation<T: CitationBuilder>(&self, builder: T) -> String {
        match self {
            Reference::NewsArticle {title, author, date, language, site, url} => {
                let formatted_string = builder
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
                let formatted_string = builder
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
                let formatted_string = builder
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

    /// Returns a citation in BibTeX markup
    pub fn bibtex(&self) -> String {
        self.build_citation(BibTeXCitation::new())
    }

    /// Returns a citation in Wiki markup
    pub fn wiki(&self) -> String {
        self.build_citation(WikiCitation::new())
    }

    /// Returns a citation in APA style
    pub fn apa(&self) -> String {
        self.build_citation(APACitation::new())
    }
}