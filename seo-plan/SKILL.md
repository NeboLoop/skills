---
name: seo-plan
description: Create a strategic SEO plan with a phased roadmap tailored to your business type (SaaS, local, e-commerce, publisher, agency), including keyword targeting, content calendar, link building strategy, and success metrics. Use when starting SEO from scratch, building an SEO strategy, planning organic traffic growth, or needing a structured roadmap.
---

# SEO Plan

Create a strategic SEO plan tailored to your business type — a phased roadmap from foundation to scale, with specific actions for each stage.

---

## When to Use

Trigger this skill when you hear:
- "I need an SEO strategy"
- "Create an SEO plan for my business"
- "Where do I start with SEO?"
- "Help me rank on Google"
- "I want more organic traffic"
- "Build me an SEO roadmap"
- "What should I focus on for SEO?"
- "My competitor ranks higher than me"

This skill creates the big-picture plan. Other SEO skills handle the individual tactics (audits, content, sitemaps, etc.). Think of this as the playbook that tells you when to run each play.

---

## Context Gathering

Ask these questions conversationally. Understanding the business is everything — there is no generic SEO plan that works for everyone.

### Business Basics
1. **What's your website URL?** (Need to see what exists today)
2. **What type of business are you?** (SaaS, local service, e-commerce, publisher, agency, marketplace — this determines the entire strategy)
3. **How long has your site been live?** (Brand new sites need different strategies than established ones)

