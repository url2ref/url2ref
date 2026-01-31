# Configuration

## Config File

`url2ref-web/config.toml`:

```toml
[default]
address = "127.0.0.1"
port = 8000

[release]
address = "0.0.0.0"
log_level = "critical"
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ROCKET_ADDRESS` | Bind address | `127.0.0.1` |
| `ROCKET_PORT` | Port | `8000` |
| `ROCKET_LOG_LEVEL` | `off`, `critical`, `normal`, `debug` | `normal` |
| `DEEPL_API_KEY` | DeepL translation | — |
| `GOOGLE_TRANSLATE_API_KEY` | Google translation | — |

```bash
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT="3000"
cargo run --bin url2ref-web
```

## TLS

```toml
[default.tls]
certs = "/path/to/cert.pem"
key = "/path/to/key.pem"
```

Or use a reverse proxy (nginx, Caddy) in production.

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
