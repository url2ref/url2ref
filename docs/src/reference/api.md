# API Documentation

Reference documentation for the url2ref Rust library.

## Overview

The `url2ref` crate provides programmatic access to citation generation. Use it to integrate url2ref into your Rust applications.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
url2ref = { path = "path/to/url2ref/url2ref" }
```

Or from git:

```toml
[dependencies]
url2ref = { git = "https://github.com/url2ref/url2ref" }
```

## Quick Start

```rust
use url2ref::{generate, GenerationOptions, Reference};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use default options
    let options = GenerationOptions::default();
    
    // Generate reference
    let reference = generate("https://example.com/article", &options)?;
    
    // Output in different formats
    println!("Wiki: {}", reference.wiki());
    println!("BibTeX: {}", reference.bibtex());
    println!("Harvard: {}", reference.harvard());
    
    Ok(())
}
```

## Core Types

### `GenerationOptions`

Configuration for reference generation.

```rust
use url2ref::{GenerationOptions, GenerationOptionsBuilder};
use url2ref::generator::{
    TranslationOptions, 
    ArchiveOptions, 
    AiExtractionOptions
};
use url2ref::generator::attribute_config::AttributeConfig;

// Using builder pattern
let options = GenerationOptionsBuilder::default()
    .attribute_config(AttributeConfig::default())
    .translation_options(TranslationOptions::default())
    .archive_options(ArchiveOptions::default())
    .ai_options(AiExtractionOptions::default())
    .build()?;

// Or use defaults
let options = GenerationOptions::default();
```

### `Reference`

Represents a generated citation with all extracted metadata.

```rust
use url2ref::Reference;

// Reference is an enum with variants:
pub enum Reference {
    NewsArticle { /* fields */ },
    ScholarlyArticle { /* fields */ },
    GenericReference { /* fields */ },
}

// Get formatted citations
let wiki = reference.wiki();
let bibtex = reference.bibtex();
let harvard = reference.harvard();
```

### `Attribute`

Wrapper for metadata values.

```rust
use url2ref::attribute::{Attribute, AttributeType, Author, Date};

pub enum Attribute {
    Title(String),
    TranslatedTitle(Translation),
    Authors(Vec<Author>),
    Date(Date),
    Language(String),
    Site(String),
    Url(String),
    // ... more variants
}
```

## Core Functions

### `generate`

Generate a reference from a URL.

```rust
pub fn generate(url: &str, options: &GenerationOptions) -> Result<Reference>
```

**Parameters:**
- `url` - The URL to generate a citation for
- `options` - Generation options

**Returns:**
- `Ok(Reference)` - Successfully generated reference
- `Err(ReferenceGenerationError)` - Generation failed

**Example:**

```rust
use url2ref::{generate, GenerationOptions};

let options = GenerationOptions::default();
let reference = generate("https://www.bbc.com/news/article", &options)?;
```

### `generate_from_file`

Generate a reference from a local HTML file.

```rust
pub fn generate_from_file(path: &str, options: &GenerationOptions) -> Result<Reference>
```

**Example:**

```rust
let reference = generate_from_file("./page.html", &options)?;
```

### `fetch_parse_info`

Fetch and parse a URL without generating a reference. Useful for caching.

```rust
pub fn fetch_parse_info<'a>(
    url: &'a str, 
    options: Option<&GenerationOptions>
) -> Result<ParseInfo<'a>>
```

**Example:**

```rust
use url2ref::{fetch_parse_info, generate_from_parse_info};

// Fetch once
let parse_info = fetch_parse_info("https://example.com", None)?;

// Reuse for multiple operations
let reference = generate_from_parse_info(&parse_info, &options)?;
let metadata = parse_all_metadata_from_parse_info(&parse_info);
```

### `parse_all_metadata`

Extract metadata from all sources without generating a reference.

```rust
pub fn parse_all_metadata(url: &str) -> Result<MultiSourceAttributeCollection>
```

**Example:**

```rust
use url2ref::parse_all_metadata;

let metadata = parse_all_metadata("https://example.com")?;

// Access metadata by attribute type and source
use url2ref::attribute::AttributeType;
use url2ref::generator::MetadataType;

if let Some(title_sources) = metadata.get(AttributeType::Title) {
    if let Some(og_title) = title_sources.get(&MetadataType::OpenGraph) {
        println!("OpenGraph title: {}", og_title);
    }
}
```

## Configuration Types

### `TranslationOptions`

Configure title translation.

```rust
use url2ref::generator::{TranslationOptions, TranslationProvider};

