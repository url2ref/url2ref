# Citation Formats Reference

## MediaWiki (`wiki`)

```
{{cite web
  | url = https://example.com/article
  | title = Article Title
  | trans-title = Translated Title
  | author = Jane Smith
  | date = 2024-01-15
  | website = Example News
  | publisher = Example Media Inc.
  | language = en
  | access-date = 2024-01-20
  | archive-url = https://web.archive.org/...
  | archive-date = 2024-01-15
}}
```

Multiple authors: `author1`, `author2`, `author3`

## BibTeX (`bibtex`)

```bibtex
@misc{example2024,
  author = {Smith, Jane},
  title = {Article Title},
  year = {2024},
  month = {jan},
  url = {https://example.com/article},
  urldate = {2024-01-20}
}
```

Entry types: `@misc` (web pages), `@article` (journals)

Citation key format: `{domain}{year}`

Special characters (`& % $ # _ { } ~ ^`) are auto-escaped.

## Harvard (`harvard`)

```
Smith, J. (2024) 'Article Title', Example News. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

**Variations:**
- No author: `'Title' (2024) Website. Available at: URL (Accessed: Date).`
- No date: `Author (n.d.) 'Title', Website. Available at: URL (Accessed: Date).`

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
