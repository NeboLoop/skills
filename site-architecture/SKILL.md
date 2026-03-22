---
name: site-architecture
description: Design and optimize website page hierarchy, navigation menus, URL structure, and internal linking architecture. Use when planning a site redesign, organizing confusing navigation, choosing URL patterns, fixing buried pages, or structuring a growing site so visitors and search engines can find content easily.
---

# Site Architecture

Design and optimize your website's page hierarchy, navigation, and URL structure so visitors find what they need and search engines understand your content.

---

## When to Use

Trigger this skill when you hear:
- "My site navigation is confusing"
- "How should I organize my pages?"
- "What URL structure should I use?"
- "Help me plan my site structure"
- "Users can't find things on my site"
- "I'm rebuilding my website"
- "Should my URLs be flat or nested?"
- "My site has too many pages and no clear hierarchy"
- "I need a sitemap plan"
- "How should I structure my blog categories?"

This skill covers the strategic layout of your entire site — page hierarchy, navigation menus, URL patterns, and internal linking architecture. It does not cover individual page design or content.

---

## Context Gathering

Ask these questions conversationally. Start broad and get specific based on answers.

### Current State
1. **What's your website URL?** (So we can look at how things are organized today)
2. **How many pages does your site have, roughly?** (Under 20, 20-100, 100-500, 500+)
3. **What are the main sections of your site right now?** (Homepage, about, services, blog, products, etc.)

### Business Needs
4. **What are the most important actions visitors take on your site?** (Buy something, sign up, contact you, read content)
5. **What are your top 3-5 most important pages?** (The ones that drive revenue or conversions)
6. **Do you have different audiences visiting your site?** (Buyers vs browsers, different industries, different roles)

### Content Landscape
7. **What types of content do you publish?** (Blog posts, case studies, product pages, landing pages, documentation, guides)
8. **How often do you add new pages?** (Daily, weekly, monthly, rarely)
9. **Do you plan to add major new sections?** (New product lines, new markets, resource library, programmatic pages)

