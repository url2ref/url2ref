# Introduction

**url2ref** generates formatted citations from URLs by extracting metadata from web pages.

## Metadata Sources

- [Open Graph](https://ogp.me/)
- [Schema.org](https://schema.org/)
- HTML Meta Tags
- DOI Resolution
- Zotero/Citoid (optional)
- AI Extraction (optional)

## Citation Formats

| Format | Use Case |
|--------|----------|
| MediaWiki | Wikipedia editing |
| BibTeX | LaTeX/academic papers |
| Harvard | Academic writing |

## Example

```bash
url2ref-cli --url "https://www.bbc.com/news/technology-12345678"
```

Output:
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

## Next

See [Installation](./installation.md) to get started.
