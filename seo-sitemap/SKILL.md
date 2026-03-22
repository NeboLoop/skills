---
name: seo-sitemap
description: Analyze, validate, and generate XML sitemaps to ensure search engines discover and crawl all important pages. Use when checking sitemap health, Google isn't indexing all pages, creating a new sitemap, fixing Search Console sitemap errors, or auditing robots.txt for crawl issues.
---

# SEO Sitemap

Analyze, validate, and generate XML sitemaps so search engines can find and crawl every important page on your site.

---

## When to Use

Trigger this skill when you hear:
- "Check my sitemap"
- "I don't have a sitemap"
- "Generate a sitemap for my site"
- "Google isn't indexing all my pages"
- "Are there pages missing from my sitemap?"
- "Help me with my sitemap.xml"
- "My sitemap has errors in Search Console"
- After an seo-audit flags sitemap issues

A sitemap is like a table of contents for search engines. Without one, Google relies on crawling links to discover your pages — and it will miss some.

---

## Context Gathering

Ask these questions conversationally. Get the essentials first.

### The Basics
1. **What's your website URL?** (Need to check the current sitemap)
2. **How many pages does your site have roughly?** (10 pages vs. 10,000 pages need very different sitemap strategies)
3. **Do you know if you have a sitemap already?** (Many CMS platforms generate one automatically)

### Technical Setup
4. **What platform or CMS are you on?** (WordPress, Shopify, Next.js, custom — affects how sitemaps are managed)
5. **Do you have access to your site's root directory or hosting?** (Needed to upload or modify sitemap files)
6. **Is your sitemap submitted in Google Search Console?** (Submitted vs. just existing are different things)

### Content Types
7. **What types of content do you have?** (Pages, blog posts, products, categories, images, videos — each may need its own sitemap)
8. **How often do you publish or update content?** (Affects `lastmod` dates and `changefreq` recommendations)

---

## Methodology

### Step 1: Check Product Marketing Context
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

Understand the business to know which pages actually matter for their goals.

### Step 2: Fetch the Current Sitemap
Try to access the sitemap at common locations:
```
web(action: "navigate", url: "their-site.com/sitemap.xml")
web(action: "read_page")
```

If not found at `/sitemap.xml`, also check:
- `/sitemap_index.xml`
- `/sitemap/sitemap.xml`
- Check `robots.txt` for a `Sitemap:` directive

If no sitemap exists at all, note that and move to Step 5 (generation).

### Step 3: Validate Sitemap Structure
Check the sitemap against XML sitemap standards:

**Format and syntax:**
- Valid XML with proper encoding declaration
- Uses the correct namespace: `xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"`
- Each URL wrapped in `<url>` tags with `<loc>` as required child
- If it's a sitemap index, uses `<sitemapindex>` with `<sitemap>` entries

