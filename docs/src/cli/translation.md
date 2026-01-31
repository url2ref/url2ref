# Title Translation

url2ref can automatically translate article titles using DeepL or Google Translate.

## Overview

When citing foreign-language sources, you may need to include a translated title. url2ref supports automatic translation via:

- **DeepL** (default) - High-quality neural machine translation
- **Google Cloud Translation** - Wide language support

## Basic Usage

To translate a title to English:

```bash
url2ref-cli -u "https://www.lemonde.fr/article" --target-lang EN
```

To specify both source and target languages:

```bash
url2ref-cli -u "https://www.spiegel.de/article" \
  --source-lang DE \
  --target-lang EN
```

## API Keys

Translation services require API keys set as environment variables.

### DeepL (Default)

1. Get a free API key at [deepl.com/pro-api](https://www.deepl.com/pro-api)
2. Set the environment variable:

```bash
export DEEPL_API_KEY="your-api-key-here"
```

### Google Cloud Translation

1. Set up a Google Cloud project with the Translation API enabled
2. Create an API key
3. Set the environment variable:

```bash
export GOOGLE_TRANSLATE_API_KEY="your-api-key-here"
```

## Choosing a Translation Provider

Use `--translation-provider` to select the service:

```bash
# Use DeepL (default)
url2ref-cli -u "https://example.fr" --target-lang EN --translation-provider deepl

# Use Google Translate
url2ref-cli -u "https://example.fr" --target-lang EN --translation-provider google
```

## Language Codes

Use ISO 639-1 language codes:

| Code | Language |
|------|----------|
| `EN` | English |
| `DE` | German |
| `FR` | French |
| `ES` | Spanish |
| `IT` | Italian |
| `JA` | Japanese |
| `ZH` | Chinese |
| `RU` | Russian |
| `PT` | Portuguese |
| `NL` | Dutch |

### DeepL-Specific Codes

DeepL uses some variant codes:
- `EN-US` - American English
- `EN-GB` - British English
- `PT-BR` - Brazilian Portuguese
- `PT-PT` - European Portuguese

## Output Examples

### MediaWiki with Translation

```bash
url2ref-cli -u "https://www.lemonde.fr/technologie/article" --target-lang EN
```

Output:
```
{{cite web
  | title = L'intelligence artificielle transforme l'industrie
  | trans-title = Artificial intelligence transforms industry
  | date = 2024-01-15
  | website = Le Monde
  | url = https://www.lemonde.fr/technologie/article
  | language = fr
}}
```

### BibTeX with Translation

```bash
url2ref-cli -u "https://www.spiegel.de/article" -f bibtex --target-lang EN
```

Output:
```bibtex
@misc{spiegel2024,
  title = {Die Zukunft der Technologie},
  note = {English translation: The Future of Technology},
  year = {2024},
  url = {https://www.spiegel.de/article},
  language = {de}
}
```

## Automatic Language Detection

If you omit `--source-lang`, the translation service will attempt to detect the source language automatically:

```bash
# Auto-detect source language
url2ref-cli -u "https://www.asahi.com/article" --target-lang EN
```

This works well for most languages but you can specify the source for better accuracy:

```bash
# Explicit source language
url2ref-cli -u "https://www.asahi.com/article" --source-lang JA --target-lang EN
```

## When to Use Translation

### Wikipedia Citations

Wikipedia guidelines often require translated titles for non-English sources:

```
{{cite web
  | title = Original Title in Foreign Language
  | trans-title = English Translation of Title
  | language = xx
}}
```

### Academic Papers

When citing non-English papers, include both original and translated titles.

## Cost Considerations

### DeepL

- Free tier: 500,000 characters/month
- Pay-as-you-go above the limit
- Generally more accurate for European languages

### Google Cloud Translation

- $20 per million characters
- First 500,000 characters free per month
- Better support for Asian languages

## Troubleshooting

### "DeepL translation failed"

1. Check your API key is set correctly:
   ```bash
   echo $DEEPL_API_KEY
   ```

2. Verify your API key is valid:
   ```bash
   curl -H "Authorization: DeepL-Auth-Key $DEEPL_API_KEY" \
        https://api-free.deepl.com/v2/usage
   ```

3. Check if you've exceeded your quota

### "Translation provider not available"

Ensure the appropriate environment variable is set for your chosen provider.

### Wrong translation

Try specifying the source language explicitly:
```bash
url2ref-cli -u "URL" --source-lang XX --target-lang EN
```

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
