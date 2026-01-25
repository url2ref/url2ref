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
                format!("| author{i} = {}", normalized)
            };
            match author {
                Author::Person(str) => {
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] => {
                            let first_names = first_names.join(" ");
                            format!("| last{i} = {last_name}\n| first{i} = {first_names}")
                        }
                        _ => default(str),
                    }
                },
                Author::Organization(str) => default(str),
                Author::Generic(str) => {
                    // Try to split generic author names into first/last like Person
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] if !first_names.is_empty() => {
                            let first_names = first_names.join(" ");
                            format!("| last{i} = {last_name}\n| first{i} = {first_names}")
                        }
                        _ => default(str),
                    }
                },
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
            Attribute::Title(val) => Some(format!("| title = {}", val.to_string())),
            Attribute::TranslatedTitle(trans) => Some(format!("| trans-title = {}", trans.text)),
            Attribute::Authors(vals) => Some(self.handle_authors(vals)),
            Attribute::Date(val) => Some(format!("| date = {}", self.handle_date(val))),
            Attribute::ArchiveDate(val) => Some(format!("| archive-date = {}", self.handle_date(val))),
            Attribute::Language(val) => Some(format!("| language = {}", val.to_string())),
            Attribute::Site(val) => Some(format!("| site = {}", val.to_string())),
            Attribute::Url(val) => Some(format!("| url = {}", val.to_string())),
            Attribute::ArchiveUrl(val) => Some(format!("| archive-url = {}", val.to_string())),
            Attribute::Journal(val) => Some(format!("| journal = {}", val.to_string())),
            Attribute::Publisher(val) => Some(format!("| publisher = {}", val.to_string())),
            _ => None
        };

        if let Some(parsed_value) = result_option {
            self.formatted_string.push_str(&format!("\n{}", parsed_value));
        }
        self
    }

    fn build(self) -> String {
        format!("{{{{cite web{}\n}}}}", self.formatted_string)
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
                Author::Organization(str) => default(str),
                Author::Generic(str) => {
                    // Try to split generic author names into first/last like Person
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] if !first_names.is_empty() => {
                            let first_names = first_names.join(" ");
                            format!("{last_name}, {first_names}")
                        }
                        _ => default(str),
                    }
                },
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
            Attribute::TranslatedTitle(trans) => Some(format!("note = \"Translated title: {}\"", trans.text)),
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

/// Builds a citation using the Harvard referencing style.
///
/// Harvard style uses author-date format: Author (Year) 'Title', Site/Publisher. Available at: URL (Accessed: Date).
pub struct HarvardCitation {
    authors: Option<String>,
    year: Option<String>,
    title: Option<String>,
    site: Option<String>,
    publisher: Option<String>,
    url: Option<String>,
    access_date: Option<String>,
}
impl HarvardCitation {
    fn format_authors(&self, authors: &[Author]) -> String {
        // Harvard style: "LastName, F." for single author, "LastName, F. and LastName, F." for two,
        // "LastName, F. et al." for three or more
        fn format_single_author(author: &Author) -> String {
            let get_initials = |first_names: &str| -> String {
                first_names
                    .split_whitespace()
                    .map(|name| format!("{}.", name.chars().next().unwrap_or(' ')))
                    .collect::<Vec<String>>()
                    .join("")
            };

            match author {
                Author::Person(str) | Author::Generic(str) => {
                    let parts: Vec<&str> = str.split_whitespace().collect();
                    match parts.as_slice() {
                        [first_names @ .., last_name] if !first_names.is_empty() => {
                            let initials = get_initials(&first_names.join(" "));
                            format!("{}, {}", last_name, initials)
                        }
                        [single_name] => single_name.to_string(),
                        _ => str.to_string(),
                    }
                }
                Author::Organization(str) => str.to_string(),
            }
        }

        match authors.len() {
            0 => String::new(),
            1 => format_single_author(&authors[0]),
            2 => format!("{} and {}", 
                format_single_author(&authors[0]), 
                format_single_author(&authors[1])),
            _ => format!("{} et al.", format_single_author(&authors[0])),
        }
    }

    fn extract_year(&self, date: &Date) -> String {
        match date {
            Date::DateTime(dt) => dt.format("%Y").to_string(),
            Date::YearMonthDay(nd) => nd.format("%Y").to_string(),
            Date::YearMonth { year, .. } => year.to_string(),
            Date::Year(year) => year.to_string(),
        }
    }

    fn format_access_date(&self, date: &Date) -> String {
        // Harvard style access date: "1 January 2024"
        match date {
            Date::DateTime(dt) => dt.format("%-d %B %Y").to_string(),
            Date::YearMonthDay(nd) => nd.format("%-d %B %Y").to_string(),
            Date::YearMonth { year, month } => format!("{} {}", month_name(*month), year),
            Date::Year(year) => year.to_string(),
        }
    }
}

/// Convert month number to name
fn month_name(month: i32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

impl CitationBuilder for HarvardCitation {
    fn new() -> Self {
        Self {
            authors: None,
            year: None,
            title: None,
            site: None,
            publisher: None,
            url: None,
            access_date: None,
        }
    }

    fn try_add(self, attribute_option: &Option<Attribute>) -> Self {
        match attribute_option {
            Some(attribute) => self.add(attribute),
            None => self,
        }
    }

    fn add(mut self, attribute: &Attribute) -> Self {
        match attribute {
            Attribute::Title(val) => self.title = Some(val.to_string()),
            Attribute::Authors(vals) => self.authors = Some(self.format_authors(vals)),
            Attribute::Date(val) => self.year = Some(self.extract_year(val)),
            Attribute::Site(val) => self.site = Some(val.to_string()),
            Attribute::Publisher(val) => self.publisher = Some(val.to_string()),
            Attribute::Url(val) => self.url = Some(val.to_string()),
            Attribute::ArchiveDate(val) => self.access_date = Some(self.format_access_date(val)),
            _ => {}
        }
        self
    }

    fn build(self) -> String {
        // Harvard format: Author (Year) 'Title', Site/Publisher. Available at: URL (Accessed: Date).
        let mut result = String::new();

        // Author and year
        match (&self.authors, &self.year) {
            (Some(authors), Some(year)) => result.push_str(&format!("{} ({})", authors, year)),
            (Some(authors), None) => result.push_str(&format!("{} (n.d.)", authors)),
            (None, Some(year)) => result.push_str(&format!("({})", year)),
            (None, None) => result.push_str("(n.d.)"),
        }

        // Title (in single quotes for web pages)
        if let Some(title) = &self.title {
            result.push_str(&format!(" '{}'", title));
        }

        // Site or Publisher
        let source = self.site.as_ref().or(self.publisher.as_ref());
        if let Some(src) = source {
            result.push_str(&format!(", {}", src));
        }

        result.push('.');

        // URL
        if let Some(url) = &self.url {
            result.push_str(&format!(" Available at: {}", url));
        }

        // Access date
        if let Some(date) = &self.access_date {
            result.push_str(&format!(" (Accessed: {}).", date));
        } else if self.url.is_some() {
            // Add a period after URL if no access date
            // (period already added after source)
        }

        result
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
        let expected_result = format!("{{{{cite web\n| title = {title}\n}}}}");

        assert_eq!(wiki_citation, expected_result)
    }
}