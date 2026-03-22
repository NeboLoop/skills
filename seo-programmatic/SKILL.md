---
name: seo-programmatic
description: Design scaled page generation strategies using templates and structured data to create city pages, comparison pages, and category pages at scale. Use when building landing pages for every city or service area, creating template-driven content at scale, or planning programmatic SEO with quality gates and phased rollouts.
---

# SEO Programmatic Pages

Create scaled page generation strategies that turn structured data into hundreds of rankable pages — city pages, comparison pages, template-driven content, and more.

---

## When to Use

Trigger this skill when you hear:
- "I want to create pages for every city we serve"
- "Can we build landing pages at scale?"
- "Template pages for each [category/location/product]"
- "Programmatic SEO"
- "How do I rank in every city?"
- "I need pages for all our service areas"
- "Scaled content strategy"
- "Location pages" or "city pages"

This skill is for situations where you have a repeatable page type and a data source that can populate many versions of it. If the user only needs a few hand-crafted pages, use **copywriting** instead.

---

## Context Gathering

Ask these questions conversationally. Don't dump them all at once.

### The Data Source
1. **What data do you have to work with?** (Cities you serve, products you sell, comparisons you can make, categories you cover)
2. **Where does this data live?** (Database, spreadsheet, API, CMS, manually collected)
3. **How many unique entries are there?** (10 cities? 500 products? This determines the approach)

