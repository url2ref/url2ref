//! Integration testing suite.

use url2ref::GenerationOptions;
use url2ref::generator::*;
use url2ref::generator::attribute_config::*;

mod utils;
use utils::{get_file_pairs, get_expected_results, compared_attributes_with_expected};

const DATA_SAMPLES_PATH: &str = "./tests/data";

/// Create generation options with archive fetching disabled (for testing)
fn test_generation_options(priorities: AttributePriority) -> GenerationOptions {
    GenerationOptions {
        attribute_config: AttributeConfig::new(priorities),
        translation_options: TranslationOptions::default(),
        archive_options: ArchiveOptions {
            include_archived: false,
            perform_archival: false,
        },
        ai_options: AiExtractionOptions::default(),
    }
}

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
    use MetadataType::*;
    let expected_results = get_expected_results(expected_results_path);

    for (metadata_parser, expected_attributes) in expected_results.iter() {
        let generation_options = match metadata_parser {
            OpenGraph => {
                let priorities = AttributePriority { priority: vec!(OpenGraph)};
                test_generation_options(priorities)
            },
            SchemaOrg => {
                let priorities = AttributePriority { priority: vec!(SchemaOrg)};
                test_generation_options(priorities)
            },
            Doi => {
                let priorities = AttributePriority { priority: vec!(Doi)};
                test_generation_options(priorities)
            },
            HtmlMeta => {
                let priorities = AttributePriority { priority: vec!(HtmlMeta)};
                test_generation_options(priorities)
            },
            Zotero => {
                let priorities = AttributePriority { priority: vec!(Zotero)};
                test_generation_options(priorities)
            },
            Ai => {
                // AI tests are handled separately
                continue;
            }
        };

        compared_attributes_with_expected(html_path, &expected_attributes, &generation_options);
    }
}