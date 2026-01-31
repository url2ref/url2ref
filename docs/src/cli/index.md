# Command-Line Interface

The `url2ref-cli` provides a powerful command-line interface for generating citations from URLs.

## Overview

The CLI is designed for:

- **Scripting and automation** - Integrate citation generation into your workflows
- **Batch processing** - Process multiple URLs efficiently
- **Power users** - Full control over all options via command-line flags

## Quick Start

Generate a citation in MediaWiki format:

```bash
url2ref-cli --url "https://www.example.com/article"
```

Generate a BibTeX citation:

```bash
url2ref-cli --url "https://www.example.com/article" --format bibtex
```

## Command Synopsis

```
url2ref-cli [OPTIONS] --url <URL>

Options:
  -u, --url <URL>                    URL to generate citation for
  -f, --format <FORMAT>              Citation format [default: wiki]
                                     [possible values: wiki, bibtex, harvard]
  -m, --metadata-priority <TYPE>     Prioritize a specific metadata source
                                     [possible values: opengraph, schemaorg, htmlmeta, doi, zotero]
  -s, --source-lang <LANG>           Source language code (ISO 639)
  -t, --target-lang <LANG>           Target language for translation
      --translation-provider <PROV>  Translation service [default: deepl]
                                     [possible values: deepl, google]
  -i, --include-archived             Include archive URL [default: true]
      --ai-extraction                Enable AI metadata extraction
      --ai-provider <PROVIDER>       AI provider [default: openai]
                                     [possible values: openai, anthropic]
      --ai-model <MODEL>             AI model to use
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

### Basic Usage

```bash
# MediaWiki citation (default)
url2ref-cli -u "https://www.bbc.com/news/technology-12345"

# BibTeX for academic papers
url2ref-cli -u "https://arxiv.org/abs/2301.00001" -f bibtex

# Harvard style
url2ref-cli -u "https://www.nature.com/articles/s12345" -f harvard
```

### With Translation

Translate the title to English:

```bash
url2ref-cli \
  -u "https://www.lemonde.fr/article" \
  -t EN \
  --translation-provider deepl
```

### Prioritizing Metadata Sources

Force using Schema.org metadata:

```bash
url2ref-cli \
  -u "https://www.example.com/article" \
  -m schemaorg
```

### AI-Assisted Extraction

Fill in missing metadata using AI:

```bash
url2ref-cli \
  -u "https://www.example.com/article" \
  --ai-extraction \
  --ai-provider openai \
  --ai-model gpt-4o-mini
```

## In This Section

- [Basic Usage](./basic-usage.md) - Detailed usage examples
- [Citation Formats](./citation-formats.md) - Understanding output formats
- [Metadata Priority](./metadata-priority.md) - Controlling data sources
- [Translation](./translation.md) - Title translation options
- [AI Extraction](./ai-extraction.md) - AI-powered metadata extraction
