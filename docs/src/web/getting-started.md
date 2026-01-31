# Getting Started with the Web Interface

This guide walks you through setting up and using the url2ref web interface.

## Prerequisites

Before starting, ensure you have:

- Rust toolchain installed ([rustup.rs](https://rustup.rs/))
- Node.js and npm installed
- Git (to clone the repository)

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/url2ref/url2ref.git
cd url2ref
```

### Step 2: Build Frontend Assets

The web interface uses Bootstrap and custom SCSS. Build these first:

```bash
cd url2ref-web/npm
npm install
./build.sh
cd ../..
```

### Step 3: Start the Server

```bash
cargo run --bin url2ref-web
```

You should see output like:
```
🔧 Configured for development.
   >> address: 127.0.0.1
   >> port: 8000
🚀 Rocket has launched from http://127.0.0.1:8000
```

### Step 4: Open the Interface

Navigate to `http://localhost:8000` in your browser.

## Your First Citation

### 1. Enter a URL

Type or paste a URL into the input field:

```
https://www.bbc.com/news/technology-12345678
```

### 2. Click Generate

Press the "Generate" button or hit Enter.

### 3. View Results

The interface displays:

- **Extracted Fields**: Title, author, date, etc.
- **Citation Formats**: Wiki, BibTeX, Harvard tabs
- **Metadata Sources**: Where each field came from

### 4. Copy the Citation

Click the copy button (📋) next to your preferred format.

## Interface Overview

### URL Input Section

The main input area where you enter the URL to cite.

```
┌─────────────────────────────────────────────────────┐
│ URL: [________________________________] [Generate]  │
└─────────────────────────────────────────────────────┘
```

### Options Panel

Expand to access advanced features:

- **Translation**: Translate the title to your language
- **Zotero/Citoid**: Use Wikipedia's citation service
- **AI Extraction**: Fill missing fields with AI

### Extracted Fields

Shows the metadata extracted from the page:

```
┌─────────────────────────────────────────────────────┐
│ Title:     The Future of Technology                 │
│ Author:    Jane Smith                               │
│ Date:      2024-01-15                               │
│ Website:   TechNews                                 │
│ Publisher: TechNews Media Inc.                      │
│ Language:  en                                       │
│ URL:       https://technews.com/article/12345      │
└─────────────────────────────────────────────────────┘
```

### Citation Output

View and copy citations in different formats:

```
┌─────────────────────────────────────────────────────┐
│ [Wiki] [BibTeX] [Harvard]                           │
├─────────────────────────────────────────────────────┤
│ {{cite web                                          │
│   | title = The Future of Technology                │
│   | author = Jane Smith                             │
│   | date = 2024-01-15                               │
│   | website = TechNews                              │
│   | url = https://technews.com/article/12345       │
│   | access-date = 2024-01-20                        │
│ }}                                              [📋]│
└─────────────────────────────────────────────────────┘
```

## Enabling Optional Features

### Translation

1. Expand the Options panel
2. Check "Translate title"
3. Select your target language
4. (Optional) Choose provider: DeepL or Google

Note: Server must have API keys configured. See [Configuration](./configuration.md).

### Zotero/Citoid

1. Expand the Options panel
2. Check "Enable Zotero/Citoid"

This uses Wikipedia's citation service for enhanced metadata extraction.

### AI Extraction

1. Expand the Options panel
2. Check "Enable AI extraction"
3. Select provider (OpenAI or Anthropic)
4. Enter your API key (processed client-side, not stored)

## Multi-Source View

Click "Show sources" to see metadata from all available sources:

```
┌─────────────────────────────────────────────────────────────────┐
│ Title                                                           │
│ ─────────────────────────────────────────────────────────────── │
│ ○ OpenGraph:  The Future of Technology                    [use] │
│ ○ Schema.org: Future of Technology | TechNews             [use] │
│ ○ HTML Meta:  TechNews - Future of Technology             [use] │
└─────────────────────────────────────────────────────────────────┘
```

Click "[use]" to select a specific source's value for that field.

## Archive Integration

The interface automatically checks for Wayback Machine archives:

- **✓ Archived**: Shows archive URL and date
- **⚠ Not archived**: Offers to create an archive

Click "Create archive" to save the page to the Wayback Machine.

## Tips for Best Results

### For News Articles

Most news sites have excellent metadata. Just paste the URL and generate.

### For Academic Papers

Use DOI URLs when available:
```
https://doi.org/10.1000/xyz123
```

### For Problematic Pages

If extraction fails:
1. Enable Zotero/Citoid
2. If still missing data, enable AI extraction

### For Non-English Content

1. Enable translation
2. Select your target language
3. Both original and translated titles will appear in the citation

## Troubleshooting

### "Failed to fetch metadata"

- Check if the URL is accessible
- Some sites block automated requests
- Try the Zotero option

### Missing Fields

- Enable AI extraction for stubborn pages
- Check the multi-source view for alternate values

### Server Won't Start

1. Ensure port 8000 is available
2. Check that frontend assets are built:
   ```bash
   ls url2ref-web/static/css/
   ```
3. Rebuild if needed:
   ```bash
   cd url2ref-web/npm && ./build.sh
   ```

## Next Steps

- [Features](./features.md) - Detailed feature documentation
- [Configuration](./configuration.md) - Server configuration options
- [CLI Usage](../cli/index.md) - Command-line alternative
