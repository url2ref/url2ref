# Installation

## Prerequisites

- **Rust** 1.70+ ([rustup.rs](https://rustup.rs/))
- **libcurl** and **OpenSSL** dev headers (Linux)
- **Node.js** 16+ (for web interface)

## Quick Start

```bash
git clone https://github.com/url2ref/url2ref.git
cd url2ref

# CLI
cargo run --bin url2ref-cli -- --url "https://example.com"

# Web interface
cd url2ref-web/npm && ./build.sh && cd ../..
cargo run --bin url2ref-web
```

## System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt install build-essential libcurl4-openssl-dev libssl-dev pkg-config
```

**Fedora/RHEL:**
```bash
sudo dnf install gcc libcurl-devel openssl-devel
```

**macOS:**
```bash
xcode-select --install
```

## Next

- [From Source](./installation/from-source.md) - Detailed build instructions
- [Docker](./installation/docker.md) - Container deployment
