use crate::attribute::{Attribute, AttributeType, Author};
use crate::generator::ReferenceGenerationError;
use crate::parser::{AttributeParser, ParseInfo};

use biblatex::{Bibliography, Entry, Chunk, PermissiveType, Date};
use curl::easy::{Easy, List};
use regex::Regex;

fn regex_match(string: &str) -> Option<&str> {
    let doi_pattern = r#"(\b10\.\d{4,9}/[-.;()/:\w]+)"#;
    let re = Regex::new(doi_pattern).unwrap();

    let mut results = vec![];
    for (_, [found_doi]) in re.captures_iter(string).map(|c| c.extract()) {
        results.push(found_doi);
    }

    // TODO: what to do if more matches?
    let result = results.get(0);
    result.cloned()
}

pub fn try_find_doi_in_html(html: &webpage::HTML) -> Option<String> {
    let text = &html.text_content;

    let doi_in_text = regex_match(text.as_str());
    return doi_in_text.map(str::to_string);
}

/// See https://citation.crosscite.org/docs.html for more information.
pub fn send_doi_request(
    doi_option: Option<&str>,
) -> std::result::Result<String, ReferenceGenerationError> {
    let doi = doi_option.ok_or(ReferenceGenerationError::DoiError)?;

    let mut easy = Easy::new();
    let mut buf = Vec::new();

    let full_doi = format!("https://doi.org/{}", doi);

    // Header determines output format
    let mut header_list = List::new();
    let header = "Accept: application/x-bibtex";
    header_list.append(header)?;

    easy.follow_location(true)?;
    easy.http_headers(header_list)?;
    easy.url(full_doi.as_str())?;

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


fn author_to_attribute(entry: &Entry) -> Option<Attribute> {
    let persons = entry.author().ok()?;

    let authors: Vec<Author> = persons.iter()
        .map(|p| {
        let formatted_name = format!("{} {}", p.given_name, p.name);
        Author::Person(formatted_name)
    }).collect();

    Some(Attribute::Authors(authors))
}


fn string_from_chunk(chunk: &Chunk) -> Option<&str> {
    match chunk {
        Chunk::Normal(string) => Some(string.as_str()),
        Chunk::Verbatim(string) => Some(string.as_str()),
        Chunk::Math(_) => None
    }
}

fn permissive_to_string<T: biblatex::Type + std::fmt::Debug>(pt: &PermissiveType<T>) -> Option<String> {
    match pt {
        PermissiveType::Typed(value) => Some(format!("{:?}", value)),
        PermissiveType::Chunks(chunks) => {
            let chunk = &chunks.get(0)?.v;
            string_from_chunk(&chunk).map(str::to_string)
        },
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
        AttributeType::Site     => None,
        AttributeType::Url      => {
            let url = entry.url().ok()?;
            Some(Attribute::Url(url))
        },
        AttributeType::Date     => {
            let pt = entry.date().ok()?;
            println!("Date: {:?}", pt);
            None
        },
        AttributeType::Type     => {
            let chunks = entry.entry_subtype().ok()?;
            println!("Chunks: {:?}", chunks);
            None
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
            Some(Attribute::Journal(value.to_string()))
        },
        AttributeType::Locale => None,
        AttributeType::Language => None,
        AttributeType::Publisher => None
    }
}

pub struct Doi;

impl AttributeParser for Doi {
    fn parse_attribute(parse_info: &ParseInfo, attribute_type: AttributeType) -> Option<Attribute> {
        let bibtex_string = parse_info.doi_response.clone()?;

        let bib = Bibliography::parse(bibtex_string.as_str()).ok()?;
        assert!(bib.len() == 1, "Parsed Bibtex contained more than one entry, was input ok?");

        let root_entry = bib.iter().next()?;
        attribute_type_to_attribute(root_entry, attribute_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{regex_match, send_doi_request};

    #[test]
    fn match_regex_doi() {
        let text = r#"https://doi.org/10.48550/arXiv.1712.01815"#;

        let result_option = regex_match(text);
        assert!(result_option.is_some());

        let result = result_option.unwrap();
        let expected = r#"10.48550/arXiv.1712.01815"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_doi_request() {
        let doi = "10.1126/science.169.3946.635";
        let expected = " @article{Frank_1970, title={The Structure of Ordinary Water: New data and interpretations are yielding new insights into this fascinating substance.}, volume={169}, ISSN={1095-9203}, url={http://dx.doi.org/10.1126/science.169.3946.635}, DOI={10.1126/science.169.3946.635}, number={3946}, journal={Science}, publisher={American Association for the Advancement of Science (AAAS)}, author={Frank, Henry S.}, year={1970}, month=aug, pages={635–641} }\n";

        let result = send_doi_request(Some(doi));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
