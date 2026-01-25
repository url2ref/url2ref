//! Reference generation from a web address.
//!
//! Provides a simple way to generate a reference to a source on the web.
//! The reference information is primarily collected from the metadata used
//! to annotate the web page using the [Open Graph] protocol and the
//! [Schema.org] vocabulary. The library supports different citation formats
//! (e.g. [BibTeX] and [MediaWiki]).
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

use generator::{attribute_config::{AttributeConfig, AttributeConfigBuilder}, TranslationOptions, ReferenceGenerationError, ArchiveOptions, AiExtractionOptions};
pub use reference::*;
pub use parser::{MultiSourceAttributeCollection, MultiSourceMetadata, ParseInfo};
pub use generator::MetadataType;
pub use generator::AiProvider;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Options for reference generation.
#[derive(Builder)]
#[builder(setter(into))]
pub struct GenerationOptions {
    pub attribute_config: AttributeConfig,
    pub translation_options: TranslationOptions,
    pub archive_options: ArchiveOptions,
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

pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_url(url, options)
}

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