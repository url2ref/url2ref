mod form;
mod scss;
use std::env;

use form::{CITATION_FORMATS, LANGUAGE_CODES};
use rocket::form::Form;
use scss::compile;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::{catch, catchers, get, launch, post, routes, uri};
use rocket_dyn_templates::Template;
use tera::Context;
use url2ref::generator::attribute_config::AttributeConfig;
use url2ref::generator::{ArchiveOptions, TranslationOptions};
use url2ref::{generate, GenerationOptions};

use crate::form::{ReferenceInput, BIBTEX_FORMAT, WIKI_FORMAT};

mod env_vars {
    pub const DEEPL_API_KEY: &str = "DEEPL_API_KEY";
}


#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(home))
}

fn render_home(mut context: Context, current_url: &String, current_format: &String) -> Template {
    context.insert("current_url", &current_url);
    context.insert("current_format", &current_format);
    context.insert("language_codes", LANGUAGE_CODES);
    context.insert("citation_formats", CITATION_FORMATS);

    Template::render("home", &context.into_json())
}

#[get("/")]
fn home() -> Template {
    render_home(Context::new(), &"".to_string(), &"".to_string())
}

#[post("/submit_url", data = "<input>")]
fn submit_url(input: Form<ReferenceInput>) -> Template {
    println!("{:?}", input);

    let mut context = Context::new();

    let deepl_key = env::var(env_vars::DEEPL_API_KEY).ok();

    let translation_options = TranslationOptions {
        source: Some(input.source_lang.clone()),
        target: Some(input.target_lang.clone()),
        deepl_key: deepl_key,
    };

    let attribute_config = AttributeConfig::default();
    
    let archive_options = ArchiveOptions {
        include_archived: input.archive,
        perform_archival: false
    };

    let generation_options = GenerationOptions {
        attribute_config,
        translation_options,
        archive_options
    };

    let reference_option = generate(&input.url, &generation_options);
    println!("{:?}", reference_option);
    if let Ok(reference) = reference_option {
        let output = match input.format.as_str() {
            BIBTEX_FORMAT => reference.bibtex(),
            WIKI_FORMAT => reference.wiki(),
            _ => panic!(),
        };
        
        context.insert("reference_output", &output);
    };

    render_home(context, &input.url, &input.format)
}

#[launch]
fn rocket() -> _ {
    let _compile_result = {
        match compile() {
            Ok(()) => (),
            Err(error) => panic!("SCSS compilation failed: {}", error),
        }
    };

    rocket::build()
        .mount("/", routes![home, submit_url])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
