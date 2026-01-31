# Introduction

<div align="center">
  <h2>url2ref</h2>
  <strong>Automatic reference generation for web resources</strong>
</div>

## What is url2ref?

**url2ref** is a tool that automatically generates properly formatted citations from URLs. Whether you're writing a research paper, creating Wikipedia articles, or documenting sources, url2ref extracts metadata from web pages and formats it into standard citation styles.

## Why url2ref?

Manually citing web resources can be tedious and error-prone. You need to:

- Find the article title
- Identify the author(s)
- Locate the publication date
- Note the website name
- Format everything correctly for your citation style

url2ref automates this entire process by intelligently parsing web page metadata.

## How It Works

url2ref extracts metadata from multiple sources embedded in web pages:

1. **[Open Graph](https://ogp.me/)** - Metadata protocol originally created by Facebook
2. **[Schema.org](https://schema.org/)** - Structured data vocabulary supported by major search engines
3. **HTML Meta Tags** - Standard HTML metadata elements
4. **DOI Resolution** - Digital Object Identifier metadata for scholarly articles
5. **Zotero/Citoid** - Wikipedia's citation service (optional)
6. **AI Extraction** - LLM-powered extraction for missing fields (optional)

The tool then merges this information using a priority system and outputs citations in your desired format.

## Supported Citation Formats

| Format | Description | Example Use Case |
|--------|-------------|------------------|
| **MediaWiki** | `{{cite web}}` template | Wikipedia editing |
| **BibTeX** | LaTeX bibliography format | Academic papers |
| **Harvard** | In-text referencing style | Academic writing |

## Interfaces

url2ref provides two ways to generate citations:

### Command-Line Interface (CLI)

Perfect for scripting, automation, and power users:

```bash
url2ref-cli --url "https://example.com/article" --format wiki
```

### Web Interface

A user-friendly browser-based interface with:

- Real-time preview of citations
- Interactive metadata source selection
- Copy-to-clipboard functionality
- Archive URL integration via Wayback Machine

## Quick Example

Input a URL like:
```
https://www.bbc.com/news/technology-12345678
```

Get a formatted citation:
```
{{cite web
  | title = Example News Article
  | author = John Smith
  | date = 2024-01-15
  | website = BBC News
  | url = https://www.bbc.com/news/technology-12345678
  | access-date = 2024-01-20
}}
```

## Getting Started

Ready to start generating citations? Head to the [Installation](./installation.md) guide to set up url2ref on your system.