### The Page Type
4. **What kind of page are you building?** (City landing pages, product category pages, comparison pages, directory listings, glossary pages)
5. **What's the goal of each page?** (Get leads, drive signups, rank for local searches, capture long-tail traffic)
6. **Do you have any existing pages like this?** (Share a URL so we can see what's working or not)

### The Content
7. **What unique information exists for each page?** (Local stats, pricing differences, testimonials from that area, product-specific details)
8. **What stays the same across pages?** (Company description, process overview, call-to-action)
9. **Can you source unique content for each variation?** (Local reviews, city-specific data, product specs — this is critical for quality)

### Technical Setup
10. **What platform is your site on?** (WordPress, Next.js, Webflow, custom CMS)
11. **Can you generate pages dynamically from a data source?** (Or do they need to be created manually?)
12. **What's your current domain authority?** (Helps determine how aggressive the strategy can be)

---

## Methodology

### Step 1: Check Marketing Context
Pull the user's product marketing context from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If none exists, run **product-marketing-context** first. You need to understand the business before building pages at scale.

### Step 2: Identify the Data Source and Page Type
Map out what you're working with:

- **Data source:** What structured data will populate each page (cities, products, categories, etc.)
- **Page type:** What the final pages will look like (landing page, comparison, directory listing)
- **Volume:** How many pages will be created

**Volume guardrails:**
- **Under 30 pages:** Green light. Proceed normally.
- **30-50 pages:** Yellow flag. Warn the user: *"At 30+ pages, you need strong unique content per page or Google may see these as thin/duplicate content. Let's make sure each page has enough unique value."*
- **Over 50 pages:** Hard stop. Say: *"50+ programmatic pages requires careful planning. Let's start with a batch of 10-20, measure performance, then scale. Launching hundreds of thin pages can trigger a site-wide quality penalty."*

### Step 3: Design the Page Template
Build a template that balances consistency with uniqueness. Every page needs:

**Required unique content (must be different per page):**
- H1 title with the variable (city name, product name, etc.)
- At least 2-3 paragraphs of unique body content
- Unique meta title and meta description
- At least one unique data point, stat, or detail

**Shared content (can repeat across pages):**
- Company overview (keep it short — 1-2 sentences max)
- Process or how-it-works section
- Call-to-action
- Trust signals (reviews, certifications)

**Quality gate: Each page must have 60%+ unique content.** If you can't hit that threshold, the page shouldn't exist. Calculate this by comparing the word count of unique content vs. total word count.

### Step 4: Plan the Internal Linking Strategy
Programmatic pages need strong internal linking to pass authority and help Google discover them:

- **Hub page:** Create a parent page that links to all programmatic pages (e.g., "Cities We Serve" or "Product Comparisons")
- **Cross-links between programmatic pages:** Each page links to 3-5 related pages (nearby cities, similar products)
- **Links from existing content:** Identify blog posts or pages that can naturally link to the new pages
- **Breadcrumb navigation:** Every programmatic page should show its place in the site hierarchy

### Step 5: Prevent Index Bloat
Not every page deserves to be indexed. Plan for this upfront:

- **Noindex thin pages:** If a page can't hit the 60% unique content threshold, noindex it
- **Canonical tags:** Point near-duplicate pages to the strongest version
- **Sitemap management:** Only include high-quality pages in the XML sitemap
- **Monitor crawl budget:** If you have thousands of pages, check Google Search Console for crawl stats
- **Pagination:** If pages are paginated, use proper pagination markup

### Step 6: Create the Content Strategy per Page
For each page variation, document:

- **Target keyword:** The primary keyword this page targets (e.g., "plumber in Austin TX")
- **Secondary keywords:** 2-3 related terms to include naturally
- **Unique content sources:** Where the unique content will come from (local data, reviews, stats)
- **Unique hook:** What makes this specific page worth visiting beyond the template

### Step 7: Build a Pilot Batch
Never launch all pages at once. Start small:

1. Create 5-10 pages as a pilot batch
2. Submit them to Google Search Console
3. Wait 2-4 weeks for indexing and initial ranking data
4. Measure: Are they getting indexed? Ranking? Getting traffic?
5. If yes, scale up. If no, fix the template before creating more.

### Step 8: Document the Playbook
Create a repeatable process document that covers:
- How to add new pages (what data is needed)
- Quality checklist for each new page
- Internal linking updates needed when adding pages
- How to monitor performance

---

## Output Format

Deliver the programmatic SEO strategy in this structure:

```markdown
# Programmatic SEO Strategy
Date: [DATE]

## Overview
**Page type:** [City pages / Product comparisons / Category pages / etc.]
**Data source:** [Where the data comes from]
**Total pages planned:** [Number]
**Pilot batch:** [First 5-10 pages]

## Page Template

### URL Structure
[example.com/services/[city-name]]

### Meta Title
[Template] — e.g., "[Service] in [City] | [Brand Name]"

### Meta Description
[Template] — e.g., "Looking for [service] in [city]? [Brand] offers [unique value]. [CTA]."

### Page Structure
1. **H1:** [Template with variable]
2. **Intro paragraph:** [Unique — 2-3 sentences about this specific variation]
3. **Key details section:** [Unique data, stats, or information]
4. **How it works:** [Shared — process overview]
5. **Social proof:** [Unique if possible — local reviews, case studies]
6. **CTA:** [Shared — consistent call-to-action]
7. **Related pages:** [Cross-links to 3-5 related programmatic pages]

### Unique Content Requirements
- Minimum unique word count per page: [number]
- Unique content percentage target: 60%+
- Sources of unique content: [list]

## Internal Linking Plan
- **Hub page:** [URL and description]
- **Cross-linking rules:** [How pages link to each other]
- **Existing pages to update:** [Pages that should link to new programmatic pages]

## Index Management
- **Indexing criteria:** [What qualifies a page for indexing]
- **Noindex rules:** [When to noindex]
- **Canonical strategy:** [How to handle near-duplicates]

## Pilot Plan
**Pages to create first:**
1. [Page 1 — keyword, unique angle]
2. [Page 2 — keyword, unique angle]
3. [Page 3 — keyword, unique angle]
...

**Success metrics:**
- Indexed within 2 weeks
- Ranking on page 1-3 for target keyword within 4 weeks
- Generating organic traffic within 6 weeks

## Scaling Plan
[How to go from pilot to full rollout, including timeline and quality gates]
```

---

## Quality Checks

Before delivering the strategy, verify:

- [ ] **Each page has 60%+ unique content** — not just a city name swapped into a template
- [ ] **Volume is within guardrails** — warned at 30+, hard stopped at 50+ without a phased plan
- [ ] **Internal linking strategy exists** — pages aren't orphaned
- [ ] **Index bloat prevention is planned** — noindex rules, canonical tags, sitemap management
- [ ] **Target keywords are mapped** — every page targets a specific keyword
- [ ] **A pilot batch is defined** — not launching everything at once
- [ ] **Unique content sources are identified** — you know where the unique content per page comes from
- [ ] **URL structure is clean and consistent** — follows a logical hierarchy

---

## Examples

### Example 1: City Service Pages

**User says:** "I want to create landing pages for every city we serve. We're a home cleaning company in Texas."

**You do:**
1. Check marketing context (found: home cleaning service, targeting homeowners)
2. Ask: "How many cities do you serve?"
3. User: "25 cities across the Dallas-Fort Worth metro."
4. Ask: "What unique info do you have for each city? Local pricing, number of cleaners in that area, customer reviews from that city?"
5. User: "We have reviews from most cities and can pull local pricing."
6. Design template with city-specific reviews, local pricing, neighborhood mentions
7. Calculate unique content: reviews (unique) + local pricing (unique) + neighborhood details (unique) = ~65% unique. Green light.
8. Plan hub page: "Service Areas" linking to all 25 city pages
9. Define pilot: Start with top 5 cities by search volume
10. Deliver full strategy document

**Key output:**
```
URL: /cleaning-services/dallas-tx
H1: Home Cleaning Services in Dallas, TX
Unique: Local reviews, Dallas pricing, Dallas neighborhoods served
Shared: Cleaning process, company overview, CTA
Unique content: 68% — passes threshold
```

### Example 2: Too Many Pages, Not Enough Content

**User says:** "I want to create 200 pages, one for every neighborhood in my city."

**You do:**
1. Hard stop: "200 pages is a lot. What unique content will each neighborhood page have?"
2. User: "Just the neighborhood name and our standard service description."
3. Explain: "That would mean each page is 95% identical — only the neighborhood name changes. Google will see these as duplicate content and may not index any of them. It could also hurt your existing pages."
4. Recommend: "Let's do 10-15 pages for your top neighborhoods where you have real data — customer reviews, photos of work done there, local pricing. Make each page genuinely useful."
5. User agrees to start with 12 neighborhoods
6. Proceed with quality-focused approach

### Example 3: Product Comparison Pages

**User says:** "We sell 40 types of running shoes. I want a page for each one comparing it to competitors."

**You do:**
1. Yellow flag at 40 pages: "40 pages is doable, but each one needs real comparison data. Do you have specs, pricing, and honest pros/cons for each shoe vs. competitors?"
2. User: "Yes, we have detailed specs for all of them."
3. Design template with comparison tables, spec differences, use-case recommendations
4. Unique content per page: specs (unique), comparison matrix (unique), recommendation (unique) = ~70% unique
5. Plan internal linking: category hub pages linking to individual comparisons
6. Define pilot: Top 10 shoes by search volume
7. Deliver strategy with phased rollout (10 pages per month)

---

## Related Skills

- **product-marketing-context** — must exist before building programmatic pages (defines what you're marketing)
- **seo-competitor-pages** — for hand-crafted comparison pages instead of templated ones
- **copywriting** — for writing the unique content sections of each template
- **page-cro** — optimize conversion on programmatic pages after they're live
- **seo-hreflang** — if programmatic pages need international versions
- **ai-seo** — optimize programmatic pages for AI search results

**Always check product-marketing-context before starting.** Programmatic pages at scale need a clear understanding of the business, customer, and positioning.
