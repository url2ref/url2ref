
pub mod generator;
use generator::ReferenceGenerationError;
mod reference;
pub use reference::*;
mod parser;
use parser::AttributeExtractor;

pub struct GenerationOptions {
    extractors: Vec<AttributeExtractor>,
    // translate_title: bool,
    // include_archived: bool,
    // user_language: &str,
    // ...
}
impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            extractors: vec![AttributeExtractor::OpenGraphExtractor, 
                             AttributeExtractor::SchemaOrgExtractor],
        }
    }
}

pub fn generate(url: &str, options: GenerationOptions) -> Result<Reference, ReferenceGenerationError> {
    generator::generate(url, options)
}