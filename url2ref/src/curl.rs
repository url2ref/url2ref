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
}

pub fn get_html(url: &str) -> Result<String> {
    get(url, None, false)
}

pub fn get(url: &str, header_opt: Option<&str>, follow_location: bool) -> Result<String> {
    let mut easy = Easy::new();
    let mut buf = Vec::new();

    // Header determines output format
    if let Some(header) = header_opt {
        let mut header_list = List::new();
        header_list.append(header)?;
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
