# Contributing

## Setup

```bash
git clone https://github.com/YOUR_USERNAME/url2ref.git
cd url2ref
cargo build
cargo test
```

For web interface:
```bash
cd url2ref-web/npm && npm install && ./build.sh
```

## Workflow

1. Create branch: `git checkout -b feature/your-feature`
2. Make changes
3. Run `cargo fmt` and `cargo clippy`
4. Run `cargo test`
5. Commit with clear message
6. Open PR

## Commit Format

```
type(scope): description

feat(parser): add Dublin Core support
fix(cli): handle URLs with special characters
docs: update installation guide
```

## Project Structure

```
url2ref/           # Core library
├── src/
│   ├── lib.rs          # Public API
│   ├── generator.rs    # Reference generation
│   ├── parser.rs       # Metadata parsing
│   ├── opengraph.rs    # Open Graph parser
│   ├── schema_org.rs   # Schema.org parser
│   ├── doi.rs          # DOI resolution
│   ├── zotero.rs       # Zotero/Citoid
│   ├── ai_extractor.rs # AI extraction
│   └── citation.rs     # Citation formatters
url2ref-cli/       # CLI application
url2ref-web/       # Web interface
docs/              # This documentation
```

## Adding a Metadata Source

1. Create parser in `url2ref/src/`
2. Add to `MetadataType` enum
3. Add tests
4. Update docs

## Adding a Citation Format

1. Add formatter in `citation.rs`
2. Add method to `Reference`
3. Update CLI and web interface
4. Add tests
