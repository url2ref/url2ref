//! # url2ref - Automatic Reference Generation
//!
//! `url2ref` is a library for automatically generating properly formatted citations
//! from web URLs. It extracts metadata from web pages and formats it into standard
//! citation styles like BibTeX, MediaWiki, and Harvard.
//!
//! ## Features
//!
//! - **Multiple metadata sources**: Extracts from [Open Graph], [Schema.org], HTML meta tags, DOI, and more
//! - **Citation formats**: Outputs BibTeX, MediaWiki `{{cite web}}`, and Harvard styles
//! - **Translation**: Optional title translation via DeepL or Google Translate
//! - **Archiving**: Integration with the Wayback Machine for archive URLs
//! - **AI extraction**: Optional LLM-powered metadata extraction for difficult pages
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use url2ref::{generate, GenerationOptions};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let options = GenerationOptions::default();
//!     let reference = generate("https://example.com/article", &options)?;
//!     
//!     // Output in different formats
//!     println!("Wiki:\n{}", reference.wiki());
//!     println!("BibTeX:\n{}", reference.bibtex());
//!     println!("Harvard:\n{}", reference.harvard());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Metadata Sources
//!
//! The library extracts metadata from multiple sources in order of priority:
//!
//! | Source | Description |
//! |--------|-------------|
//! | [Open Graph] | Facebook's metadata protocol (`og:*` tags) |
//! | [Schema.org] | Structured data (JSON-LD, microdata) |
//! | HTML Meta | Standard `<meta>` tags and `<title>` |
//! | DOI | Digital Object Identifier resolution |
//! | Zotero | Wikipedia's Citoid service |
//! | AI | LLM-powered extraction (OpenAI, Anthropic) |
//!
//! ## Advanced Usage
//!
//! ### Custom Priority
//!
//! Configure which metadata sources to prefer:
//!
//! ```rust,no_run
//! use url2ref::{generate, GenerationOptions};
//! use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
//! use url2ref::generator::MetadataType;
//!
//! let priority = AttributePriority::new(&[
//!     MetadataType::SchemaOrg,
//!     MetadataType::OpenGraph,
//!     MetadataType::HtmlMeta,
//! ]);
//! let config = AttributeConfig::new(priority);
//!
//! let options = GenerationOptions {
//!     attribute_config: config,
//!     ..Default::default()
//! };
//! ```
//!
//! ### Reusing Fetched Content
//!
//! For efficiency, fetch once and reuse:
//!
//! ```rust,no_run
//! use url2ref::{fetch_parse_info, generate_from_parse_info, parse_all_metadata_from_parse_info, GenerationOptions};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let options = GenerationOptions::default();
//!
//! // Fetch HTML once
//! let parse_info = fetch_parse_info("https://example.com", Some(&options))?;
//!
//! // Generate citation
//! let reference = generate_from_parse_info(&parse_info, &options)?;
//!
//! // Also get raw metadata from all sources
//! let multi_source = parse_all_metadata_from_parse_info(&parse_info);
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Overview
//!
//! - [`attribute`] - Metadata attribute types and values
//! - [`generator`] - Reference generation and configuration
//! - [`parser`] - HTML parsing and metadata extraction
//! - [`ai_extractor`] - AI-powered metadata extraction
//! - [`zotero`] - Zotero/Citoid integration
//!
//! [Open Graph]: https://ogp.me/
//! [Schema.org]: https://schema.org/
//! [BibTeX]: https://www.bibtex.org/
//! [MediaWiki]: https://www.mediawiki.org/wiki/Help:Cite

use std::result;
use derive_builder::Builder;

pub mod attribute;
pub mod generator;
mod schema_org;
mod opengraph;
mod html_meta;
mod doi;
mod curl;
mod citation;
pub mod parser;
mod reference;
pub mod ai_extractor;
pub mod zotero;

use generator::{attribute_config::{AttributeConfig, AttributeConfigBuilder}, TranslationOptions, ReferenceGenerationError, ArchiveOptions, AiExtractionOptions};
pub use reference::*;
pub use parser::{MultiSourceAttributeCollection, MultiSourceMetadata, ParseInfo};
pub use generator::MetadataType;
pub use generator::AiProvider;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Options for reference generation.
///
/// Controls all aspects of the citation generation process including:
/// - Which metadata sources to use and their priority
/// - Title translation settings
/// - Archive URL fetching
/// - AI-powered extraction
///
/// # Example
///
/// ```rust
/// use url2ref::GenerationOptions;
///
/// // Use defaults
/// let options = GenerationOptions::default();
///
/// // Or customize with builder
/// use url2ref::generator::{TranslationOptions, ArchiveOptions, AiExtractionOptions};
/// use url2ref::generator::attribute_config::AttributeConfig;
///
/// let options = GenerationOptions {
///     attribute_config: AttributeConfig::default(),
///     translation_options: TranslationOptions::default(),
///     archive_options: ArchiveOptions {
///         include_archived: true,
///         perform_archival: false,
///     },
///     ai_options: AiExtractionOptions::default(),
/// };
/// ```
#[derive(Builder)]
#[builder(setter(into))]
pub struct GenerationOptions {
    /// Configuration for metadata source priority per attribute.
    pub attribute_config: AttributeConfig,
    /// Options for title translation (DeepL, Google).
    pub translation_options: TranslationOptions,
    /// Options for Wayback Machine archive integration.
    pub archive_options: ArchiveOptions,
    /// Options for AI-powered metadata extraction.
    pub ai_options: AiExtractionOptions,
}
impl Default for GenerationOptions {
    fn default() -> Self {
        let attribute_config = AttributeConfigBuilder::default()
            .build()
            .unwrap();
        let translation_options = TranslationOptions::default();
        let archive_options = ArchiveOptions::default();
        let ai_options = AiExtractionOptions::default();

        Self {
            attribute_config,
            translation_options,
            archive_options,
            ai_options,
        }
    }
}
impl GenerationOptions {
    pub fn new(attribute_config: AttributeConfig, translation_options: TranslationOptions, archive_options: ArchiveOptions) -> Self {
        Self {
            attribute_config,
            translation_options,
            archive_options,
            ai_options: AiExtractionOptions::default(),
        }
    }

