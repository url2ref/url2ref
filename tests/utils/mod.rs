//! Common utilities for integration testing.

use serde_yaml::{Value, from_reader, Mapping};
use std::{collections::HashMap, path::PathBuf, fs::File, fs::read_dir};

use url2ref::attribute::{Attribute, Author};

/// Parser types to distinguish between reference generation results
/// obtained from different metadata annotations.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Parser {
    OpenGraph,
    SchemaOrg,
}

pub fn parse_mapping(map: &Mapping) -> Vec<Attribute> {
    map.iter().map(|(key, value)| match (key, value) {
        (Value::String(attribute_name), Value::String(attribute_value)) => string_to_attribute(attribute_name, attribute_value),
        (Value::String(_), Value::Sequence(seq)) => seq_to_attribute(seq),
        _ => panic!("Unexpected attribute")

    }).collect()
}

pub fn get_expected_results(path: &str) -> HashMap<Parser, Vec<Attribute>> {
    let f = File::open(path).unwrap();
    let d: Value = from_reader(f).unwrap();

    let mut expected_attributes: HashMap<Parser, Vec<Attribute>> = HashMap::new();

    if let Value::Mapping(root) = d {
        for (key, value) in root.iter() {

            match (key, value) {
                (Value::String(parser_string), Value::Mapping(fields)) => {
                    //println!("{:?} {:?}", parser_string, fields);
                    //let mut attribute_list = Vec::new();
                    let parser = string_to_parser(parser_string);
                    let attributes = parse_mapping(&fields);

                    expected_attributes.insert(parser, attributes);
                },
                _ => panic!("Invalid .yml")
            }
        }
    }

    expected_attributes
}

pub fn string_to_attribute(field: &String, value: &String) -> Attribute {
    match field.as_str() {
       "title"     => Attribute::Title(value.clone()),
       "author"    => Attribute::Authors(vec![Author::Generic(value.clone())]),
       "site"      => Attribute::Site(value.clone()),
       "date"      => {
            let date = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap();
            Attribute::Date(date)
       },
       "language"  => Attribute::Language(value.clone()),
       "locale"    => Attribute::Locale(value.clone()),
       "url"       => Attribute::Url(value.clone()),
       "journal"   => Attribute::Journal(value.clone()),
       "publisher" => Attribute::Publisher(value.clone()),
       _ => panic!("Unknown attribute")
    }
}

pub fn seq_to_attribute(seq: &Vec<Value>) -> Attribute {
    let mut authors = Vec::new();
    for value in seq {
        match value {
            Value::String(name) => authors.push(Author::Generic(name.clone())),
            _ => panic!("Invalid type in sequence")
        };
    }

    Attribute::Authors(authors)
}

pub fn string_to_parser(field: &String) -> Parser {
    match field.as_str() {
        "opengraph" => Parser::OpenGraph,
        "schema_org" => Parser::SchemaOrg,
        _ => panic!("Unknown parser specified")
    }
}

/// Collects pairs of .html samples and .yml files describing the expected
/// reference attributes.
pub fn get_file_pairs(path: &str) -> Vec<(String, String)> {
    let dirs = read_dir(path).unwrap();

    // Get case directory files (e.g. case1, case2)
    let case_dirs: Vec<PathBuf> = dirs.map(|dir| dir.unwrap().path())
                                      .filter(|dir| dir.is_dir().clone()).collect();

    // Collect all files from each directory [[files in case1], [files in case2], ...]
    let collected_files: Vec<Vec<PathBuf>> = case_dirs.iter().map(|dir| {
        let dirs = read_dir(dir.as_path()).unwrap();
        let files = dirs.map(|f| f.unwrap()
                                                                         .path()
                                                                         .clone()).collect::<Vec<PathBuf>>();
        files

    }).collect();

    let sorted_file_pairs: Vec<(String, String)> = collected_files.iter().map(|files| {
        assert!(files.len() == 2);

        let p1 = &files[0];
        let p2 = &files[1];

        let p1_string = p1.clone().into_os_string().into_string().unwrap();
        let p2_string = p2.clone().into_os_string().into_string().unwrap();

        if p1.extension().unwrap() == "html" {
            assert!(p2.extension().unwrap() == "yml" || p2.extension().unwrap() == "yaml");
            return (p1_string, p2_string)
        }

        assert!(p1.extension().unwrap() == "yml" || p1.extension().unwrap() == "yaml");
        (p2_string, p1_string)
    }).collect();

    sorted_file_pairs
}
