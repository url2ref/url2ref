# Web Interface Configuration

Configure the url2ref web server for your deployment needs.

## Configuration File

The web server reads configuration from `config.toml` in the `url2ref-web` directory.

### Default Configuration

```toml
[default]
address = "127.0.0.1"
port = 8000
```

### Full Configuration Example

```toml
[default]
# Server binding
address = "127.0.0.1"
port = 8000

# Logging
log_level = "normal"

# Static file caching (seconds)
# static_cache_max_age = 86400

[release]
# Production overrides
address = "0.0.0.0"
port = 8000
log_level = "critical"

[debug]
# Development overrides
log_level = "debug"
```

## Environment Variables

Configure the server using environment variables:

### Server Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `ROCKET_ADDRESS` | Bind address | `127.0.0.1` |
| `ROCKET_PORT` | Listen port | `8000` |
| `ROCKET_LOG_LEVEL` | Log verbosity | `normal` |

### API Keys

| Variable | Description | Required For |
|----------|-------------|--------------|
| `DEEPL_API_KEY` | DeepL API key | Translation (DeepL) |
| `GOOGLE_TRANSLATE_API_KEY` | Google API key | Translation (Google) |

### Example

```bash
# Set configuration
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT="3000"
export DEEPL_API_KEY="your-key-here"

# Start server
cargo run --bin url2ref-web
```

## Rocket Configuration

The web server uses [Rocket](https://rocket.rs/). Full configuration options are available in the [Rocket documentation](https://rocket.rs/guide/v0.5/configuration/).

### Profiles

Rocket supports configuration profiles:

- `debug` - Development mode (default with `cargo run`)
- `release` - Production mode (default with `--release`)

Set the profile:

```bash
ROCKET_PROFILE=release cargo run --bin url2ref-web
```

### Common Options

```toml
[default]
# Bind to all interfaces (for Docker/containers)
address = "0.0.0.0"

# Custom port
port = 3000

# Worker threads (default: CPU count)
workers = 16

# Keep-alive timeout (seconds)
keep_alive = 5

# Request size limits
limits = { forms = "64 kB", json = "1 MiB" }

# Log level: "off", "critical", "normal", "debug"
log_level = "normal"
```

## TLS/HTTPS Configuration

For production, configure TLS:

```toml
[default.tls]
certs = "/path/to/cert.pem"
key = "/path/to/key.pem"
```

Or use a reverse proxy (recommended):

```nginx
# nginx configuration
server {
    listen 443 ssl;
    server_name url2ref.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Static File Configuration

Static files are served from `url2ref-web/static/`.

### Directory Structure

```
static/
├── css/           # Compiled stylesheets
├── js/            # JavaScript files
├── icons/         # Favicon and icons
├── popper/        # Popper.js library
└── custom/        # Custom assets
```

### Caching

Configure static file caching for production:

```toml
[release]
# Cache static files for 24 hours
static_cache_max_age = 86400
```

## Template Configuration

Templates use [Tera](https://tera.netlify.app/) and are located in `url2ref-web/templates/`.

### Template Files

| File | Purpose |
|------|---------|
| `layout.html.tera` | Base layout |
| `home.html.tera` | Main interface |

### Customizing Templates

1. Edit templates in `templates/`
2. Restart the server to see changes
3. In debug mode, templates auto-reload

## CORS Configuration

If accessing the API from a different domain, configure CORS:

```rust
// In main.rs (requires code modification)
use rocket_cors::{AllowedOrigins, CorsOptions};

let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .to_cors()?;
```

## Rate Limiting

For production deployments, consider adding rate limiting via a reverse proxy:

```nginx
# nginx rate limiting
limit_req_zone $binary_remote_addr zone=url2ref:10m rate=10r/s;

server {
    location /api/ {
        limit_req zone=url2ref burst=20 nodelay;
        proxy_pass http://127.0.0.1:8000;
    }
}
```

## Docker Configuration

When running in Docker, use environment variables:

```yaml
# docker-compose.yml
services:
  url2ref-web:
    build: .
    ports:
      - "8000:8000"
    environment:
      - ROCKET_ADDRESS=0.0.0.0
      - ROCKET_PORT=8000
      - ROCKET_LOG_LEVEL=normal
      - DEEPL_API_KEY=${DEEPL_API_KEY}
```

## Health Checks

The server responds to requests at the root path. For health checks:

```bash
curl -f http://localhost:8000/ || exit 1
```

Or add a dedicated health endpoint (requires code modification).

## Monitoring

### Logging

Logs are written to stderr. Redirect for persistence:

```bash
cargo run --bin url2ref-web 2>&1 | tee -a url2ref.log
```

### Log Levels

| Level | Description |
|-------|-------------|
| `off` | No logging |
| `critical` | Only critical errors |
| `normal` | Standard logging |
| `debug` | Verbose debugging |

## Production Checklist

Before deploying to production:

- [ ] Set `ROCKET_PROFILE=release`
- [ ] Bind to `0.0.0.0` if using containers
- [ ] Configure TLS (directly or via reverse proxy)
- [ ] Set appropriate log level
- [ ] Configure API keys securely (not in code)
- [ ] Set up rate limiting
- [ ] Enable static file caching
- [ ] Configure health checks
- [ ] Set up log rotation

## Next Steps

- [Docker Deployment](../installation/docker.md) - Container setup
- [Environment Variables](../reference/environment-variables.md) - Complete reference
- [CLI Alternative](../cli/index.md) - Command-line interface
