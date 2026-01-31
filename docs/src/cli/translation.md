# Translation

Translate article titles using DeepL or Google Translate.

## Usage

```bash
# Translate to English (auto-detect source)
url2ref-cli -u "https://www.lemonde.fr/article" --target-lang EN

# Specify source language
url2ref-cli -u "https://www.spiegel.de/article" --source-lang DE --target-lang EN

# Use Google instead of DeepL
url2ref-cli -u "https://example.fr" --target-lang EN --translation-provider google
```

## API Keys

```bash
export DEEPL_API_KEY="your-key"        # deepl.com/pro-api
export GOOGLE_TRANSLATE_API_KEY="your-key"  # console.cloud.google.com
```

## Output

MediaWiki includes `trans-title`:
```
{{cite web
  | title = L'intelligence artificielle transforme l'industrie
  | trans-title = Artificial intelligence transforms industry
  | language = fr
  ...
}}
```

## Language Codes

Use ISO 639-1: `EN`, `DE`, `FR`, `ES`, `IT`, `JA`, `ZH`, `RU`, `PT`, `NL`

DeepL variants: `EN-US`, `EN-GB`, `PT-BR`, `PT-PT`

## Shell Configuration

Add to your shell profile for convenience:

```bash
# ~/.bashrc or ~/.zshrc
export DEEPL_API_KEY="your-key"

# Alias for common translation tasks
alias cite-translate='url2ref-cli --target-lang EN'
```

## Next Steps

- [AI Extraction](./ai-extraction.md) - Use AI to extract missing metadata
- [Environment Variables](../reference/environment-variables.md) - All configuration options
- [Web Interface](../web/index.md) - Translate via the browser interface
