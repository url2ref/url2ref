//! Reference generation from a web address.
//!
//! Provides a simple way to generate a reference to a source on the web.
//! The reference information is primarily collected from the metadata used
//! to annotate the web page using the [Open Graph] protocol and the
//! [Schema.org] vocabulary. The library supports different citation styles
//! (e.g. [APA] and [MLA]) as well as specific citation formats (e.g.
//! [BibTeX] and [MediaWiki]).
//!
//! [Open Graph]: https://ogp.me/
//! [Schema.org]: https://schema.org/
//! [APA]: https://en.wikipedia.org/wiki/APA_style
//! [MLA]: https://en.wikipedia.org/wiki/MLA_Handbook
//! [BibTeX]: https://www.bibtex.org/
//! [MediaWiki]: https://www.mediawiki.org/wiki/Help:Cite

pub mod attribute;
pub mod generator;
pub mod schema_org;
pub mod opengraph;
use generator::{AttributeConfigList, ReferenceGenerationError};
mod reference;
pub use reference::*;
mod parser;

pub struct GenerationOptions {
    recipes: Vec<AttributeConfigList>,
    // translate_title: bool,
    // include_archived: bool,
    // user_language: &str,
    // ...
}
impl Default for GenerationOptions {
    fn default() -> Self {
        let og = AttributeConfigList::default_schema_org();

        Self {
            recipes: vec![og],
        }
    }
}

impl GenerationOptions {
    pub fn default_opengraph() -> GenerationOptions {
        let og = AttributeConfigList::default_opengraph();

        Self {
            recipes: vec![og],
        }
    }

    pub fn default_schema_org() -> GenerationOptions {
        let og = AttributeConfigList::default_schema_org();

        Self {
            recipes: vec![og],
        }
    }
}

pub fn generate(url: &str, options: GenerationOptions) -> Result<Reference, ReferenceGenerationError> {
    generator::generate(url, options)
}

pub fn generate_from_file(path: &str, options: GenerationOptions) -> Result<Reference, ReferenceGenerationError> {
    generator::generate_from_file(path, options)
}