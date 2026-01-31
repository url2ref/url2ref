# Architecture

## Overview

```
┌─────────────────────────────────────────────────────┐
│              url2ref-cli / url2ref-web              │
└─────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────┐
│                 url2ref (Core Library)              │
├─────────────────────────────────────────────────────┤
│  Generator → Parser → Citation Formatters           │
│       │         │                                   │
│       ▼         ▼                                   │
│  ┌─────────────────────────────────────────────┐   │
│  │ OpenGraph│Schema.org│HTML Meta│DOI│Zotero│AI│   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

## Data Flow

```
URL → HTTP Fetch → HTML Parse → Metadata Extract → Merge → Reference → Citation
```

## Core Modules

| Module | Purpose |
|--------|---------|
| `generator.rs` | Orchestrates generation |
| `parser.rs` | Coordinates extraction |
| `attribute.rs` | Metadata types |
| `reference.rs` | Reference enum |
| `citation.rs` | Format builders |

## Parsers

Each source has a dedicated parser:
- `opengraph.rs` - `<meta property="og:*">`
- `schema_org.rs` - JSON-LD and microdata
- `html_meta.rs` - `<title>`, `<meta name="*">`
- `doi.rs` - CrossRef/DataCite resolution
- `zotero.rs` - Wikipedia Citoid API
- `ai_extractor.rs` - LLM extraction

## Attribute System

```rust
// Per-source collection
type AttributeCollection = HashMap<AttributeType, Attribute>;

// Multi-source for comparison
type MultiSourceAttributeCollection = 
    HashMap<AttributeType, HashMap<MetadataType, Attribute>>;
```

Priority determines which source's value is used when merging.

## Citation Builders

```rust
trait CitationBuilder {
    fn try_add(self, attr: &Option<Attribute>) -> Self;
    fn build(self) -> String;
}
```

Implementations: `WikiCitation`, `BibTeXCitation`, `HarvardCitation`

## Web Interface

```
Browser → Rocket Route → Handler → url2ref → JSON → Browser
```

| Endpoint | Purpose |
|----------|---------|
| `GET /` | Home page |
| `POST /generate` | Generate citation |
| `POST /archive` | Create archive |
    bibtex: Option<String>,
    wiki: Option<String>,
    harvard: Option<String>,
    fields: Option<ReferenceFields>,
    multi_source: Option<MultiSourceFields>,
    error: Option<String>,
}
```

## CLI Architecture

### Argument Parsing

Uses `clap` derive macros:

```rust
#[derive(Parser)]
struct CommandLineArgs {
    #[clap(short, long)]
    url: String,
    
    #[clap(short, long, value_enum)]
    format: CitationFormat,
    // ...
}
```

### Flow

```
Args → Parse → Build Options → generate() → Format Output → Print
```

## Error Handling

### Error Types

```rust
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

### Error Propagation

Uses `thiserror` for error definitions and `?` operator for propagation.

## Dependencies

### Key Crates

| Crate | Purpose |
|-------|---------|
| `scraper` | HTML parsing |
| `serde_json` | JSON handling |
| `curl` | HTTP requests |
| `chrono` | Date handling |
| `clap` | CLI argument parsing |
| `rocket` | Web framework |
| `tera` | Templating |
| `deepl-api` | Translation |

## Performance Considerations

### HTTP Caching

`ParseInfo` allows caching fetched HTML:

```rust
// Fetch once
let parse_info = fetch_parse_info(url, None)?;

// Reuse multiple times
let reference = generate_from_parse_info(&parse_info, &opts)?;
let metadata = parse_all_metadata_from_parse_info(&parse_info);
```

### Parser Initialization

Parsers are only initialized for configured sources:

```rust
let parsers = options
    .attribute_config
    .parsers_used();  // Only active parsers
```

## Extension Points

### Adding a Metadata Source

1. Create parser module implementing extraction
2. Add variant to `MetadataType`
3. Update `parser.rs` initialization
4. Add to priority options

### Adding a Citation Format

1. Implement `CitationBuilder` trait
2. Add method to `Reference` enum
3. Update CLI and web interfaces

### Adding an Attribute

1. Add to `AttributeType` enum
2. Add to `Attribute` enum
3. Update parsers to extract it
4. Update citation formatters

## Testing Strategy

### Unit Tests

Test individual parsers with known HTML:

```rust
#[test]
fn test_opengraph_title() {
    let html = r#"<meta property="og:title" content="Test">"#;
    // ...
}
```

### Integration Tests

Test full generation with real/mock URLs:

```rust
#[test]
fn test_full_generation() {
    let reference = generate("file://test.html", &opts)?;
    assert!(reference.wiki().contains("title"));
}
```

### Test Data

Located in `url2ref/tests/data/` - HTML fixtures for various scenarios.
