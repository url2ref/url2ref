//! CLI application for interfacing with [`url2ref`].

use clap::{Parser, ValueEnum};

use url2ref::*;

/// Supported command-line arguments.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLineArgs {
    #[clap(short, long)]
    url: String,

    #[clap(short, long, value_enum, default_value_t=CitationFormat::Wiki)]
    format: CitationFormat,
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

fn main() {
    let args = CommandLineArgs::parse();
    let query = args.url;

    let reference = generate(&query, &GenerationOptions::default()).unwrap();

    let output = match args.format {
        CitationFormat::Wiki => reference.wiki(),
        CitationFormat::Bibtex => reference.bibtex(),
    };

    println!("{}", output);
}