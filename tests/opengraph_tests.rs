use url2ref::*;

#[test]
fn opengraph_1() {
    let path = "./tests/data/Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA - politiken.dk.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA"
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://politiken.dk/internationalt/art9658207/Ledende-universitetsrektorers-holdninger-til-antisemitisme-udl%C3%B8ser-ramaskrig-i-USA".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("Politiken".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }
}

#[test]
fn opengraph_2() {
    let path = "./tests/data/»I år skal jeg for første gang lave kamstegen på grillen«.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "»En hyldest til mine afdøde forældre«: Topkok serverer en forret, de færreste nok får juleaften"
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://jyllands-posten.dk/jpaarhus/ECE16679033/i-aar-skal-jeg-for-foerste-gang-lave-kamstegen-paa-grillen/".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("Jyllands-Posten".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }

}

#[test]
fn opengraph_3() {
    let path = "./tests/data/'Vi er lidt forvirrede over, hvad der er sket_' Ekstremt klimaudsatte ø-stater kom for sent, da klimaaftale blev vedtaget _ Klima _ DR.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "'Vi er lidt forvirrede over, hvad der er sket?' Ekstremt klimaudsatte ø-stater kom for sent, da klimaaftale blev vedtaget"
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://www.dr.dk/nyheder/viden/klima/vi-er-lidt-forvirrede-over-hvad-der-er-sket-ekstremt-klimaudsatte-oe-stater-kom".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("DR".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }
}

#[test]
fn opengraph_4() {
    let path = "./tests/data/Efter den vildeste lynmanøvre_ Klimaaftale på plads i Dubai _ Information.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "Efter den vildeste lynmanøvre: Klimaaftale på plads i Dubai"
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://www.information.dk/udland/2023/12/vildeste-lynmanoevre-klimaaftale-paa-plads-dubai".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("Information".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }

}

#[test]
fn opengraph_5() {
    let path = "./tests/data/Nyt kompromis kan blive »historisk«. Men der er stadig ingen udfasning af fossile brændsler _ Information.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "Nyt kompromis kan blive »historisk«. Men der er stadig ingen udfasning af fossile brændsler"
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://www.information.dk/udland/2023/12/nyt-kompromis-kan-historisk-stadig-ingen-udfasning-fossile-braendsler".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("Information".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }

}

#[test]
fn opengraph_6() {
    let path = "./tests/data/He Might Have Been Pope. Instead, He May Go to Prison. - WSJ.html";
    let options = url2ref::GenerationOptions::default_opengraph();

    let reference_result = url2ref::generate_from_file(path, options);

    assert!(
        !reference_result.is_err(),
        "Reference generation should not fail"
    );

    let reference = reference_result.unwrap();

    assert!(matches!(reference, Reference::NewsArticle { .. }));

    if let url2ref::Reference::NewsArticle { title, url, site, .. } = reference {
        let expected_title = url2ref::attribute::Attribute::Title(
            "He Might Have Been Pope. Instead, He May Go to Prison."
                .to_string(),
        );
        assert!(title.is_some());
        assert_eq!(title.unwrap(), expected_title);

        let expected_url = url2ref::attribute::Attribute::Url("https://www.wsj.com/world/europe/cardinal-vatican-embezzlement-pope-francis-d66b1c83".to_string());
        assert!(url.is_some());
        assert_eq!(url.unwrap(), expected_url);

        let expected_site = url2ref::attribute::Attribute::Site("WSJ".to_string());
        assert!(site.is_some());
        assert_eq!(site.unwrap(), expected_site);
    }

}
