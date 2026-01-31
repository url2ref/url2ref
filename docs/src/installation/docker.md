# Docker

## Quick Start

```bash
git clone https://github.com/url2ref/url2ref.git
cd url2ref
docker-compose up
```

Web interface at `http://localhost:8000`.

## Manual Build

```bash
docker build -t url2ref .
docker run -p 8000:8000 url2ref
```

## CLI via Docker

```bash
docker run --rm url2ref url2ref-cli --url "https://example.com"
docker run --rm url2ref url2ref-cli --url "https://example.com" --format bibtex
```

## Environment Variables

```bash
docker run -p 8000:8000 \
  -e DEEPL_API_KEY="your-key" \
  -e ROCKET_ADDRESS="0.0.0.0" \
  url2ref
```

## Docker Compose with API Keys

```yaml
# docker-compose.override.yml
services:
  url2ref-web:
    environment:
      - DEEPL_API_KEY=your-key
      - OPENAI_API_KEY=your-key
```
