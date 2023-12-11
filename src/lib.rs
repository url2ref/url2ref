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
use attribute::{AttributeConfigList, AttributeConfig, MetaDataType, InternalAttributeKey};
use generator::ReferenceGenerationError;
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
        let og = AttributeConfigList {
                list: vec![
                    AttributeConfig { internal_key: InternalAttributeKey::Title, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Author, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Locale, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Site, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Url, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Date, priority: 1},
                    AttributeConfig { internal_key: InternalAttributeKey::Type, priority: 1},
                ],
                meta_data_type: MetaDataType::OpenGraph
        };

        Self {
            recipes: vec![og],
        }
    }
}

pub fn generate(url: &str, options: GenerationOptions) -> Result<Reference, ReferenceGenerationError> {
    generator::generate(url, options)
}