use url2ref::*;
use url2ref::attribute::Attribute;

fn schema_org_test(path: &str, test_attributes: Vec<Attribute>) {
    let options = url2ref::GenerationOptions::default_schema_org();

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
                    assert_eq!(title_cloned.unwrap(), attribute);
                }
                Attribute::Author(_) => {
                    let author_cloned = author.clone();
                    assert!(author_cloned.is_some());
                    assert_eq!(author_cloned.unwrap(), attribute);
                }
                Attribute::Date(_) => {
                    let date_cloned = date.clone();
                    assert!(date_cloned.is_some());
                    assert_eq!(date_cloned.unwrap(), attribute);
                }
                Attribute::Language(_) => {
                    let language_cloned = language.clone();
                    assert!(language_cloned.is_some());
                    assert_eq!(language_cloned.unwrap(), attribute);
                }
                Attribute::Site(_) => {
                    let site_cloned = site.clone();
                    assert!(site_cloned.is_some());
                    assert_eq!(site_cloned.unwrap(), attribute);
                }
                Attribute::Url(_) => {
                    let url_cloned = url.clone();
                    assert!(url_cloned.is_some());
                    assert_eq!(url_cloned.unwrap(), attribute);
                }
                _ => panic!("Non-viable test attribute used"),
            }
        }
    }
}

#[test]
fn schema_org_1() {
    let path = "./tests/data/Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA - politiken.dk.html";

    let expected_title = Attribute::Title(
        "Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA"
            .to_string(),
    );
    let expected_site = Attribute::Site("Politiken".to_string());
    let expected_author = Attribute::Author(vec!["Søren Astrup".to_string()]);
    let expected_date = Attribute::Date(chrono::NaiveDate::parse_from_str("2023-12-11", "%Y-%m-%d").unwrap());

    let test_attributes = vec![expected_title, expected_author, expected_site, expected_date];

    schema_org_test(path, test_attributes);
}

#[test]
fn schema_org_2() {
    let path = "./tests/data/»I år skal jeg for første gang lave kamstegen på grillen«.html";

    let expected_title = Attribute::Title(
        "»I år skal jeg for første gang lave kamstegen på grillen«"
            .to_string(),
    );
    let expected_url = Attribute::Url("jyllands-posten.dk/jpaarhus/ECE16679033/i-aar-skal-jeg-for-foerste-gang-lave-kamstegen-paa-grillen/".to_string());
    let expected_date = Attribute::Date(chrono::NaiveDate::parse_from_str("2023-12-13", "%Y-%m-%d").unwrap());
    let expected_author = Attribute::Author(vec!["Marie Duedahl".to_string()]);
    let expected_site = Attribute::Site("JP/Politikens Hus A/S".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site, expected_author, expected_date];

    schema_org_test(path, test_attributes);
}