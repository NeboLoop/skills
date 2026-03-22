---
name: programmatic-seo
description: Plan and execute scaled content strategies using templates and data sources to publish hundreds of targeted pages like location pages, category pages, and comparison pages. Use when creating pages at scale, building template-based SEO content, ranking for long-tail keywords in bulk, or planning data-driven content generation with quality gates.
---

# Programmatic SEO

Plan and execute scaled content strategies using templates and data to publish hundreds or thousands of targeted pages.

---

## When to Use

Trigger this skill when you hear:
- "I want to create pages at scale"
- "How do I build programmatic content?"
- "I need landing pages for every city/category/product"
- "Can I automate content creation?"
- "Template-based SEO pages"
- "I want to rank for hundreds of long-tail keywords"
- "Scaled content strategy"
- "How do competitors have so many pages?"

This skill focuses on **strategy and planning** — deciding what to build, how to template it, and where the data comes from. It does not handle technical implementation or coding.

---

## Context Gathering

Ask these questions conversationally. Don't dump them all at once — have a real conversation.

### Business Foundation
1. **What does your business do?** (One sentence — what you sell or offer)
2. **What pages do you already have on your site?** (Get a sense of current content)
3. **What's your website URL?** (So we can look at what exists today)

### Opportunity Identification
4. **What repeatable patterns exist in your business?** (Locations you serve, products you sell, categories, comparisons, integrations, use cases)
5. **What do your customers search for?** (Common questions, "best X in Y" searches, comparison queries)
6. **Are there competitors doing programmatic content well?** (Sites with thousands of similar-looking pages)

### Data Availability
7. **What data do you already have?** (Product database, location list, customer questions, pricing data, feature lists)
8. **What data could you get?** (Public datasets, APIs, industry data, user-generated content)
9. **How often does this data change?** (Static like city names, or dynamic like prices)

### Quality Standards
10. **What would make these pages genuinely useful?** (Not just keyword-stuffed — what value would a visitor get?)
11. **Do you have subject matter expertise to add unique angles?** (Original data, proprietary insights, real experience)

### Goals
12. **What keywords or search patterns are you targeting?** (The query templates like "[service] in [city]" or "[product A] vs [product B]")
13. **What does success look like?** (Traffic numbers, leads, revenue from these pages)
14. **What's your timeline?** (When do you want pages live?)

---

## Methodology

### Step 1: Check for Existing Context
Search memory for product marketing context and any previous programmatic SEO work:
```
agent(resource: "memory", action: "search", query: "product marketing context")
agent(resource: "memory", action: "search", query: "programmatic seo")
```

If no product marketing context exists, run the **product-marketing-context** skill first.

If the user provides a website, browse it to understand current structure:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

### Step 2: Identify Content Patterns
Help the user discover repeatable patterns in their business. Look for:

- **Location pages:** "[Service] in [City]" — works for local businesses, agencies, service providers
- **Category pages:** "Best [Product Type] for [Use Case]" — works for marketplaces, review sites, e-commerce
- **Comparison pages:** "[Product A] vs [Product B]" — works for SaaS, tools, services
- **Integration pages:** "[Product] + [Integration]" — works for software companies
- **Question pages:** "How to [Task] with [Product]" — works for any product with multiple use cases
- **Stats/data pages:** "[Industry] statistics in [Year]" — works when you have access to data

Ask: *"Based on your business, which of these patterns could work for you? Let's pick one or two to start."*

### Step 3: Design the Template
For each content pattern, define:

**Page title formula:**
- Example: "[Service Name] in [City], [State] | [Brand]"

**Required sections on every page:**
- What sections must appear on every page (intro, features, pricing, FAQ, CTA)
- What content is unique per page vs shared across all pages
- What dynamic data slots exist (city name, population, local stats, pricing)

**Unique value per page:**
- What makes each individual page worth visiting (not just the city name swapped in)
- Local data, specific examples, custom recommendations, real reviews
- The "so what" test: would someone searching this term find this page useful?

