# CLI

## Usage

```
url2ref-cli [OPTIONS] --url <URL>

Options:
  -u, --url <URL>                    URL to cite
  -f, --format <FORMAT>              wiki, bibtex, harvard [default: wiki]
  -m, --metadata-priority <TYPE>     opengraph, schemaorg, htmlmeta, doi, zotero
  -t, --target-lang <LANG>           Translate title (ISO 639)
      --translation-provider <PROV>  deepl, google [default: deepl]
      --ai-extraction                Enable AI extraction
      --ai-provider <PROVIDER>       openai, anthropic [default: openai]
  -h, --help                         Print help
```

## Examples

```bash
# MediaWiki (default)
url2ref-cli -u "https://www.bbc.com/news/technology-12345"

# BibTeX
url2ref-cli -u "https://arxiv.org/abs/2301.00001" -f bibtex

# With translation
url2ref-cli -u "https://www.lemonde.fr/article" -t EN

# With AI extraction
url2ref-cli -u "https://example.com" --ai-extraction
```

## In This Section

- [Basic Usage](./basic-usage.md)
- [Citation Formats](./citation-formats.md)
- [Metadata Priority](./metadata-priority.md)
- [Translation](./translation.md)
- [AI Extraction](./ai-extraction.md)
