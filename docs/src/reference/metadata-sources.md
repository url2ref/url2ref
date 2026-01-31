# Metadata Sources

## Overview

| Source | Best For |
|--------|----------|
| Open Graph | News, blogs, social content |
| Schema.org | News orgs with structured data |
| HTML Meta | Older/simple websites |
| DOI | Academic papers |
| Zotero/Citoid | Academic databases, complex pages |
| AI | Fallback for missing data |

## Open Graph

Extracts `og:title`, `og:site_name`, `article:author`, `article:published_time` from `<meta property="og:*">` tags.

## Schema.org

Parses JSON-LD (`<script type="application/ld+json">`) for `Article`, `NewsArticle`, `ScholarlyArticle` types. Often has multiple authors and publisher info.

## HTML Meta

Extracts `<title>` and `<meta name="author">`. Universal but limited.

## DOI

Resolves DOIs via CrossRef/DataCite. Authoritative metadata for academic papers including all authors, journal, and publication details.

## Zotero/Citoid

Uses Wikipedia's Citoid API with specialized translators for JSTOR, PubMed, Google Scholar, news sites, and hundreds more.

## AI Extraction

Sends page content to OpenAI or Anthropic to extract fields when traditional parsing fails. Requires API key.

## Default Priority

1. Open Graph
2. Schema.org
3. HTML Meta
4. DOI (if available)
5. Zotero (if enabled)
6. AI (if enabled)
