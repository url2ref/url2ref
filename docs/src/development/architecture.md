# Architecture

Technical overview of url2ref's internal architecture.

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interfaces                          │
├────────────────────────────┬────────────────────────────────────┤
│        url2ref-cli         │          url2ref-web               │
│     (Command Line)         │        (Web Server)                │
└────────────────────────────┴────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        url2ref (Core Library)                    │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │  Generator  │──│   Parser    │──│   Citation Formatters   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│         │               │                      │                 │
│         ▼               ▼                      ▼                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Metadata Sources                          ││
│  ├──────────┬──────────┬──────────┬──────────┬────────┬───────┤│
│  │ OpenGraph│Schema.org│ HTML Meta│   DOI    │ Zotero │   AI  ││
│  └──────────┴──────────┴──────────┴──────────┴────────┴───────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Core Library (`url2ref`)

### Module Responsibilities

| Module | Purpose |
|--------|---------|
| `lib.rs` | Public API, re-exports |
| `generator.rs` | Orchestrates reference generation |
| `parser.rs` | Coordinates metadata extraction |
| `attribute.rs` | Type definitions for metadata |
| `reference.rs` | Reference enum and accessors |
| `citation.rs` | Citation format builders |

### Data Flow

```
URL → HTTP Fetch → HTML Parse → Metadata Extract → Merge → Reference → Citation
```

1. **HTTP Fetch** (`curl.rs`)
   - Fetches HTML content from URL
   - Handles redirects, encoding

2. **HTML Parse** (`parser.rs`)
   - Parses HTML into DOM
   - Initializes source-specific parsers

3. **Metadata Extract** (various modules)
   - Each parser extracts attributes
   - Results collected into `AttributeCollection`

4. **Merge** (`generator.rs`)
   - Applies priority configuration
   - Selects best value for each field

5. **Reference Creation** (`reference.rs`)
   - Constructs appropriate `Reference` variant
   - Applies post-processing (translation, archive)

6. **Citation Formatting** (`citation.rs`)
   - Formats `Reference` into output string
   - Builder pattern for each format

## Metadata Parsers

### Parser Architecture

Each metadata source has a dedicated parser:

```rust
// Simplified parser interface
trait MetadataParser {
    fn parse(&self, html: &Html) -> AttributeCollection;
}
```

### Open Graph Parser (`opengraph.rs`)

Extracts `<meta property="og:*">` tags:

```rust
// Key extractions
og:title       → Title
og:site_name   → Site
article:author → Author
article:published_time → Date
```

### Schema.org Parser (`schema_org.rs`)

Parses JSON-LD and microdata:

```rust
// Supports multiple formats
<script type="application/ld+json">  // JSON-LD
<div itemscope itemtype="...">       // Microdata
```

Handles nested structures:
- `Article`, `NewsArticle`, `ScholarlyArticle`
- `Person`, `Organization`
- `WebPage`, `WebSite`

### HTML Meta Parser (`html_meta.rs`)

Extracts standard HTML elements:

```rust
<title>           → Title
<meta name="..."> → Various attributes
```

### DOI Parser (`doi.rs`)

Resolves DOIs via CrossRef/DataCite:

```rust
// Flow
URL → Extract DOI → Query API → Parse response
```

### Zotero/Citoid (`zotero.rs`)

Uses Wikipedia's Citoid API:

```rust
// API call
POST https://en.wikipedia.org/api/rest_v1/data/citation/mediawiki/{url}
```

### AI Extractor (`ai_extractor.rs`)

Sends page content to LLM:

```rust
// Prompt structure
System: "Extract citation metadata..."
User: <page content>
Response: JSON with fields
```

## Attribute System

### Type Definitions

```rust
// Attribute type identifiers
pub enum AttributeType {
    Title, Author, Date, Site, Url, Language, ...
}

// Attribute values
pub enum Attribute {
    Title(String),
    Authors(Vec<Author>),
    Date(Date),
    ...
}
```

### Priority System

```rust
pub struct AttributePriority {
    pub priority: Vec<MetadataType>,
}

// Example: [SchemaOrg, OpenGraph, HtmlMeta]
// SchemaOrg value used if available, else OpenGraph, else HtmlMeta
```

### Attribute Collection

```rust
// Map of attribute type → attribute value per source
type AttributeCollection = HashMap<AttributeType, Attribute>;

// Multi-source: attribute type → (source → value)
type MultiSourceAttributeCollection = 
    HashMap<AttributeType, HashMap<MetadataType, Attribute>>;
```

## Citation Formatters

### Builder Pattern

```rust
pub trait CitationBuilder {
    fn try_add(self, attr: &Option<Attribute>) -> Self;
    fn build(self) -> String;
}

// Usage in Reference
impl Reference {
    pub fn wiki(&self) -> String {
        self.build_citation(WikiCitation::new())
    }
}
```

### Format Implementations

| Format | Module | Builder |
|--------|--------|---------|
| MediaWiki | `citation.rs` | `WikiCitation` |
| BibTeX | `citation.rs` | `BibTeXCitation` |
| Harvard | `citation.rs` | `HarvardCitation` |

## Web Interface Architecture

### Stack

```
Frontend                    Backend
──────────────────────────────────────
Bootstrap 5 (CSS)          Rocket (Rust)
Vanilla JS                 Tera (Templates)
                          url2ref (Library)
```

### Request Flow

```
Browser → Rocket Route → Handler → url2ref → Response → JSON → Browser
```

### API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/` | Home page |
| POST | `/generate` | Generate citation |
| POST | `/archive` | Create archive |

### Response Format

```rust
struct GenerateResponse {
    success: bool,
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
