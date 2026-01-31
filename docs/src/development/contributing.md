# Contributing

Thank you for your interest in contributing to url2ref!

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- Node.js and npm (for web interface development)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/url2ref.git
   cd url2ref
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/url2ref/url2ref.git
   ```

### Build the Project

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build web interface
cd url2ref-web/npm && npm install && ./build.sh && cd ../..
cargo build --bin url2ref-web
```

## Development Workflow

### Create a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### Make Changes

1. Write your code
2. Add tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Format code: `cargo fmt`
5. Check lints: `cargo clippy`

### Commit Guidelines

Use clear, descriptive commit messages:

```
type(scope): brief description

Longer description if needed.

Fixes #123
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `refactor` - Code refactoring
- `test` - Adding tests
- `chore` - Maintenance

**Examples:**
```
feat(parser): add support for Dublin Core metadata
fix(cli): handle URLs with special characters
docs(readme): update installation instructions
```

### Submit a Pull Request

1. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a Pull Request on GitHub

3. Fill out the PR template:
   - Describe your changes
   - Link related issues
   - Add screenshots if applicable

4. Wait for review and address feedback

## Project Structure

```
url2ref/
├── url2ref/           # Core library
│   ├── src/
│   │   ├── lib.rs           # Public API
│   │   ├── generator.rs     # Reference generation
│   │   ├── parser.rs        # Metadata parsing
│   │   ├── opengraph.rs     # Open Graph parser
│   │   ├── schema_org.rs    # Schema.org parser
│   │   ├── html_meta.rs     # HTML meta parser
│   │   ├── doi.rs           # DOI resolution
│   │   ├── zotero.rs        # Zotero/Citoid
│   │   ├── ai_extractor.rs  # AI extraction
│   │   ├── attribute.rs     # Attribute types
│   │   ├── reference.rs     # Reference types
│   │   └── citation.rs      # Citation formatters
│   └── tests/
├── url2ref-cli/       # CLI application
├── url2ref-web/       # Web interface
│   ├── src/
│   ├── templates/
│   ├── static/
│   └── npm/
└── docs/              # mdBook documentation
```

## Code Guidelines

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Document public APIs with doc comments

### Documentation

- Add `///` doc comments to public items
- Include examples in doc comments
- Update mdBook docs for user-facing changes

### Testing

- Write unit tests for new functions
- Add integration tests for major features
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "...";
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

## Adding a New Metadata Source

1. Create parser module in `url2ref/src/`
2. Implement the `Parser` trait
3. Add to `MetadataType` enum in `generator.rs`
4. Update attribute config options
5. Add tests
6. Document in mdBook

## Adding a New Citation Format

1. Add formatter in `url2ref/src/citation.rs`
2. Implement `CitationBuilder` trait
3. Add method to `Reference` enum
4. Update CLI arguments
5. Update web interface
6. Add tests and documentation

## Reporting Bugs

### Before Reporting

1. Search existing issues
2. Check if it's already fixed in `main`
3. Gather relevant information

### Bug Report Template

```markdown
## Description
Brief description of the bug.

## Steps to Reproduce
1. Step one
2. Step two
3. ...

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: 
- Rust version: 
- url2ref version:

## Additional Context
Screenshots, logs, example URLs, etc.
```

## Requesting Features

### Feature Request Template

```markdown
## Problem
Describe the problem or use case.

## Proposed Solution
Your idea for solving it.

## Alternatives Considered
Other approaches you've thought about.

## Additional Context
Any other relevant information.
```

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open an Issue
- **Chat**: [Link to Discord/Matrix if available]

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

## Thank You!

Every contribution helps make url2ref better. We appreciate your time and effort!
