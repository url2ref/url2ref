# Building from Source

## Clone

```bash
git clone https://github.com/url2ref/url2ref.git
cd url2ref
```

## Build CLI

```bash
cargo build --release --bin url2ref-cli
```

Binary: `target/release/url2ref-cli`

**Install globally:**
```bash
cargo install --path url2ref-cli
```

## Build Web Interface

```bash
cd url2ref-web/npm
npm install
./build.sh
cd ../..
cargo build --release --bin url2ref-web
```

**Run:**
```bash
cargo run --bin url2ref-web
# Server at http://localhost:8000
```

## Build Library Only

```bash
cargo build -p url2ref
```

Or as a dependency:
```toml
[dependencies]
url2ref = { git = "https://github.com/url2ref/url2ref" }
```

## Tests & Docs

```bash
cargo test
cargo doc --no-deps --open
cargo build --target x86_64-unknown-linux-musl --release
```

## Troubleshooting

### libcurl not found

**Linux:**
```bash
sudo apt install libcurl4-openssl-dev  # Debian/Ubuntu
sudo dnf install libcurl-devel          # Fedora
```

**macOS:**
```bash
brew install curl
export PKG_CONFIG_PATH="/usr/local/opt/curl/lib/pkgconfig"
```

### OpenSSL errors

**Linux:**
```bash
sudo apt install libssl-dev pkg-config  # Debian/Ubuntu
```

**macOS:**
```bash
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

### SCSS compilation fails

Ensure you have the latest npm packages:

```bash
cd url2ref-web/npm
rm -rf node_modules
npm install
./build.sh
```

## Next Steps

- [CLI Usage](../cli/index.md) - Learn to use the command-line tool
- [Web Interface](../web/index.md) - Set up the web interface
- [API Documentation](../reference/api.md) - Library API reference
