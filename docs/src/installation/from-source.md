# Building from Source

This guide walks you through building url2ref from source code.

## Clone the Repository

```bash
git clone https://github.com/url2ref/url2ref.git
cd url2ref
```

## Project Structure

The repository is organized as a Cargo workspace with multiple crates:

```
url2ref/
├── url2ref/          # Core library
├── url2ref-cli/      # Command-line interface
├── url2ref-web/      # Web interface
├── Cargo.toml        # Workspace configuration
└── docs/             # This documentation
```

## Building the CLI

The CLI has minimal dependencies and is the easiest to build:

```bash
# Debug build (faster compilation, slower execution)
cargo build --bin url2ref-cli

# Release build (slower compilation, optimized execution)
cargo build --release --bin url2ref-cli
```

The binary will be located at:
- Debug: `target/debug/url2ref-cli`
- Release: `target/release/url2ref-cli`

### Installing Globally

To install the CLI to your Cargo bin directory (`~/.cargo/bin/`):

```bash
cargo install --path url2ref-cli
```

## Building the Web Interface

The web interface requires additional setup for frontend assets.

### Step 1: Install Node.js Dependencies

```bash
cd url2ref-web/npm
npm install
```

### Step 2: Build Frontend Assets

Run the build script to compile SCSS and copy JavaScript dependencies:

```bash
./build.sh
```

This script:
- Compiles SCSS to CSS
- Copies Bootstrap and Popper.js files
- Prepares static assets

### Step 3: Build the Web Server

```bash
cd ../..  # Return to project root
cargo build --bin url2ref-web
```

### Step 4: Run the Web Server

```bash
cargo run --bin url2ref-web
```

The server will start at `http://localhost:8000` by default.

## Building the Library Only

If you're integrating url2ref into another Rust project:

```bash
cargo build -p url2ref
```

Or add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
url2ref = { path = "path/to/url2ref/url2ref" }
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p url2ref

# Run tests with output
cargo test -- --nocapture
```

## Generating API Documentation

Generate and view the Rust API documentation:

```bash
# Generate docs
cargo doc --no-deps

# Generate and open in browser
cargo doc --no-deps --open
```

## Build Options

### Feature Flags

The library supports optional features:

```bash
# Build with all features
cargo build --all-features

# Build with specific features
cargo build --features "feature-name"
```

### Cross-Compilation

To build for a different target:

```bash
# Add target
rustup target add x86_64-unknown-linux-musl

# Build for target
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
