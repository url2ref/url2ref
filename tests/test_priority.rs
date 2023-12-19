//! Integration testing for prioritizing different metadata types.

mod utils;
use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
use url2ref::generator::MetadataType;
use MetadataType::*;
use url2ref::GenerationOptions;

use utils::{compared_attributes_with_expected, get_expected_results};

fn check(html_path: &str, expected_results_path: &str, order: Vec<MetadataType>) {
    let priority_type = order[0].clone();
    let config = AttributeConfig::new(AttributePriority { priority: order });

    let options = GenerationOptions {
        config: config,
        ..Default::default()
    };

    let results = get_expected_results(expected_results_path);
    let metadata_results = &results[&priority_type];

    compared_attributes_with_expected(html_path, metadata_results, &options);
}


#[test]
fn test_priority_opengraph_over_schema_case2() {
    let html_path = "./tests/data/case2/jyllands-posten_dk_2023-12-13.html";
    let expected_results_path = "./tests/data/case2/expected.yml";
    let order = vec![OpenGraph, SchemaOrg];

    check(html_path, expected_results_path, order);
}

#[test]
fn test_priority_schema_over_opengraph_case2() {
    let html_path = "./tests/data/case2/jyllands-posten_dk_2023-12-13.html";
    let expected_results_path = "./tests/data/case2/expected.yml";
    let order = vec![SchemaOrg, OpenGraph];

    check(html_path, expected_results_path, order);
}

#[test]
fn test_priority_opengraph_over_schema_case7() {
    let html_path = "./tests/data/case7/the-sun_co-uk_2023-12-14.html";
    let expected_results_path = "./tests/data/case7/expected.yml";
    let order = vec![OpenGraph, SchemaOrg];

    check(html_path, expected_results_path, order);
}

#[test]
fn test_priority_schema_over_opengraph_case7() {
    let html_path = "./tests/data/case7/the-sun_co-uk_2023-12-14.html";
    let expected_results_path = "./tests/data/case7/expected.yml";
    let order = vec![SchemaOrg, OpenGraph];

    check(html_path, expected_results_path, order);
}
