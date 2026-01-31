# Web Interface

The url2ref web interface provides a user-friendly way to generate citations directly in your browser.

## Overview

The web interface offers:

- **Visual feedback** - See extracted metadata in real-time
- **Multi-source comparison** - Compare metadata from different sources
- **One-click copying** - Copy citations to clipboard instantly
- **Archive integration** - Check and create Wayback Machine archives
- **Interactive editing** - Choose which metadata values to use

## Screenshot

```
┌────────────────────────────────────────────────────────────────┐
│  url2ref - Automatic Reference Generation                      │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  URL: [https://example.com/article________________] [Generate] │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Options                                              [-] │  │
│  │  ☐ Translate title to: [English    ▾]                   │  │
│  │  ☐ Enable Zotero/Citoid                                 │  │
│  │  ☐ Enable AI extraction                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Extracted Fields                                         │  │
│  │                                                          │  │
│  │ Title:     Example Article Title                         │  │
│  │ Author:    John Doe                                      │  │
│  │ Date:      2024-01-15                                    │  │
│  │ Website:   Example.com                                   │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Citations                     [Wiki] [BibTeX] [Harvard]  │  │
│  │                                                          │  │
│  │ {{cite web                                               │  │
│  │   | title = Example Article Title                        │  │
│  │   | author = John Doe                                    │  │
│  │   ...                                              [📋]  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

## Quick Start

1. Start the web server:
   ```bash
   cargo run --bin url2ref-web
   ```

2. Open your browser to `http://localhost:8000`

3. Enter a URL and click "Generate"

4. Copy the citation in your preferred format

## Features at a Glance

| Feature | Description |
|---------|-------------|
| Multiple formats | Switch between Wiki, BibTeX, Harvard |
| Source comparison | See metadata from all sources |
| Translation | Translate titles via DeepL/Google |
| Archiving | Check/create Wayback Machine archives |
| AI extraction | Fill gaps with AI (OpenAI/Anthropic) |
| Copy to clipboard | One-click citation copying |

## In This Section

- [Getting Started](./getting-started.md) - Setup and first use
- [Features](./features.md) - Detailed feature guide
- [Configuration](./configuration.md) - Server configuration options

## Browser Support

The web interface works in all modern browsers:

- Chrome/Chromium 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Enter` | Generate citation (when URL field focused) |
| `Ctrl+C` | Copy selected text |

## Technology Stack

The web interface is built with:

- **Backend**: [Rocket](https://rocket.rs/) - Rust web framework
- **Templates**: [Tera](https://tera.netlify.app/) - Template engine
- **Frontend**: [Bootstrap 5](https://getbootstrap.com/) - CSS framework
- **Styling**: Custom SCSS

## Next Steps

- [Getting Started](./getting-started.md) - Set up the web interface
- [Features](./features.md) - Explore all capabilities
- [Configuration](./configuration.md) - Customize your deployment
