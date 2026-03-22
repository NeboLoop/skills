---
name: seo-technical
description: Audit technical SEO foundations including crawlability, indexability, HTTPS, canonical tags, mobile-friendliness, Core Web Vitals, URL structure, and internal linking. Use when checking if search engines can properly crawl and index a site, diagnosing technical ranking issues, or evaluating site speed and mobile readiness.
---

# Technical SEO Audit

Check crawlability, indexability, security, and Core Web Vitals — then score each area and prioritize what to fix first.

---

## When to Use

Trigger this skill when you hear:
- "Check my technical SEO"
- "Is my site crawlable?"
- "Check my Core Web Vitals"
- "Run a technical audit on my site"
- "Why isn't my site ranking?"
- "Check my site's SEO health"
- "Is my site mobile-friendly?"
- "Are there any indexing issues?"

This skill focuses on the technical foundation — the stuff search engines need to find, read, and trust your site. It doesn't cover content quality, keyword strategy, or backlinks (those are other skills).

---

## What You'll Need

Before starting, ask the user:
1. **What's your website URL?** (The homepage — you'll explore from there)
2. **Do you have Google Search Console access?** (Helpful but not required)

That's it. Everything else you discover by checking the site directly.

---

## Methodology

### Step 1: Check robots.txt

Navigate to the site's robots.txt file:
```
web(action: "navigate", url: "https://example.com/robots.txt")
web(action: "read_page")
```

**What to look for:**
- Does the file exist? (Missing = not necessarily bad, but worth noting)
- Are important pages blocked? (Look for `Disallow: /` or broad blocks that prevent crawling)
- Is the sitemap referenced? (Good practice — `Sitemap: https://example.com/sitemap.xml`)
- Are there separate rules for different bots? (Googlebot, Bingbot, etc.)

**Red flags:**
- Blocking CSS or JS files (search engines need these to render pages)
- Blocking entire directories that contain important content
- `Disallow: /` with no exceptions (blocks everything)
- No robots.txt at all on a large site

**Score this area:** Healthy / Needs Attention / Critical

### Step 2: Check XML Sitemap

Navigate to the sitemap:
```
web(action: "navigate", url: "https://example.com/sitemap.xml")
web(action: "read_page")
```

If not found, try common locations:
- `/sitemap_index.xml`
- `/sitemap/sitemap.xml`
- Check the robots.txt for a sitemap reference

**What to look for:**
- Does a sitemap exist?
- Is it valid XML? (Not returning errors or HTML)
- Does it include important pages?
- Are the `<lastmod>` dates recent and accurate?
- Is the sitemap reasonably sized? (Under 50,000 URLs per file, under 50MB)
- If it's a sitemap index, do the child sitemaps load?

