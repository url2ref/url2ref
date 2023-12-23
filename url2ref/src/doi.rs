//! Parser responsible for producing an [`Attribute`] from a Bibtex entry
//! retrieved from a DOI.

use crate::attribute::{Attribute, AttributeType, Author, Date};
use crate::curl::{get, CurlError};
use crate::generator::ReferenceGenerationError;
use crate::parser::{AttributeParser, ParseInfo};

use biblatex::{Bibliography, Chunk, Entry, PermissiveType};
use chrono::NaiveDate;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DoiError {
    #[error("Curl could not retrieve DOI")]
    CurlError(#[from] CurlError),

    #[error("DOI was not found in HTML")]
    DoiNotInHtmlError,

    #[error("Bibtex string could not be parsed")]
    BibtexParseError,
}

fn doi_regex_match(string: &str) -> Result<&str, DoiError> {
    // Pattern taken from: https://www.crossref.org/blog/dois-and-matching-regular-expressions/
    // Matches 97% of tested DOIs.
    let doi_pattern = r#"(\b10\.\d{4,9}/[-.;()/:\w]+)"#;
    let re = Regex::new(doi_pattern).unwrap();

    let mut results = vec![];
    for (_, [found_doi]) in re.captures_iter(string).map(|c| c.extract()) {
        results.push(found_doi);
    }

    // TODO: what to do if more matches?
    let result = results.get(0);
    result.cloned().ok_or(DoiError::DoiNotInHtmlError)
}

/// Tries to find a DOI link in the HTML.
fn try_find_doi_in_html(html: &str) -> Result<String, DoiError> {
    let doi_in_text = doi_regex_match(html);
    return doi_in_text.map(str::to_string);
}

/// Returns a Bibtex entry in string format by calling the DOI API.
/// See https://citation.crosscite.org/docs.html for more information.
fn send_doi_request(doi: &str) -> std::result::Result<String, DoiError> {
    let full_doi = format!("https://doi.org/{}", doi);
    let header_opt = Some("Accept: application/x-bibtex");
    let follow_location = true;

    Ok(get(full_doi.as_str(), header_opt, follow_location)?)
}

/// The function first tries to find a DOI address in the HTML.
/// If found,
pub fn try_doi_to_bib(html: &str, contained: &bool) -> Result<Bibliography, ReferenceGenerationError> {
    if !contained {
        return Err(ReferenceGenerationError::ParseSkip)
    }

    let doi_address = try_find_doi_in_html(html)?;
    let doi_response = send_doi_request(doi_address.as_str())?;

    let bib = Bibliography::parse(doi_response.as_str()).map_err(|_| DoiError::BibtexParseError)?;
    Ok(bib)
}

fn author_to_attribute(entry: &Entry) -> Option<Attribute> {
    let persons = entry.author().ok()?;

    let authors: Vec<Author> = persons
        .iter()
        .map(|p| {
            let formatted_name = format!("{} {}", p.given_name, p.name);
            Author::Person(formatted_name)
        })
        .collect();

    Some(Attribute::Authors(authors))
}

fn try_create_internal_date(datetime: &biblatex::Datetime) -> Option<Date> {
    match (datetime.year, datetime.month, datetime.day) {
        (year, Some(month), Some(day)) => {
            let naive_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)?;
            Some(Date::YearMonthDay(naive_date))
        }
        (year, Some(month), None) => Some(Date::YearMonth {
            year: year,
            month: month as i32,
        }),
        (year, None, None) => Some(Date::Year(year)),
        _ => None,
    }
}

fn date_to_attribute(pt: &PermissiveType<biblatex::Date>) -> Option<Attribute> {
    match pt {
        PermissiveType::Typed(date_type) => {
            if date_type.approximate || date_type.uncertain {
                return None;
            }

            match date_type.value {
                biblatex::DateValue::At(datetime) => {
                    let date = try_create_internal_date(&datetime)?;
                    Some(Attribute::Date(date))
                }
                _ => None,
            }
        }
        PermissiveType::Chunks(_) => None,
    }
}

