use url2ref::*;

#[test]
fn politiken_tests() {
    let url = "https://politiken.dk/internationalt/art9658207/En-h%C3%B8ring-om-antisemitisme-afsl%C3%B8rede-ledende-universitetsrektorers-j%C3%B8dehad";
    let options = GenerationOptions::default();
    
    let reference_result = url2ref::generate(url, options);
    assert!(!reference_result.is_err(), "Reference generation should not fail");
    
    let reference = reference_result.unwrap();
    let expected_wiki = r"{{cite web |title=Ledende universitetsrektorers holdninger til antisemitisme udløser ramaskrig i USA |site=Politiken |url=https://politiken.dk/internationalt/art9658207/Ledende-universitetsrektorers-holdninger-til-antisemitisme-udl%C3%B8ser-ramaskrig-i-USA }}";
    
    assert_eq!(reference.wiki(), expected_wiki);
}