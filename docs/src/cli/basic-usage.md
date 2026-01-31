# Basic CLI Usage

This guide covers everyday usage patterns for the url2ref command-line interface.

## Your First Citation

The simplest usage requires only a URL:

```bash
url2ref-cli --url "https://www.example.com/article"
```

Or using the short form:

```bash
url2ref-cli -u "https://www.example.com/article"
```

### Output

The tool outputs the formatted citation to stdout:

```
{{cite web
  | title = Example Article Title
  | author = John Doe
  | date = 2024-01-15
  | website = Example.com
  | url = https://www.example.com/article
  | access-date = 2024-01-20
}}
```

## Choosing a Citation Format

Use the `--format` (or `-f`) flag to specify your desired output format:

```bash
# MediaWiki (default)
url2ref-cli -u "https://example.com" -f wiki

# BibTeX
url2ref-cli -u "https://example.com" -f bibtex

# Harvard
url2ref-cli -u "https://example.com" -f harvard
```

## Working with Different Content Types

### News Articles

News sites typically have rich Open Graph metadata:

```bash
url2ref-cli -u "https://www.nytimes.com/2024/01/15/technology/ai-news.html"
```

### Academic Papers

For DOI-based content, the tool automatically extracts scholarly metadata:

```bash
url2ref-cli -u "https://doi.org/10.1000/example" -f bibtex
```

### Blog Posts

```bash
url2ref-cli -u "https://blog.example.com/post/my-article"
```

### Wikipedia Articles

```bash
url2ref-cli -u "https://en.wikipedia.org/wiki/Example"
```

## Saving Output to a File

Redirect output to save your citations:

```bash
# Single citation
url2ref-cli -u "https://example.com" > citation.txt

# Append to existing file
url2ref-cli -u "https://example.com" >> citations.bib
```

## Batch Processing

Process multiple URLs using a loop:

```bash
# From a file with one URL per line
while read url; do
    url2ref-cli -u "$url" -f bibtex >> references.bib
done < urls.txt
```

Using xargs:

```bash
cat urls.txt | xargs -I {} url2ref-cli -u "{}" -f bibtex >> references.bib
```

## Combining with Other Tools

### Clipboard Integration

**Linux (with xclip):**
```bash
url2ref-cli -u "https://example.com" | xclip -selection clipboard
```

**macOS:**
```bash
url2ref-cli -u "https://example.com" | pbcopy
```

**Windows (PowerShell):**
```powershell
url2ref-cli -u "https://example.com" | Set-Clipboard
```

### Piping to Text Editors

```bash
# Open in vim
url2ref-cli -u "https://example.com" | vim -

# Append to file in VS Code
url2ref-cli -u "https://example.com" >> references.bib && code references.bib
```

## Handling Errors

The CLI provides informative error messages:

```bash
# Invalid URL
url2ref-cli -u "not-a-valid-url"
# Error: curl GET failed

# Page without metadata
url2ref-cli -u "https://example.com/empty-page"
# Error: All provided parsers failed
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |

Use exit codes in scripts:

```bash
if url2ref-cli -u "$url" > citation.txt; then
    echo "Citation generated successfully"
else
    echo "Failed to generate citation"
fi
```

## Tips and Tricks

### Alias for Common Formats

Add to your shell configuration (`~/.bashrc` or `~/.zshrc`):

```bash
alias cite='url2ref-cli -f wiki -u'
alias bibtex='url2ref-cli -f bibtex -u'
```

Then use:
```bash
cite "https://example.com"
bibtex "https://arxiv.org/abs/2301.00001"
```

### Function for Quick Citations

```bash
function ref() {
    url2ref-cli -u "$1" -f "${2:-wiki}" | tee /dev/tty | xclip -selection clipboard
    echo "📋 Copied to clipboard!"
}
```

Usage:
```bash
ref "https://example.com"        # Wiki format, copied to clipboard
ref "https://example.com" bibtex # BibTeX format
```

## Next Steps

- [Citation Formats](./citation-formats.md) - Deep dive into output formats
- [Metadata Priority](./metadata-priority.md) - Control which data sources are used
- [Translation](./translation.md) - Translate titles to other languages
