mod scss;
use scss::compile;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{self, catch, catchers, get, launch, post, routes, uri};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::env;
use tera::Context;
use url2ref::attribute::AttributeType;
use url2ref::generator::{ArchiveOptions, MetadataType, TranslationOptions, TranslationProvider};
use url2ref::{
    generate, generate_from_parse_info, fetch_parse_info, parse_all_metadata_from_parse_info,
    GenerationOptions, Reference
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerateRequest {
    url: String,
    /// Optional target language code (e.g., "EN", "DE", "FR") for title translation
    target_lang: Option<String>,
    /// Optional translation provider ("deepl" or "google")
    translation_provider: Option<String>,
    /// Optional list of enabled metadata sources
    metadata_sources: Option<Vec<String>>,
    /// Optional field selections: maps field name to selected source
    field_selections: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ArchiveRequest {
    url: String,
}

/// Metadata values from different sources for a single field
#[derive(Serialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
struct FieldSources {
    opengraph: Option<String>,
    schemaorg: Option<String>,
    htmlmeta: Option<String>,
    doi: Option<String>,
}

/// Multi-source metadata for all fields
#[derive(Serialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
struct MultiSourceFields {
    title: Option<FieldSources>,
    author: Option<FieldSources>,
    date: Option<FieldSources>,
    site: Option<FieldSources>,
    publisher: Option<FieldSources>,
    language: Option<FieldSources>,
    url: Option<FieldSources>,
}

/// Extracted metadata fields from the reference (selected values)
#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct ReferenceFields {
    title: Option<String>,
    translated_title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    site: Option<String>,
    publisher: Option<String>,
    language: Option<String>,
    url: Option<String>,
    archive_url: Option<String>,
    archive_date: Option<String>,
}

/// Selected source for each field based on priority
#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct FieldSelections {
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    site: Option<String>,
    publisher: Option<String>,
    language: Option<String>,
    url: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GenerateResponse {
    success: bool,
    bibtex: Option<String>,
    wiki: Option<String>,
    fields: Option<ReferenceFields>,
    /// Multi-source metadata for interactive selection
    multi_source: Option<MultiSourceFields>,
    /// Which source is selected for each field
    selections: Option<FieldSelections>,
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
            title, translated_title, author, date, language, site, url, publisher, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                translated_title: translated_title.as_ref().map(|a| a.to_string()),
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
            title, translated_title, author, date, language, url, publisher, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                translated_title: translated_title.as_ref().map(|a| a.to_string()),
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
            title, translated_title, author, date, language, site, url, archive_url, archive_date, .. 
        } => {
            ReferenceFields {
                title: title.as_ref().map(|a| a.to_string()),
                translated_title: translated_title.as_ref().map(|a| a.to_string()),
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

/// Extract multi-source metadata from pre-parsed ParseInfo
/// This avoids duplicate HTTP requests by reusing the cached HTML.
fn extract_multi_source_fields_from_parse_info(parse_info: &url2ref::ParseInfo) -> MultiSourceFields {
    let multi_source = parse_all_metadata_from_parse_info(parse_info);
    
    let get_field_sources = |attr_type: AttributeType| -> Option<FieldSources> {
        let metadata = multi_source.get(attr_type)?;
        let mut sources = FieldSources {
            opengraph: None,
            schemaorg: None,
            htmlmeta: None,
            doi: None,
        };
        
        if let Some(attr) = metadata.get(&MetadataType::OpenGraph) {
            sources.opengraph = Some(attr.to_string());
        }
        if let Some(attr) = metadata.get(&MetadataType::SchemaOrg) {
            sources.schemaorg = Some(attr.to_string());
        }
        if let Some(attr) = metadata.get(&MetadataType::HtmlMeta) {
            sources.htmlmeta = Some(attr.to_string());
        }
        if let Some(attr) = metadata.get(&MetadataType::Doi) {
            sources.doi = Some(attr.to_string());
        }
        
        // Only return if at least one source has a value
        if sources.opengraph.is_some() || sources.schemaorg.is_some() 
            || sources.htmlmeta.is_some() || sources.doi.is_some() {
            Some(sources)
        } else {
            None
        }
    };
    
    MultiSourceFields {
        title: get_field_sources(AttributeType::Title),
        author: get_field_sources(AttributeType::Author),
        date: get_field_sources(AttributeType::Date),
        site: get_field_sources(AttributeType::Site),
        publisher: get_field_sources(AttributeType::Publisher),
        language: get_field_sources(AttributeType::Language),
        url: get_field_sources(AttributeType::Url),
    }
}

/// Determine which source is selected by default for each field (priority order)
fn get_default_selections(multi_source: &MultiSourceFields) -> FieldSelections {
    fn select_first_available(sources: &Option<FieldSources>) -> Option<String> {
        let sources = sources.as_ref()?;
        // Priority: OpenGraph > Schema.org > HTML Meta > DOI
        // Note: Return values match JSON field names (no underscores)
        if sources.opengraph.is_some() {
            Some("opengraph".to_string())
        } else if sources.schemaorg.is_some() {
            Some("schemaorg".to_string())
        } else if sources.htmlmeta.is_some() {
            Some("htmlmeta".to_string())
        } else if sources.doi.is_some() {
            Some("doi".to_string())
        } else {
            None
        }
    }
    
    FieldSelections {
        title: select_first_available(&multi_source.title),
        author: select_first_available(&multi_source.author),
        date: select_first_available(&multi_source.date),
        site: select_first_available(&multi_source.site),
        publisher: select_first_available(&multi_source.publisher),
        language: select_first_available(&multi_source.language),
        url: select_first_available(&multi_source.url),
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

/// Load DeepL API key from environment variable
fn load_deepl_key() -> Option<String> {
    env::var("DEEPL_API_KEY").ok()
}

/// Load Google Cloud Translation API key from environment variable
fn load_google_key() -> Option<String> {
    env::var("GOOGLE_TRANSLATE_API_KEY").ok()
}

/// Generate reference - checks for existing archive but doesn't create new ones
#[post("/api/generate", data = "<request>")]
fn generate_reference(request: Json<GenerateRequest>) -> Json<GenerateResponse> {
    // Determine translation provider from request (default to DeepL)
    let provider = match request.translation_provider.as_deref() {
        Some("google") => TranslationProvider::Google,
        _ => TranslationProvider::DeepL,
    };

    // Build translation options if target language is specified
    let translation_options = TranslationOptions {
        provider,
        source: None, // Let the API auto-detect source language
        target: request.target_lang.clone(),
        deepl_key: load_deepl_key(),
        google_key: load_google_key(),
    };

    // Use archive options that only check for existing archives, don't create new ones
    let options = GenerationOptions {
        archive_options: ArchiveOptions {
            include_archived: true,
            perform_archival: false, // Don't wait for archive creation
        },
        translation_options,
        ..Default::default()
    };
    
    // Fetch HTML once and reuse for both reference generation and multi-source extraction
    let parse_info = match fetch_parse_info(&request.url, Some(&options)) {
        Ok(info) => info,
        Err(e) => {
            return Json(GenerateResponse {
                success: false,
                bibtex: None,
                wiki: None,
                fields: None,
                multi_source: None,
                selections: None,
                archive_status: None,
                error: Some(format!("{}", e)),
            });
        }
    };
    
    match generate_from_parse_info(&parse_info, &options) {
        Ok(reference) => {
            let fields = extract_fields(&reference);
            let archive_status = if fields.archive_url.is_some() {
                "available"
            } else {
                "not_found"
            };
            
            // Extract multi-source metadata from the cached ParseInfo (no additional HTTP request!)
            let multi_source = Some(extract_multi_source_fields_from_parse_info(&parse_info));
            let selections = multi_source.as_ref().map(get_default_selections);
            
            Json(GenerateResponse {
                success: true,
                bibtex: Some(reference.bibtex()),
                wiki: Some(reference.wiki()),
                fields: Some(fields),
                multi_source,
                selections,
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
                multi_source: None,
                selections: None,
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

/// Request structure for multi-source metadata endpoint
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MultiSourceRequest {
    url: String,
}

/// Response structure for multi-source metadata endpoint
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MultiSourceResponse {
    success: bool,
    multi_source: Option<MultiSourceFields>,
    selections: Option<FieldSelections>,
    error: Option<String>,
}

/// Fetch multi-source metadata for a URL.
/// This endpoint is useful for lazy/background loading of multi-source data
/// when the client wants to defer the metadata comparison UI.
/// 
/// Note: This performs a separate HTTP request to fetch the page.
/// For best performance, use the main generate endpoint which now includes
/// multi-source data in the response (with no additional HTTP cost).
#[post("/api/multisource", data = "<request>")]
fn fetch_multi_source(request: Json<MultiSourceRequest>) -> Json<MultiSourceResponse> {
    match fetch_parse_info(&request.url, None) {
        Ok(parse_info) => {
            let multi_source = extract_multi_source_fields_from_parse_info(&parse_info);
            let selections = get_default_selections(&multi_source);
            
            Json(MultiSourceResponse {
                success: true,
                multi_source: Some(multi_source),
                selections: Some(selections),
                error: None,
            })
        }
        Err(e) => {
            Json(MultiSourceResponse {
                success: false,
                multi_source: None,
                selections: None,
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
        .mount("/", routes![home, generate_reference, create_archive, fetch_multi_source])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
