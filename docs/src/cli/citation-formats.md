# Citation Formats

## MediaWiki (`wiki`)

```bash
url2ref-cli -u "https://example.com" -f wiki
```

```
{{cite web
  | title = Article Title
  | author = Jane Smith
  | date = 2024-01-15
  | website = Example News
  | url = https://example.com/article
  | access-date = 2024-01-20
}}
```

**Fields:** `title`, `trans-title`, `author`, `date`, `website`, `publisher`, `url`, `archive-url`, `archive-date`, `access-date`, `language`

## BibTeX (`bibtex`)

```bash
url2ref-cli -u "https://example.com" -f bibtex
```

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

## Harvard (`harvard`)

```bash
url2ref-cli -u "https://example.com" -f harvard
```

```
Smith, J. (2024) 'Article Title', Example News. 
Available at: https://example.com/article (Accessed: 20 January 2024).
```

**Variations:**
- Without author: `'Title' (2024) Website. Available at: URL (Accessed: Date).`
- Without date: `Author (n.d.) 'Title', Website. Available at: URL (Accessed: Date).`