fn string_from_chunk(chunk: &Chunk) -> Option<&str> {
    match chunk {
        Chunk::Normal(string) => Some(string.as_str()),
        Chunk::Verbatim(string) => Some(string.as_str()),
        Chunk::Math(_) => None,
    }
}

fn permissive_to_string<T: biblatex::Type + std::fmt::Debug>(
    pt: &PermissiveType<T>,
) -> Option<String> {
    match pt {
        PermissiveType::Typed(value) => Some(format!("{:?}", value)),
        PermissiveType::Chunks(chunks) => {
            let chunk = &chunks.get(0)?.v;
            string_from_chunk(&chunk).map(str::to_string)
        }
    }
}

#[rustfmt::skip]
fn attribute_type_to_attribute(entry: &Entry, attribute_type: AttributeType) -> Option<Attribute> {
    match attribute_type {
        AttributeType::Title    => {
            let chunks = entry.title().ok()?;
            let chunk = &chunks.get(0)?.v;
            let value = string_from_chunk(&chunk)?;

            Some(Attribute::Title(value.to_string()))
        },
        AttributeType::Author   => author_to_attribute(entry),
        AttributeType::Url      => {
            let url = entry.url().ok()?;
            Some(Attribute::Url(url))
        },
        AttributeType::Date     => {
            let pt = entry.date().ok()?;
            date_to_attribute(&pt)
        },
        AttributeType::Type     => {
            let entry_type = entry.type_().ok()?;
            Some(Attribute::Type(entry_type))
        },
        AttributeType::Journal  => {
            let chunks = entry.journal().ok()?;
            let chunk = &chunks.get(0)?.v;
            let value = string_from_chunk(&chunk)?;

            Some(Attribute::Journal(value.to_string()))
        },
        AttributeType::Volume  => {
            let pt = entry.volume().ok()?;
            let value = permissive_to_string(&pt)?;
            Some(Attribute::Volume(value.to_string()))
        },
        AttributeType::Language => {
            let lang = entry.language().ok()?;
            Some(Attribute::Language(lang))
        },
        AttributeType::Publisher => {
            // TODO: this type is extremely complicated. Probably need to go through
            // some examples and see what values publisher can take.
            let vec_of_chunks = entry.publisher().ok()?;
            let chunks = vec_of_chunks.get(0)?;
            let chunk = &chunks.get(0)?.v;
            let value = string_from_chunk(&chunk)?;

            Some(Attribute::Publisher(value.to_string()))
        },
        AttributeType::Institution  => {
            let chunks = entry.journal().ok()?;
            let chunk = &chunks.get(0)?.v;
            let value = string_from_chunk(&chunk)?;

            Some(Attribute::Institution(value.to_string()))
        },
        AttributeType::Locale   => None,
        AttributeType::Site     => None,
    }
}

pub struct Doi;

impl AttributeParser for Doi {
    fn parse_attribute(parse_info: &ParseInfo, attribute_type: AttributeType) -> Option<Attribute> {
        let bib = parse_info.bibliography.as_ref()?;

        assert!(
            bib.len() == 1,
            "Parsed Bibtex contained more than one entry, was input ok?"
        );

        // Ignore any extra entries.
        let root_entry = bib.iter().next()?;
        attribute_type_to_attribute(root_entry, attribute_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{doi_regex_match, send_doi_request};

    #[test]
    fn match_regex_doi() {
        let text = r#"https://doi.org/10.48550/arXiv.1712.01815"#;

        let result_option = doi_regex_match(text);
        assert!(result_option.is_ok());

        let result = result_option.unwrap();
        let expected = r#"10.48550/arXiv.1712.01815"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_doi_request() {
        let doi = "10.1126/science.169.3946.635";
        let expected = " @article{Frank_1970, title={The Structure of Ordinary Water: New data and interpretations are yielding new insights into this fascinating substance.}, volume={169}, ISSN={1095-9203}, url={http://dx.doi.org/10.1126/science.169.3946.635}, DOI={10.1126/science.169.3946.635}, number={3946}, journal={Science}, publisher={American Association for the Advancement of Science (AAAS)}, author={Frank, Henry S.}, year={1970}, month=aug, pages={635–641} }\n";

        let result = send_doi_request(doi);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
