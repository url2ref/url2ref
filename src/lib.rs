use thiserror::Error;

mod parsing {
    use webpage::{Webpage, WebpageOptions, HTML};

    pub fn parse_html(url: &str) -> Result<HTML, std::io::Error>  {
        let page = Webpage::from_url(url, WebpageOptions::default())?;
        let parsed_html = page.html;
        Ok(parsed_html)
    }
}

#[derive(Error, Debug)]
pub enum ReferenceGenerationError {
    #[error("URL failed to parse")]
    URLParseError(#[from] std::io::Error),
}

pub fn generate_reference(url: &str) -> Result<String, ReferenceGenerationError> {
    // Parse the HTML to gain access Schema.org and Open Graph metadata
    let parsed_html = parsing::parse_html(url)?;

    // TODO: Build upon this trivial example
    let reference = parsed_html.title.unwrap_or(String::from(""));
    Ok(reference)
}