**URL checks:**
- All URLs are absolute (full URLs, not relative paths)
- URLs are consistent (all www or all non-www, all https)
- No duplicate URLs
- No URLs that return 404, 301, or other non-200 status codes
- No URLs blocked by robots.txt
- No URLs with `noindex` meta tags (contradicts being in the sitemap)
- Total URLs under 50,000 per sitemap file (Google's limit)
- File size under 50MB uncompressed (Google's limit)

**Optional but valuable tags:**
- `<lastmod>` — date the page was last modified (should be accurate, not auto-generated to today's date)
- `<changefreq>` — how often the page changes (Google mostly ignores this, but it doesn't hurt)
- `<priority>` — relative importance of pages on your site (0.0 to 1.0, Google mostly ignores this too)

### Step 4: Check for Missing and Orphan Pages
**Find missing pages** — pages that exist on the site but aren't in the sitemap:
- Browse the site's main navigation, footer links, and internal links
- Compare discovered URLs against sitemap entries
- Flag any important pages not included

```
web(action: "navigate", url: "their-site.com")
web(action: "read_page")
```

Walk through key navigation paths to discover pages.

**Find orphan pages** — pages in the sitemap that can't be reached through any link on the site:
- These pages exist but have no internal links pointing to them
- Search engines may crawl them via the sitemap but rank them lower without internal link support
- Either add internal links to these pages or remove them from the sitemap if they're no longer needed

**Check for pages that shouldn't be in the sitemap:**
- Admin pages, login pages, thank-you pages
- Paginated archives (page/2/, page/3/ — usually excluded)
- Parameter-heavy URLs (search results, filtered views)
- Thin or duplicate content pages

### Step 5: Generate or Improve the Sitemap
If the sitemap is missing or needs significant fixes, generate a new one.

**Sitemap structure for small sites (under 1,000 pages):**
A single `sitemap.xml` file with all URLs.

**Sitemap structure for larger sites:**
A sitemap index that references multiple sitemaps split by content type:
- `sitemap-pages.xml` — static pages
- `sitemap-posts.xml` — blog posts or articles
- `sitemap-products.xml` — product pages (e-commerce)
- `sitemap-categories.xml` — category and tag pages
- `sitemap-images.xml` — image entries (if image search matters)

**Set accurate `lastmod` dates:**
- Use the actual date the content was last meaningfully updated
- Don't set all dates to today — Google notices and ignores fake dates
- Pages that haven't changed in years should show their real last-modified date

### Step 6: Check robots.txt Reference
Verify that `robots.txt` includes a `Sitemap:` directive:
```
web(action: "navigate", url: "their-site.com/robots.txt")
web(action: "read_page")
```

The file should contain:
```
Sitemap: https://their-site.com/sitemap.xml
```

If missing, recommend adding it.

### Step 7: Search Console Submission
Advise the user on submitting the sitemap:
- Log into Google Search Console
- Go to Sitemaps in the left menu
- Enter the sitemap URL and click Submit
- Check back in a few days for any errors Google reports

### Step 8: Compile Recommendations
Prioritize by impact:
1. **Critical** — No sitemap exists, sitemap has broken URLs, important pages missing
2. **Important** — Sitemap includes noindex pages, incorrect lastmod dates, not submitted to Search Console
3. **Nice to have** — Adding image sitemap entries, splitting into multiple sitemaps, removing low-value pages

---

## Output Format

```markdown
# Sitemap Audit
**Site:** [URL]
**Sitemap location:** [URL or "Not found"]
**Date:** [DATE]

## Summary
- **Sitemap exists:** Yes/No
- **Total URLs in sitemap:** [number]
- **Valid URLs:** [number]
- **Broken URLs (non-200):** [number]
- **Missing important pages:** [number]
- **Pages that shouldn't be included:** [number]

## Issues Found

### Critical
[List each critical issue with specifics]

### Important
[List each important issue]

### Minor
[List minor improvements]

## Missing Pages
[Table of important pages found on the site but not in the sitemap]

| Page | Why It Matters |
|------|---------------|
| /pricing | High-intent page for conversions |
| /blog/top-performing-post | Gets organic traffic, should be in sitemap |

## Pages to Remove
[Table of pages currently in the sitemap that shouldn't be]

| Page | Why Remove |
|------|-----------|
| /admin/login | Not for public indexing |
| /search?q=test | Parameter URL, low value |

## Recommended Sitemap
[If generating a new sitemap, provide the full XML or a structured outline]

## Next Steps
1. [Highest priority action]
2. [Second priority]
3. [Third priority]
```

---

## Quality Checks

Before delivering your audit, verify:

- [ ] **Checked all common sitemap locations** — not just `/sitemap.xml`
- [ ] **Validated XML syntax** — broken XML means search engines can't read it at all
- [ ] **Every URL in the sitemap was verified** — no assumptions about whether they work
- [ ] **Important pages were cross-checked** — browsed the site to find pages the sitemap missed
- [ ] **No noindex pages in the sitemap** — this is a direct contradiction that confuses crawlers
- [ ] **lastmod dates are realistic** — not all set to today or suspiciously identical
- [ ] **robots.txt was checked for Sitemap directive** — easy win if missing
- [ ] **Recommendations account for the CMS** — don't suggest manual XML editing if their CMS auto-generates it

---

## Examples

### Example 1: WordPress Site with Plugin-Generated Sitemap

**User says:** "Google isn't finding all my blog posts. Can you check my sitemap?"

**You do:**
1. Check product marketing context — learn they're a marketing agency
2. Fetch `/sitemap.xml` — find a Yoast-generated sitemap index with 4 sub-sitemaps
3. Check `sitemap-posts.xml` — find 45 posts listed, but the site has 62 published posts
4. Discover 17 posts are set to "noindex" in Yoast, so they're excluded from the sitemap
5. Verify those 17 posts — 14 of them should actually be indexed (the noindex was set accidentally)

**Key recommendations:**
- Remove the accidental noindex tags on 14 blog posts
- Those posts will automatically appear in the sitemap once noindex is removed
- Submit the updated sitemap in Search Console
- Check back in 2 weeks to confirm Google is indexing them

### Example 2: Custom Site with No Sitemap

**User says:** "I built my site from scratch. Do I need a sitemap?"

**You do:**
1. Check context — learn they're a freelance designer with a portfolio site
2. Try `/sitemap.xml` — 404
3. Check `robots.txt` — no Sitemap directive
4. Browse the site — find 18 pages total (home, about, services, contact, 14 portfolio pieces)
5. Generate a clean sitemap with all 18 pages

**Key recommendations:**
- Provide the complete sitemap XML ready to upload
- Add a `Sitemap:` line to robots.txt
- Submit to Google Search Console
- For 18 pages, a single sitemap file is all they need — no index required

---

## Related Skills

- **seo-audit** — comprehensive SEO check that includes sitemap validation as one component
- **seo-images** — image sitemap entries help Google discover and index product or portfolio images
- **seo-technical** — broader technical SEO including crawlability, robots.txt, and indexation
- **seo-plan** — strategic roadmap where sitemap setup is part of the foundation phase
- **seo-geo** — AI search optimization that considers crawler access alongside sitemap coverage
- **product-marketing-context** — helps identify which pages matter most for the business
