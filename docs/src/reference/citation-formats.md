# Citation Formats Reference

Detailed specification of all citation formats supported by url2ref.

## MediaWiki (`{{cite web}}`)

### Template Structure

```
{{cite web
  | url = 
  | title = 
  | author = 
  | date = 
  | website = 
  | publisher = 
  | language = 
  | access-date = 
  | archive-url = 
  | archive-date = 
}}
```

### Field Mapping

| url2ref Field | Wiki Parameter | Required |
|---------------|----------------|----------|
| URL | `url` | Yes |
| Title | `title` | Yes |
| Translated Title | `trans-title` | No |
| Author | `author` | No |
| Date | `date` | No |
| Website | `website` | No |
| Publisher | `publisher` | No |
| Language | `language` | No |
| Access Date | `access-date` | Yes |
| Archive URL | `archive-url` | No |
| Archive Date | `archive-date` | No |

### Date Formats

| Input | Output |
|-------|--------|
| `2024-01-15` | `2024-01-15` |
| `2024-01-15T10:30:00Z` | `2024-01-15` |
| `January 15, 2024` | `2024-01-15` |

### Multiple Authors

```
{{cite web
  | author1 = First Author
  | author2 = Second Author
  | author3 = Third Author
  ...
}}
```

### Example Output

```
{{cite web
  | url = https://www.example.com/article
  | title = Understanding Modern Web Development
  | author = Jane Smith
  | date = 2024-01-15
  | website = Tech Blog
  | publisher = Tech Media Inc.
  | language = en
  | access-date = 2024-01-20
  | archive-url = https://web.archive.org/web/20240115/https://www.example.com/article
  | archive-date = 2024-01-15
}}
```

### Wikipedia Guidelines

For Wikipedia citations:
- `url` and `title` are required
- `access-date` should always be included
- `archive-url` recommended for link persistence
- Use ISO 8601 dates (`YYYY-MM-DD`)

---

## BibTeX

### Entry Types

| Content Type | BibTeX Entry |
|--------------|--------------|
| News article | `@misc` |
| Scholarly article | `@article` |
| Generic web page | `@misc` |

### Field Mapping

| url2ref Field | BibTeX Field |
|---------------|--------------|
| URL | `url` |
| Title | `title` |
| Author | `author` |
| Date | `year`, `month` |
| Website | `howpublished` or `note` |
| Publisher | `publisher` |
| Language | `language` |
| Access Date | `urldate`, `note` |
| Journal | `journal` |

### Citation Key Generation

Format: `{domain}{year}`

Examples:
- `bbc2024`
- `arxiv2024`
- `nature2024`

### Author Formatting

BibTeX author format:
```
author = {Last1, First1 and Last2, First2 and Last3, First3}
```

### Date Handling

```bibtex
year = {2024},
month = {jan},
```

Month abbreviations: `jan`, `feb`, `mar`, `apr`, `may`, `jun`, `jul`, `aug`, `sep`, `oct`, `nov`, `dec`

### Special Character Escaping

| Character | Escaped |
|-----------|---------|
| `&` | `\&` |
| `%` | `\%` |
| `$` | `\$` |
| `#` | `\#` |
| `_` | `\_` |
| `{` | `\{` |
| `}` | `\}` |
| `~` | `\~{}` |
| `^` | `\^{}` |

### Example Output

```bibtex
@misc{techblog2024,
  author = {Smith, Jane},
  title = {Understanding Modern Web Development},
  year = {2024},
  month = {jan},
  url = {https://www.example.com/article},
  urldate = {2024-01-20},
  note = {Accessed: 2024-01-20}
}
```

### For Articles

```bibtex
@article{nature2024,
  author = {Smith, Jane and Doe, John},
  title = {Advances in Machine Learning},
  journal = {Nature},
  year = {2024},
  volume = {625},
  pages = {123--130},
  publisher = {Nature Publishing Group},
  url = {https://doi.org/10.1038/example},
  doi = {10.1038/example}
}
```

---

## Harvard

### Format Structure

```
Author (Year) 'Title', Source. Available at: URL (Accessed: Date).
```

### Variations

**With author and date:**
```
Smith, J. (2024) 'Understanding Modern Web Development', Tech Blog. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

**Without author:**
```
'Understanding Modern Web Development' (2024) Tech Blog. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

**Without date:**
```
Smith, J. (n.d.) 'Understanding Modern Web Development', Tech Blog. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

**Without author or date:**
```
'Understanding Modern Web Development' (n.d.) Tech Blog. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

### Author Name Formatting

| Full Name | Harvard Format |
|-----------|----------------|
| Jane Smith | Smith, J. |
| John A. Doe | Doe, J.A. |
| Organization Name | Organization Name |

### Multiple Authors

**Two authors:**
```
Smith, J. and Doe, J. (2024) 'Title'...
```

**Three or more:**
```
Smith, J., Doe, J. and Johnson, A. (2024) 'Title'...
```

**Many authors (in-text):**
```
Smith, J. et al. (2024) 'Title'...
```

### Date Formatting

| Format | Access Date Display |
|--------|---------------------|
| Day Month Year | 20 January 2024 |

### Example Output

```
Smith, J. (2024) 'Understanding Modern Web Development', Tech Blog. 
Available at: https://www.example.com/article (Accessed: 20 January 2024).
```

---

## Format Comparison

### Same Source

**URL:** `https://www.bbc.com/news/technology-12345`

**MediaWiki:**
```
{{cite web
  | url = https://www.bbc.com/news/technology-12345
  | title = AI Revolution: What's Next?
  | author = Tech Correspondent
  | date = 2024-01-15
  | website = BBC News
  | access-date = 2024-01-20
}}
```

**BibTeX:**
```bibtex
@misc{bbc2024,
  author = {Tech Correspondent},
  title = {AI Revolution: What's Next?},
  year = {2024},
  month = {jan},
  howpublished = {BBC News},
  url = {https://www.bbc.com/news/technology-12345},
  urldate = {2024-01-20}
}
```

**Harvard:**
```
Tech Correspondent (2024) 'AI Revolution: What's Next?', BBC News. 
Available at: https://www.bbc.com/news/technology-12345 (Accessed: 20 January 2024).
```

---

## Field Availability

| Field | Wiki | BibTeX | Harvard |
|-------|------|--------|---------|
| URL | ✓ | ✓ | ✓ |
| Title | ✓ | ✓ | ✓ |
| Trans-Title | ✓ | note | note |
| Author | ✓ | ✓ | ✓ |
| Date | ✓ | ✓ | ✓ |
| Website | ✓ | howpublished | ✓ |
| Publisher | ✓ | ✓ | ✓ |
| Language | ✓ | ✓ | — |
| Access Date | ✓ | ✓ | ✓ |
| Archive URL | ✓ | note | — |
| Archive Date | ✓ | note | — |
| Journal | — | ✓ | ✓ |
| Volume | — | ✓ | ✓ |
| DOI | — | ✓ | ✓ |

## Next Steps

- [API Documentation](./api.md) - Programmatic access
- [Environment Variables](./environment-variables.md) - Configuration
- [CLI Citation Formats](../cli/citation-formats.md) - Usage guide
