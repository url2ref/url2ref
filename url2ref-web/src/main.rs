mod scss;
use scss::compile;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{self, catch, catchers, get, launch, post, routes, uri};
use rocket::request::{FromRequest, Outcome, Request};
use rocket_dyn_templates::Template;
use std::env;
use std::path::{Path, PathBuf};
use tera::Context;
use url2ref::attribute::AttributeType;
use url2ref::generator::{ArchiveOptions, MetadataType, TranslationOptions, TranslationProvider, AiExtractionOptions, AiProvider};
use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
use url2ref::ai_extractor::{self, AiExtractedMetadata};
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
    translation_provider: Option<String>,    /// Enable Zotero/Citoid metadata extraction
    zotero_enabled: Option<bool>,    /// Enable AI-based metadata extraction for missing fields
    ai_enabled: Option<bool>,
    /// AI provider ("openai" or "anthropic")
    ai_provider: Option<String>,
    /// AI API key (passed from client for security)
    ai_api_key: Option<String>,
    /// AI model to use
    ai_model: Option<String>,
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
    zotero: Option<String>,
    ai: Option<String>,
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
    harvard: Option<String>,
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
    harvard: Option<String>,
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
            zotero: None,
            ai: None,
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
        if let Some(attr) = metadata.get(&MetadataType::Zotero) {
            sources.zotero = Some(attr.to_string());
        }
        
        // Only return if at least one source has a value
        if sources.opengraph.is_some() || sources.schemaorg.is_some() 
            || sources.htmlmeta.is_some() || sources.doi.is_some() 
            || sources.zotero.is_some() || sources.ai.is_some() {
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

/// Merge AI-extracted metadata into multi-source fields
fn merge_ai_metadata(multi_source: &mut MultiSourceFields, ai_metadata: &AiExtractedMetadata) {
    // Helper to get or create field sources
    fn ensure_sources(sources: &mut Option<FieldSources>) -> &mut FieldSources {
        if sources.is_none() {
            *sources = Some(FieldSources::default());
        }
        sources.as_mut().unwrap()
    }
    
    if let Some(ref title) = ai_metadata.title {
        ensure_sources(&mut multi_source.title).ai = Some(title.clone());
    }
    if let Some(ref authors) = ai_metadata.authors {
        if !authors.is_empty() {
            ensure_sources(&mut multi_source.author).ai = Some(authors.join(", "));
        }
    }
    if let Some(ref date) = ai_metadata.date {
        ensure_sources(&mut multi_source.date).ai = Some(date.clone());
    }
    if let Some(ref site) = ai_metadata.site {
        ensure_sources(&mut multi_source.site).ai = Some(site.clone());
    }
    if let Some(ref publisher) = ai_metadata.publisher {
        ensure_sources(&mut multi_source.publisher).ai = Some(publisher.clone());
    }
    if let Some(ref language) = ai_metadata.language {
        ensure_sources(&mut multi_source.language).ai = Some(language.clone());
    }
}

/// Determine which source is selected by default for each field (priority order)
fn get_default_selections(multi_source: &MultiSourceFields) -> FieldSelections {
    fn select_first_available(sources: &Option<FieldSources>) -> Option<String> {
        let sources = sources.as_ref()?;
        // Priority: OpenGraph > Schema.org > HTML Meta > DOI > Zotero > AI (AI is fallback)
        // Note: Return values match JSON field names (no underscores)
        if sources.opengraph.is_some() {
            Some("opengraph".to_string())
        } else if sources.schemaorg.is_some() {
            Some("schemaorg".to_string())
        } else if sources.htmlmeta.is_some() {
            Some("htmlmeta".to_string())
        } else if sources.doi.is_some() {
            Some("doi".to_string())
        } else if sources.zotero.is_some() {
            Some("zotero".to_string())
        } else if sources.ai.is_some() {
            Some("ai".to_string())
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

#[get("/", rank = 3)]
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

/// Build AI extraction options from request
fn build_ai_options(request: &GenerateRequest) -> AiExtractionOptions {
    let enabled = request.ai_enabled.unwrap_or(false);
    
    if !enabled {
        return AiExtractionOptions::default();
    }
    
    let provider = match request.ai_provider.as_deref() {
        Some("anthropic") => AiProvider::Anthropic,
        _ => AiProvider::OpenAI,
    };
    
    AiExtractionOptions {
        enabled,
        provider,
        api_key: request.ai_api_key.clone(),
        model: request.ai_model.clone(),
    }
}

/// Generate reference - checks for existing archive but doesn't create new ones
#[post("/api/generate", data = "<request>")]
fn generate_reference(request: Json<GenerateRequest>) -> Json<GenerateResponse> {
    // Determine translation provider from request (default to DeepL)
    let provider = match request.translation_provider.as_deref() {
        Some("google") => TranslationProvider::Google,
        _ => TranslationProvider::DeepL,
    };

    println!(
        "[url2ref] Generate request: target_lang={:?}, provider={:?}, ai_enabled={:?}",
        request.target_lang, request.translation_provider, request.ai_enabled
    );

    // Build translation options if target language is specified
    let deepl_key = load_deepl_key();
    let google_key = load_google_key();
    
    println!(
        "[url2ref] Translation config: provider={:?}, target={:?}, deepl_key={}, google_key={}",
        provider,
        request.target_lang,
        deepl_key.as_ref().map(|k| format!("SET ({} chars)", k.len())).unwrap_or("NOT SET".to_string()),
        google_key.as_ref().map(|k| format!("SET ({} chars)", k.len())).unwrap_or("NOT SET".to_string())
    );

    let translation_options = TranslationOptions {
        provider,
        source: None, // Let the API auto-detect source language
        target: request.target_lang.clone(),
        deepl_key,
        google_key,
    };

    // Build AI extraction options
    let ai_options = build_ai_options(&request);
    
    println!(
        "[url2ref] AI extraction: enabled={}, provider={:?}, model={:?}, key_len={}",
        ai_options.enabled,
        ai_options.provider,
        ai_options.model,
        ai_options.api_key.as_ref().map(|k| k.len()).unwrap_or(0)
    );

    // Build attribute config with optional Zotero support
    let zotero_enabled = request.zotero_enabled.unwrap_or(false);
    println!("[url2ref] Zotero enabled: {}", zotero_enabled);
    
    let attribute_config = if zotero_enabled {
        // Include Zotero in the priority chain
        let priority = AttributePriority::new(&[
            MetadataType::OpenGraph,
            MetadataType::SchemaOrg,
            MetadataType::HtmlMeta,
            MetadataType::Doi,
            MetadataType::Zotero,
        ]);
        AttributeConfig::new(priority)
    } else {
        AttributeConfig::default()
    };

    // Use archive options that only check for existing archives, don't create new ones
    let options = GenerationOptions {
        attribute_config,
        archive_options: ArchiveOptions {
            include_archived: true,
            perform_archival: false, // Don't wait for archive creation
        },
        translation_options,
        ai_options,
    };
    
    // Fetch HTML once and reuse for both reference generation and multi-source extraction
    let parse_info = match fetch_parse_info(&request.url, Some(&options)) {
        Ok(info) => info,
        Err(e) => {
            return Json(GenerateResponse {
                success: false,
                bibtex: None,
                wiki: None,
                harvard: None,
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
            let mut multi_source = extract_multi_source_fields_from_parse_info(&parse_info);
            
            // If AI extraction is enabled, also extract AI metadata and merge it
            if options.ai_options.enabled {
                match ai_extractor::extract_metadata(
                    &request.url,
                    &parse_info.raw_html,
                    &options.ai_options
                ) {
                    Ok(ai_metadata) => {
                        println!("[url2ref] Merging AI metadata into multi_source");
                        merge_ai_metadata(&mut multi_source, &ai_metadata);
                    }
                    Err(e) => {
                        println!("[url2ref] AI extraction for multi_source failed: {:?}", e);
                    }
                }
            }
            
            let multi_source = Some(multi_source);
            let selections = multi_source.as_ref().map(get_default_selections);
            
            Json(GenerateResponse {
                success: true,
                bibtex: Some(reference.bibtex()),
                wiki: Some(reference.wiki()),
                harvard: Some(reference.harvard()),
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
                harvard: None,
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
                    harvard: Some(reference.harvard()),
                    error: None,
                })
            } else {
                Json(ArchiveResponse {
                    success: false,
                    archive_url: None,
                    archive_date: None,
                    bibtex: Some(reference.bibtex()),
                    wiki: Some(reference.wiki()),
                    harvard: Some(reference.harvard()),
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
                harvard: None,
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

/// Log the status of translation API keys at startup
fn log_api_key_status() {
    println!("[url2ref] Checking translation API key configuration...");
    
    let deepl_set = env::var("DEEPL_API_KEY").is_ok();
    let google_set = env::var("GOOGLE_TRANSLATE_API_KEY").is_ok();
    
    if deepl_set {
        let key = env::var("DEEPL_API_KEY").unwrap();
        println!("[url2ref] DEEPL_API_KEY: SET (length: {} chars)", key.len());
    } else {
        eprintln!("[url2ref] DEEPL_API_KEY: NOT SET");
    }
    
    if google_set {
        let key = env::var("GOOGLE_TRANSLATE_API_KEY").unwrap();
        println!("[url2ref] GOOGLE_TRANSLATE_API_KEY: SET (length: {} chars)", key.len());
    } else {
        eprintln!("[url2ref] GOOGLE_TRANSLATE_API_KEY: NOT SET");
    }
    
    if !deepl_set && !google_set {
        eprintln!("[url2ref] WARNING: No translation API keys configured - title translation will be disabled");
    }
}

/// Guard that matches requests to docs.url2ref.com or docs.localhost
struct DocsHost;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DocsHost {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.host() {
            Some(host) => {
                let hostname = host.domain().as_str();
                if hostname.starts_with("docs.") || hostname == "docs" {
                    Outcome::Success(DocsHost)
                } else {
                    Outcome::Forward(rocket::http::Status::NotFound)
                }
            }
            None => Outcome::Forward(rocket::http::Status::NotFound),
        }
    }
}

/// Serve docs index
#[get("/", rank = 1)]
async fn docs_index(_host: DocsHost) -> Option<NamedFile> {
    NamedFile::open("../docs/book/index.html").await.ok()
}

/// Serve docs static files
#[get("/<path..>", rank = 2)]
async fn docs_files(_host: DocsHost, path: PathBuf) -> Option<NamedFile> {
    let docs_path = Path::new("../docs/book").join(&path);
    
    // Try the exact path first
    if let Ok(file) = NamedFile::open(&docs_path).await {
        return Some(file);
    }
    
    // If it's a directory, try index.html
    let index_path = docs_path.join("index.html");
    NamedFile::open(index_path).await.ok()
}

/// Serve favicon.ico from static directory
#[get("/favicon.ico", rank = 0)]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/custom/favicon.ico").await.ok()
}

#[launch]
fn rocket() -> _ {
    // Load .env file if present (for API keys, etc.)
    dotenv::dotenv().ok();

    let _compile_result = {
        match compile() {
            Ok(()) => (),
            Err(error) => panic!("SCSS compilation failed: {}", error),
        }
    };

    // Log API key status at startup
    log_api_key_status();

    rocket::build()
        .mount("/", routes![favicon, docs_index, docs_files, home, generate_reference, create_archive, fetch_multi_source])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
