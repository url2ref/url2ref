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

pub mod attribute;
pub mod generator;
mod schema_org;
mod opengraph;
mod citation;
mod parser;
mod reference;

use generator::{RecipeOptions, TranslationOptions, ReferenceGenerationError};
pub use reference::*;

type Result<T> = result::Result<T, ReferenceGenerationError>;

/// Options for reference generation.
pub struct GenerationOptions {
    recipes: Vec<RecipeOptions>,
    translation_options: TranslationOptions,
    // include_archived: bool,
    // user_language: &str,
    // ...
}
impl Default for GenerationOptions {
    fn default() -> Self {
        let recipes = vec!(RecipeOptions::default_schema_org(), RecipeOptions::default_opengraph());
        let translation_options = TranslationOptions::default();

        Self { 
            recipes,
            translation_options,
        }
    }
}
impl<'a> GenerationOptions {
    pub fn new(recipes: Vec<RecipeOptions>, translation_options: TranslationOptions) -> Self {
        Self { 
            recipes,
            translation_options,
        }
    }

    pub fn with_translation(translation_options: TranslationOptions) -> Self {
        let recipes = vec!(RecipeOptions::default_schema_org(), RecipeOptions::default_opengraph());

        Self {
            recipes,
            translation_options
        }
    }
}

pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::generate(url, options)
}

pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference> {
    generator::generate_from_file(path, options)
}