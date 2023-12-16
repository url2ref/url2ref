//! Module providing functionality for building up citations
//! in various formats using the Builder pattern.

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::attribute::{Attribute, Author};

pub trait CitationBuilder {
    fn new() -> Self;
    fn try_add(self, attribute_option: &Option<Attribute>) -> Self;
    fn add(self, attribute: &Attribute) -> Self;
    fn build(self) -> String;
}

/// Builds a citation using the [{{cite web}} template] from the English Wikipedia
///
/// [{{cite web}} template]: https://en.wikipedia.org/wiki/Template:Cite_web
pub struct WikiCitation {
    formatted_string: String,
}
impl WikiCitation {
    // TODO: Implement
    fn handle_authors(&self, authors: &Vec<Author>) -> String {
        todo!()
    }
}
impl CitationBuilder for WikiCitation {
    fn new() -> Self {
        WikiCitation { formatted_string: String::from("") }
    }

    fn try_add(self, attribute_option: &Option<Attribute>) -> Self {
        match attribute_option {
            Some(attribute) => self.add(&attribute),
            None => self,
        }
    }

    fn add(mut self,  attribute: &Attribute) -> Self {
        let result_option = match attribute {
            Attribute::Title(val) => Some(format!("|title={}", val.to_string())),
            Attribute::Authors(vals) => Some(self.handle_authors(vals)),
            Attribute::Date(val) => Some(format!("|date={}", val.format("%Y-%m-%d").to_string())),
            Attribute::Language(val) => Some(format!("|language={}", val.to_string())),
            Attribute::Site(val) => Some(format!("|site={}", val.to_string())),
            Attribute::Url(val) => Some(format!("|url={}", val.to_string())),
            Attribute::Journal(val) => Some(format!("|journal={}", val.to_string())),
            Attribute::Publisher(val) => Some(format!("|publisher={}", val.to_string())),
            _ => None
        };

        if let Some(parsed_value) = result_option {
            self.formatted_string.push_str(&format!(" {}", parsed_value));
        }
        self
    }

    fn build(self) -> String {
        format!("{{{{cite web{} }}}}", self.formatted_string)
    }
}

/// Builds a citation using the [BibTeX entry template] for LaTeX.
///
/// [BibTeX entry template]: https://www.bibtex.org/Format/
pub struct BibTeXCitation {
    formatted_string: String,
}
impl BibTeXCitation {
    // TODO: Implement
    fn handle_authors(&self, authors: &Vec<String>) -> String {
        todo!()
    }
}
impl CitationBuilder for BibTeXCitation {
    fn new() -> Self {
        BibTeXCitation { formatted_string: String::from("") }
    }

    fn try_add(self, attribute_option: &Option<Attribute>) -> Self {
        match attribute_option {
            Some(attribute) => self.add(&attribute),
            None => self,
        }
    }

    fn add(mut self,  attribute: &Attribute) -> Self {
        let result_option = match attribute {
            Attribute::Title(val) => Some(format!("title = {{{}}}", val.to_string())),
            Attribute::Authors(vals) => None, // TODO: Implement
            Attribute::Date(val) => Some(format!("date = {{{}}}", val.format("%Y-%m-%d").to_string())),
            Attribute::Language(val) => Some(format!("language = {{{}}}", val.to_string())),
            Attribute::Site(val) => Some(format!("site = {{{}}}", val.to_string())),
            Attribute::Url(val) => Some(format!("url = {{{}}}", val.to_string())),
            Attribute::Journal(val) => Some(format!("journal = {{{}}}", val.to_string())),
            Attribute::Publisher(val) => Some(format!("publisher = {{{}}}", val.to_string())),
            _ => None
        };

        if let Some(parsed_value) = result_option {
            self.formatted_string.push_str(&format!("\n{},", parsed_value));
        }
        self
    }

    fn build(self) -> String {
        format!("@misc{{ url2ref,{} }}", self.formatted_string)
    }
}

/// Builds a citation in [APA style].
///
/// [APA style]: https://en.wikipedia.org/wiki/APA_style
pub struct APACitation {
    formatted_string: String,
}
impl APACitation {
    // TODO: Implement
    fn handle_authors(&self, authors: &Vec<String>) -> String {
        todo!();
    }
}
impl CitationBuilder for APACitation {
    fn new() -> Self {
        APACitation { formatted_string: String::from("") }
    }

    fn try_add(self, attribute_option: &Option<Attribute>) -> Self {
        match attribute_option {
            Some(attribute) => self.add(&attribute),
            None => self,
        }
    }

    fn add(self,  attribute: &Attribute) -> Self {
        let result_option = match attribute {
            Attribute::Title(val) => Some(("title", val)),
            Attribute::Authors(vals) => None, // TODO: Implement
            Attribute::Date(val) => None, // TODO: Implement
            Attribute::Language(val) => Some(("language", val)),
            Attribute::Site(val) => Some(("site", val)),
            Attribute::Url(val) => Some(("url", val)),
            Attribute::Journal(val) => Some(("journal", val)),
            Attribute::Publisher(val) => Some(("publisher", val)),
            _ => None
        };

        self
    }

    fn build(self) -> String {
        format!("{}", self.formatted_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_citation_try_add() {
        let title = "Test title";
        let attribute = Attribute::Title(title.to_string());

        let wiki_citation = WikiCitation::new()
            .try_add(&Some(attribute))
            .build();
        let expected_result = format!("{{{{cite web |title={title} }}}}");

        assert_eq!(wiki_citation, expected_result)
    }
}