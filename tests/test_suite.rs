use serde_yaml::{Value, from_reader, Mapping};
use std::{collections::HashMap, path::PathBuf, fs::File, fs::read_dir};
use url2ref::{attribute::Attribute, Reference, GenerationOptions};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Parser {
    OpenGraph,
    SchemaOrg,
}

#[test]
fn test_all() {
    let test_path = "./tests/data";
    let file_pairs = gather_file_pairs(test_path);

    for (html_path, expected_path) in file_pairs {
        check(html_path.as_str(), expected_path.as_str());
    }
}

fn check(html_path: &str, expected_path: &str) {
    let expected_results = open_expected(expected_path);

    for (parser, expected_attributes) in expected_results.iter() {
        let options = match parser {
            Parser::OpenGraph => GenerationOptions::default_opengraph(),
            Parser::SchemaOrg => GenerationOptions::default_schema_org(),
        };

        actual_check(html_path, &expected_attributes, options);
    }
}


fn string_to_attribute(field: &String, value: &String) -> Attribute {
    match field.as_str() {
       "title"     => Attribute::Title(value.clone()),
       "author"    => Attribute::Author(vec![value.clone()]),
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

fn seq_to_attribute(seq: &Vec<Value>) -> Attribute {
    let mut authors = Vec::new();
    for value in seq {
        match value {
            Value::String(name) => authors.push(name.clone()),
            _ => panic!("Invalid type in sequence")
        };
    }

    Attribute::Author(authors)
}

fn string_to_parser(field: &String) -> Parser {
    match field.as_str() {
        "opengraph" => Parser::OpenGraph,
        "schema_org" => Parser::SchemaOrg,
        _ => panic!("Unknown parser specified")
    }
}

fn parse_mapping(map: &Mapping) -> Vec<Attribute> {
    map.iter().map(|(key, value)| match (key, value) {
        (Value::String(attribute_name), Value::String(attribute_value)) => string_to_attribute(attribute_name, attribute_value),
        (Value::String(_), Value::Sequence(seq)) => seq_to_attribute(seq),
        _ => panic!("Unexpected attribute")

    }).collect()
}

fn open_expected(path: &str) -> HashMap<Parser, Vec<Attribute>> {
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


fn gather_file_pairs(path: &str) -> Vec<(String, String)> {
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


fn actual_check(path: &str, test_attributes: &Vec<Attribute>, options: url2ref::GenerationOptions) {
    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle {
        title,
        author,
        date,
        language,
        site,
        url,
    } = reference
    {
        for attribute in test_attributes {
            match attribute {
                Attribute::Title(_) => {
                    let title_cloned = title.clone();
                    assert!(title_cloned.as_ref().is_some());
                    assert_eq!(title_cloned.unwrap(), attribute.clone());
                }
                Attribute::Author(_) => {
                    let author_cloned = author.clone();
                    assert!(author_cloned.is_some());
                    assert_eq!(author_cloned.unwrap(), attribute.clone());
                }
                Attribute::Date(_) => {
                    let date_cloned = date.clone();
                    assert!(date_cloned.is_some());
                    assert_eq!(date_cloned.unwrap(), attribute.clone());
                }
                Attribute::Language(_) => {
                    let language_cloned = language.clone();
                    assert!(language_cloned.is_some());
                    assert_eq!(language_cloned.unwrap(), attribute.clone());
                }
                Attribute::Site(_) => {
                    let site_cloned = site.clone();
                    assert!(site_cloned.is_some());
                    assert_eq!(site_cloned.unwrap(), attribute.clone());
                }
                Attribute::Url(_) => {
                    let url_cloned = url.clone();
                    assert!(url_cloned.is_some());
                    assert_eq!(url_cloned.unwrap(), attribute.clone());
                }
                _ => panic!("Non-viable test attribute used"),
            }
        }
    }
}