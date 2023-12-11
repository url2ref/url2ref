//! Generator responsible for producing a [`Reference`]

use thiserror::Error;

use crate::parser::*;
use crate::GenerationOptions;
use crate::reference::Reference;

/// Errors encountered during reference generation are
/// wrapped in this enum.
#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("URL failed to parse")]
    URLParseError(#[from] std::io::Error),
}

/// Create [`Reference`] by combining the extracted Open Graph and
/// Schema.org metadata.
/// TODO: Add support for prioritization
/// such that certain Open Graph or Schema.org values
/// can be prioritized over the other, allowing for flexibility
/// and giving the user options (also as a way to decide tie breaks).
fn form_reference(results: Vec<ExtractResult>) -> Reference {
    todo!()
}

/// Generate a [`Reference`] from a URL string.
pub fn generate(url: &str, options: GenerationOptions) -> Result<Reference, ReferenceGenerationError> {
    // Parse the HTML to gain access Schema.org and Open Graph metadata
    let attribs = extract_attribs(url, options.extractors)?;
    let reference = form_reference(attribs);

    Ok(reference)
}