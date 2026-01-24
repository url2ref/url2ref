//! CLI application for interfacing with [`url2ref`].

use std::env;
use std::env::VarError;

use clap::{Parser, ValueEnum};

use url2ref::generator::{TranslationOptions, TranslationProvider, ArchiveOptions};
use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
use url2ref::*;

mod env_vars {
    pub const DEEPL_API_KEY: &str = "DEEPL_API_KEY";
    pub const GOOGLE_TRANSLATE_API_KEY: &str = "GOOGLE_TRANSLATE_API_KEY";
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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MetadataType {
    Opengraph,
    Schemaorg
}

/// Translation provider selection for CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TranslationProviderArg {
    /// DeepL translation service
    Deepl,
    /// Google Cloud Translation API
    Google,
}

fn load_deepl_key() -> Result<String, VarError> {
    env::var(env_vars::DEEPL_API_KEY)
}

fn load_google_key() -> Result<String, VarError> {
    env::var(env_vars::GOOGLE_TRANSLATE_API_KEY)
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
        };
        let attribute_priorities = AttributePriority::new(&[metadata_type]);
        AttributeConfig::new(attribute_priorities)
    } else {
        AttributeConfig::default()
    };

    let archive_options = ArchiveOptions::default();

    let generation_options = GenerationOptions {
        attribute_config,
        translation_options,
        archive_options
    };

    let reference = generate(&query, &generation_options).unwrap();

    let output = match args.format {
        CitationFormat::Wiki => reference.wiki(),
        CitationFormat::Bibtex => reference.bibtex(),
    };

    println!("{}", output);
}