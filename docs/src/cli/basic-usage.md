# Basic Usage

## Generate a Citation

```bash
url2ref-cli --url "https://www.example.com/article"
# or
url2ref-cli -u "https://www.example.com/article"
```

## Choose Format

```bash
url2ref-cli -u "https://example.com" -f wiki    # MediaWiki (default)
url2ref-cli -u "https://example.com" -f bibtex  # BibTeX
url2ref-cli -u "https://example.com" -f harvard # Harvard
```

## Copy to Clipboard

**Linux:**
```bash
url2ref-cli -u "https://example.com" | xclip -selection clipboard
```

**macOS:**
```bash
url2ref-cli -u "https://example.com" | pbcopy
```

## Shell Alias

```bash
# ~/.bashrc or ~/.zshrc
alias cite='url2ref-cli -f wiki -u'
```

Usage:
```bash
cite "https://example.com"
```
