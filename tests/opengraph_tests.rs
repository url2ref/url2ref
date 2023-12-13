use url2ref::attribute::Attribute;
use url2ref::*;

fn opengraph_test(path: &str, test_attributes: Vec<Attribute>) {
    let options = url2ref::GenerationOptions::default_opengraph();

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
fn opengraph_1() {
    let path = "./tests/data/Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA - politiken.dk.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA"
            .to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url("https://politiken.dk/internationalt/art9658207/Ledende-universitetsrektorers-holdninger-til-antisemitisme-udl%C3%B8ser-ramaskrig-i-USA".to_string());
    let expected_site = url2ref::attribute::Attribute::Site("Politiken".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}

#[test]
fn opengraph_2() {
    let path = "./tests/data/»I år skal jeg for første gang lave kamstegen på grillen«.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "»En hyldest til mine afdøde forældre«: Topkok serverer en forret, de færreste nok får juleaften"
            .to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url("https://jyllands-posten.dk/jpaarhus/ECE16679033/i-aar-skal-jeg-for-foerste-gang-lave-kamstegen-paa-grillen/".to_string());
    let expected_site = url2ref::attribute::Attribute::Site("Jyllands-Posten".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}

#[test]
fn opengraph_3() {
    let path = "./tests/data/'Vi er lidt forvirrede over, hvad der er sket_' Ekstremt klimaudsatte ø-stater kom for sent, da klimaaftale blev vedtaget _ Klima _ DR.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "'Vi er lidt forvirrede over, hvad der er sket?' Ekstremt klimaudsatte ø-stater kom for sent, da klimaaftale blev vedtaget"
            .to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url("https://www.dr.dk/nyheder/viden/klima/vi-er-lidt-forvirrede-over-hvad-der-er-sket-ekstremt-klimaudsatte-oe-stater-kom".to_string());
    let expected_site = url2ref::attribute::Attribute::Site("DR".to_string());
    
    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}

#[test]
fn opengraph_4() {
    let path = "./tests/data/Efter den vildeste lynmanøvre_ Klimaaftale på plads i Dubai _ Information.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "Efter den vildeste lynmanøvre: Klimaaftale på plads i Dubai".to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url("https://www.information.dk/udland/2023/12/vildeste-lynmanoevre-klimaaftale-paa-plads-dubai".to_string());
    let expected_site = url2ref::attribute::Attribute::Site("Information".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}

#[test]
fn opengraph_5() {
    let path = "./tests/data/Nyt kompromis kan blive »historisk«. Men der er stadig ingen udfasning af fossile brændsler _ Information.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "Nyt kompromis kan blive »historisk«. Men der er stadig ingen udfasning af fossile brændsler"
            .to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url("https://www.information.dk/udland/2023/12/nyt-kompromis-kan-historisk-stadig-ingen-udfasning-fossile-braendsler".to_string());
    let expected_site = url2ref::attribute::Attribute::Site("Information".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}

#[test]
fn opengraph_6() {
    let path = "./tests/data/He Might Have Been Pope. Instead, He May Go to Prison. - WSJ.html";

    let expected_title = url2ref::attribute::Attribute::Title(
        "He Might Have Been Pope. Instead, He May Go to Prison.".to_string(),
    );
    let expected_url = url2ref::attribute::Attribute::Url(
        "https://www.wsj.com/world/europe/cardinal-vatican-embezzlement-pope-francis-d66b1c83"
            .to_string(),
    );
    let expected_site = url2ref::attribute::Attribute::Site("WSJ".to_string());

    let test_attributes = vec![expected_title, expected_url, expected_site];

    opengraph_test(path, test_attributes);
}
