# Web Interface Features

Comprehensive guide to all features available in the url2ref web interface.

## Citation Generation

### Multiple Output Formats

Switch between formats using the tabs:

| Tab | Format | Use Case |
|-----|--------|----------|
| **Wiki** | MediaWiki `{{cite web}}` | Wikipedia editing |
| **BibTeX** | LaTeX bibliography | Academic papers |
| **Harvard** | In-text citation | Essays, reports |

### One-Click Copying

Each citation has a copy button (📋) that:
- Copies the full citation to clipboard
- Shows confirmation feedback
- Works across all browsers

### Access Date

The current date is automatically added as the access date, which is required for proper web citations.

---

## Metadata Source Comparison

### Viewing All Sources

Click "Show all sources" to expand the multi-source view:

```
┌─────────────────────────────────────────────────────────────────┐
│ Field         OpenGraph    Schema.org    HTML Meta    Selected  │
├─────────────────────────────────────────────────────────────────┤
│ Title         Article...   Article...    Page Ti...   OG ✓      │
│ Author        John Doe     J. Doe        —            OG ✓      │
│ Date          2024-01-15   2024-01-15    —            OG ✓      │
│ Website       TechNews     Tech News     TechNews     OG ✓      │
└─────────────────────────────────────────────────────────────────┘
```

### Source Selection

Click on any source value to use it instead of the default:

1. Find the field you want to change
2. Click on the preferred source value
3. The citation updates automatically

### Source Priority

Default priority order:
1. Open Graph
2. Schema.org  
3. HTML Meta
4. DOI (if available)
5. Zotero (if enabled)
6. AI (if enabled)

---

## Translation

### Enabling Translation

1. Open the Options panel
2. Check "Translate title"
3. Select target language from dropdown

### Supported Languages

| Code | Language |
|------|----------|
| EN | English |
| DE | German |
| FR | French |
| ES | Spanish |
| IT | Italian |
| JA | Japanese |
| ZH | Chinese |
| And more... |

### Translation Providers

Choose between:

- **DeepL** (default) - Higher quality for European languages
- **Google** - Broader language support

### Output

Translated citations include both titles:

```
{{cite web
  | title = L'avenir de la technologie
  | trans-title = The future of technology
  | language = fr
  ...
}}
```

---

## Zotero/Citoid Integration

### What is Citoid?

Citoid is Wikipedia's citation service that uses Zotero translators to extract metadata from websites.

### Enabling Zotero

1. Open the Options panel
2. Check "Enable Zotero/Citoid"

### When to Use

Enable Zotero for:
- Academic databases (JSTOR, PubMed)
- Library catalogs
- Sites with complex JavaScript rendering
- When other sources return incomplete data

### How It Works

1. Request is sent to Wikipedia's Citoid API
2. Citoid uses specialized "translators" for many sites
3. Rich metadata is returned and merged with other sources

---

## AI-Powered Extraction

### Overview

AI extraction uses language models to analyze page content when traditional metadata parsing fails.

### Enabling AI Extraction

1. Open the Options panel
2. Check "Enable AI extraction"
3. Select your provider (OpenAI or Anthropic)
4. Enter your API key

### API Key Security

- Keys are entered in the browser
- Sent directly to AI provider
- **Never stored** on the server
- Cleared when you close the page

### Supported Providers

| Provider | Models |
|----------|--------|
| OpenAI | GPT-4o, GPT-4o-mini |
| Anthropic | Claude 3 Haiku, Sonnet, Opus |

### When AI Helps

- Pages without metadata markup
- Old websites
- Blog platforms with minimal SEO
- User-generated content

---

## Archive Integration

### Wayback Machine

url2ref integrates with the Internet Archive's Wayback Machine.

### Archive Status

After generating a citation, you'll see:

- **✓ Archived** - Page exists in Wayback Machine
  - Shows archive URL and date
  - Automatically added to citation

- **⚠ Not archived** - No archive found
  - Option to create an archive

### Creating Archives

Click "Archive this page" to:
1. Submit URL to Wayback Machine
2. Wait for archival (may take a moment)
3. Receive archive URL and date
4. Citation updates automatically

### Why Archive?

Web pages can disappear. Adding archive URLs:
- Ensures link persistence
- Provides proof of content
- Meets Wikipedia's reliability guidelines

---

## Extracted Fields Display

### Available Fields

| Field | Description |
|-------|-------------|
| Title | Article/page title |
| Author | Author name(s) |
| Date | Publication date |
| Website | Site name |
| Publisher | Publishing organization |
| Language | Content language (ISO code) |
| URL | Original URL |
| Archive URL | Wayback Machine URL |
| Archive Date | When archived |

### Field Highlighting

- **Green**: Successfully extracted
- **Yellow**: Extracted with low confidence
- **Gray**: Not found / not applicable

---

## Real-Time Updates

The interface updates in real-time:

1. **As you type** - URL validation
2. **On generate** - Loading indicator
3. **Source selection** - Instant citation update
4. **Copy action** - Confirmation feedback

---

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "Invalid URL" | Malformed URL | Check URL format |
| "Failed to fetch" | Network/access issue | URL may be blocked |
| "No metadata found" | Empty page | Try Zotero or AI |
| "Translation failed" | API error | Check API key |

### Retry Options

When an error occurs:
- Error message displays
- "Retry" button available
- Option to enable additional extractors

---

## Responsive Design

The interface adapts to different screen sizes:

- **Desktop**: Full layout with side-by-side panels
- **Tablet**: Stacked layout, full features
- **Mobile**: Simplified layout, core features

---

## Browser Storage

The interface uses local storage for:

- Last used settings (format preference)
- UI preferences (panel states)

No personal data or URLs are stored.

---

## Accessibility

The interface includes:

- Keyboard navigation
- Screen reader support
- High contrast compatibility
- Focus indicators

## Next Steps

- [Configuration](./configuration.md) - Server configuration
- [CLI Usage](../cli/index.md) - Command-line alternative
- [API Documentation](../reference/api.md) - Programmatic access
