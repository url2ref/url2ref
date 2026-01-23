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
mod parser;
mod reference;

use generator::{attribute_config::{AttributeConfig, AttributeConfigBuilder}, TranslationOptions, ReferenceGenerationError, ArchiveOptions};
pub use reference::*;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Options for reference generation.
#[derive(Builder)]
#[builder(setter(into))]
pub struct GenerationOptions {
    pub attribute_config: AttributeConfig,
    pub translation_options: TranslationOptions,
    pub archive_options: ArchiveOptions,
}
impl Default for GenerationOptions {
    fn default() -> Self {
        let attribute_config = AttributeConfigBuilder::default()
            .build()
            .unwrap();
        let translation_options = TranslationOptions::default();
        let archive_options = ArchiveOptions::default();

        Self {
            attribute_config,
            translation_options,
            archive_options,
        }
    }
}
impl GenerationOptions {
    pub fn new(attribute_config: AttributeConfig, translation_options: TranslationOptions, archive_options: ArchiveOptions) -> Self {
        Self {
            attribute_config,
            translation_options,
            archive_options,
        }
    }
}

pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_url(url, options)
}

pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_file(path, options)
}