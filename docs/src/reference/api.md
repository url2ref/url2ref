# API

Rust library for programmatic citation generation.

## Installation

```toml
[dependencies]
url2ref = { git = "https://github.com/url2ref/url2ref" }
```

## Quick Start

```rust
use url2ref::{generate, GenerationOptions};

let options = GenerationOptions::default();
let reference = generate("https://example.com/article", &options)?;

println!("{}", reference.wiki());
println!("{}", reference.bibtex());
println!("{}", reference.harvard());
```

## Configuration

```rust
use url2ref::{GenerationOptions, GenerationOptionsBuilder};
use url2ref::generator::{TranslationOptions, TranslationProvider, ArchiveOptions, AiExtractionOptions};

let options = GenerationOptions {
    attribute_config: AttributeConfig::default(),
    translation_options: TranslationOptions {
        provider: TranslationProvider::DeepL,
        target: Some("EN".to_string()),
        deepl_key: std::env::var("DEEPL_API_KEY").ok(),
        ..Default::default()
    },
    archive_options: ArchiveOptions {
        include_archived: true,
        perform_archival: false,
    },
    ai_options: AiExtractionOptions::default(),
};
```

## Functions

| Function | Description |
|----------|-------------|
| `generate(url, options)` | Generate reference from URL |
| `generate_from_file(path, options)` | Generate from local HTML |
| `parse_all_metadata(url)` | Extract all metadata without generating |
| `fetch_parse_info(url, options)` | Fetch and parse for reuse |

## Types

| Type | Description |
|------|-------------|
| `Reference` | Enum with `NewsArticle`, `ScholarlyArticle`, `GenericReference` |
| `Attribute` | Enum for `Title`, `Authors`, `Date`, `Url`, etc. |
| `GenerationOptions` | Configuration for generation |

## Error Handling

```rust
match generate(url, &options) {
    Ok(reference) => println!("{}", reference.wiki()),
    Err(ReferenceGenerationError::CurlError(e)) => eprintln!("Network: {}", e),
    Err(ReferenceGenerationError::ParseFailure) => eprintln!("No metadata found"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Generate Docs

```bash
cargo doc --no-deps --open
```
