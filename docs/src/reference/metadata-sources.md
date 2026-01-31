# Supported Metadata Sources

url2ref extracts citation information from multiple metadata sources embedded in web pages.

## Overview

| Source | Protocol | Common On |
|--------|----------|-----------|
| [Open Graph](#open-graph) | `og:*` meta tags | Social media, news sites |
| [Schema.org](#schemaorg) | JSON-LD, microdata | News, e-commerce, search-optimized |
| [HTML Meta](#html-meta) | Standard meta tags | All websites |
| [DOI](#doi) | DOI resolution | Academic papers |
| [Zotero/Citoid](#zoterocitoid) | Wikipedia API | Academic databases |
| [AI Extraction](#ai-extraction) | LLM analysis | Fallback for any page |

---

## Open Graph

### What is Open Graph?

[Open Graph](https://ogp.me/) is a protocol created by Facebook that allows web pages to become rich objects in a social graph. Most websites implement Open Graph tags for better social media sharing.

### Extracted Tags

| Tag | Maps To |
|-----|---------|
| `og:title` | Title |
| `og:site_name` | Website |
| `og:url` | URL |
| `og:locale` | Language |
| `article:author` | Author |
| `article:published_time` | Date |
| `article:publisher` | Publisher |

### Example HTML

```html
<meta property="og:title" content="Article Title" />
<meta property="og:site_name" content="Example News" />
<meta property="og:url" content="https://example.com/article" />
<meta property="article:author" content="Jane Smith" />
<meta property="article:published_time" content="2024-01-15T10:30:00Z" />
```

### Pros & Cons

✅ **Pros:**
- Widely implemented
- Reliable title and site name
- Good date formatting

❌ **Cons:**
- Author often missing
- Limited to basic fields

---

## Schema.org

### What is Schema.org?

[Schema.org](https://schema.org/) is a collaborative vocabulary for structured data, supported by Google, Microsoft, Yahoo, and Yandex. It provides detailed semantic markup.

### Supported Types

url2ref recognizes these Schema.org types:

| Type | Use Case |
|------|----------|
| `Article` | General articles |
| `NewsArticle` | News content |
| `ScholarlyArticle` | Academic papers |
| `WebPage` | Generic pages |
| `Organization` | Publisher info |
| `Person` | Author info |

### Extracted Properties

| Property | Maps To |
|----------|---------|
| `headline` | Title |
| `author.name` | Author |
| `datePublished` | Date |
| `publisher.name` | Publisher |
| `inLanguage` | Language |
| `mainEntityOfPage` | URL |

### Example JSON-LD

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "NewsArticle",
  "headline": "Article Title",
  "author": {
    "@type": "Person",
    "name": "Jane Smith"
  },
  "datePublished": "2024-01-15",
  "publisher": {
    "@type": "Organization",
    "name": "Example News"
  }
}
</script>
```

### Pros & Cons

✅ **Pros:**
- Rich, detailed metadata
- Multiple authors supported
- Publisher information
- Article type classification

❌ **Cons:**
- Not all sites implement it
- Can be inconsistently structured

---

## HTML Meta

### Overview

Standard HTML meta tags provide basic metadata. This is the oldest and most universal source.

### Extracted Tags

| Tag | Maps To |
|-----|---------|
| `<title>` | Title |
| `<meta name="author">` | Author |
| `<meta name="description">` | (context) |
| `<meta name="language">` | Language |
| `<meta http-equiv="content-language">` | Language |

### Example HTML

```html
<head>
  <title>Article Title - Example Site</title>
  <meta name="author" content="Jane Smith" />
  <meta name="language" content="en" />
</head>
```

### Pros & Cons

✅ **Pros:**
- Universal availability
- Simple structure

❌ **Cons:**
- Limited information
- Title often includes site name
- No date field

---

## DOI

### What is DOI?

[Digital Object Identifier](https://www.doi.org/) is a persistent identifier for digital objects, primarily used for academic publications.

### How It Works

1. url2ref detects DOI in the URL or page content
2. Queries DOI resolution service (CrossRef, DataCite)
3. Retrieves authoritative metadata

### Extracted Fields

| Field | Source |
|-------|--------|
| Title | DOI metadata |
| Authors | DOI metadata (all authors) |
| Date | Publication date |
| Journal | Container title |
| Publisher | Publisher name |
| Volume/Issue | Publication details |

### DOI URL Formats

url2ref recognizes:
- `https://doi.org/10.1000/xyz123`
- `https://dx.doi.org/10.1000/xyz123`
- DOIs embedded in publisher URLs

### Pros & Cons

✅ **Pros:**
- Authoritative source
- Complete author lists
- Accurate publication dates
- Journal/publisher info

❌ **Cons:**
- Only for DOI-registered content
- Primarily academic

---

## Zotero/Citoid

### What is Citoid?

[Citoid](https://www.mediawiki.org/wiki/Citoid) is Wikipedia's citation service. It uses [Zotero translators](https://www.zotero.org/support/translators) to extract metadata from hundreds of websites.

### How It Works

1. url2ref sends URL to Citoid API
2. Citoid runs appropriate translator
3. Structured metadata is returned

### Supported Sites

Zotero has translators for:
- Academic databases (JSTOR, PubMed, Google Scholar)
- News sites (NYTimes, Guardian, BBC)
- Library catalogs
- Government publications
- And hundreds more...

### Pros & Cons

✅ **Pros:**
- Site-specific extractors
- High accuracy for supported sites
- Handles JavaScript-rendered content

❌ **Cons:**
- Requires network request to Wikipedia
- Not all sites supported
- May be slow

---

## AI Extraction

### Overview

When traditional extraction methods fail, AI language models can analyze page content directly.

### How It Works

1. Page HTML is fetched
2. Content is sent to AI provider
3. AI identifies citation fields
4. Results merged with other sources

### Supported Providers

| Provider | Models |
|----------|--------|
| OpenAI | GPT-4o, GPT-4o-mini |
| Anthropic | Claude 3 Haiku, Sonnet, Opus |

### Pros & Cons

✅ **Pros:**
- Works on any page
- Handles unstructured content
- Fills gaps from other sources

❌ **Cons:**
- Requires API key
- Costs per request
- May hallucinate data
- Slower than parsing

---

## Source Priority

### Default Order

1. **Open Graph** - Most reliable for general sites
2. **Schema.org** - Rich data when available
3. **HTML Meta** - Universal fallback
4. **DOI** - Authoritative for academic content
5. **Zotero** - Specialized extractors
6. **AI** - Last resort gap-filling

### Customizing Priority

CLI:
```bash
url2ref-cli -u "URL" -m schemaorg
```

The web interface allows interactive source selection per field.

---

## Checking What's Available

### View Page Metadata

Use browser developer tools:

1. Open DevTools (F12)
2. Elements tab → Search for `og:` or `schema`
3. Or use: `document.querySelectorAll('meta[property^="og:"]')`

### Online Tools

- [Open Graph Debugger](https://developers.facebook.com/tools/debug/)
- [Schema.org Validator](https://validator.schema.org/)
- [Google Rich Results Test](https://search.google.com/test/rich-results)

## Next Steps

- [Citation Formats](./citation-formats.md) - Output format reference
- [API Documentation](./api.md) - Library API
- [CLI Metadata Priority](../cli/metadata-priority.md) - CLI usage
