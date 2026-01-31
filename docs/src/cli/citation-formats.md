# Citation Formats

url2ref supports three citation formats, each suited for different use cases.

## MediaWiki (`wiki`)

The default format, perfect for Wikipedia and other MediaWiki-based wikis.

### Usage

```bash
url2ref-cli -u "https://example.com" -f wiki
```

### Output Example

```
{{cite web
  | title = Example Article: A Comprehensive Guide
  | author = Jane Smith
  | date = 2024-01-15
  | website = Example News
  | publisher = Example Media Inc.
  | url = https://example.com/article
  | access-date = 2024-01-20
  | language = en
}}
```

### Supported Fields

| Field | Description |
|-------|-------------|
| `title` | Article title |
| `trans-title` | Translated title (if translation enabled) |
| `author` | Author name(s) |
| `date` | Publication date |
| `website` | Website name |
| `publisher` | Publisher organization |
| `url` | Original URL |
| `archive-url` | Wayback Machine URL |
| `archive-date` | Archive date |
| `access-date` | Date the URL was accessed |
| `language` | Content language |

### Use Cases

- Wikipedia article editing
- MediaWiki-based documentation
- Any wiki using the `{{cite web}}` template

---

## BibTeX (`bibtex`)

Standard format for LaTeX documents and academic papers.

### Usage

```bash
url2ref-cli -u "https://example.com" -f bibtex
```

### Output Example

```bibtex
@misc{example2024,
  author = {Jane Smith},
  title = {Example Article: A Comprehensive Guide},
  year = {2024},
  month = {jan},
  url = {https://example.com/article},
  urldate = {2024-01-20},
  note = {Accessed: 2024-01-20}
}
```

### Entry Types

The tool automatically selects the appropriate entry type:

| Content Type | BibTeX Entry |
|--------------|--------------|
| News article | `@misc` |
| Scholarly article | `@article` |
| Generic web page | `@misc` |

### Supported Fields

| Field | Description |
|-------|-------------|
| `author` | Author name(s) |
| `title` | Article title |
| `year` | Publication year |
| `month` | Publication month |
| `url` | Original URL |
| `urldate` | Access date |
| `journal` | Journal name (for articles) |
| `publisher` | Publisher |
| `note` | Additional notes |

### Use Cases

- LaTeX documents
- Academic papers
- Reference management software (Zotero, Mendeley, etc.)
- BibTeX-compatible systems

### Tips for BibTeX

1. **Citation Key**: The tool generates a key from the domain and year (e.g., `example2024`). You may want to customize this.

2. **Special Characters**: The tool escapes LaTeX special characters automatically.

3. **Importing**: Most reference managers can import the BibTeX output directly.

---

## Harvard (`harvard`)

In-text referencing style commonly used in academia.

### Usage

```bash
url2ref-cli -u "https://example.com" -f harvard
```

### Output Example

```
Smith, J. (2024) 'Example Article: A Comprehensive Guide', Example News. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

### Format Structure

```
Author (Year) 'Title', Source. Available at: URL (Accessed: Date).
```

### Variations

The exact format adapts based on available information:

**With author:**
```
Smith, J. (2024) 'Title', Website. Available at: URL (Accessed: Date).
```

**Without author:**
```
'Title' (2024) Website. Available at: URL (Accessed: Date).
```

**Without date:**
```
Smith, J. (n.d.) 'Title', Website. Available at: URL (Accessed: Date).
```

### Use Cases

- Academic essays
- University assignments
- Business reports
- Any context requiring Harvard referencing

---

## Comparing Formats

### Same Source, Different Formats

**Input URL:** `https://www.bbc.com/news/technology-12345`

**MediaWiki:**
```
{{cite web
  | title = AI Revolution: What's Next?
  | author = Tech Reporter
  | date = 2024-01-15
  | website = BBC News
  | url = https://www.bbc.com/news/technology-12345
  | access-date = 2024-01-20
}}
```

**BibTeX:**
```bibtex
@misc{bbc2024,
  author = {Tech Reporter},
  title = {AI Revolution: What's Next?},
  year = {2024},
  month = {jan},
  url = {https://www.bbc.com/news/technology-12345},
  urldate = {2024-01-20}
}
```

**Harvard:**
```
Tech Reporter (2024) 'AI Revolution: What's Next?', BBC News. 
Available at: https://www.bbc.com/news/technology-12345 (Accessed: 20 January 2024).
```

## Choosing the Right Format

| Situation | Recommended Format |
|-----------|-------------------|
| Editing Wikipedia | `wiki` |
| Writing LaTeX papers | `bibtex` |
| University essays | `harvard` |
| Quick reference | `wiki` (most readable) |
| Reference manager import | `bibtex` |

## Next Steps

- [Metadata Priority](./metadata-priority.md) - Control data source selection
- [Translation](./translation.md) - Add translated titles
- [AI Extraction](./ai-extraction.md) - Fill in missing fields
