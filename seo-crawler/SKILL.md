---
name: seo-crawler
description: Crawl a website and extract SEO data — meta tags, headings, links, images, schema markup, sitemaps, and robots.txt. Use when running SEO audits, checking site health, or analyzing site structure at scale.
---

# SEO Crawler

Crawl a website and extract structured SEO data for analysis.

---

## When to Use

This is a **tool skill** — it's called by other skills, not directly by users. The following skills depend on it:

- **seo-audit** — full site audit needs crawl data
- **seo-technical** — technical checks need robots.txt, sitemaps, meta tags
- **seo-sitemap** — sitemap validation needs crawl vs sitemap comparison
- **seo-images** — image audit needs all image elements extracted
- **seo-schema** — schema validation needs JSON-LD from every page

If a user says "audit my site" or "check my SEO", the audit skill triggers this crawler first.

---

## How It Works

The crawler is a compiled binary at `scripts/seo-crawler`. Zero dependencies — just run it.

```
scripts/seo-crawler --url https://example.com --max-pages 50 --output crawl.json
```

### Arguments

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--url` | yes | — | Starting URL to crawl |
| `--max-pages` | no | 50 | Maximum pages to crawl |
| `--depth` | no | 3 | Maximum link depth from start URL |
| `--delay` | no | 200 | Milliseconds between requests to same host |
| `--output` | no | stdout | Write JSON output to file |
| `--sitemap-only` | no | false | Only fetch and parse sitemaps, don't crawl |
| `--include-body` | no | false | Include cleaned body text (for content analysis) |
| `--user-agent` | no | `NeboBot/1.0` | Custom user agent string |

### What It Extracts Per Page

```json
{
  "url": "https://example.com/about",
  "status": 200,
  "redirect_url": null,
  "load_time_ms": 342,
  "title": "About Us - Example",
  "meta_description": "Learn about our team...",
  "canonical": "https://example.com/about",
  "robots_meta": "index, follow",
  "og_tags": { "og:title": "...", "og:image": "..." },
  "headings": {
    "h1": ["About Us"],
    "h2": ["Our Team", "Our Mission"],
    "h3": []
  },
  "links": {
    "internal": [{ "href": "/contact", "text": "Contact us", "nofollow": false }],
    "external": [{ "href": "https://twitter.com/example", "text": "Twitter", "nofollow": true }]
  },
  "images": [
    { "src": "/img/team.jpg", "alt": "Our team photo", "width": 800, "height": 600, "loading": "lazy" }
  ],
  "schema": [
    { "type": "Organization", "raw": "{...}" }
  ],
  "hreflang": [
    { "lang": "en", "href": "https://example.com/about" },
    { "lang": "es", "href": "https://example.com/es/about" }
  ],
  "word_count": 450,
  "body_text": "..."
}
```

### Full Output Structure

```json
{
  "crawl": {
    "start_url": "https://example.com",
    "timestamp": "2026-03-22T10:00:00Z",
    "pages_crawled": 34,
    "pages_skipped": 5,
    "duration_ms": 12400
  },
  "robots_txt": {
    "exists": true,
    "raw": "User-agent: *\nDisallow: /admin\nSitemap: https://example.com/sitemap.xml",
    "disallowed": ["/admin"],
    "sitemaps": ["https://example.com/sitemap.xml"],
    "ai_crawlers": {
      "GPTBot": "blocked",
      "ClaudeBot": "allowed",
      "PerplexityBot": "not_mentioned"
    }
  },
  "sitemaps": [
    {
      "url": "https://example.com/sitemap.xml",
      "type": "index|urlset",
      "urls": 156,
      "entries": [
        { "loc": "https://example.com/about", "lastmod": "2026-01-15", "priority": 0.8 }
      ]
    }
  ],
  "pages": [ ... ],
  "summary": {
    "total_pages": 34,
    "total_images": 89,
    "total_internal_links": 245,
    "total_external_links": 23,
    "pages_without_title": 2,
    "pages_without_description": 5,
    "pages_without_h1": 1,
    "pages_with_multiple_h1": 3,
    "pages_without_canonical": 8,
    "pages_with_schema": 12,
    "orphan_pages": 4,
    "redirect_chains": 1,
    "broken_links": [{ "from": "/about", "to": "/old-page", "status": 404 }],
    "duplicate_titles": [{ "title": "Home", "urls": ["/", "/home"] }],
    "duplicate_descriptions": []
  }
}
```

---

## Quality Checks

- [ ] Respects robots.txt — never crawls disallowed paths
- [ ] Rate limits requests (default 200ms between requests)
- [ ] Follows redirects but records them
- [ ] Handles timeouts gracefully (10s per page)
- [ ] Deduplicates URLs (strips fragments, normalizes trailing slashes)
- [ ] Output is valid JSON
- [ ] Summary counts match actual data

---

## Related Skills

- **seo-audit** — consumes crawl data to produce scored audit report
- **seo-technical** — uses robots.txt, sitemaps, meta tags from crawl
- **seo-sitemap** — compares sitemap entries against crawled pages
- **seo-content** — uses body text for E-E-A-T analysis
- **seo-schema** — validates schema markup extracted by crawler
- **seo-images** — audits image elements from crawl data