### Current SEO State
4. **Are you getting any organic traffic today?** (If they know — Google Analytics or Search Console data helps)
5. **Have you done any SEO work before?** (Starting from zero vs. building on existing efforts)
6. **Do you have a blog or content section?** (Content infrastructure affects what's possible in the first months)

### Competition and Market
7. **Who are your main competitors?** (Need to see what you're up against in search results)
8. **What would your ideal customer search for to find you?** (Their intuition about keywords is a starting point)
9. **Is your business local, national, or international?** (Local SEO is a completely different game)

### Resources
10. **Who will do the SEO work?** (You, a team member, an agency, or are you looking for Nebo to handle it?)
11. **Can you commit to publishing content regularly?** (SEO without content is like a restaurant without food)
12. **What's your timeline expectation?** (SEO takes 3-6 months minimum — set realistic expectations early)

---

## Methodology

### Step 1: Check Product Marketing Context
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If marketing context doesn't exist, run the product-marketing-context skill first. You can't build an SEO plan without knowing the product, customer, and positioning.

### Step 2: Identify the Business Type
The entire SEO strategy depends on the business model. Determine which type fits:

**SaaS / Software:**
- Target: feature-related searches, comparison keywords, problem-aware searches
- Content focus: how-to guides, comparisons, use cases, integrations
- Key differentiator: product-led content that shows the tool solving real problems

**Local Service (plumber, dentist, lawyer, restaurant):**
- Target: "[service] + [location]" searches
- Content focus: service pages, local guides, FAQ pages
- Key differentiator: Google Business Profile optimization, reviews, local citations

**E-commerce:**
- Target: product searches, category searches, buying-intent keywords
- Content focus: product descriptions, category pages, buying guides
- Key differentiator: product schema, image SEO, internal linking between related products

**Publisher / Media:**
- Target: high-volume informational searches
- Content focus: news, guides, evergreen reference content
- Key differentiator: topical authority, publishing velocity, content freshness

**Agency / Consultancy:**
- Target: service searches, industry expertise searches
- Content focus: case studies, industry insights, process explanations
- Key differentiator: demonstrated expertise (E-E-A-T), thought leadership

### Step 3: Analyze Current State
Browse the site and assess what exists:
```
web(action: "navigate", url: "their-site.com")
web(action: "read_page")
```

Check:
- How many pages exist and what types (product, blog, landing pages)
- Current site structure and navigation
- Whether basic SEO foundations are in place (title tags, meta descriptions, headings)
- Page speed (quick assessment)
- Mobile responsiveness
- Existing content quality and depth

Also check:
```
web(action: "navigate", url: "their-site.com/robots.txt")
web(action: "read_page")
```
```
web(action: "navigate", url: "their-site.com/sitemap.xml")
web(action: "read_page")
```

### Step 4: Define Target Keywords
Based on the business type, product context, and competitive landscape, identify keyword themes:

**Keyword categories to define:**
- **Money keywords** — directly tied to revenue (buy, pricing, vs, alternative, best)
- **Problem keywords** — the pain points customers search for before they know the solution
- **Informational keywords** — how-to, guides, explanations that build topical authority
- **Brand keywords** — searches for the brand name and product names
- **Long-tail keywords** — specific, lower-volume phrases with high intent and less competition

**For each keyword theme, note:**
- Estimated search volume (high/medium/low is fine — exact numbers aren't needed for a plan)
- Competition level (who's ranking now and how strong they are)
- Intent (buying, researching, comparing, learning)
- Priority (which to target first based on business impact and achievability)

### Step 5: Build the 4-Phase Roadmap

**Phase 1: Foundation (Month 1)**
The basics that everything else builds on:
- Fix technical SEO issues (crawl errors, broken links, slow pages)
- Set up or clean up Google Search Console and Analytics
- Create or fix the XML sitemap
- Optimize robots.txt
- Fix title tags and meta descriptions on existing pages
- Set up proper heading hierarchy on all pages
- Ensure mobile responsiveness
- Add structured data (Schema.org) for the business type
- Set up the content section/blog if it doesn't exist

**Phase 2: Content (Month 2-3)**
Start building the content that will rank:
- Create pillar pages for each main keyword theme (comprehensive, authoritative pages)
- Write 2-4 supporting articles per pillar topic (interlinked with the pillar page)
- Optimize existing pages for target keywords
- Create content calendar for ongoing publishing
- Build out service/product pages with depth (not thin pages)
- Add FAQ sections to high-priority pages
- Start building internal linking structure between related content

For local businesses: create individual service pages for each service + location combination.
For e-commerce: optimize category pages and write buying guides.
For SaaS: create comparison pages, use case pages, and integration guides.

**Phase 3: Authority (Month 4-6)**
Build the signals that make search engines trust you:
- Start link building through relevant outreach (guest posts, partnerships, industry publications)
- Get listed on relevant directories and review sites
- Create original research, data, or tools that attract natural links
- Build relationships with industry publishers who might cite your content
- Optimize for featured snippets on informational queries
- Start monitoring rankings and adjusting strategy based on what's working
- Begin GEO optimization (see seo-geo skill)

For local businesses: focus on local citations, Google Business Profile optimization, and review generation.
For e-commerce: pursue product review links and shopping comparison sites.

**Phase 4: Scale (Month 7+)**
Expand what's working and maintain what's built:
- Increase content publishing frequency based on what topics are performing
- Expand into adjacent keyword territories
- Update and refresh existing content that's ranking (keep it current)
- Build out more advanced content types (video, interactive tools, templates)
- Pursue higher-authority link building opportunities
- Monitor and respond to algorithm updates
- Track revenue impact, not just rankings
- Review and revise the strategy quarterly

### Step 6: Create the Content Calendar
Based on the keyword themes and phased roadmap, create a specific content calendar:

**Month 1:** Focus on fixing/optimizing existing content (no new content needed yet)
**Month 2:** Publish 1 pillar page + 2-3 supporting articles
**Month 3:** Publish 1 pillar page + 2-3 supporting articles
**Month 4-6:** Publish 1-2 articles per week, focused on the keyword themes showing traction
**Month 7+:** Adjust frequency based on results and capacity

For each piece of content, define:
- Target keyword/topic
- Content type (pillar page, supporting article, comparison, how-to, case study)
- Target word count range
- Internal linking targets (which existing pages to link to/from)

### Step 7: Define Link Building Strategy
Based on the business type:

**SaaS:** Product integrations, industry roundups, comparison articles, guest posts on tech blogs
**Local:** Local directories, chamber of commerce, local news, community sponsorships
**E-commerce:** Product reviews, shopping guides, supplier relationships, influencer partnerships
**Publisher:** Original research, expert quotes, data journalism, syndication
**Agency:** Case studies, industry speaking, professional associations, client partnerships

### Step 8: Set Success Metrics
Define what to measure and when to expect results:

**Month 1-2:** Technical health metrics (crawl errors fixed, pages indexed, site speed)
**Month 3-4:** Ranking improvements for target keywords (expect movement, not top 3 yet)
**Month 5-6:** Organic traffic growth (should start seeing measurable increases)
**Month 7+:** Revenue impact (leads, sales, or conversions from organic traffic)

**Key metrics to track:**
- Organic sessions (Google Analytics)
- Keyword rankings for target terms (Search Console)
- Pages indexed (Search Console)
- Backlinks acquired (Search Console or third-party tool)
- Conversions from organic traffic (Analytics goals/events)

---

## Output Format

```markdown
# SEO Plan
**Business:** [Name]
**Website:** [URL]
**Business type:** [SaaS / Local / E-commerce / Publisher / Agency]
**Date:** [DATE]

## Current State
[Brief assessment of where the site stands today]

## Target Keywords

### Money Keywords (high priority)
| Keyword Theme | Intent | Competition | Priority |
|--------------|--------|-------------|----------|
| [keyword] | Buying | High/Med/Low | 1 |

### Problem Keywords
| Keyword Theme | Intent | Competition | Priority |
|--------------|--------|-------------|----------|
| [keyword] | Research | High/Med/Low | 2 |

### Informational Keywords
| Keyword Theme | Intent | Competition | Priority |
|--------------|--------|-------------|----------|
| [keyword] | Learning | High/Med/Low | 3 |

## Roadmap

### Phase 1: Foundation (Month 1)
- [ ] [Specific action item]
- [ ] [Specific action item]
- [ ] [Specific action item]

### Phase 2: Content (Month 2-3)
- [ ] [Specific action item]
- [ ] [Specific action item]
- [ ] [Specific action item]

### Phase 3: Authority (Month 4-6)
- [ ] [Specific action item]
- [ ] [Specific action item]
- [ ] [Specific action item]

### Phase 4: Scale (Month 7+)
- [ ] [Specific action item]
- [ ] [Specific action item]
- [ ] [Specific action item]

## Content Calendar

### Month 2
| Week | Topic | Type | Target Keyword |
|------|-------|------|---------------|
| 1 | [Topic] | Pillar page | [keyword] |
| 2 | [Topic] | Supporting article | [keyword] |
| 3 | [Topic] | Supporting article | [keyword] |

### Month 3
[Same format]

## Link Building Strategy
[Specific approach based on business type]

## Success Metrics
| Timeframe | Metric | Target |
|-----------|--------|--------|
| Month 1-2 | Technical health | [specific target] |
| Month 3-4 | Keyword movement | [specific target] |
| Month 5-6 | Organic traffic | [specific target] |
| Month 7+ | Revenue impact | [specific target] |

## Next Steps
1. [Immediate action to take today]
2. [Action for this week]
3. [Action for this month]
```

---

## Quality Checks

Before delivering the plan, verify:

- [ ] **Business type is correctly identified** — the entire strategy depends on getting this right
- [ ] **Product marketing context was referenced** — keywords and positioning align with the brand
- [ ] **Keywords match real user behavior** — terms people actually search, not industry jargon
- [ ] **Timeline expectations are realistic** — SEO takes months, not days; don't overpromise
- [ ] **Actions are specific, not vague** — "Write a comparison page for [keyword]" not "Create content"
- [ ] **Resource constraints are considered** — a solo founder can't publish 5 articles a week
- [ ] **The plan is phased logically** — foundations before content, content before link building
- [ ] **Success metrics are measurable** — the user can actually check these numbers

---

## Examples

### Example 1: SaaS Startup

**User says:** "I just launched my SaaS product and I want to start ranking on Google."

**You do:**
1. Check marketing context — learn they're a project management tool for remote teams, $29/month
2. Browse their site — find 5 pages: home, features, pricing, about, contact. No blog.
3. Identify business type: SaaS
4. Define keyword themes:
   - Money: "project management tool for remote teams", "[competitor] alternative"
   - Problem: "how to manage remote team projects", "remote team collaboration challenges"
   - Informational: "remote work best practices", "asynchronous project management"

**Key plan elements:**
- Month 1: Fix title tags, add Schema, create blog section, set up Search Console
- Month 2-3: Write pillar pages on "remote project management" and "remote team collaboration," plus comparison pages vs. Asana and Monday.com
- Month 4-6: Guest post on remote work blogs, get listed on G2 and Capterra, create original survey about remote team challenges
- Month 7+: Expand into adjacent topics, increase publishing to weekly, build out integration pages

### Example 2: Local Plumbing Company

**User says:** "My competitor is #1 on Google for plumber in our city. How do I beat them?"

**You do:**
1. Check marketing context — local plumbing company in Denver, CO
2. Browse their site — basic 3-page site, no individual service pages
3. Identify business type: Local Service
4. Define keyword themes:
   - Money: "plumber Denver", "emergency plumber Denver", "drain cleaning Denver"
   - Service-specific: "water heater repair Denver", "sewer line replacement Denver"
   - Informational: "how to unclog a drain", "signs you need a new water heater"

**Key plan elements:**
- Month 1: Optimize Google Business Profile, fix site structure, create individual service pages for each service
- Month 2-3: Create location-specific service pages (e.g., "Plumber in Capitol Hill Denver"), write helpful blog posts answering common plumbing questions
- Month 4-6: Build local citations (Yelp, Angi, BBB, local directories), start a review generation campaign, get listed in local news or community sites
- Month 7+: Expand to nearby service areas, create seasonal content (winterizing pipes in Colorado), video content showing common repairs

---

## Related Skills

- **product-marketing-context** — MUST run before this skill; provides the positioning and customer understanding that shapes the entire plan
- **seo-audit** — detailed technical audit that feeds into Phase 1 action items
- **seo-sitemap** — sitemap creation and optimization, a Phase 1 foundation task
- **seo-images** — image optimization for e-commerce and visual businesses
- **seo-content** — executes the content pieces defined in the content calendar
- **seo-geo** — AI search optimization that integrates into Phase 3 and beyond
- **seo-local** — deep dive into local SEO tactics for service businesses
- **seo-technical** — handles the technical foundation work in Phase 1
- **seo-speed** — page performance optimization, part of the technical foundation
- **page-cro** — conversion optimization that ensures SEO traffic actually converts