### Step 4: Plan Data Sources
For each dynamic element in the template, identify where the data comes from:

| Data Element | Source | Update Frequency | Format |
|---|---|---|---|
| City names | Census data | Rarely | CSV |
| Local stats | Government APIs | Annually | API |
| Pricing | Internal database | Monthly | Database |
| Reviews | Customer feedback | Ongoing | CRM |

Ask: *"Do you have access to this data already, or do we need to find sources?"*

### Step 5: Set Quality Gates
Define minimum quality standards before any page goes live:

- **Unique content threshold:** At least X% of each page must be unique (not just template boilerplate)
- **Word count minimum:** Each page should have at least [N] words of substantive content
- **Data completeness:** All dynamic fields must be populated — no blank spots or placeholder text
- **Internal linking:** Each page links to at least [N] related pages
- **User value test:** Would you be embarrassed if a customer saw this page?

### Step 6: Plan Internal Linking
Design how programmatic pages connect to each other and to existing content:

- **Hub pages:** Category or index pages that link to all child pages (e.g., "All Cities We Serve")
- **Cross-links:** Related pages linking to each other (e.g., nearby cities, similar categories)
- **Upward links:** Every programmatic page links back to main service/product pages
- **Sitemaps:** Plan for XML sitemaps that include all programmatic URLs

### Step 7: Plan the Rollout
Recommend a phased approach:

1. **Pilot batch (10-20 pages):** Test the template with a small set, measure indexing and traffic
2. **Quality review:** Check pilot pages for thin content, crawl errors, duplicate content issues
3. **Scale batch (100-500 pages):** Expand to more variations if pilot performs well
4. **Full rollout:** Complete the full set with monitoring in place
5. **Ongoing maintenance:** Plan for updating data, refreshing content, adding new pages

### Step 8: Store the Strategy
Save the programmatic SEO plan to memory:
```
agent(resource: "memory", action: "store", key: "seo/programmatic/patterns", value: "Content patterns identified", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/programmatic/template", value: "Template structure", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/programmatic/data_sources", value: "Data source plan", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/programmatic/quality_gates", value: "Quality standards", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/programmatic/rollout_plan", value: "Phased rollout plan", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/programmatic/date", value: "YYYY-MM-DD", layer: "tacit")
```

---

## Output Format

The programmatic SEO strategy document follows this structure:

```markdown
# Programmatic SEO Strategy
Last updated: [DATE]

## Content Patterns
**Pattern 1:** [Query template] — [Number of potential pages]
**Pattern 2:** [Query template] — [Number of potential pages]

## Page Template: [Pattern Name]
**Title formula:** [Title template with variables]
**URL structure:** /[path]/[variable-slug]

### Page Sections
1. [Section name] — [Dynamic or static] — [Data source]
2. [Section name] — [Dynamic or static] — [Data source]
3. [Section name] — [Dynamic or static] — [Data source]

### Unique Value
[What makes each page individually useful]

## Data Sources
| Data Element | Source | Format | Update Frequency |
|---|---|---|---|
| [Element] | [Source] | [Format] | [Frequency] |

## Quality Standards
- Unique content: [Threshold]
- Minimum word count: [Number]
- Required data fields: [List]
- Internal links per page: [Number]

## Internal Linking Plan
- Hub pages: [List]
- Cross-linking rules: [Description]
- Sitemap approach: [Plan]

## Rollout Plan
- Phase 1 (Pilot): [Number] pages by [Date]
- Phase 2 (Scale): [Number] pages by [Date]
- Phase 3 (Full): [Number] pages by [Date]
- Maintenance: [Ongoing plan]

## Success Metrics
- Target traffic per page: [Number]
- Total traffic goal: [Number]
- Conversion goal: [Number or percentage]
```

---

## Quality Checks

Before finalizing the strategy, verify:

