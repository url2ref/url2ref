# Environment Variables

## Translation

| Variable | Description |
|----------|-------------|
| `DEEPL_API_KEY` | [DeepL API](https://www.deepl.com/pro-api) key |
| `GOOGLE_TRANSLATE_API_KEY` | [Google Cloud Translation](https://console.cloud.google.com) API key |

## AI Extraction

| Variable | Description |
|----------|-------------|
| `OPENAI_API_KEY` | [OpenAI](https://platform.openai.com) API key |
| `ANTHROPIC_API_KEY` | [Anthropic](https://console.anthropic.com) API key |

## Web Server

| Variable | Description | Default |
|----------|-------------|---------|
| `ROCKET_ADDRESS` | IP address to bind | `127.0.0.1` |
| `ROCKET_PORT` | Port to listen on | `8000` |
| `ROCKET_WORKERS` | Worker threads | CPU count |
| `ROCKET_LOG_LEVEL` | `off`, `critical`, `normal`, `debug` | `normal` |
| `ROCKET_TLS_CERTS` | Path to TLS certificate | — |
| `ROCKET_TLS_KEY` | Path to TLS private key | — |

## Examples

**With translation:**
```bash
export DEEPL_API_KEY="your-key"
url2ref-cli --url "https://example.com" --target-lang EN
```

**With AI extraction:**
```bash
export OPENAI_API_KEY="your-key"
url2ref-cli --url "https://example.com" --ai-extraction
```

**Web server on all interfaces:**
```bash
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT="3000"
```

**Docker:**
```bash
docker run -e DEEPL_API_KEY="your-key" -e ROCKET_ADDRESS="0.0.0.0" url2ref
```