    pub fn with_ai(mut self, ai_options: AiExtractionOptions) -> Self {
        self.ai_options = ai_options;
        self
    }
}

/// Generate a citation reference from a URL.
///
/// This is the primary entry point for generating citations. It fetches the web page,
/// extracts metadata from multiple sources, and returns a [`Reference`] that can be
/// formatted into different citation styles.
///
/// # Arguments
///
/// * `url` - The URL of the web page to cite
/// * `options` - Configuration options for generation
///
/// # Returns
///
/// * `Ok(Reference)` - A reference that can be formatted via `.wiki()`, `.bibtex()`, or `.harvard()`
/// * `Err(ReferenceGenerationError)` - If fetching or parsing fails
///
/// # Example
///
/// ```rust,no_run
/// use url2ref::{generate, GenerationOptions};
///
/// let options = GenerationOptions::default();
/// let reference = generate("https://www.bbc.com/news/article", &options)?;
///
/// println!("{}", reference.wiki());
/// # Ok::<(), url2ref::generator::ReferenceGenerationError>(())
/// ```
pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_url(url, options)
}

/// Generate a citation reference from a local HTML file.
///
/// Useful for testing or processing saved web pages.
///
/// # Arguments
///
/// * `path` - Path to the HTML file
/// * `options` - Configuration options for generation
///
/// # Example
///
/// ```rust,no_run
/// use url2ref::{generate_from_file, GenerationOptions};
///
/// let options = GenerationOptions::default();
/// let reference = generate_from_file("./saved_page.html", &options)?;
/// # Ok::<(), url2ref::generator::ReferenceGenerationError>(())
/// ```
pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_file(path, options)
}

/// Generate a reference from pre-fetched ParseInfo.
/// This allows reusing cached HTML between multiple operations.
/// 
/// # Example
/// ```ignore
/// let parse_info = fetch_parse_info("https://example.com", None)?;
/// let reference = generate_from_parse_info(&parse_info, &options)?;
/// let multi_source = parse_all_metadata_from_parse_info(&parse_info);
/// ```
pub fn generate_from_parse_info(parse_info: &ParseInfo, options: &GenerationOptions) -> Result<Reference> {
    generator::create_reference_from_parse_info(parse_info, options)
}

/// Fetch and parse HTML from a URL into a reusable ParseInfo struct.
/// This performs the HTTP request once, and the result can be passed to:
/// - `generate_from_parse_info()` for reference generation
/// - `parse_all_metadata_from_parse_info()` for multi-source metadata
/// 
/// # Arguments
/// * `url` - The URL to fetch
/// * `options` - Optional generation options. If provided, only parsers specified
///               in the attribute config will be initialized. If None, all parsers
///               are initialized.
pub fn fetch_parse_info<'a>(url: &'a str, options: Option<&GenerationOptions>) -> Result<ParseInfo<'a>> {
    let parsers = options
        .map(|o| o.attribute_config.parsers_used())
        .unwrap_or_else(|| vec![
            MetadataType::OpenGraph,
            MetadataType::SchemaOrg,
            MetadataType::HtmlMeta,
            MetadataType::Doi,
        ]);
    ParseInfo::from_url(url, &parsers)
}

/// Parse metadata from all sources for a URL without generating a reference.
/// Returns metadata from OpenGraph, Schema.org, HTML Meta, and DOI sources.
/// 
/// Note: This performs an HTTP request. If you also need to generate a reference,
/// use `fetch_parse_info()` followed by `generate_from_parse_info()` and 
/// `parse_all_metadata_from_parse_info()` to avoid duplicate requests.
pub fn parse_all_metadata(url: &str) -> Result<MultiSourceAttributeCollection> {
    let parsers = vec![
        MetadataType::OpenGraph,
        MetadataType::SchemaOrg,
        MetadataType::HtmlMeta,
        MetadataType::Doi,
    ];
    
    let parse_info = ParseInfo::from_url(url, &parsers)?;
    Ok(MultiSourceAttributeCollection::parse_all(&parse_info))
}

/// Parse multi-source metadata from pre-fetched ParseInfo.
/// This allows reusing cached HTML between generate() and multi-source extraction.
pub fn parse_all_metadata_from_parse_info(parse_info: &ParseInfo) -> MultiSourceAttributeCollection {
    MultiSourceAttributeCollection::parse_all(parse_info)
}