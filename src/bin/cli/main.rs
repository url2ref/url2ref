//! CLI application for interfacing with [`url2ref`].

use std::env;
use std::env::VarError;

use clap::{Parser, ValueEnum};

use url2ref::generator::TranslationOptions;
use url2ref::*;

mod env_vars {
    pub const DEEPL_API_KEY: &str = "DEEPL_API_KEY";
}

/// Supported command-line arguments.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArgs {
    #[clap(short, long)]
    url: String,

    #[clap(short, long, value_enum, default_value_t=CitationFormat::Wiki)]
    format: CitationFormat,

    #[clap(short, long)]
    source_lang: Option<String>,

    #[clap(short, long)]
    target_lang: Option<String>,
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

fn load_deepl_key() -> Result<String, VarError> {
    let deepl_key = env::var(env_vars::DEEPL_API_KEY)?;
    Ok(deepl_key)
}

fn main() {
    let args = CommandLineArgs::parse();
    let query = args.url;

    let deepl_key = load_deepl_key();

    let translation_options = TranslationOptions {
        source: args.source_lang,
        target: args.target_lang,
        deepl_key: deepl_key.ok()
    };

    let reference = generate(&query, &GenerationOptions::with_translation(translation_options)).unwrap();

    let output = match args.format {
        CitationFormat::Wiki => reference.wiki(),
        CitationFormat::Bibtex => reference.bibtex(),
    };

    println!("{}", output);
}