# Installation

url2ref can be installed in several ways depending on your needs.

## Prerequisites

### For CLI and Library Usage

- **Rust** 1.70 or later (install via [rustup](https://rustup.rs/))
- **libcurl** development headers
- **OpenSSL** development headers (on Linux)

### For Web Interface

In addition to the above:
- **Node.js** 16+ and npm (for building frontend assets)

## Quick Start

The fastest way to get started is building from source:

```bash
# Clone the repository
git clone https://github.com/url2ref/url2ref.git
cd url2ref

# Build and run CLI
cargo run --bin url2ref-cli -- --url "https://example.com"

# Or build and run web interface
cd url2ref-web/npm && ./build.sh && cd ../..
cargo run --bin url2ref-web
```

## Installation Methods

Choose the installation method that best fits your needs:

| Method | Best For |
|--------|----------|
| [From Source](./installation/from-source.md) | Development, customization |
| [Using Docker](./installation/docker.md) | Quick deployment, isolation |

## Platform-Specific Notes

### Linux (Ubuntu/Debian)

Install required system dependencies:

```bash
sudo apt update
sudo apt install build-essential libcurl4-openssl-dev libssl-dev pkg-config
```

### Linux (Fedora/RHEL)

```bash
sudo dnf install gcc libcurl-devel openssl-devel
```

### macOS

Dependencies are typically available via Xcode Command Line Tools:

```bash
xcode-select --install
```

If you need curl with specific features:

```bash
brew install curl openssl
```

### Windows

We recommend using [Windows Subsystem for Linux (WSL)](https://docs.microsoft.com/en-us/windows/wsl/) for the best experience. Alternatively, ensure you have the Visual Studio Build Tools installed.

## Verifying Installation

After installation, verify everything works:

```bash
# Check CLI
url2ref-cli --help

# Or if running from source
cargo run --bin url2ref-cli -- --help
```

You should see the help output listing available options.

## Next Steps

- [Building from Source](./installation/from-source.md) - Detailed build instructions
- [Docker Setup](./installation/docker.md) - Container-based deployment
- [CLI Usage](./cli/index.md) - Start using the command-line tool
- [Web Interface](./web/index.md) - Set up the browser-based interface
