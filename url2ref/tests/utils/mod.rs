//! Common utilities for integration testing.

use chrono::{DateTime, Utc, TimeZone};
use serde_yaml::{from_reader, Mapping, Value};
use std::{collections::HashMap, fs::read_dir, fs::File, path::PathBuf};

use url2ref::{
    attribute::{Attribute, Author, Date},
    GenerationOptions, Reference,
    generator::MetadataType
};

pub fn parse_mapping(map: &Mapping) -> Vec<Attribute> {
    map.iter()
        .map(|(key, value)| match (key, value) {
            (Value::String(attribute_name), Value::String(attribute_value)) => {
                string_to_attribute(attribute_name, attribute_value)
            }
            (Value::String(_), Value::Sequence(seq)) => seq_to_attribute(seq),
            _ => panic!("Unexpected attribute"),
        })
        .collect()
}

pub fn get_expected_results(path: &str) -> HashMap<MetadataType, Vec<Attribute>> {
    let f = File::open(path).unwrap();
    let d: Value = from_reader(f).unwrap();

    let mut expected_attributes: HashMap<MetadataType, Vec<Attribute>> = HashMap::new();

    if let Value::Mapping(root) = d {
        for (key, value) in root.iter() {
            match (key, value) {
                (Value::String(parser_string), Value::Mapping(fields)) => {
                    //println!("{:?} {:?}", parser_string, fields);
                    //let mut attribute_list = Vec::new();
                    let parser = string_to_parser(parser_string);
                    let attributes = parse_mapping(&fields);

                    expected_attributes.insert(parser, attributes);
                }
                _ => panic!("Invalid .yml"),
            }
        }
    }

    expected_attributes
}

fn parse_date(date_str: &str) -> Date {
    let dt_opt = DateTime::parse_from_rfc3339(date_str).ok();
    if let Some(dt) = dt_opt {
        let dt_utc = Utc.from_utc_datetime(&dt.naive_utc());
        return Date::DateTime(dt_utc)
    }

    println!("{:?}", date_str);
    let naive_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .expect("No valid conversion of date string found");
    Date::YearMonthDay(naive_date)
}

pub fn string_to_attribute(field: &String, value: &String) -> Attribute {
    match field.as_str() {
        "title" => Attribute::Title(value.clone()),
        "author" => Attribute::Authors(vec![Author::Generic(value.clone())]),
        "site" => Attribute::Site(value.clone()),
        "date" => Attribute::Date(parse_date(value.as_str())),
        "language" => Attribute::Language(value.clone()),
        "locale" => Attribute::Locale(value.clone()),
        "url" => Attribute::Url(value.clone()),
        "journal" => Attribute::Journal(value.clone()),
        "publisher" => Attribute::Publisher(value.clone()),
        _ => panic!("Unknown attribute"),
    }
}

pub fn seq_to_attribute(seq: &Vec<Value>) -> Attribute {
    let mut authors = Vec::new();
    for value in seq {
        match value {
            Value::String(name) => authors.push(Author::Generic(name.clone())),
            _ => panic!("Invalid type in sequence"),
        };
    }

    Attribute::Authors(authors)
}

pub fn string_to_parser(field: &String) -> MetadataType {
    match field.as_str() {
        "opengraph" => MetadataType::OpenGraph,
        "schema_org" => MetadataType::SchemaOrg,
        _ => panic!("Unknown parser specified"),
    }
}

/// Collects pairs of .html samples and .yml files describing the expected
/// reference attributes.
#[allow(dead_code)]
pub fn get_file_pairs(path: &str) -> Vec<(String, String)> {
    let dirs = read_dir(path).unwrap();

    // Get case directory files (e.g. case1, case2)
    let case_dirs: Vec<PathBuf> = dirs
        .map(|dir| dir.unwrap().path())
        .filter(|dir| dir.is_dir().clone())
        .collect();

    // Collect all files from each directory [[files in case1], [files in case2], ...]
    let collected_files: Vec<Vec<PathBuf>> = case_dirs
        .iter()
        .map(|dir| {
            let dirs = read_dir(dir.as_path()).unwrap();
            let files = dirs
                .map(|f| f.unwrap().path().clone())
                .collect::<Vec<PathBuf>>();
            files
        })
        .collect();

    let sorted_file_pairs: Vec<(String, String)> = collected_files
        .iter()
        .map(|files| {
            assert!(files.len() == 2);

            let p1 = &files[0];
            let p2 = &files[1];

            let p1_string = p1.clone().into_os_string().into_string().unwrap();
            let p2_string = p2.clone().into_os_string().into_string().unwrap();

            if p1.extension().unwrap() == "html" {
                assert!(p2.extension().unwrap() == "yml" || p2.extension().unwrap() == "yaml");
                return (p1_string, p2_string);
            }

            assert!(p1.extension().unwrap() == "yml" || p1.extension().unwrap() == "yaml");
            (p2_string, p1_string)
        })
        .collect();

    sorted_file_pairs
}

/// Compares a HTML data sample to the expected reference generation results
/// obtained according to a particular set of [`GenerationOptions`].
pub fn compared_attributes_with_expected(
    html_path: &str,
    expected_attributes: &Vec<Attribute>,
    generation_options: &GenerationOptions,
) {
    let reference_result = url2ref::generate_from_file(html_path, generation_options);
    assert_eq!(
        reference_result.is_err(),
        false,
        "Reference generation shouldn't fail with error"
    );
    let reference = reference_result.unwrap();

    /// Function for comparing two [`Attribute`]s
    fn compare_attributes(actual_attribute: &Option<Attribute>, expected_attribute: &Attribute) {
        assert!(actual_attribute.is_some());
        assert_eq!(
            actual_attribute.clone().unwrap(),
            expected_attribute.clone()
        );
    }

    // TODO: Resolve this...
    println!("{:?}", reference);
    assert!(matches!(reference, Reference::NewsArticle { .. }));
    if let Reference::NewsArticle {
        title,
        translated_title,
        author,
        date,
        language,
        site,
        url,
    } = reference
    {
        for attribute in expected_attributes {
            match attribute {
                Attribute::Title(_) => {
                    compare_attributes(&title, attribute);
                }
                Attribute::TranslatedTitle(_) => {
                    compare_attributes(&translated_title, attribute)
                },
                Attribute::Authors(author_list) => {
                    let authors_cloned = author.clone();

                    // Small hack: .yml files do not discern between author types,
                    // so all are converted to Author::Generic.
                    assert!(authors_cloned.is_some());
                    let to_generic: Vec<Author> = author_list
                        .iter()
                        .map(|a| match a {
                            Author::Person(s) => Author::Generic(s.clone()),
                            Author::Organization(s) => Author::Generic(s.clone()),
                            Author::Generic(_) => a.clone(),
                        })
                        .collect();

                    assert_eq!(Attribute::Authors(to_generic), attribute.clone());
                }
                Attribute::Date(_) => {
                    compare_attributes(&date, attribute);
                }
                Attribute::Language(_) => {
                    compare_attributes(&language, attribute);
                }
                Attribute::Site(_) => {
                    compare_attributes(&site, attribute);
                }
                Attribute::Url(_) => {
                    compare_attributes(&url, attribute);
                }
                _ => panic!("Non-viable test attribute used"),
            }
        }
    }
}
