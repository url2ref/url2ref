# AI-Powered Metadata Extraction

url2ref can use AI language models to extract metadata when traditional parsing methods fail or return incomplete results.

## Overview

Some web pages lack proper metadata markup, making it difficult to extract citation information. AI extraction analyzes the page content directly to identify:

- Article title
- Author names
- Publication date
- Publisher/website name

## Supported AI Providers

| Provider | Models | Best For |
|----------|--------|----------|
| **OpenAI** | GPT-4o, GPT-4o-mini | General extraction, accuracy |
| **Anthropic** | Claude 3 Haiku, Claude 3 Sonnet | Nuanced content understanding |

## Basic Usage

Enable AI extraction with the `--ai-extraction` flag:

```bash
url2ref-cli -u "https://example.com/article" --ai-extraction
```

### Specifying Provider and Model

```bash
# Use OpenAI with GPT-4o-mini (default)
url2ref-cli -u "https://example.com" \
  --ai-extraction \
  --ai-provider openai \
  --ai-model gpt-4o-mini

# Use Anthropic with Claude 3 Haiku
url2ref-cli -u "https://example.com" \
  --ai-extraction \
  --ai-provider anthropic \
  --ai-model claude-3-haiku-20240307
```

## API Keys

AI extraction requires API keys set as environment variables.

### OpenAI

1. Get an API key at [platform.openai.com](https://platform.openai.com/api-keys)
2. Set the environment variable:

```bash
export OPENAI_API_KEY="sk-your-api-key-here"
```

### Anthropic

1. Get an API key at [console.anthropic.com](https://console.anthropic.com/)
2. Set the environment variable:

```bash
export ANTHROPIC_API_KEY="sk-ant-your-api-key-here"
```

## How It Works

1. **Traditional Parsing First**: url2ref attempts to extract metadata from Open Graph, Schema.org, and HTML meta tags

2. **Gap Identification**: The system identifies which fields are missing or incomplete

3. **AI Analysis**: If enabled, the AI provider analyzes the page content to fill gaps

4. **Merge Results**: AI-extracted data is merged with traditionally extracted data

## When to Use AI Extraction

### Good Use Cases

- **Older websites** without modern metadata
- **Blog posts** with minimal markup
- **User-generated content** platforms
- **Pages with incorrect metadata**

### When to Avoid

- **DOI-based content** - DOI metadata is authoritative
- **Major news sites** - Usually have excellent metadata
- **High-volume batch processing** - Cost considerations

## Model Selection Guide

### OpenAI Models

| Model | Speed | Cost | Accuracy |
|-------|-------|------|----------|
| `gpt-4o` | Medium | Higher | Highest |
| `gpt-4o-mini` | Fast | Lower | High |
| `gpt-4-turbo` | Medium | Medium | Very High |

### Anthropic Models

| Model | Speed | Cost | Accuracy |
|-------|-------|------|----------|
| `claude-3-opus-20240229` | Slow | Highest | Highest |
| `claude-3-sonnet-20240229` | Medium | Medium | High |
| `claude-3-haiku-20240307` | Fast | Lowest | Good |

### Recommendations

- **Quick extractions**: `gpt-4o-mini` or `claude-3-haiku-20240307`
- **Maximum accuracy**: `gpt-4o` or `claude-3-sonnet-20240229`
- **Cost-sensitive**: `gpt-4o-mini` (best value)

## Examples

### Basic AI Extraction

```bash
url2ref-cli -u "https://obscure-blog.com/post" --ai-extraction
```

### With Specific Model

```bash
url2ref-cli -u "https://example.com" \
  --ai-extraction \
  --ai-provider openai \
  --ai-model gpt-4o
```

### Combined with Translation

```bash
url2ref-cli -u "https://foreign-site.com/article" \
  --ai-extraction \
  --target-lang EN \
  --translation-provider deepl
```

## Output Example

Traditional extraction might return:
```
{{cite web
  | title = 
  | date = 
  | url = https://example.com/article
  | access-date = 2024-01-20
}}
```

With AI extraction:
```
{{cite web
  | title = Understanding Modern Web Development
  | author = Sarah Johnson
  | date = 2024-01-15
  | website = Tech Insights Blog
  | url = https://example.com/article
  | access-date = 2024-01-20
}}
```

## Cost Considerations

### Typical Costs Per Request

| Model | Approximate Cost |
|-------|------------------|
| GPT-4o-mini | ~$0.001 |
| GPT-4o | ~$0.01 |
| Claude 3 Haiku | ~$0.001 |
| Claude 3 Sonnet | ~$0.01 |

### Reducing Costs

1. **Try traditional extraction first** - Only enable AI when needed
2. **Use faster models** - `gpt-4o-mini` and `claude-3-haiku` are very capable
3. **Batch wisely** - Don't enable AI for sites with good metadata

## Troubleshooting

### "AI extraction failed"

1. **Check API key**:
   ```bash
   echo $OPENAI_API_KEY
   echo $ANTHROPIC_API_KEY
   ```

2. **Verify API access**:
   ```bash
   # OpenAI
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer $OPENAI_API_KEY"
   ```

3. **Check rate limits** - You may have exceeded your API quota

### Poor Extraction Quality

- Try a more capable model (`gpt-4o` instead of `gpt-4o-mini`)
- The page content might be JavaScript-rendered (AI can only see the initial HTML)

### Slow Response

- AI extraction adds 1-5 seconds depending on the model
- Use faster models for better performance

## Shell Configuration

```bash
# ~/.bashrc or ~/.zshrc
export OPENAI_API_KEY="your-key"
export ANTHROPIC_API_KEY="your-key"

# Alias for AI-assisted citations
alias cite-ai='url2ref-cli --ai-extraction --ai-provider openai --ai-model gpt-4o-mini'
```

## Security Notes

- API keys are read from environment variables only
- Keys are never logged or stored
- In the web interface, AI keys can be entered per-request (not stored server-side)

## Next Steps

- [Environment Variables](../reference/environment-variables.md) - Complete configuration reference
- [Web Interface](../web/index.md) - Use AI extraction in the browser
- [API Documentation](../reference/api.md) - Programmatic access
