# Metadata Priority

## Sources

| Source | Flag | Best For |
|--------|------|----------|
| Open Graph | `opengraph` | News, blogs, social content |
| Schema.org | `schemaorg` | News orgs with structured data |
| HTML Meta | `htmlmeta` | Older/simple websites |
| DOI | `doi` | Academic papers |
| Zotero | `zotero` | Academic databases, complex pages |

## Usage

```bash
url2ref-cli -u "https://example.com" -m schemaorg
url2ref-cli -u "https://doi.org/10.1038/nature12373" -m doi
url2ref-cli -u "https://www.jstor.org/stable/12345" -m zotero
```

## Recommendations

- **News articles:** Default priority works well (Open Graph first)
- **Academic papers with DOI:** Use `-m doi`
- **JSTOR/PubMed:** Use `-m zotero`
- **Sites with rich snippets:** Try `-m schemaorg`
