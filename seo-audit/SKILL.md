---
name: seo-audit
description: Run a full website SEO audit with a weighted health score and prioritized fixes. Use when auditing a website for SEO, checking site health, diagnosing search traffic issues, or getting a comprehensive overview of technical, content, on-page, schema, performance, and AI search readiness factors.
---

# SEO Audit

Run a full website SEO audit and get an actionable health score with prioritized fixes.

---

## When to Use

Trigger this skill when you hear:
- "Audit my website's SEO"
- "How's my site doing for search?"
- "Check my SEO"
- "What's wrong with my website?"
- "Why isn't my site ranking?"
- "Run an SEO audit"
- "Give me an SEO health score"
- "What should I fix on my site for SEO?"

This is the orchestrator skill. It coordinates all SEO sub-skills into a single comprehensive audit. If someone asks about a specific area (just images, just schema), route to the relevant sub-skill instead.

---

## Context Gathering

Before running the audit, get this information:

### Required
1. **What's the website URL?** (The site to audit — must be a live, publicly accessible URL)

### Optional but Helpful
2. **What keywords are you trying to rank for?** (Top 3-5 target keywords help evaluate content alignment)
3. **Who's your main competitor?** (A competitor URL helps benchmark performance)
4. **What's your target audience?** (Helps evaluate content quality and E-E-A-T signals)
5. **Are you targeting a specific location?** (Local SEO checks differ from national/global)
6. **Have you noticed any recent traffic drops?** (Helps prioritize where to look first)

If the user only provides a URL, that's enough to start. Ask the rest conversationally as the audit runs.

Check for existing product marketing context in memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to inform the audit — you already know their product, audience, and goals.

---

## Methodology

### Step 1: Get the Website URL
Ask for the URL if not provided. Validate it's accessible:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

If the site is down or blocked, tell the user immediately. Don't proceed with a broken URL.

### Step 2: Crawl Key Pages
Gather data from multiple pages to get a full picture:

**Homepage:**
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

**Sitemap (check for existence):**
```
web(action: "navigate", url: "their-website.com/sitemap.xml")
web(action: "read_page")
```

**Robots.txt:**
```
web(action: "navigate", url: "their-website.com/robots.txt")
web(action: "read_page")
```

**Top interior pages** — pick 3-5 from the sitemap or main navigation:
```
web(action: "navigate", url: "their-website.com/about")
web(action: "read_page")
```

Repeat for key product pages, blog posts, and service pages. Prioritize pages linked from the main navigation.

### Step 3: Run Parallel Checks

Run these six check categories across all crawled pages. Each maps to a component of the final score.

#### 3A: Technical SEO (22% of score)
Check each page and the site overall for:
- **HTTPS** — is the site served over HTTPS? Mixed content warnings?
- **Crawlability** — does robots.txt block important pages? Are there noindex tags where there shouldn't be?
- **Indexability** — are canonical tags present and correct? Any conflicting signals?
- **Site structure** — is the URL structure clean and logical? Excessive depth (more than 3 clicks from homepage)?
- **Mobile-friendliness** — does the viewport meta tag exist? Is text readable without zooming?
- **Internal linking** — are important pages linked from the homepage and navigation?
- **404 errors** — do any internal links point to dead pages?
- **Redirects** — are there redirect chains (more than one hop)?
- **Hreflang** — for multilingual sites, are hreflang tags present and correct?

#### 3B: Content Quality (23% of score)
Evaluate content across key pages for:
- **E-E-A-T signals** — Experience, Expertise, Authoritativeness, Trustworthiness. Are there author bios? About pages? Credentials?
- **Content depth** — are pages thin (under 300 words) or substantive? Do they fully answer the topic?
- **Readability** — is the content written at an appropriate level for the audience? Short paragraphs? Clear language?
- **Freshness** — are dates visible? Is content clearly outdated?
- **Duplicate content** — are there pages with substantially similar content?
- **Keyword alignment** — if target keywords were provided, does the content naturally address them?
- **User intent match** — does the content match what someone searching for the topic would want?

#### 3C: On-Page SEO (20% of score)
Check every crawled page for:
- **Title tags** — present? Under 60 characters? Unique across pages? Include target keywords?
- **Meta descriptions** — present? Under 160 characters? Compelling and unique?
- **H1 tags** — exactly one per page? Descriptive and keyword-relevant?
- **Heading hierarchy** — proper H1 > H2 > H3 structure? No skipped levels?
- **Image alt text** — do all images have descriptive alt attributes?
- **Image file size** — are images optimized (not serving 5MB files)?
- **Image format** — are modern formats (WebP, AVIF) used where possible?
- **URL structure** — clean, readable URLs? No excessive parameters or IDs?
- **Internal links** — does each page link to other relevant pages?

