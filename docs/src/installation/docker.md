# Using Docker

Docker provides an easy way to run url2ref without installing Rust or managing dependencies.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed on your system
- [Docker Compose](https://docs.docker.com/compose/install/) (optional, for easier management)

## Quick Start with Docker Compose

The easiest way to run the web interface:

```bash
# Clone the repository
git clone https://github.com/url2ref/url2ref.git
cd url2ref

# Start the web interface
docker-compose up
```

The web interface will be available at `http://localhost:8000`.

## Using the Dockerfile Directly

### Build the Image

```bash
docker build -t url2ref .
```

### Run the Web Interface

```bash
docker run -p 8000:8000 url2ref
```

### Run the CLI

```bash
# Generate a citation
docker run --rm url2ref url2ref-cli --url "https://example.com/article"

# With a specific format
docker run --rm url2ref url2ref-cli --url "https://example.com" --format bibtex
```

## Docker Compose Configuration

The included `docker-compose.yml` provides a ready-to-use configuration:

```yaml
version: '3.8'

services:
  url2ref-web:
    build: .
    ports:
      - "8000:8000"
    environment:
      - ROCKET_ADDRESS=0.0.0.0
      - ROCKET_PORT=8000
    restart: unless-stopped
```

### Customizing the Configuration

Create a `docker-compose.override.yml` for local customizations:

```yaml
version: '3.8'

services:
  url2ref-web:
    environment:
      # Add translation API keys
      - DEEPL_API_KEY=your-deepl-key
      - GOOGLE_TRANSLATE_API_KEY=your-google-key
      # Add AI extraction keys
      - OPENAI_API_KEY=your-openai-key
      - ANTHROPIC_API_KEY=your-anthropic-key
    ports:
      - "3000:8000"  # Use a different port
```

## Environment Variables

Pass environment variables for additional features:

```bash
docker run -p 8000:8000 \
  -e DEEPL_API_KEY=your-key \
  -e OPENAI_API_KEY=your-key \
  url2ref
```

See [Environment Variables](../reference/environment-variables.md) for a complete list.

## Volume Mounts

### Custom Configuration

Mount a custom configuration file:

```bash
docker run -p 8000:8000 \
  -v $(pwd)/my-config.toml:/app/url2ref-web/config.toml:ro \
  url2ref
```

### Persistent Data

If you need to persist any data:

```bash
docker run -p 8000:8000 \
  -v url2ref-data:/app/data \
  url2ref
```

## Building a Minimal Image

For production deployments, you might want a smaller image. The Dockerfile uses multi-stage builds to minimize the final image size.

To build an even smaller image using Alpine:

```dockerfile
# Example custom Dockerfile
FROM rust:alpine AS builder
# ... build steps ...

FROM alpine:latest
COPY --from=builder /app/target/release/url2ref-web /usr/local/bin/
CMD ["url2ref-web"]
```

## Health Checks

Add a health check to your Docker Compose configuration:

```yaml
services:
  url2ref-web:
    # ... other config ...
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
```

## Running Behind a Reverse Proxy

When running behind nginx or another reverse proxy:

```yaml
services:
  url2ref-web:
    environment:
      - ROCKET_ADDRESS=0.0.0.0
    expose:
      - "8000"
    # Don't publish ports directly
```

Example nginx configuration:

```nginx
location /url2ref/ {
    proxy_pass http://url2ref-web:8000/;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}
```

## Troubleshooting

### Container won't start

Check the logs:
```bash
docker-compose logs url2ref-web
```

### Port already in use

Change the port mapping:
```bash
docker run -p 3000:8000 url2ref
```

### Permission denied

On Linux, you might need to run with your user:
```bash
docker run --user $(id -u):$(id -g) -p 8000:8000 url2ref
```

## Next Steps

- [Web Interface Features](../web/features.md) - Explore all web interface capabilities
- [Configuration](../web/configuration.md) - Customize your deployment
- [Environment Variables](../reference/environment-variables.md) - All configuration options