### Pain Points
10. **What frustrates you about your current site structure?** (Hard to navigate, confusing URLs, buried pages, duplicate content)
11. **What do visitors complain about or struggle with?** (Can't find things, get lost, hit dead ends)
12. **Have you noticed SEO issues related to structure?** (Pages not getting indexed, cannibalization, low crawl rates)

---

## Methodology

### Step 1: Check for Existing Context
Search memory for product marketing context and any previous site architecture work:
```
agent(resource: "memory", action: "search", query: "product marketing context")
agent(resource: "memory", action: "search", query: "site architecture")
```

If no product marketing context exists, run the **product-marketing-context** skill first. You need to understand the business before organizing its site.

### Step 2: Map the Current Site
If the user provides a URL, browse the site to understand its current structure:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

Document what you find:
- **Top navigation:** What's in the main menu?
- **Footer navigation:** What's linked from the footer?
- **URL patterns:** Are URLs flat (/page-name) or nested (/category/page-name)? Are they consistent?
- **Depth:** How many clicks from the homepage to the deepest page?
- **Orphan pages:** Are there pages with no clear path from the navigation?

Navigate to key sections to understand the full picture:
```
web(action: "navigate", url: "their-website.com/blog")
web(action: "read_page")
```

### Step 3: Identify Architecture Issues
Look for common problems:

- **Too deep:** Important pages buried 4+ clicks from the homepage
- **Too flat:** Everything dumped at the top level with no logical grouping
- **Inconsistent URLs:** Mixed patterns like /services/web-design and /seo-services
- **Missing hub pages:** Blog posts or product pages with no category index
- **Competing pages:** Multiple pages targeting the same keyword or topic
- **Dead ends:** Pages with no links to related content or next steps
- **Bloated navigation:** Mega menus with 50+ links that overwhelm visitors

Share findings with the user: *"Here's what I noticed about your current structure. Let me walk you through the issues I found."*

### Step 4: Recommend URL Hierarchy
Design a URL structure that balances user clarity and SEO:

**Flat structure** (best for smaller sites, under 50 pages):
```
/about
/services
/pricing
/contact
/blog/post-title
```

**Nested structure** (best for larger sites with clear categories):
```
/services/web-design
/services/seo
/services/content-marketing
/industries/healthcare
/industries/finance
/blog/seo/post-title
/blog/content/post-title
```

**Hybrid structure** (best for medium sites that will grow):
```
/services/web-design
/services/seo
/pricing
/blog/post-title
/resources/guide-title
```

Recommend one approach based on the user's site size, growth plans, and content types. Explain why.

### Step 5: Design Page Hierarchy
Create a visual hierarchy showing how pages relate:

```
Homepage
  Services (hub)
    Service A (detail)
    Service B (detail)
    Service C (detail)
  Industries (hub)
    Industry A (detail)
    Industry B (detail)
  Resources (hub)
    Blog (listing)
      Blog posts
    Case Studies (listing)
      Case study pages
    Guides (listing)
      Guide pages
  About
    Team
    Careers
  Contact
  Pricing
```

Rules of thumb:
- No important page should be more than 3 clicks from the homepage
- Every page should have a clear parent
- Hub pages (category/listing pages) should exist for any group of 5+ related pages
- Top navigation should have 5-7 items maximum

### Step 6: Plan Navigation
Design the navigation menus:

**Primary navigation** (top menu): 5-7 items representing your most important sections. These should map to your main business goals.

**Secondary navigation** (footer or utility bar): Supporting pages like legal, privacy, sitemap, social links.

**Breadcrumbs:** Recommended for any site with nested URLs. Shows the path: Home > Services > Web Design.

**Sidebar or contextual navigation:** For content-heavy sections like blogs or documentation. Shows related content within the current section.

### Step 7: Plan Internal Linking
Design how pages connect beyond navigation:

- **Contextual links:** Within page content, link to related pages naturally
- **Related content blocks:** "Related articles" or "You might also like" sections
- **CTA links:** Every content page should link to a conversion page (pricing, contact, signup)
- **Cross-section links:** Connect services to relevant case studies, blog posts to related services
- **Hub-to-spoke:** Category pages link to all pages in that category; detail pages link back to the category

### Step 8: Store the Plan
Save the site architecture plan to memory:
```
agent(resource: "memory", action: "store", key: "site/architecture/hierarchy", value: "Page hierarchy tree", layer: "tacit")
agent(resource: "memory", action: "store", key: "site/architecture/url_pattern", value: "URL structure recommendation", layer: "tacit")
agent(resource: "memory", action: "store", key: "site/architecture/navigation", value: "Navigation plan", layer: "tacit")
agent(resource: "memory", action: "store", key: "site/architecture/linking_strategy", value: "Internal linking plan", layer: "tacit")
agent(resource: "memory", action: "store", key: "site/architecture/issues", value: "Current issues identified", layer: "tacit")
agent(resource: "memory", action: "store", key: "site/architecture/date", value: "YYYY-MM-DD", layer: "tacit")
```

Tell the user: *"I've saved your site architecture plan. Other skills like programmatic-seo and schema-markup will reference this when working on your site."*

---

## Output Format

The site architecture document follows this structure:

```markdown
# Site Architecture Plan
Last updated: [DATE]

## Current State Summary
**Total pages:** [Estimate]
**Current URL pattern:** [Flat/nested/inconsistent]
**Max click depth:** [Number]
**Key issues:** [List of problems found]

## Recommended URL Structure
**Pattern:** [Flat/nested/hybrid]
**Why:** [Reasoning based on site size and growth plans]

### URL Examples
| Page Type | URL Pattern | Example |
|---|---|---|
| [Type] | [Pattern] | [Example URL] |

## Page Hierarchy
[Visual tree showing all major pages and their relationships]

## Navigation Plan
### Primary Navigation (Top Menu)
1. [Item] — links to [Page]
2. [Item] — links to [Page]
...

### Footer Navigation
[Footer link groups]

### Breadcrumbs
[Yes/No, with format example]

## Internal Linking Strategy
- Contextual links: [Rules]
- Related content: [Approach]
- CTA placement: [Plan]
- Cross-section links: [Key connections]

## Issues to Fix
1. [Issue] — [Recommended fix]
2. [Issue] — [Recommended fix]

## Implementation Priority
1. [First change — highest impact]
2. [Second change]
3. [Third change]
```

---

## Quality Checks

Before finalizing the architecture plan, verify:

- [ ] **No important page is more than 3 clicks from homepage** — check the hierarchy depth
- [ ] **URL structure is consistent** — one pattern, not a mix of flat and nested
- [ ] **Every page has a clear parent** — no orphan pages floating without context
- [ ] **Hub pages exist for content groups** — categories, service areas, and resource types have index pages
- [ ] **Navigation has 7 or fewer top-level items** — more than that overwhelms visitors
- [ ] **Internal linking connects sections** — content pages link to conversion pages, related topics link to each other
- [ ] **The structure supports growth** — adding 50 more pages wouldn't break the hierarchy
- [ ] **URLs are human-readable** — no IDs, random strings, or unnecessary parameters

If any check fails, revise the plan before presenting it.

---

## Examples

### Example 1: Small Business Redesign

**User says:** "I'm rebuilding my website. I have about 15 pages right now but it feels messy."

**You do:**
1. Check marketing context (found: digital marketing agency targeting small businesses)
2. Browse current site — find flat URLs with inconsistent naming, no blog categories, services buried in a single long page
3. Ask: "What are the most important things visitors do on your site?"
4. User: "Contact us for a consultation and see our work."
5. Recommend hybrid structure for a site that will grow:

```
Homepage
  Services (hub page listing all services)
    SEO (/services/seo)
    PPC (/services/ppc)
    Web Design (/services/web-design)
    Content Marketing (/services/content-marketing)
  Work (hub page with case studies)
    Case Study 1 (/work/client-name)
    Case Study 2 (/work/client-name)
  Blog (/blog)
    Posts (/blog/post-title)
  About (/about)
  Contact (/contact)
```

6. Navigation: Services, Work, Blog, About, Contact (5 items)
7. Internal linking: each service page links to related case studies, blog posts link to relevant services
8. Store plan and present to user

### Example 2: Growing E-commerce Site

**User says:** "We have 200 products and our site navigation is a mess. Customers can't find things."

**You do:**
1. Check marketing context (found: outdoor gear e-commerce, 8 product categories)
2. Browse site — find products at random depths, some under /shop/category/product, others at /product-name, no consistent pattern
3. Ask: "What are your main product categories?"
4. User: "Camping, hiking, climbing, cycling, running, skiing, water sports, and accessories."
5. Map out nested structure:

```
Homepage
  Shop (hub)
    Camping (/shop/camping)
      Tents (/shop/camping/tents)
      Sleeping Bags (/shop/camping/sleeping-bags)
      [Individual products: /shop/camping/tents/product-name]
    Hiking (/shop/hiking)
      ...
  Brands (/brands)
    Brand pages (/brands/brand-name)
  Guides (/guides)
    Buying guides (/guides/guide-title)
  Sale (/sale)
```

6. Navigation: Shop (with dropdown showing categories), Brands, Guides, Sale, About (5 items)
7. Recommend breadcrumbs on every product page: Home > Shop > Camping > Tents > Product Name
8. Internal linking: products link to related products, buying guides link to recommended products, category pages feature top sellers
9. Store plan and walk through implementation priority

---

## Related Skills

- **product-marketing-context** — must exist before designing architecture (determines what pages matter most)
- **programmatic-seo** — plans large-scale page generation that must fit within the site hierarchy
- **schema-markup** — implements structured data that reflects the site's organization
- **seo-audit** — identifies technical SEO issues caused by poor site structure
- **page-cro** — optimizes individual pages within the architecture for conversions
- **content-brief** — creates content plans for hub pages and new sections
- **internal-linking** — detailed linking strategy that builds on the architecture plan
- **keyword-research** — informs which pages need to exist based on search demand

**Always check product-marketing-context before starting.** Site architecture must reflect business priorities — what you sell, who you serve, and what actions matter most.
