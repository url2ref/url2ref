//! [`Reference`] definitions

use crate::attribute::Attribute;
use crate::citation::*;

/// Enum for types of references.
/// The names generally mirror the ones in the Schema.org vocabulary.
#[derive(Debug)]
pub enum Reference {
    NewsArticle {
        title: Option<Attribute>,
        translated_title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        site: Option<Attribute>,
        url: Option<Attribute>,
        publisher: Option<Attribute>,
        archive_url: Option<Attribute>,
        archive_date: Option<Attribute>,
    },
    ScholarlyArticle {
        title: Option<Attribute>,
        translated_title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        url: Option<Attribute>,
        journal: Option<Attribute>,
        publisher: Option<Attribute>,
        archive_url: Option<Attribute>,
        archive_date: Option<Attribute>,
    },
    GenericReference {
        title: Option<Attribute>,
        translated_title: Option<Attribute>,
        author: Option<Attribute>,
        date: Option<Attribute>,
        language: Option<Attribute>,
        site: Option<Attribute>,
        url: Option<Attribute>,
        archive_url: Option<Attribute>,
        archive_date: Option<Attribute>,
    }
}
impl Reference {
    fn build_citation<T: CitationBuilder>(&self, builder: T) -> String {
        match self {
            Reference::NewsArticle { title, translated_title, author, date, language, site, url, archive_url, archive_date, publisher } => {
                let formatted_string = builder
                    .try_add(title)
                    .try_add(translated_title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(site)
                    .try_add(url)
                    .try_add(archive_url)
                    .try_add(archive_date)
                    .try_add(publisher)
                    .build();
                formatted_string
            }
            Reference::ScholarlyArticle { title, translated_title, author, date, language, url, archive_url, archive_date, publisher, journal } => {
                let formatted_string = builder
                    .try_add(title)
                    .try_add(translated_title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(url)
                    .try_add(archive_url)
                    .try_add(archive_date)
                    .try_add(journal)
                    .try_add(publisher)
                    .build();
                formatted_string
            }
            Reference::GenericReference { title, translated_title, author, date, language, site, url, archive_url, archive_date } => {
                let formatted_string = builder
                    .try_add(title)
                    .try_add(translated_title)
                    .try_add(author)
                    .try_add(date)
                    .try_add(language)
                    .try_add(site)
                    .try_add(url)
                    .try_add(archive_url)
                    .try_add(archive_date)
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
}