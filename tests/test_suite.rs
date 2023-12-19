//! Integration testing suite.

use url2ref::GenerationOptions;
use url2ref::generator::*;
use url2ref::generator::attribute_config::*;

mod utils;
use utils::{get_file_pairs, get_expected_results, compared_attributes_with_expected};

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
            MetadataType::OpenGraph => {
                let priorities = AttributePriority { priority: vec!(MetadataType::OpenGraph)};
                GenerationOptions {
                    attribute_config: AttributeConfig::new(priorities),
                    ..Default::default()
                }
            },
            MetadataType::SchemaOrg => {
                let priorities = AttributePriority { priority: vec!(MetadataType::SchemaOrg)};
                GenerationOptions {
                    attribute_config: AttributeConfig::new(priorities),
                    ..Default::default()
                }
            },
        };

        compared_attributes_with_expected(html_path, &expected_attributes, &generation_options);
    }
}