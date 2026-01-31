# Metadata Priority

Control which metadata source url2ref uses to extract information from web pages.

## Understanding Metadata Sources

Web pages can contain metadata from multiple sources, and they don't always agree. url2ref extracts from:

| Source | Flag Value | Description |
|--------|------------|-------------|
| Open Graph | `opengraph` | Facebook's metadata protocol |
| Schema.org | `schemaorg` | Structured data vocabulary |
| HTML Meta | `htmlmeta` | Standard HTML meta tags |
| DOI | `doi` | Digital Object Identifier metadata |
| Zotero | `zotero` | Wikipedia's Citoid service |

## Default Priority

By default, url2ref uses this priority order:

1. **Open Graph** - Most websites implement this for social sharing
2. **Schema.org** - Rich structured data, often more detailed
3. **HTML Meta** - Basic metadata, widely available

## Specifying Priority

Use the `--metadata-priority` (or `-m`) flag to prioritize a specific source:

```bash
# Prioritize Schema.org metadata
url2ref-cli -u "https://example.com" -m schemaorg

# Prioritize DOI metadata (for academic papers)
url2ref-cli -u "https://doi.org/10.1000/example" -m doi

# Prioritize Zotero/Citoid
url2ref-cli -u "https://example.com" -m zotero
```

## When to Use Each Source

### Open Graph (`opengraph`)

Best for:
- News articles
- Blog posts
- Social media content
- Most general web pages

```bash
url2ref-cli -u "https://www.theguardian.com/article" -m opengraph
```

Open Graph typically provides:
- `og:title` → Title
- `og:site_name` → Website name
- `article:author` → Author
- `article:published_time` → Date

### Schema.org (`schemaorg`)

Best for:
- News organizations with structured data
- E-commerce sites
- Pages with rich snippets

```bash
url2ref-cli -u "https://www.nytimes.com/article" -m schemaorg
```

Schema.org often provides more detailed information:
- Multiple author names
- Publisher information
- Article type classification

### HTML Meta (`htmlmeta`)

Best for:
- Older websites
- Simple pages without Open Graph
- Fallback when other sources fail

```bash
url2ref-cli -u "https://old-website.com/page" -m htmlmeta
```

HTML Meta extracts from:
- `<title>` tag
- `<meta name="author">`
- `<meta name="description">`

### DOI (`doi`)

Best for:
- Academic papers
- Journal articles
- Scientific publications

```bash
url2ref-cli -u "https://doi.org/10.1038/nature12373" -m doi
```

DOI resolution provides authoritative:
- Paper title
- All authors
- Publication date
- Journal name
- Publisher

### Zotero (`zotero`)

Best for:
- Complex pages
- Sites supported by Zotero translators
- Wikipedia citations

```bash
url2ref-cli -u "https://www.jstor.org/stable/12345" -m zotero
```

Zotero uses Wikipedia's Citoid service which has specialized extractors for many websites.

## Comparing Sources

You can see what each source extracts using the web interface, which shows all sources side-by-side. For CLI, try different priorities:

```bash
# Compare outputs
echo "=== Open Graph ===" && url2ref-cli -u "$URL" -m opengraph
echo "=== Schema.org ===" && url2ref-cli -u "$URL" -m schemaorg
echo "=== HTML Meta ===" && url2ref-cli -u "$URL" -m htmlmeta
```

## Real-World Examples

### News Article

```bash
# BBC has excellent Open Graph metadata
url2ref-cli -u "https://www.bbc.com/news/technology-12345" -m opengraph
```

### Academic Paper

```bash
# Nature articles work best with DOI
url2ref-cli -u "https://www.nature.com/articles/s41586-024-00001-0" -m doi
```

### Wikipedia Article

```bash
# Wikipedia works well with Zotero
url2ref-cli -u "https://en.wikipedia.org/wiki/Rust_(programming_language)" -m zotero
```

## Tips

1. **Try the default first** - The default priority works well for most sites

2. **Use DOI for papers** - If the page has a DOI, always prefer `-m doi`

3. **Check multiple sources** - The web interface lets you compare all sources

4. **Schema.org for news** - Major news sites often have the best Schema.org data

## Next Steps

- [Translation](./translation.md) - Translate titles to your language
- [AI Extraction](./ai-extraction.md) - Use AI to fill missing fields
- [Web Interface](../web/index.md) - Visually compare metadata sources