- [ ] **Content pattern has real search volume** — people actually search for these queries
- [ ] **Each page provides genuine value** — not just keyword stuffing with swapped variables
- [ ] **Data sources are identified and accessible** — no vague "we'll figure it out later"
- [ ] **Unique content threshold is realistic** — at least 30-40% unique content per page
- [ ] **Quality gates are defined** — clear standards before pages go live
- [ ] **Internal linking plan exists** — pages connect to each other and to the rest of the site
- [ ] **Rollout is phased** — never launch thousands of pages at once
- [ ] **The "embarrassment test" passes** — you wouldn't be ashamed to show these pages to a customer

If any check fails, revise the strategy before moving forward.

---

## Examples

### Example 1: Local Service Business

**User says:** "I run a plumbing company in Texas. I want to rank in every city we serve."

**You do:**
1. Check for marketing context (found: plumbing company, serves Dallas-Fort Worth metro)
2. Ask: "How many cities do you serve?"
3. User: "About 45 cities in the DFW area."
4. Ask: "What services do you offer in each city?"
5. User: "Emergency plumbing, drain cleaning, water heater repair, and repiping."
6. Identify pattern: "[Service] in [City], TX" = 45 cities x 4 services = 180 pages
7. Design template with sections: service description, local pricing ranges, response times for that city, customer reviews from that area, related services
8. Data sources: city names (known), pricing (internal), reviews (Google Business Profile), response times (dispatch data)
9. Quality gate: each page needs at least 2 local reviews and city-specific pricing
10. Rollout: 20 pages first (top 5 cities x 4 services), monitor for 30 days, then expand

**Strategy output includes:**
```
Pattern: [Service] in [City], TX — 180 pages
Unique value: Real pricing ranges, actual customer reviews from each city, verified response times
Pilot: 20 pages (5 highest-population cities x 4 services)
```

### Example 2: SaaS Comparison Pages

**User says:** "Our competitors all have comparison pages. We need those too."

**You do:**
1. Check marketing context (found: project management SaaS)
2. Ask: "Who are your top competitors?"
3. User: "Asana, Monday, ClickUp, Basecamp, Notion, Trello, Jira."
4. Ask: "Do you have feature comparison data for each?"
5. User: "Yes, we track their features and pricing quarterly."
6. Identify pattern: "[Our Product] vs [Competitor]" = 7 pages + "[Competitor A] vs [Competitor B]" for additional traffic = 21 more pages
7. Design template: feature-by-feature comparison table, pricing comparison, pros/cons of each, migration guide, real user quotes
8. Quality gate: every comparison must be factually accurate and updated quarterly
9. Unique value: original screenshots, honest pros/cons (not just "we're better at everything"), migration difficulty ratings
10. Rollout: 7 direct competitor pages first, then competitor-vs-competitor pages

**Strategy output includes:**
```
Pattern 1: [Our Product] vs [Competitor] — 7 pages
Pattern 2: [Competitor A] vs [Competitor B] — 21 pages
Unique value: Honest feature comparisons updated quarterly, migration guides, real user feedback
Quality gate: Fact-check all claims, update pricing/features every 90 days
```

---

## Related Skills

- **product-marketing-context** — must exist before planning programmatic content (provides positioning, customer profile, and voice)
- **keyword-research** — identifies which query patterns have real search volume
- **site-architecture** — plans how programmatic pages fit into overall site structure and URL hierarchy
- **content-brief** — creates detailed briefs for the unique content portions of each template
- **seo-audit** — checks programmatic pages for technical SEO issues after launch
- **schema-markup** — adds structured data to programmatic pages for rich results
- **page-cro** — optimizes programmatic pages for conversions, not just traffic
- **internal-linking** — designs the cross-linking strategy between programmatic pages

**Always check product-marketing-context before starting.** Programmatic content without clear positioning creates hundreds of mediocre pages instead of hundreds of valuable ones.
