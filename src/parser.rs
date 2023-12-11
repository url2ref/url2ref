//! Parser which extracts the metadata to be combined into the final [`Reference`]

use webpage::{Webpage, WebpageOptions, HTML};
use crate::generator::ReferenceGenerationError;

pub mod opengraph {
    pub const TITLE:  &[&str; 1] = &["og:title"];
    pub const AUTHOR: &[&str; 1] = &["article:author"];
    pub const LOCALE: &[&str; 1] = &["og:locale"];
    pub const SITE:   &[&str; 1] = &["og:site_name"];
    pub const DATE:   &[&str; 2] = &["article:published_time", 
                                      "article:modified_time"];
    pub const TYPE:   &[&str; 1] = &["og:type"];

}

pub mod schema_org {
    pub const TITLE: &[&str; 1] = &["title"];
}

/// Parses the web page into an HTML object using [`webpage`].
fn parse_html(url: &str) -> Result<HTML, ReferenceGenerationError>  {
    let html = Webpage::from_url(url, WebpageOptions::default())?.html;
    Ok(html)
}

pub fn extract_attribs(url: &str, extractors: Vec<AttributeExtractor>) -> Result<Vec<ExtractResult>, ReferenceGenerationError> {
    let html = parse_html(url)?;
    // for each extractor call extractor.extract();
    todo!();
}

pub struct ExtractResult {
    extractor: AttributeExtractor,
    //mappings: Vec<(String, String)>,
}

pub enum AttributeExtractor {
    OpenGraphExtractor,
    SchemaOrgExtractor,
    EstimatorExtractor,
}

impl AttributeExtractor {
    fn extract(&self, html: HTML) -> ExtractResult {
        // Match on self...
        todo!();
    }
}