#### 3D: Schema & Structured Data (10% of score)
Check for structured data across the site:
- **Organization schema** — is the business identified with structured data?
- **Breadcrumb schema** — are breadcrumbs marked up for better SERP display?
- **Page-type schema** — Article, Product, FAQ, HowTo, LocalBusiness — whatever fits the page type
- **Validation** — is the schema valid JSON-LD? Any errors in the markup?
- **Completeness** — are required and recommended properties filled in?

#### 3E: Performance (10% of score)
Assess page load and performance indicators:
- **Page load feel** — does the page load quickly when navigated to?
- **Large resources** — are there oversized images, videos, or scripts that would slow things down?
- **Render-blocking resources** — are CSS and JS files blocking the initial render?
- **Compression** — are text resources (HTML, CSS, JS) compressed?
- **Caching headers** — are static assets served with cache headers?

#### 3F: AI Search Readiness (10% of score)
Evaluate how well the site is positioned for AI-powered search (Google AI Overviews, ChatGPT search, Perplexity):
- **Clear, direct answers** — does content provide concise answers that AI can extract?
- **FAQ sections** — are common questions answered explicitly on the page?
- **Structured data** — schema markup that helps AI understand content
- **Authoritative sourcing** — does the site cite sources and demonstrate expertise?
- **Content organization** — are pages well-structured with clear sections AI can parse?

### Step 4: Calculate SEO Health Score

Score each component 0-100, then calculate the weighted total:

| Component | Weight | What It Measures |
|-----------|--------|-----------------|
| Technical | 22% | Can search engines crawl and index the site? |
| Content | 23% | Is the content high-quality and trustworthy? |
| On-Page | 20% | Are individual pages properly optimized? |
| Schema | 10% | Is structured data present and valid? |
| Performance | 10% | Does the site load fast? |
| AI Search | 10% | Is content ready for AI-powered search? |
| Images | 5% | Are images optimized with alt text and proper sizing? |

**Total SEO Health Score = sum of (component score x weight)**

Score ranges:
- **90-100:** Excellent — minor tweaks only
- **70-89:** Good — some opportunities to improve
- **50-69:** Needs Work — significant issues hurting rankings
- **30-49:** Poor — major problems preventing rankings
- **0-29:** Critical — fundamental issues need fixing before anything else

### Step 5: Rank Issues by Business Impact
Don't just list everything that's wrong. Prioritize by:

1. **Quick wins** — easy to fix, meaningful impact (missing meta descriptions, missing alt text)
2. **High impact** — harder to fix but biggest ranking gains (thin content, missing schema, crawl issues)
3. **Long-term improvements** — ongoing work that compounds (content strategy, E-E-A-T building, internal linking)

### Step 6: Generate the Report
Compile everything into the output format below. Focus on the top 5 fixes — the things that will move the needle most.

---

## Output Format

Present the audit results in this structure:

```markdown
# SEO Audit Report
**Website:** [URL]
**Date:** [DATE]
**SEO Health Score:** [SCORE]/100

---

## Score Breakdown

| Component | Score | Weight | Weighted |
|-----------|-------|--------|----------|
| Technical | XX/100 | 22% | XX |
| Content | XX/100 | 23% | XX |
| On-Page | XX/100 | 20% | XX |
| Schema | XX/100 | 10% | XX |
| Performance | XX/100 | 10% | XX |
| AI Search | XX/100 | 10% | XX |
| Images | XX/100 | 5% | XX |
| **Total** | | | **XX/100** |

---

## Top 5 Fixes (Do These First)

### 1. [Issue Title]
**Impact:** High | Medium | Low
**Effort:** Quick Win | Moderate | Major Project
**What's wrong:** [Plain English explanation]
**How to fix it:** [Specific, actionable steps]
**Pages affected:** [List specific URLs]

### 2. [Issue Title]
...

### 3. [Issue Title]
...

### 4. [Issue Title]
...

### 5. [Issue Title]
...

---

## Detailed Findings

### Technical SEO (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### Content Quality (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### On-Page SEO (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### Schema & Structured Data (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### Performance (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### AI Search Readiness (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

### Images (XX/100)
**What's working:**
- [Good things found]

**Issues found:**
- [Issue] — [Impact] — [Fix]

---

## Next Steps
[2-3 sentences on what to tackle first and what ongoing work looks like]
```

