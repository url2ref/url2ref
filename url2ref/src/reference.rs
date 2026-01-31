//! Reference type definitions and citation formatting.
//!
//! This module defines the [`Reference`] enum which represents extracted citation
//! metadata and provides methods to format it into different citation styles.
//!
//! # Example
//!
//! ```rust,no_run
//! use url2ref::{generate, GenerationOptions};
//!
//! let options = GenerationOptions::default();
//! let reference = generate("https://example.com", &options)?;
//!
//! // Format as different citation styles
//! let wiki_citation = reference.wiki();
//! let bibtex_citation = reference.bibtex();
//! let harvard_citation = reference.harvard();
//! # Ok::<(), url2ref::generator::ReferenceGenerationError>(())
//! ```

use crate::attribute::Attribute;
use crate::citation::*;

/// A citation reference extracted from a web page.
///
/// This enum represents different types of content that can be cited,
/// with variants matching common Schema.org types. Each variant contains
/// the metadata fields relevant to that content type.
///
/// # Formatting
///
/// Use the formatting methods to output citations:
/// - [`wiki()`](Reference::wiki) - MediaWiki `{{cite web}}` template
/// - [`bibtex()`](Reference::bibtex) - BibTeX entry
/// - [`harvard()`](Reference::harvard) - Harvard referencing style
///
/// # Example
///
/// ```rust,no_run
/// use url2ref::{generate, GenerationOptions};
///
/// let reference = generate("https://example.com", &GenerationOptions::default())?;
///
/// // Get MediaWiki citation
/// println!("{}", reference.wiki());
/// // Output: {{cite web | title = ... | url = ... }}
/// # Ok::<(), url2ref::generator::ReferenceGenerationError>(())
/// ```
#[derive(Debug)]
pub enum Reference {
    /// A news article (from news websites, blogs, etc.)
    NewsArticle {
        /// The article title
        title: Option<Attribute>,
        /// Translated title (if translation was requested)
        translated_title: Option<Attribute>,
        /// Author name(s)
        author: Option<Attribute>,
        /// Publication date
        date: Option<Attribute>,
        /// Content language (ISO 639 code)
        language: Option<Attribute>,
        /// Website/publication name
        site: Option<Attribute>,
        /// Original URL
        url: Option<Attribute>,
        /// Publisher organization
        publisher: Option<Attribute>,
        /// Wayback Machine archive URL
        archive_url: Option<Attribute>,
        /// Date the page was archived
        archive_date: Option<Attribute>,
    },
    /// A scholarly/academic article (journal papers, preprints, etc.)
    ScholarlyArticle {
        /// The article title
        title: Option<Attribute>,
        /// Translated title (if translation was requested)
        translated_title: Option<Attribute>,
        /// Author name(s)
        author: Option<Attribute>,
        /// Publication date
        date: Option<Attribute>,
        /// Content language (ISO 639 code)
        language: Option<Attribute>,
        /// Original URL
        url: Option<Attribute>,
        /// Journal name
        journal: Option<Attribute>,
        /// Publisher organization
        publisher: Option<Attribute>,
        /// Wayback Machine archive URL
        archive_url: Option<Attribute>,
        /// Date the page was archived
        archive_date: Option<Attribute>,
    },
    /// A generic web page reference
    GenericReference {
        /// The page title
        title: Option<Attribute>,
        /// Translated title (if translation was requested)
        translated_title: Option<Attribute>,
        /// Author name(s)
        author: Option<Attribute>,
        /// Publication date
        date: Option<Attribute>,
        /// Content language (ISO 639 code)
        language: Option<Attribute>,
        /// Website name
        site: Option<Attribute>,
        /// Original URL
        url: Option<Attribute>,
        /// Wayback Machine archive URL
        archive_url: Option<Attribute>,
        /// Date the page was archived
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

    /// Returns a citation formatted as BibTeX markup.
    ///
    /// BibTeX is the standard bibliography format for LaTeX documents.
    ///
    /// # Example Output
    ///
    /// ```text
    /// @misc{example2024,
    ///   author = {Smith, Jane},
    ///   title = {Article Title},
    ///   year = {2024},
    ///   url = {https://example.com/article}
    /// }
    /// ```
    pub fn bibtex(&self) -> String {
        self.build_citation(BibTeXCitation::new())
    }

    /// Returns a citation formatted as MediaWiki markup.
    ///
    /// Uses the `{{cite web}}` template format used by Wikipedia and
    /// other MediaWiki-based wikis.
    ///
    /// # Example Output
    ///
    /// ```text
    /// {{cite web
    ///   | title = Article Title
    ///   | author = Jane Smith
    ///   | url = https://example.com/article
    ///   | access-date = 2024-01-20
    /// }}
    /// ```
    pub fn wiki(&self) -> String {
        self.build_citation(WikiCitation::new())
    }

    /// Returns a citation formatted in Harvard referencing style.
    ///
    /// Harvard style is commonly used in academic writing.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Smith, J. (2024) 'Article Title', Example Site.
    /// Available at: https://example.com/article (Accessed: 20 January 2024).
    /// ```
    pub fn harvard(&self) -> String {
        self.build_citation(HarvardCitation::new())
    }
}