//! Module providing functionality for building up citations
//! in various formats using the Builder pattern.

use crate::attribute::{Attribute, Author, Date};

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
    // Author handling; the {{cite web}} Wikipedia template
    // uses different parameters depending on the number and type of authors.
    fn handle_authors(&self, authors: &[Author]) -> String {

        // Creates a string representing an author
        // according to the {{cite web}} Wikipedia template.
        fn stringify_author(author: &Author, count: Option<i32>) -> String {
            // Determine whether index should be inserted after author parameters;
            // this must be done when there are multiple authors.
            let i = count.map(|v| v.to_string()).unwrap_or_default();
            // Trivial default case - normalize whitespace
            let default = |a: &str| {
                let normalized: String = a.split_whitespace().collect::<Vec<&str>>().join(" ");
                format!("|author{i}={}", normalized)
            };
            match author {
                Author::Person(str) => {
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] => {
                            let first_names = first_names.join(" ");
                            format!("|last{i}={last_name}|first{i}={first_names}")
                        }
                        _ => default(str),
                    }
                },
                Author::Organization(str) | Author::Generic(str) => default(str),
            }
        }

        let output: String = authors
            .iter()
            .enumerate()
            .map(|(i, author)| stringify_author(author, (authors.len() > 1).then(|| (i + 1) as i32)))
            .collect::<Vec<String>>()
            .join(" ");
        output
    }

    fn handle_date(&self, date: &Date) -> String {
        let ymd_pattern = "%Y-%m-%d";

        fn format(input: String) -> String {
            format!("{}", input)
        }

        match date {
            Date::DateTime(dt) => format(dt.format(ymd_pattern).to_string()),
            Date::YearMonthDay(nd) => format(nd.format(ymd_pattern).to_string()),
            Date::YearMonth { year, month } => format!("{}-{}", year, month),
            Date::Year(year) => format!("{}", year),
        }
    }

}
impl CitationBuilder for WikiCitation {
    fn new() -> Self {
        Self { formatted_string: String::from("") }
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
            Attribute::TranslatedTitle(trans) => Some(format!("|trans-title={} |language={}", trans.text, trans.language)),
            Attribute::Authors(vals) => Some(self.handle_authors(vals)),
            Attribute::Date(val) => Some(format!("|date={}", self.handle_date(val))),
            Attribute::ArchiveDate(val) => Some(format!("|archive-date={}", self.handle_date(val))),
            Attribute::Language(val) => Some(format!("|language={}", val.to_string())),
            Attribute::Site(val) => Some(format!("|site={}", val.to_string())),
            Attribute::Url(val) => Some(format!("|url={}", val.to_string())),
            Attribute::ArchiveUrl(val) => Some(format!("|archive-url={}", val.to_string())),
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
    fn handle_authors(&self, authors: &[Author]) -> String {

        // Creates a string representing an author in a style compatible with BibTeX markup
        fn stringify_author(author: &Author) -> String {
            // Normalize whitespace and wrap in braces for organization/generic
            let default = |a: &str| {
                let normalized: String = a.split_whitespace().collect::<Vec<&str>>().join(" ");
                format!("{{{}}}", normalized)
            };
            match author {
                Author::Person(str) => {
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] => {
                            let first_names = first_names.join(" ");
                            format!("{last_name}, {first_names}")
                        }
                        _ => default(str),
                    }
                },
                Author::Organization(str) | Author::Generic(str) => default(str),
            }
        }

        let author_list: String = authors
            .iter()
            .map(|author| stringify_author(author))
            .collect::<Vec<String>>()
            .join(" and ");
        let output = format!("author = \"{}\"", author_list);
        output
    }

    fn handle_date_value(&self, date: &Date) -> String {
        let ymd_pattern = "%Y-%m-%d";

        match date {
            Date::DateTime(dt) => dt.format(ymd_pattern).to_string(),
            Date::YearMonthDay(nd) => nd.format(ymd_pattern).to_string(),
            Date::YearMonth { year, month } => format!("{}-{:02}", year, month),
            Date::Year(year) => format!("{}", year),
        }
    }

    fn handle_date(&self, date: &Date) -> String {
        let ymd_pattern = "%Y-%m-%d";

        fn format(input: String) -> String {
            format!("date = \"{}\"", input)
        }

        match date {
            Date::DateTime(dt) => format(dt.format(ymd_pattern).to_string()),
            Date::YearMonthDay(nd) => format(nd.format(ymd_pattern).to_string()),
            Date::YearMonth { year, month } => format!("year = \"{}\",\nmonth = \"{}\"", year, month),
            Date::Year(year) => format!("year = \"{}\"", year),
        }
    }
}

impl CitationBuilder for BibTeXCitation {
    fn new() -> Self {
        Self { formatted_string: String::from("") }
    }

    fn try_add(self, attribute_option: &Option<Attribute>) -> Self {
        match attribute_option {
            Some(attribute) => self.add(&attribute),
            None => self,
        }
    }

    fn add(mut self,  attribute: &Attribute) -> Self {
        let result_option = match attribute {
            Attribute::Title(val)      => Some(format!("title = \"{}\"", val.to_string())),
            Attribute::Authors(vals)   => Some(self.handle_authors(vals)),
            Attribute::Date(val)       => Some(self.handle_date(val)),
            Attribute::Url(val)        => Some(format!("url = \\url{{{}}}", val.to_string())),
            Attribute::Site(val)       => Some(format!("howpublished = \"{}\"", val.to_string())),
            Attribute::Publisher(val)  => Some(format!("publisher = \"{}\"", val.to_string())),
            Attribute::Language(val)   => Some(format!("language = \"{}\"", val.to_string())),
            Attribute::Journal(val)    => Some(format!("journal = \"{}\"", val.to_string())),
            Attribute::ArchiveUrl(val) => Some(format!("archiveurl = \\url{{{}}}", val.to_string())),
            Attribute::ArchiveDate(val) => Some(format!("archivedate = \"{}\"", self.handle_date_value(val))),
            _ => None
        };

        if let Some(parsed_value) = result_option {
            self.formatted_string.push_str(&format!("{},\n", parsed_value));
        }
        self
    }

    fn build(self) -> String {
        format!("@misc{{ url2ref,\n{}}}", self.formatted_string)
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