---
name: web-scraper
description: Extract clean, structured content from any web page by stripping navigation, ads, and clutter. Use when you need to grab an article, research a competitor's site, build a knowledge base from URLs, or pull content for use in other skills.
---

# Web Scraper

Extract clean, useful content from any web page — no clutter, no ads, just the stuff that matters.

---

## When to Use

Trigger this skill when you hear:
- "Grab the content from this URL"
- "Scrape this page for me"
- "Save this article"
- "What does this page say?"
- "Pull the text from this website"
- "I need to research this company's website"
- "Build a knowledge base from these links"
- "Get me the content from these pages"

This skill handles any situation where a user wants to read, save, or work with content from a web page without visiting it themselves.

---

## Context Gathering

Before scraping, clarify what the user needs. Ask conversationally — don't interrogate.

### The URL
1. **What page do you want me to grab?** (Get the full URL — if they give a domain, ask which page specifically)
2. **Is it a single page or multiple pages?** (A list of URLs, or pages linked from a starting page)

### The Purpose
3. **What are you going to do with this content?** (Research? Save for later? Feed into another skill? Compare with competitors?)
4. **Do you need the full page or just specific parts?** (Main article? Product details? Pricing table? Contact info?)

### The Format
5. **How do you want the output?** (Clean text in chat? Saved to a file? Stored in memory for later use?)
6. **Do you need images saved too?** (Sometimes the images matter — product shots, diagrams, infographics)

### Metadata
7. **Do you need metadata?** (Title, author, publish date, description — useful for citations or content curation)

You won't always need to ask all of these. If someone says "grab this article for me," just do it and deliver clean markdown. Use judgment.

---

## Methodology

### Step 1: Load the Page

Navigate to the URL using Nebo's browser:

```
web(action: "navigate", url: "https://example.com/article")
```

