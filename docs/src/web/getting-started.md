# Getting Started

## Setup

```bash
# Build frontend
cd url2ref-web/npm
npm install
./build.sh
cd ../..

# Start server
cargo run --bin url2ref-web
```

Open `http://localhost:8000`.

## Usage

1. Enter a URL
2. Click "Generate" (or press Enter)
3. Switch formats using Wiki/BibTeX/Harvard tabs
4. Click 📋 to copy

## Options

Expand the Options panel for:

- **Translation:** Select target language, requires `DEEPL_API_KEY` or `GOOGLE_TRANSLATE_API_KEY`
- **Zotero/Citoid:** Use Wikipedia's citation service
- **AI Extraction:** Fill missing fields (requires `OPENAI_API_KEY` or `ANTHROPIC_API_KEY`)

## Multi-Source View

Click "Show sources" to compare metadata from OpenGraph, Schema.org, HTML Meta, etc. Click a value to use it.

## Archive Integration

Automatically checks Wayback Machine. Click "Create archive" to save the page.
