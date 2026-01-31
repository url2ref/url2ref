use curl::easy::{Easy, List};
use std::result;
use thiserror::Error;

type Result<T> = result::Result<T, CurlError>;

#[derive(Error, Debug)]
pub enum CurlError {
    #[error("Curl could not GET url")]
    GetError(#[from] curl::Error),

    #[error("Curl response is not valid UTF8")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("No redirect location found")]
    NoRedirectLocation,
}

pub fn get_html(url: &str) -> Result<String> {
    get(url, None, false)
}

pub fn get(url: &str, header_opt: Option<&str>, follow_location: bool) -> Result<String> {
    let headers: Vec<&str> = header_opt.map(|h| vec![h]).unwrap_or_default();
    get_with_headers(url, &headers, follow_location)
}

pub fn get_with_headers(url: &str, headers: &[&str], follow_location: bool) -> Result<String> {
    let mut easy = Easy::new();
    let mut buf = Vec::new();

    // Set timeouts to prevent hanging
    easy.timeout(std::time::Duration::from_secs(30))?;
    easy.connect_timeout(std::time::Duration::from_secs(10))?;

    // Add all headers
    if !headers.is_empty() {
        let mut header_list = List::new();
        for header in headers {
            header_list.append(header)?;
        }
        easy.http_headers(header_list)?;
    }

    easy.follow_location(follow_location)?;
    easy.url(url)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    let reponse_string = String::from_utf8(buf)?;
    Ok(reponse_string)
}

/// Make a GET request and return the redirect Location header without following it.
/// Used for the Wayback Machine save endpoint which returns a 302 redirect.
pub fn get_redirect_location(url: &str) -> Result<String> {
    let mut easy = Easy::new();
    let mut location: Option<String> = None;

    // Set timeouts - use longer timeout for archive.org which can be slow
    easy.timeout(std::time::Duration::from_secs(120))?;
    easy.connect_timeout(std::time::Duration::from_secs(30))?;

    // Don't follow redirects - we want to capture the Location header
    easy.follow_location(false)?;
    easy.url(url)?;

    // Capture the Location header from the response
    {
        let mut transfer = easy.transfer();
        
        transfer.header_function(|header| {
            if let Ok(header_str) = std::str::from_utf8(header) {
                let header_lower = header_str.to_lowercase();
                if header_lower.starts_with("location:") {
                    // Extract the URL after "Location: "
                    if let Some(loc) = header_str.get(9..).map(|s| s.trim().to_string()) {
                        location = Some(loc);
                    }
                }
            }
            true
        })?;

        // We don't care about the body, but we need a write function
        transfer.write_function(|data| Ok(data.len()))?;
        transfer.perform()?;
    }

    location.ok_or(CurlError::NoRedirectLocation)
}

/// Make a POST request with JSON body and custom headers
pub fn post_json(url: &str, body: &str, headers: &[(&str, &str)]) -> Result<String> {
    let mut easy = Easy::new();
    let mut buf = Vec::new();

    // Set timeouts
    easy.timeout(std::time::Duration::from_secs(60))?;
    easy.connect_timeout(std::time::Duration::from_secs(10))?;

    // Set up headers
    let mut header_list = List::new();
    header_list.append("Content-Type: application/json")?;
    for (key, value) in headers {
        header_list.append(&format!("{}: {}", key, value))?;
    }
    easy.http_headers(header_list)?;

    // Set up POST
    easy.post(true)?;
    easy.post_fields_copy(body.as_bytes())?;
    easy.url(url)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    let response_string = String::from_utf8(buf)?;
    Ok(response_string)
}
