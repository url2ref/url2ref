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
mod doi;
mod citation;
mod parser;
mod reference;

use generator::{attribute_config::{AttributeConfig, AttributeConfigBuilder}, TranslationOptions, ReferenceGenerationError};
pub use reference::*;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Options for reference generation.
#[derive(Builder)]
#[builder(setter(into))]
pub struct GenerationOptions {
    pub config: AttributeConfig,
    pub translation_options: TranslationOptions,
    // include_archived: bool,
    // user_language: &str,
    // ...
}
impl Default for GenerationOptions {
    fn default() -> Self {
        let config = AttributeConfigBuilder::default()
            .build()
            .unwrap();
        let translation_options = TranslationOptions::default();

        Self {
            config,
            translation_options,
        }
    }
}
impl GenerationOptions {
    pub fn new(config: AttributeConfig, translation_options: TranslationOptions) -> Self {
        Self {
            config,
            translation_options,
        }
    }

    pub fn with_translation(translation_options: TranslationOptions) -> Self {
        let config = AttributeConfigBuilder::default()
            .build()
            .unwrap();

        Self {
            config,
            translation_options
        }
    }
}

pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_url(url, options)
}

pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::from_file(path, options)
}