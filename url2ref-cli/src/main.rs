//! CLI application for interfacing with [`url2ref`].

use std::env;
use std::env::VarError;

use clap::{Parser, ValueEnum};

use url2ref::generator::{TranslationOptions, TranslationProvider, ArchiveOptions, AiExtractionOptions, AiProvider};
use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
use url2ref::*;

mod env_vars {
    pub const DEEPL_API_KEY: &str = "DEEPL_API_KEY";
    pub const GOOGLE_TRANSLATE_API_KEY: &str = "GOOGLE_TRANSLATE_API_KEY";
    pub const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
    pub const ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";
}

/// Supported command-line arguments.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArgs {
    #[clap(short, long)]
    url: String,

    #[clap(short, long, value_enum, default_value_t=CitationFormat::Wiki)]
    format: CitationFormat,

    #[clap(short, long, default_value=None)]
    metadata_priority: Option<MetadataType>,

    #[clap(short, long, default_value=None, requires("target_lang"))]
    source_lang: Option<String>,

    #[clap(short, long, default_value=None)]
    target_lang: Option<String>,

    /// Translation provider to use (deepl or google)
    #[clap(long, value_enum, default_value_t=TranslationProviderArg::Deepl)]
    translation_provider: TranslationProviderArg,

    #[clap(short, long, default_value_t=true)]
    include_archived: bool,

    /// Enable AI-based metadata extraction for missing fields
    #[clap(long, default_value_t=false)]
    ai_extraction: bool,

    /// AI provider to use for metadata extraction (openai or anthropic)
    #[clap(long, value_enum, default_value_t=AiProviderArg::OpenAI)]
    ai_provider: AiProviderArg,

    /// AI model to use (e.g., gpt-4o-mini, claude-3-haiku-20240307)
    #[clap(long, default_value=None)]
    ai_model: Option<String>,
}

/// Supported citation formats.
/// [`clap`] is capable of automatically converting the variants
/// into appropriate command-line arguments via the [`ValueEnum`] macro.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CitationFormat {
    /// Using {{cite web}} MediaWiki markup
    Wiki,
    /// Using BibTeX markup
    Bibtex,
    /// Using Harvard referencing style
    Harvard,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MetadataType {
    Opengraph,
    Schemaorg,
    Htmlmeta,
    Doi,
    Zotero,
}

/// Translation provider selection for CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TranslationProviderArg {
    /// DeepL translation service
    Deepl,
    /// Google Cloud Translation API
    Google,
}

/// AI provider selection for CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AiProviderArg {
    /// OpenAI (GPT models)
    OpenAI,
    /// Anthropic (Claude models)
    Anthropic,
}

fn load_deepl_key() -> Result<String, VarError> {
    env::var(env_vars::DEEPL_API_KEY)
}

fn load_google_key() -> Result<String, VarError> {
    env::var(env_vars::GOOGLE_TRANSLATE_API_KEY)
}

fn load_openai_key() -> Result<String, VarError> {
    env::var(env_vars::OPENAI_API_KEY)
}

fn load_anthropic_key() -> Result<String, VarError> {
    env::var(env_vars::ANTHROPIC_API_KEY)
}

fn main() {
    let args = CommandLineArgs::parse();
    let query = args.url;

    let deepl_key = load_deepl_key().ok();
    let google_key = load_google_key().ok();

    let provider = match args.translation_provider {
        TranslationProviderArg::Deepl => TranslationProvider::DeepL,
        TranslationProviderArg::Google => TranslationProvider::Google,
    };

    let translation_options = TranslationOptions {
        provider,
        source: args.source_lang,
        target: args.target_lang,
        deepl_key,
        google_key,
    };

    let attribute_config = if args.metadata_priority.is_some() {
        let metadata_type = match args.metadata_priority.unwrap() {
            MetadataType::Opengraph => generator::MetadataType::OpenGraph,
            MetadataType::Schemaorg => generator::MetadataType::SchemaOrg,
            MetadataType::Htmlmeta => generator::MetadataType::HtmlMeta,
            MetadataType::Doi => generator::MetadataType::Doi,
            MetadataType::Zotero => generator::MetadataType::Zotero,
        };
        let attribute_priorities = AttributePriority::new(&[metadata_type]);
        AttributeConfig::new(attribute_priorities)
    } else {
        AttributeConfig::default()
    };

    let archive_options = ArchiveOptions::default();

    // AI extraction options
    let ai_options = if args.ai_extraction {
        let api_key = match args.ai_provider {
            AiProviderArg::OpenAI => load_openai_key().ok(),
            AiProviderArg::Anthropic => load_anthropic_key().ok(),
        };
        
        AiExtractionOptions {
            enabled: true,
            provider: match args.ai_provider {
                AiProviderArg::OpenAI => AiProvider::OpenAI,
                AiProviderArg::Anthropic => AiProvider::Anthropic,
            },
            api_key,
            model: args.ai_model,
        }
    } else {
        AiExtractionOptions::default()
    };

    let generation_options = GenerationOptions {
        attribute_config,
        translation_options,
        archive_options,
        ai_options,
    };

    let reference = generate(&query, &generation_options).unwrap();

    let output = match args.format {
        CitationFormat::Wiki => reference.wiki(),
        CitationFormat::Bibtex => reference.bibtex(),
        CitationFormat::Harvard => reference.harvard(),
    };

    println!("{}", output);
}