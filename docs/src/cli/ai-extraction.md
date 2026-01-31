# AI Extraction

Use AI to extract metadata when traditional parsing fails.

## Usage

```bash
url2ref-cli -u "https://example.com" --ai-extraction

# Specify provider and model
url2ref-cli -u "https://example.com" \
  --ai-extraction \
  --ai-provider openai \
  --ai-model gpt-4o-mini
```

## API Keys

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

## Providers & Models

| Provider | Models |
|----------|--------|
| OpenAI | `gpt-4o`, `gpt-4o-mini` |
| Anthropic | `claude-3-haiku-20240307`, `claude-3-sonnet-20240229` |

**Recommended:** `gpt-4o-mini` or `claude-3-haiku` for cost-effective extraction.

## When to Use

- Pages without metadata markup
- Old websites or blogs
- When traditional extraction returns incomplete data

## Combined with Translation

```bash
url2ref-cli -u "https://foreign-site.com/article" \
  --ai-extraction \
  --target-lang EN
```

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