Always lead with what's working before listing issues. Nobody wants to hear only bad news.

---

## Quality Checks

Before delivering the report, verify:

- [ ] **Score is justified** — every component score has specific evidence backing it up
- [ ] **Top 5 fixes are actionable** — someone could hand this to a developer or content writer and they'd know exactly what to do
- [ ] **No false positives** — don't flag things as issues if they're intentional (e.g., noindex on a staging page)
- [ ] **Pages are cited** — every issue references the specific URL(s) where it was found
- [ ] **Plain language** — a non-technical business owner can understand every finding
- [ ] **Priorities make sense** — quick wins before major projects, high-impact before low-impact
- [ ] **Positives are included** — the report acknowledges what's done well, not just problems
- [ ] **No generic advice** — every recommendation is specific to this website, not boilerplate SEO tips

If any check fails, revise the report before presenting it.

---

## Examples

### Example 1: Small Business Website

**User says:** "Can you audit my website's SEO? It's www.example-bakery.com"

**You do:**
1. Navigate to the site, read the homepage
2. Check for sitemap.xml and robots.txt
3. Crawl 4-5 key pages (menu, about, contact, a blog post)
4. Run all six check categories
5. Calculate score: Technical 78, Content 45, On-Page 62, Schema 20, Performance 71, AI Search 35, Images 55
6. Weighted total: 53/100 — Needs Work
7. Top 5 fixes:
   - Add schema markup for LocalBusiness (no structured data at all)
   - Rewrite thin product pages (menu page has only dish names, no descriptions)
   - Add meta descriptions to all pages (none exist)
   - Compress hero images (3MB each, should be under 200KB)
   - Add FAQ section to homepage for AI search visibility

**You say:** "Your site scored 53/100 — there's real opportunity here. The good news: your technical foundation is solid (HTTPS, clean URLs, mobile-friendly). The biggest gaps are content and structured data. Here are the 5 things that'll move the needle most..."

### Example 2: E-commerce Site with Specific Keywords

**User says:** "We're trying to rank for 'organic dog treats' but we're stuck on page 3. Can you audit our SEO?"

**You do:**
1. Note the target keyword before starting
2. Crawl the site with keyword alignment in mind
3. Discover: the product page targeting "organic dog treats" has a generic title ("Products"), thin content (80 words), no schema markup, and the H1 says "Shop"
4. Also find: strong technical foundation, good site speed, but no blog content supporting the keyword cluster
5. Score: 61/100 — Needs Work, but fixable
6. Top 5 fixes focus on the keyword gap:
   - Rewrite the product page title, H1, and meta description around "organic dog treats"
   - Add 500+ words of content to the product page (ingredients, sourcing, benefits)
   - Add Product schema with reviews, price, and availability
   - Create 3 supporting blog posts (keyword cluster: "best organic dog treats," "organic vs natural dog treats," "are organic dog treats worth it")
   - Add internal links from blog posts to the product page

**You say:** "I can see why you're stuck on page 3. Your site is technically sound, but the page targeting 'organic dog treats' isn't giving Google enough to work with. Here's what to fix..."

### Example 3: Site with Existing Marketing Context

**User says:** "Run an SEO audit on my site"

**You do:**
1. Search memory — find existing product marketing context with URL, target audience, and goals
2. Say: "I have your marketing context from Feb 10 — I know you're targeting sales managers at mid-size B2B companies and your top goal is ranking for 'sales email automation.' I'll use that to make the audit more relevant. Let me check your site."
3. Run the full audit with keyword alignment and audience awareness baked in
4. Tailor recommendations to their specific business goals

---

## Related Skills

- **seo-technical** — deep dive into crawlability, indexability, HTTPS, and site architecture
- **seo-content** — detailed content quality audit with E-E-A-T analysis and readability scoring
- **seo-schema** — comprehensive structured data audit and implementation guidance
- **seo-images** — image optimization audit covering alt text, file size, format, and lazy loading
- **seo-sitemap** — sitemap and robots.txt audit with crawl budget analysis
- **seo-geo** — local SEO audit for businesses targeting specific geographic areas

**This skill orchestrates all sub-skills into a single audit.** For a deep dive on any specific area, use the individual skill directly.