**Red flags:**
- No sitemap at all
- Sitemap includes pages that return 404s or redirects
- Sitemap is missing important pages
- `<lastmod>` dates all show the same date (usually means they're fake)
- Sitemap returns a 404 or 500 error

**Score this area:** Healthy / Needs Attention / Critical

### Step 3: Verify HTTPS and Security Headers

Check that the site uses HTTPS and has proper security headers:
```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

Also try the HTTP version to check redirects:
```
web(action: "navigate", url: "http://example.com")
```

**What to look for:**
- Does the site load over HTTPS?
- Does HTTP redirect to HTTPS? (It should — a 301 redirect)
- Is the SSL certificate valid and not expired?
- Are there mixed content warnings? (HTTPS page loading HTTP resources)

**Security headers to check (nice-to-haves, not SEO-critical):**
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options` or `Content-Security-Policy` with frame-ancestors
- `Strict-Transport-Security` (HSTS)

**Plain-language explanation for user:** HTTPS is a ranking factor. Google prefers secure sites. If your site isn't on HTTPS or has mixed content, it hurts trust and rankings.

**Score this area:** Healthy / Needs Attention / Critical

### Step 4: Check Canonical Tags and Duplicate Content

Visit the homepage and several key pages:
```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

**What to look for:**
- Does each page have a `<link rel="canonical" href="...">` tag?
- Does the canonical URL point to the correct version of the page?
- Can the site be accessed with and without `www`? (Both should redirect to one version)
- Can the site be accessed with and without trailing slashes? (Pick one, redirect the other)
- Are there URL parameters creating duplicate pages? (e.g., `?sort=price` generating indexable copies)

**Common duplicate content issues:**
- `www` and non-`www` versions both resolve (should redirect one to the other)
- HTTP and HTTPS versions both resolve (should redirect HTTP to HTTPS)
- Trailing slash and non-trailing slash both resolve
- Pagination pages without proper `rel="canonical"` pointing to the main page
- Print-friendly versions of pages that are indexable

**Score this area:** Healthy / Needs Attention / Critical

### Step 5: Check Mobile-Friendliness

Load the site and check for mobile considerations:
```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

**What to look for:**
- Does the page have a viewport meta tag? (`<meta name="viewport" content="width=device-width, initial-scale=1">`)
- Is text readable without zooming?
- Are tap targets (buttons, links) appropriately sized and spaced?
- Does the site use responsive design or a separate mobile site?
- Are there any mobile-specific redirects? (e.g., `m.example.com`)

**Plain-language explanation for user:** Google uses mobile-first indexing — it looks at the mobile version of your site to decide rankings. If your site doesn't work well on mobile, you're invisible to most of Google's index.

**Red flags:**
- No viewport meta tag
- Fixed-width layouts that don't adapt
- Tiny text that requires pinch-to-zoom
- Buttons too small or too close together to tap accurately
- Content that's hidden on mobile but visible on desktop

**Score this area:** Healthy / Needs Attention / Critical

### Step 6: Measure Core Web Vitals

Check the page's performance indicators. Explain each metric in simple terms.

```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

Look at the page's structure for common performance issues and check the source for heavy resources.

**The three Core Web Vitals (explain each to the user):**

**LCP — Largest Contentful Paint**
- *What it means:* How fast the main content of the page loads. Think of it as "how long until the user sees the important stuff."
- *Good:* Under 2.5 seconds
- *Needs work:* 2.5 to 4 seconds
- *Poor:* Over 4 seconds
- *Common causes of slow LCP:* Huge hero images, slow server response, render-blocking JavaScript, unoptimized fonts

**INP — Interaction to Next Paint**
- *What it means:* How fast the page responds when you click, tap, or type something. Think of it as "does the page feel snappy or sluggish?"
- *Good:* Under 200 milliseconds
- *Needs work:* 200 to 500 milliseconds
- *Poor:* Over 500 milliseconds
- *Common causes of slow INP:* Heavy JavaScript, too many event listeners, long-running scripts blocking the main thread

**CLS — Cumulative Layout Shift**
- *What it means:* How much the page jumps around while loading. Ever try to click a button and the page shifts so you click the wrong thing? That's layout shift.
- *Good:* Under 0.1
- *Needs work:* 0.1 to 0.25
- *Poor:* Over 0.25
- *Common causes of bad CLS:* Images without width/height attributes, ads that load late and push content down, fonts that swap and change text size, dynamic content injected above existing content

**What to check in the page source:**
- Are images using `width` and `height` attributes? (Prevents CLS)
- Are images lazy-loaded? (Good for performance)
- Is there render-blocking CSS or JS in the `<head>`? (Hurts LCP)
- How many external scripts are loaded? (Each one can hurt INP)
- Are fonts preloaded? (Prevents font-swap CLS)
- Is there a large hero image? If so, is it optimized and using modern formats like WebP or AVIF?

**Score this area:** Healthy / Needs Attention / Critical

### Step 7: Check URL Structure

Browse several pages on the site to evaluate URL patterns:
```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

Follow internal links to understand the URL structure across the site.

**What good URLs look like:**
- Short and descriptive (`/pricing` not `/page?id=4827`)
- Use hyphens, not underscores (`/blog/seo-tips` not `/blog/seo_tips`)
- Lowercase (no mixed case)
- No unnecessary parameters or session IDs
- Reflect the site hierarchy (`/blog/category/post-title`)
- Human-readable (a person can guess the content from the URL)

**Red flags:**
- Dynamic URLs with long query strings (`?id=123&session=abc&ref=xyz`)
- URLs with uppercase characters (can cause duplicate content)
- Extremely deep nesting (`/a/b/c/d/e/f/page`)
- URLs that don't describe the content
- Session IDs in URLs (creates infinite duplicate pages)
- Non-ASCII characters that aren't properly encoded

**Score this area:** Healthy / Needs Attention / Critical

### Step 8: Check Internal Linking

While browsing the site, pay attention to how pages link to each other:
```
web(action: "navigate", url: "https://example.com")
web(action: "read_page")
```

Check the navigation, footer, and in-content links across several pages.

**What to look for:**
- Does the main navigation link to all important pages?
- Are there orphan pages? (Pages with no internal links pointing to them)
- Do blog posts link to related content?
- Is there a logical site hierarchy? (Homepage > Category > Subcategory > Page)
- Are important pages reachable within 3 clicks from the homepage?
- Do links use descriptive anchor text? (`"read our SEO guide"` not `"click here"`)
- Are there broken internal links? (Links pointing to 404 pages)

**Red flags:**
- Important pages buried 5+ clicks deep
- Navigation that relies entirely on JavaScript (search engines may not follow it)
- No contextual links within content (every page is an island)
- Generic anchor text everywhere ("click here," "read more," "learn more")
- Broken internal links returning 404s
- Footer stuffed with dozens of low-value links

**Score this area:** Healthy / Needs Attention / Critical

---

## Output Format

Present the audit as a clear scorecard, then detailed findings:

```markdown
# Technical SEO Audit: [Website]
Audited: [DATE]

## Overall Health: [Score] / 8 areas healthy

| Area                  | Status          | Priority |
|-----------------------|-----------------|----------|
| Robots.txt            | [Status]        | [Priority] |
| XML Sitemap           | [Status]        | [Priority] |
| HTTPS & Security      | [Status]        | [Priority] |
| Canonicals & Dupes    | [Status]        | [Priority] |
| Mobile-Friendliness   | [Status]        | [Priority] |
| Core Web Vitals       | [Status]        | [Priority] |
| URL Structure         | [Status]        | [Priority] |
| Internal Linking      | [Status]        | [Priority] |

Status: Healthy / Needs Attention / Critical
Priority: Fix Now / Fix Soon / Monitor / Good

## Priority Fixes
[Numbered list of what to fix first, with plain-language explanation of why each matters]

## Detailed Findings

### 1. Robots.txt
[What was found, what's good, what needs fixing]

### 2. XML Sitemap
[What was found, what's good, what needs fixing]

### 3. HTTPS & Security
[What was found, what's good, what needs fixing]

### 4. Canonical Tags & Duplicate Content
[What was found, what's good, what needs fixing]

### 5. Mobile-Friendliness
[What was found, what's good, what needs fixing]

### 6. Core Web Vitals
[LCP, INP, CLS — current state, what's causing issues, how to fix]

### 7. URL Structure
[What was found, what's good, what needs fixing]

### 8. Internal Linking
[What was found, what's good, what needs fixing]
```

---

## Quality Checks

Before delivering the audit, verify:

- [ ] **All 8 areas are scored** — no gaps or "couldn't check" without explanation
- [ ] **Priorities are ranked** — user knows what to fix first, second, third
- [ ] **Explanations are plain language** — no jargon without a simple definition
- [ ] **Each finding has a "why it matters"** — not just what's wrong, but why they should care
- [ ] **Recommendations are actionable** — not "improve performance" but "compress your hero image from 2.4MB to under 200KB"
- [ ] **Critical issues are flagged clearly** — anything blocking indexing or causing major ranking harm stands out
- [ ] **Healthy areas are acknowledged** — don't just list problems, confirm what's working well
- [ ] **No false alarms** — don't flag something as a problem if it's actually fine for their type of site

---

## Examples

### Example 1: Small Business Site

**User says:** "Can you check the technical SEO on my site? It's mybakery.com"

**You do:**
1. Navigate to `mybakery.com/robots.txt` — file exists, looks standard, no important blocks
2. Check `mybakery.com/sitemap.xml` — 404, no sitemap found
3. Try HTTP version — redirects to HTTPS properly, certificate valid
4. Check canonical tags — missing on most pages, www and non-www both resolve
5. Check viewport meta — present, site is responsive
6. Check page source — hero image is 3.2MB uncompressed JPEG, no lazy loading, no width/height on images
7. Check URLs — clean and descriptive (`/menu`, `/about`, `/catering`)
8. Check internal linking — navigation covers main pages, but blog posts don't link to each other

**You deliver:**
```
Overall Health: 4 / 8 areas healthy

Priority Fixes:
1. Add XML sitemap (Critical — search engines can't discover all your pages)
2. Redirect www to non-www (Critical — duplicate content issue)
3. Add canonical tags (Needs Attention — search engines are confused about which version of pages to index)
4. Compress hero image (Needs Attention — 3.2MB image is slowing your LCP to ~6 seconds)
5. Add cross-links in blog posts (Monitor — helps search engines understand your content)
```

### Example 2: E-commerce Site

**User says:** "We're not ranking well. Can you do a technical SEO check on shopstuff.com?"

**You do:**
1. Robots.txt — blocks `/cart/`, `/checkout/`, `/account/` (correct), but also blocks `/collections/` (bad — those are product category pages)
2. Sitemap — exists but includes 2,400 out-of-stock product URLs returning 404s
3. HTTPS — working, but some product images load over HTTP (mixed content)
4. Canonicals — faceted navigation creates thousands of duplicate URLs (`/shoes?color=red&size=10`) without canonical tags
5. Mobile — responsive but tap targets in product grid are too close together
6. Core Web Vitals — LCP at 4.8s due to unoptimized product images, CLS at 0.32 from late-loading price badges
7. URLs — clean category structure, but some product URLs have random IDs (`/product/8847283`)
8. Internal linking — category pages link well, but 340 products have no internal links from blog or other pages

**You deliver:**
```
Overall Health: 2 / 8 areas healthy

Priority Fixes:
1. Unblock /collections/ in robots.txt (Critical — your category pages can't be crawled)
2. Add canonical tags to faceted URLs (Critical — thousands of duplicate pages confusing Google)
3. Remove 404 URLs from sitemap (Needs Attention — wastes crawl budget)
4. Fix mixed content on product images (Needs Attention — undermines HTTPS trust)
5. Optimize product images for LCP (Needs Attention — 4.8s is too slow)
6. Fix CLS from price badges (Needs Attention — reserve space for dynamically loaded prices)
```

---

## Related Skills

- **seo-audit** — broader SEO audit covering content, keywords, and backlinks alongside technical factors
- **seo-sitemap** — deep dive into sitemap strategy, structure, and optimization
- **seo-schema** — add structured data (schema markup) to help search engines understand your content
- **seo-images** — optimize images for search visibility, performance, and accessibility