let translation = TranslationOptions {
    provider: TranslationProvider::DeepL,
    source: None,  // Auto-detect
    target: Some("EN".to_string()),
    deepl_key: Some("your-api-key".to_string()),
    google_key: None,
};
```

### `ArchiveOptions`

Configure Wayback Machine integration.

```rust
use url2ref::generator::ArchiveOptions;

let archive = ArchiveOptions {
    include_archived: true,   // Check for existing archives
    perform_archival: true,   // Create archive if none exists
};
```

### `AiExtractionOptions`

Configure AI-powered extraction.

```rust
use url2ref::generator::{AiExtractionOptions, AiProvider};

let ai = AiExtractionOptions {
    enabled: true,
    provider: AiProvider::OpenAI,
    api_key: Some("sk-...".to_string()),
    model: Some("gpt-4o-mini".to_string()),
};
```

### `AttributeConfig`

Configure metadata source priority.

```rust
use url2ref::generator::attribute_config::{AttributeConfig, AttributePriority};
use url2ref::generator::MetadataType;

// Prioritize Schema.org for all fields
let priority = AttributePriority::new(&[
    MetadataType::SchemaOrg,
    MetadataType::OpenGraph,
    MetadataType::HtmlMeta,
]);

let config = AttributeConfig::new(priority);
```

## Error Handling

### `ReferenceGenerationError`

```rust
use url2ref::generator::ReferenceGenerationError;

pub enum ReferenceGenerationError {
    CurlError(CurlError),
    ParseFailure,
    ParseSkip,
    HTMLParseError(std::io::Error),
    DeepLError(DeepLError),
    TranslationError,
    DoiError(DoiError),
    ArchiveError(ArchiveError),
    AiExtractionError(AiExtractionError),
}
```

**Example error handling:**

```rust
use url2ref::{generate, GenerationOptions};
use url2ref::generator::ReferenceGenerationError;

let options = GenerationOptions::default();

match generate("https://example.com", &options) {
    Ok(reference) => println!("{}", reference.wiki()),
    Err(ReferenceGenerationError::CurlError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(ReferenceGenerationError::ParseFailure) => {
        eprintln!("Could not extract metadata from page");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Complete Example

```rust
use url2ref::{
    generate, 
    GenerationOptions,
    GenerationOptionsBuilder,
};
use url2ref::generator::{
    TranslationOptions,
    TranslationProvider,
    ArchiveOptions,
    AiExtractionOptions,
    AiProvider,
};
use url2ref::generator::attribute_config::{
    AttributeConfig, 
    AttributePriority
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure translation
    let translation = TranslationOptions {
        provider: TranslationProvider::DeepL,
        source: None,
        target: Some("EN".to_string()),
        deepl_key: std::env::var("DEEPL_API_KEY").ok(),
        google_key: None,
    };

    // Configure archiving
    let archive = ArchiveOptions {
        include_archived: true,
        perform_archival: false,  // Don't create new archives
    };

    // Configure AI (disabled)
    let ai = AiExtractionOptions::default();

    // Build options
    let options = GenerationOptions {
        attribute_config: AttributeConfig::default(),
        translation_options: translation,
        archive_options: archive,
        ai_options: ai,
    };

    // Generate reference
    let url = "https://www.lemonde.fr/article";
    let reference = generate(url, &options)?;

    // Output all formats
    println!("=== MediaWiki ===");
    println!("{}", reference.wiki());
    
    println!("\n=== BibTeX ===");
    println!("{}", reference.bibtex());
    
    println!("\n=== Harvard ===");
    println!("{}", reference.harvard());

    Ok(())
}
```

## Module Structure

```
url2ref
├── lib.rs              # Main exports
├── attribute.rs        # Attribute types
├── reference.rs        # Reference enum
├── generator.rs        # Generation logic
├── parser.rs           # Metadata parsing
├── citation.rs         # Citation formatting
├── opengraph.rs        # Open Graph parser
├── schema_org.rs       # Schema.org parser
├── html_meta.rs        # HTML meta parser
├── doi.rs              # DOI resolution
├── zotero.rs           # Zotero/Citoid
├── ai_extractor.rs     # AI extraction
└── curl.rs             # HTTP client
```

## See Also

- [Full rustdoc](https://url2ref.github.io/url2ref/url2ref/) - Auto-generated API docs
- [Environment Variables](./environment-variables.md) - Configuration
- [CLI Usage](../cli/index.md) - Command-line interface