If the page fails to load:
- Check if the URL is correct (typos, missing https://)
- Try with and without "www"
- Tell the user if the page is behind a login wall or paywall — you can't bypass those

### Step 2: Read the Raw Page

Pull the full page content:

```
web(action: "read_page")
```

This gives you everything on the page — the content you want plus all the stuff you don't.

### Step 3: Strip the Clutter

Remove everything that isn't the main content:
- **Navigation menus** — top nav, sidebar nav, footer nav
- **Ads and promotional banners** — anything selling something unrelated to the content
- **Sidebar widgets** — related posts, social feeds, newsletter signups
- **Cookie notices and popups** — consent banners, modal overlays
- **Scripts and style tags** — code that runs the page, not content
- **Header and footer boilerplate** — site-wide elements that repeat on every page
- **Comments sections** — unless the user specifically asked for them

Keep:
- **The main article or page body** — the reason someone would visit this page
- **Headings and subheadings** — preserve the content structure
- **Lists and tables** — often contain the most useful information
- **Inline images** — if they add meaning to the content (diagrams, screenshots, product photos)
- **Links within the content** — preserve them in markdown format
- **Block quotes and callouts** — these are part of the content

### Step 4: Convert to Clean Markdown

Transform the cleaned content into well-formatted markdown:
- Use proper heading levels (h1, h2, h3) matching the original hierarchy
- Convert lists to markdown lists
- Convert tables to markdown tables
- Convert links to `[text](url)` format
- Convert images to `![alt text](image_url)` format
- Preserve bold, italic, and code formatting
- Remove excessive whitespace and empty lines
- Keep paragraphs readable — don't merge them into walls of text

### Step 5: Extract Metadata

Pull out key metadata from the page:

```markdown
**Title:** [Page title from <title> or <h1>]
**URL:** [The original URL]
**Author:** [If available from meta tags or byline]
**Published:** [If available from meta tags or article date]
**Description:** [Meta description or first paragraph summary]
**Scraped:** [Today's date]
```

Look for metadata in these places:
- `<title>` tag
- `<meta name="description">` tag
- `<meta name="author">` tag
- `<meta property="og:title">` and other Open Graph tags
- `<meta property="article:published_time">` tag
- Byline elements in the article body
- Schema.org structured data if present

### Step 6: Handle Images (If Requested)

If the user wants images saved:

1. Identify meaningful images in the content (skip icons, spacers, tracking pixels)
2. Note the image URLs and alt text
3. Present the image list to the user with descriptions
4. Save images locally if requested:
```
agent(resource: "memory", action: "store", key: "scraped/images/[page-slug]/[image-name]", value: "image URL and description", layer: "working")
```

### Step 7: Deliver or Store the Content

**If delivering in chat:** Present the clean markdown directly. Lead with metadata, then the content.

**If saving to memory:**
```
agent(resource: "memory", action: "store", key: "scraped/[domain]/[page-slug]", value: "Full clean markdown content", layer: "working")
agent(resource: "memory", action: "store", key: "scraped/[domain]/[page-slug]/metadata", value: "Title, author, date, URL", layer: "working")
```

**If saving to a file:** Write the markdown to a file the user specifies, or suggest a reasonable filename based on the page title.

### Step 8: Summarize What You Got

Tell the user what you scraped:
- Page title
- Word count of the cleaned content
- Number of images found
- Where you stored it (memory, file, or just displayed)
- Any issues (missing metadata, broken images, partial content)

Then ask: *"Want me to do anything with this content? Summarize it, compare it to something, or save it somewhere specific?"*

---

## Output Format

The scraped content follows this structure:

```markdown
# [Page Title]

**Source:** [URL]
**Author:** [Author if available]
**Published:** [Date if available]
**Scraped:** [Today's date]

---

[Clean markdown content of the page]

[Headings, paragraphs, lists, tables, images — all properly formatted]

---

*Scraped from [domain] on [date]*
```

---

## Quality Checks

Before delivering the scraped content, verify:

- [ ] **Content is readable** — flows naturally as markdown, not a jumbled mess
- [ ] **No navigation or ads remain** — only the actual page content
- [ ] **Headings preserve hierarchy** — h1 > h2 > h3 structure matches the original
- [ ] **Links are preserved** — important links within the content still work
- [ ] **Tables are properly formatted** — columns align, headers are clear
- [ ] **Metadata is accurate** — title matches the page, date makes sense
- [ ] **No broken formatting** — no stray HTML tags, no garbled characters
- [ ] **Content is complete** — the full article is captured, not cut off mid-sentence

If any check fails, re-scrape or manually clean up the problem areas before delivering.

---

## Examples

### Example 1: Grab an Article

**User says:** "Can you grab this article for me? https://example.com/blog/ai-trends-2026"

**You do:**
1. Navigate to the URL
2. Read the page
3. Strip navigation, ads, sidebar, comments
4. Convert to clean markdown
5. Extract metadata (title: "AI Trends to Watch in 2026", author: "Jane Smith", published: "March 15, 2026")
6. Present in chat:

*"Here's the article — 1,200 words, published March 15 by Jane Smith."*

Then deliver the clean markdown with metadata header.

### Example 2: Competitive Research

**User says:** "I need to see what my competitor's pricing page says. Here's the link."

**You do:**
1. Navigate to the pricing page
2. Read and clean the content
3. Preserve the pricing table structure carefully
4. Extract plan names, prices, features, and limits
5. Present as clean markdown with the table intact
6. Ask: *"Want me to save this to memory so we can reference it later? Or compare it to your pricing?"*

### Example 3: Build a Knowledge Base

**User says:** "I have 5 articles I want to save for research. Here are the links."

**You do:**
1. Process each URL one at a time
2. Clean and extract content from each
3. Store each article in memory with consistent keys:
   - `scraped/site1.com/article-slug`
   - `scraped/site2.com/article-slug`
4. After all 5 are done, summarize: *"Saved 5 articles to memory. Total: ~8,400 words across all articles. Here's what I grabbed:"*
5. List each article with title, source, and word count
6. Ask: *"Want me to summarize the key themes across these articles?"*

### Example 4: Quick Page Check

**User says:** "What does this page say?" (drops a URL)

**You do:**
1. Scrape it quickly
2. Don't ask questions — just deliver the content
3. Present a brief summary first, then the full clean content below
4. Keep it simple — they just want to know what's on the page

---

## Related Skills

- **product-marketing-context** — scrape competitor sites to build marketing context
- **copywriting** — scrape reference pages to match tone or style
- **social-content** — scrape articles to create social posts about them
- **cold-email** — scrape prospect websites to personalize outreach
- **page-cro** — scrape your own pages to audit them for conversion
- **deep-research** — scrape multiple sources to build comprehensive research reports

**This skill pairs well with any skill that needs external content as input.** Scrape first, then hand off to the right skill.

---

## Change Detection Binary

This skill includes a compiled binary at `scripts/web-differ` for automated change monitoring. Use it when the user wants to track competitor pages, monitor pricing changes, or detect content updates over time.

### Usage

```
scripts/web-differ --urls https://competitor.com/pricing https://competitor.com/features
```

### Arguments

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--urls` | yes (or --urls-file) | — | URLs to scrape (one or more) |
| `--urls-file` | no | — | Read URLs from a file (one per line) |
| `--cache-dir` | no | `.web-differ-cache` | Where to store previous scrapes for comparison |
| `--output` | no | stdout | Write JSON output to file |
| `--no-diff` | no | false | Only scrape, don't compare against cache |
| `--include-full-text` | no | false | Include full page text in output |

### How It Works

1. First run: scrapes pages, caches content, reports them as "new"
2. Subsequent runs: scrapes again, compares against cached version via SHA-256 hash
3. If content changed: outputs a diff showing exactly what was added and removed
4. Updates the cache with the new content for next comparison

### Output

```json
{
  "pages": [{
    "url": "https://competitor.com/pricing",
    "changed": true,
    "diff": {
      "added_lines": 3,
      "removed_lines": 1,
      "changed_sections": [
        { "type": "removed", "content": "Pro plan: $29/month" },
        { "type": "added", "content": "Pro plan: $39/month" }
      ]
    }
  }],
  "summary": {
    "pages_changed": 1,
    "pages_unchanged": 0,
    "pages_new": 0
  }
}
```

The competitive-intel and marketing-manager roles use this binary in their `competitor-alert` and `competitor-scan` workflows.
