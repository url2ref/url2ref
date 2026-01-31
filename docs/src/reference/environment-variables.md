# Environment Variables

Complete reference for all environment variables used by url2ref.

## Translation Services

### DeepL

| Variable | Description | Example |
|----------|-------------|---------|
| `DEEPL_API_KEY` | DeepL API authentication key | `a1b2c3d4-e5f6-...` |

**Getting a key:**
1. Sign up at [deepl.com/pro-api](https://www.deepl.com/pro-api)
2. Free tier includes 500,000 characters/month
3. Find your key in Account → API Keys

**Usage:**
```bash
export DEEPL_API_KEY="your-api-key-here"
```

### Google Cloud Translation

| Variable | Description | Example |
|----------|-------------|---------|
| `GOOGLE_TRANSLATE_API_KEY` | Google Cloud API key | `AIza...` |

**Getting a key:**
1. Create project at [console.cloud.google.com](https://console.cloud.google.com)
2. Enable Cloud Translation API
3. Create API key in Credentials

**Usage:**
```bash
export GOOGLE_TRANSLATE_API_KEY="your-api-key-here"
```

---

## AI Extraction

### OpenAI

| Variable | Description | Example |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | `sk-...` |

**Getting a key:**
1. Sign up at [platform.openai.com](https://platform.openai.com)
2. Go to API Keys section
3. Create new secret key

**Usage:**
```bash
export OPENAI_API_KEY="sk-your-api-key-here"
```

### Anthropic

| Variable | Description | Example |
|----------|-------------|---------|
| `ANTHROPIC_API_KEY` | Anthropic API key | `sk-ant-...` |

**Getting a key:**
1. Sign up at [console.anthropic.com](https://console.anthropic.com)
2. Go to API Keys
3. Create new key

**Usage:**
```bash
export ANTHROPIC_API_KEY="sk-ant-your-api-key-here"
```

---

## Web Server (Rocket)

### Server Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `ROCKET_ADDRESS` | IP address to bind | `127.0.0.1` |
| `ROCKET_PORT` | Port to listen on | `8000` |
| `ROCKET_WORKERS` | Number of worker threads | CPU count |
| `ROCKET_LOG_LEVEL` | Logging verbosity | `normal` |
| `ROCKET_KEEP_ALIVE` | Keep-alive timeout (seconds) | `5` |
| `ROCKET_PROFILE` | Configuration profile | `debug` or `release` |

**Log Levels:**
- `off` - No logging
- `critical` - Only critical errors
- `normal` - Standard logging
- `debug` - Verbose debugging

**Example:**
```bash
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT="3000"
export ROCKET_LOG_LEVEL="normal"
export ROCKET_PROFILE="release"
```

### TLS Configuration

| Variable | Description |
|----------|-------------|
| `ROCKET_TLS_CERTS` | Path to TLS certificate |
| `ROCKET_TLS_KEY` | Path to TLS private key |

**Example:**
```bash
export ROCKET_TLS_CERTS="/path/to/cert.pem"
export ROCKET_TLS_KEY="/path/to/key.pem"
```

---

## Setting Variables

### Linux/macOS

**Temporary (current session):**
```bash
export DEEPL_API_KEY="your-key"
```

**Permanent (add to shell profile):**
```bash
# ~/.bashrc or ~/.zshrc
export DEEPL_API_KEY="your-key"
export OPENAI_API_KEY="your-key"
```

**Load changes:**
```bash
source ~/.bashrc  # or ~/.zshrc
```

### Windows

**Command Prompt (temporary):**
```cmd
set DEEPL_API_KEY=your-key
```

**PowerShell (temporary):**
```powershell
$env:DEEPL_API_KEY = "your-key"
```

**Permanent:**
```powershell
[Environment]::SetEnvironmentVariable("DEEPL_API_KEY", "your-key", "User")
```

Or use System Properties → Environment Variables.

### Docker

**docker run:**
```bash
docker run -e DEEPL_API_KEY="your-key" url2ref
```

**docker-compose.yml:**
```yaml
services:
  url2ref:
    environment:
      - DEEPL_API_KEY=your-key
      - ROCKET_ADDRESS=0.0.0.0
```

**Using .env file:**
```bash
# .env
DEEPL_API_KEY=your-key
OPENAI_API_KEY=your-key
```

```yaml
# docker-compose.yml
services:
  url2ref:
    env_file:
      - .env
```

---

## Security Best Practices

### Do

✅ Use environment variables (not hardcoded keys)
✅ Keep `.env` files out of version control
✅ Use different keys for development and production
✅ Rotate keys periodically
✅ Use secrets managers in production (Vault, AWS Secrets Manager)

### Don't

❌ Commit API keys to git
❌ Share keys across environments
❌ Log or print API keys
❌ Use production keys in development

### .gitignore

```gitignore
# Environment files
.env
.env.local
.env.production
*.env

# Config files with secrets
config.local.toml
```

---

## Checking Configuration

### Verify Variables Are Set

```bash
# Check if variable exists
echo $DEEPL_API_KEY

# Check all url2ref-related variables
env | grep -E "(DEEPL|GOOGLE|OPENAI|ANTHROPIC|ROCKET)"
```

### Test API Keys

**DeepL:**
```bash
curl -H "Authorization: DeepL-Auth-Key $DEEPL_API_KEY" \
     "https://api-free.deepl.com/v2/usage"
```

**OpenAI:**
```bash
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
     "https://api.openai.com/v1/models" | head
```

---

## Quick Reference

### Minimal Setup (CLI only)

```bash
# No environment variables required for basic usage
url2ref-cli --url "https://example.com"
```

### With Translation

```bash
export DEEPL_API_KEY="your-key"
url2ref-cli --url "https://example.com" --target-lang EN
```

### With AI Extraction

```bash
export OPENAI_API_KEY="your-key"
url2ref-cli --url "https://example.com" --ai-extraction
```

### Full Setup

```bash
# ~/.bashrc or ~/.zshrc

# Translation
export DEEPL_API_KEY="your-deepl-key"
export GOOGLE_TRANSLATE_API_KEY="your-google-key"

# AI Extraction
export OPENAI_API_KEY="your-openai-key"
export ANTHROPIC_API_KEY="your-anthropic-key"

# Web Server (optional)
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT="8000"
```

## Next Steps

- [Installation](../installation.md) - Setup guide
- [CLI Usage](../cli/index.md) - Command-line interface
- [Web Configuration](../web/configuration.md) - Server setup
