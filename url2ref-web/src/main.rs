mod scss;
use scss::compile;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{self, catch, catchers, get, launch, post, routes, uri};
use rocket_dyn_templates::Template;
use tera::Context;
use url2ref::generator::ArchiveOptions;
use url2ref::{generate, GenerationOptions, Reference};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerateRequest {
    url: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ArchiveRequest {
    url: String,
}

/// Extracted metadata fields from the reference
#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct ReferenceFields {
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    site: Option<String>,
    publisher: Option<String>,
    language: Option<String>,
    url: Option<String>,
    archive_url: Option<String>,
    archive_date: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GenerateResponse {
    success: bool,
    bibtex: Option<String>,
    wiki: Option<String>,
    fields: Option<ReferenceFields>,
    /// "available" - archive exists, "not_found" - no archive, needs creation
    archive_status: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ArchiveResponse {
    success: bool,
    archive_url: Option<String>,
    archive_date: Option<String>,
    /// Updated citations with archive info
    bibtex: Option<String>,
    wiki: Option<String>,
    error: Option<String>,
}

/// Extract displayable fields from a Reference
fn extract_fields(reference: &Reference) -> ReferenceFields {
    match reference {
        Reference::NewsArticle { 
            title, author, date, language, site, url, publisher, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                author: author.as_ref().map(|a| a.to_string()),
                date: date.as_ref().map(|a| a.to_string()),
                site: site.as_ref().map(|a| a.to_string()),
                publisher: publisher.as_ref().map(|a| a.to_string()),
                language: language.as_ref().map(|a| a.to_string()),
                url: url.as_ref().map(|a| a.to_string()),
                archive_url: archive_url.as_ref().map(|a| a.to_string()),
                archive_date: archive_date.as_ref().map(|a| a.to_string()),
            }
        }
        Reference::ScholarlyArticle { 
            title, author, date, language, url, publisher, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                author: author.as_ref().map(|a| a.to_string()),
                date: date.as_ref().map(|a| a.to_string()),
                site: None,
                publisher: publisher.as_ref().map(|a| a.to_string()),
                language: language.as_ref().map(|a| a.to_string()),
                url: url.as_ref().map(|a| a.to_string()),
                archive_url: archive_url.as_ref().map(|a| a.to_string()),
                archive_date: archive_date.as_ref().map(|a| a.to_string()),
            }
        }
        Reference::GenericReference { 
            title, author, date, language, site, url, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                author: author.as_ref().map(|a| a.to_string()),
                date: date.as_ref().map(|a| a.to_string()),
                site: site.as_ref().map(|a| a.to_string()),
                publisher: None,
                language: language.as_ref().map(|a| a.to_string()),
                url: url.as_ref().map(|a| a.to_string()),
                archive_url: archive_url.as_ref().map(|a| a.to_string()),
                archive_date: archive_date.as_ref().map(|a| a.to_string()),
            }
        }
    }
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(home))
}

#[get("/")]
fn home() -> Template {
    let context = Context::new().into_json();
    Template::render("home", &context)
}

/// Generate reference - checks for existing archive but doesn't create new ones
#[post("/api/generate", data = "<request>")]
fn generate_reference(request: Json<GenerateRequest>) -> Json<GenerateResponse> {
    // Use archive options that only check for existing archives, don't create new ones
    let options = GenerationOptions {
        archive_options: ArchiveOptions {
            include_archived: true,
            perform_archival: false, // Don't wait for archive creation
        },
        ..Default::default()
    };
    
    match generate(&request.url, &options) {
        Ok(reference) => {
            let fields = extract_fields(&reference);
            let archive_status = if fields.archive_url.is_some() {
                "available"
            } else {
                "not_found"
            };
            
            Json(GenerateResponse {
                success: true,
                bibtex: Some(reference.bibtex()),
                wiki: Some(reference.wiki()),
                fields: Some(fields),
                archive_status: Some(archive_status.to_string()),
                error: None,
            })
        }
        Err(e) => {
            Json(GenerateResponse {
                success: false,
                bibtex: None,
                wiki: None,
                fields: None,
                archive_status: None,
                error: Some(format!("{}", e)),
            })
        }
    }
}

/// Create archive for a URL - this may take a while
#[post("/api/archive", data = "<request>")]
fn create_archive(request: Json<ArchiveRequest>) -> Json<ArchiveResponse> {
    // Use archive options that will create an archive if needed
    let options = GenerationOptions {
        archive_options: ArchiveOptions {
            include_archived: true,
            perform_archival: true, // Create archive if not exists
        },
        ..Default::default()
    };
    
    match generate(&request.url, &options) {
        Ok(reference) => {
            let fields = extract_fields(&reference);
            
            if fields.archive_url.is_some() {
                Json(ArchiveResponse {
                    success: true,
                    archive_url: fields.archive_url,
                    archive_date: fields.archive_date,
                    bibtex: Some(reference.bibtex()),
                    wiki: Some(reference.wiki()),
                    error: None,
                })
            } else {
                Json(ArchiveResponse {
                    success: false,
                    archive_url: None,
                    archive_date: None,
                    bibtex: Some(reference.bibtex()),
                    wiki: Some(reference.wiki()),
                    error: Some("Failed to create archive".to_string()),
                })
            }
        }
        Err(e) => {
            Json(ArchiveResponse {
                success: false,
                archive_url: None,
                archive_date: None,
                bibtex: None,
                wiki: None,
                error: Some(format!("{}", e)),
            })
        }
    }
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
        .mount("/", routes![home, generate_reference, create_archive])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
