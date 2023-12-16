//! Integration testing suite.

use url2ref::{Reference, GenerationOptions};
use url2ref::attribute::{Attribute, Author};
use url2ref::generator::AttributeConfigList;

mod utils;
use utils::{Parser, get_file_pairs, get_expected_results};

const DATA_SAMPLES_PATH: &str = "./tests/data";

/// Performs testing for all the external HTML data samples by comparing the
/// reference generation output to the expected values associated with each
/// test sample.
#[test]
fn test_all() {
    let file_pairs = get_file_pairs(DATA_SAMPLES_PATH);

    for (html_path, expected_results_path) in &file_pairs {
        check(html_path, expected_results_path);
    }
}

/// Prepares the appropriate [`GenerationOptions`] for the set of expected
/// reference generation results obtained using each [`Parser`] and calls the
/// [`actual_check`] function to perform the comparison.
fn check(html_path: &str, expected_results_path: &str) {
    let expected_results = get_expected_results(expected_results_path);

    for (metadata_parser, expected_attributes) in expected_results.iter() {
        let generation_options = match metadata_parser {
            Parser::OpenGraph => GenerationOptions::new(vec!(AttributeConfigList::default_opengraph())),
            Parser::SchemaOrg => GenerationOptions::new(vec!(AttributeConfigList::default_schema_org())),
        };

        actual_check(html_path, &expected_attributes, generation_options);
    }
}

/// Compares a HTML data sample to the expected reference generation results
/// obtained according to a particular set of [`GenerationOptions`].
fn actual_check(html_path: &str, expected_attributes: &Vec<Attribute>, generation_options: url2ref::GenerationOptions) {
    let reference_result = url2ref::generate_from_file(html_path, generation_options);
    assert_eq!(reference_result.is_err(), false, "Reference generation shouldn't fail with error");
    let reference = reference_result.unwrap();

    /// Function for comparing two [`Attribute`]s
    fn compare_attributes(actual_attribute: &Option<Attribute>, expected_attribute: &Attribute) {
        assert!(actual_attribute.is_some());
        assert_eq!(actual_attribute.clone().unwrap(), expected_attribute.clone());
    }

    // TODO: Resolve this...
    println!("{:?}", reference);
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
        for attribute in expected_attributes {
            match attribute {
                Attribute::Title(_) => {
                    compare_attributes(&title, attribute);
                }
                Attribute::Authors(author_list) => {
                    let authors_cloned = author.clone();

                    // Small hack: .yml files do not discern between author types,
                    // so all are converted to Author::Generic.
                    assert!(authors_cloned.is_some());
                    let to_generic: Vec<Author> = author_list.iter().map(|a| {
                        match a {
                            Author::Person(s) => Author::Generic(s.clone()),
                            Author::Organization(s) => Author::Generic(s.clone()),
                            Author::Generic(_) => a.clone()
                        }
                    }).collect();

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