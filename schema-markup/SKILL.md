---
name: schema-markup
description: Generate and implement JSON-LD structured data markup across your website for rich search results like star ratings, FAQ dropdowns, breadcrumbs, product prices, and event details. Use when adding schema to a site, wanting rich snippets in Google, implementing structured data for specific page types, or needing valid JSON-LD code ready to paste.
---

# Schema Markup

Implement structured data markup across your website so search engines understand your content and display rich results like star ratings, FAQs, prices, and event details.

---

## When to Use

Trigger this skill when you hear:
- "I want rich snippets in Google"
- "Add schema to my site"
- "How do I get star ratings in search results?"
- "Implement structured data"
- "My competitors have FAQ dropdowns in Google"
- "I need JSON-LD for my pages"
- "Help me with schema markup"
- "I want my products to show prices in search"
- "How do I get sitelinks or breadcrumbs in Google?"

This skill is **implementation-focused** — it generates the actual JSON-LD code for your pages and validates it. If you need an audit of existing schema issues, use the **seo-schema** skill instead.

---

## Context Gathering

Ask these questions conversationally to understand what schema the site needs.

### Site Basics
1. **What's your website URL?** (So we can see what pages exist and what schema is already in place)
2. **What type of site is this?** (Business website, e-commerce store, blog, SaaS, local business, portfolio, news site)
3. **What pages are most important for search visibility?** (Homepage, product pages, blog posts, service pages, location pages)

### Current State
4. **Do you have any schema markup already?** (Many sites have basic schema from WordPress plugins or themes without knowing it)
5. **Are you seeing any rich results in Google currently?** (Star ratings, FAQ dropdowns, breadcrumbs, site links)
6. **Has Google Search Console flagged any structured data issues?** (Errors or warnings in the Enhancements section)

### Business Details
7. **Do you have customer reviews or ratings?** (Needed for Review/AggregateRating schema)
8. **Do you sell products with prices?** (Needed for Product schema)
9. **Do you have physical locations?** (Needed for LocalBusiness schema)
10. **Do you publish articles or blog posts?** (Needed for Article schema)
11. **Do you have FAQs on your pages?** (Needed for FAQPage schema)
12. **Do you host or promote events?** (Needed for Event schema)

### Goals
13. **Which rich results do you most want to appear in search?** (Star ratings, FAQ dropdowns, price displays, breadcrumbs, how-to steps)
14. **Are there specific pages you want to prioritize?** (Start with highest-traffic or highest-value pages)

---

## Methodology

### Step 1: Check for Existing Context
Search memory for product marketing context and site architecture:
```
agent(resource: "memory", action: "search", query: "product marketing context")
agent(resource: "memory", action: "search", query: "site architecture")
```

If no product marketing context exists, run the **product-marketing-context** skill first. You need to understand the business to generate accurate schema.

### Step 2: Audit Current Schema
Browse the site to check what structured data already exists:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

Look for:
- Existing JSON-LD scripts in the page source
- Schema added by CMS plugins (WordPress, Shopify, etc.)
- Missing opportunities based on page content

Check multiple page types — homepage, a product/service page, a blog post, the contact page.

### Step 3: Identify Required Schema Types
Based on the site type and pages, determine which schema types to implement:

| Site Type | Essential Schema | Recommended Schema |
|---|---|---|
| Local Business | LocalBusiness, OpeningHours | FAQPage, Review, BreadcrumbList |
| E-commerce | Product, Offer, AggregateRating | BreadcrumbList, Organization, FAQPage |
| SaaS/Software | SoftwareApplication, Organization | FAQPage, HowTo, BreadcrumbList |
| Blog/Media | Article, Person, Organization | BreadcrumbList, FAQPage, VideoObject |
| Service Business | Service, Organization | FAQPage, Review, BreadcrumbList, HowTo |
| Events | Event, Organization | Offer, Performer, BreadcrumbList |
| Restaurants | Restaurant, Menu | Review, AggregateRating, OpeningHours |

Present this to the user: *"Based on your site, here are the schema types I recommend, in priority order."*

### Step 4: Generate JSON-LD Code
For each schema type needed, generate complete, valid JSON-LD code. Always use the JSON-LD format (not Microdata or RDFa — JSON-LD is what Google recommends).

**Key principles:**
- Use the most specific schema type available (Restaurant instead of LocalBusiness, SoftwareApplication instead of Product)
- Include all required properties for each type
- Add recommended properties that improve rich result chances
- Use real data from the user's site — never placeholder text
- Nest related schema (e.g., AggregateRating inside Product)

Walk the user through each code block: *"Here's the schema for your homepage. This goes in the `<head>` section of the page. Let me explain what each part does."*

### Step 5: Generate Page-Specific Schema
Create schema for each major page type:

**Homepage:**
- Organization or LocalBusiness schema
- WebSite schema with SearchAction (for sitelinks search box)

**Product/Service pages:**
- Product or Service schema with Offer
- AggregateRating if reviews exist
- FAQPage if the page has an FAQ section

**Blog posts:**
- Article or BlogPosting schema
- Person schema for the author
- BreadcrumbList for navigation path

**Contact/Location pages:**
- LocalBusiness with address, phone, hours
- GeoCoordinates for map placement

**FAQ pages:**
- FAQPage schema with all questions and answers

### Step 6: Validate the Markup
Guide the user through validation:

1. **Google Rich Results Test:** Test each page's schema at search.google.com/test/rich-results
2. **Schema.org Validator:** Validate the JSON-LD syntax at validator.schema.org
3. **Google Search Console:** After implementing, monitor the Enhancements section for errors

Common validation issues to watch for:
- Missing required fields (name, description, image for Products)
- Invalid date formats (must be ISO 8601: YYYY-MM-DD)
- URLs that don't match the actual page URL
- Rating values outside valid range (1-5)
- Prices without currency codes

Tell the user: *"After you add the code to your pages, test each one with Google's Rich Results Test to make sure it's valid. It takes a few weeks for Google to start showing rich results."*

### Step 7: Prioritize by Impact
Rank schema implementations by which will have the most visible impact in search:

1. **FAQPage** — Immediate visual impact with dropdown Q&As in search results
2. **Product/Service with ratings** — Star ratings dramatically increase click-through rates
3. **LocalBusiness** — Shows address, hours, and phone in local search results
4. **BreadcrumbList** — Improves how your URL appears in search results
5. **Article** — Helps blog content appear in Top Stories and news carousels
6. **HowTo** — Shows step-by-step instructions directly in search results
7. **Event** — Displays dates, locations, and ticket info in search
8. **Organization** — Feeds the Knowledge Panel on the right side of search results

### Step 8: Store the Plan
Save the schema implementation plan to memory:
```
agent(resource: "memory", action: "store", key: "seo/schema/types_needed", value: "List of schema types to implement", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/schema/priority_order", value: "Implementation priority", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/schema/current_state", value: "What schema already exists", layer: "tacit")
agent(resource: "memory", action: "store", key: "seo/schema/date", value: "YYYY-MM-DD", layer: "tacit")
```

Tell the user: *"I've saved your schema plan. When you add new page types in the future, we can generate the right schema for them."*

---

## Output Format

The schema markup deliverable follows this structure:

```markdown
# Schema Markup Plan
Last updated: [DATE]

## Current State
**Existing schema:** [What's already in place, if anything]
**Issues found:** [Errors or missing schema]

## Schema Types to Implement
| Priority | Schema Type | Page Type | Rich Result |
|---|---|---|---|
| 1 | [Type] | [Where it goes] | [What it shows in search] |
| 2 | [Type] | [Where it goes] | [What it shows in search] |

## Implementation Code

### [Page Type]: [Schema Type]
**Where to add:** [Which pages, where in the HTML]
**Rich result:** [What this enables in search]

```json
{
  "@context": "https://schema.org",
  "@type": "[Type]",
  ...
}
`` `

### [Next Page Type]: [Schema Type]
...

## Validation Checklist
- [ ] Tested with Google Rich Results Test
- [ ] No errors in schema validator
- [ ] All required fields populated with real data
- [ ] URLs match actual page URLs

## Next Steps
1. [First implementation step]
2. [Second step]
3. [Monitor in Search Console]
```

---

## Quality Checks

Before delivering schema markup code, verify:

- [ ] **JSON-LD format is used** — not Microdata or RDFa (Google recommends JSON-LD)
- [ ] **Most specific type is chosen** — Restaurant not LocalBusiness, BlogPosting not Article where appropriate
- [ ] **All required properties are included** — check schema.org for each type's requirements
- [ ] **Real data is used** — no placeholder text like "[Your Business Name]" left in the code
- [ ] **URLs are correct** — schema URLs match actual page URLs exactly
- [ ] **Ratings are valid** — values between 1-5, review count is a real number
- [ ] **No spammy markup** — don't mark up content that isn't visible on the page (Google penalizes this)
- [ ] **Schema matches page content** — Product schema only on pages that actually sell products

If any check fails, fix the code before delivering it.

---

## Examples

### Example 1: Local Business

**User says:** "I want my business hours and phone number to show up in Google search."

**You do:**
1. Check marketing context (found: dental practice in Austin, TX)
2. Browse site — no existing schema found
3. Identify needed schema: LocalBusiness (Dentist), FAQPage for the FAQ section, BreadcrumbList
4. Ask: "What are your business hours and phone number?"
5. User provides details
6. Generate JSON-LD:

```json
{
  "@context": "https://schema.org",
  "@type": "Dentist",
  "name": "Bright Smile Dental",
  "image": "https://brightsmile.com/images/office.jpg",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "123 Main St",
    "addressLocality": "Austin",
    "addressRegion": "TX",
    "postalCode": "78701"
  },
  "telephone": "+1-512-555-0100",
  "openingHoursSpecification": [
    {
      "@type": "OpeningHoursSpecification",
      "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
      "opens": "08:00",
      "closes": "17:00"
    }
  ],
  "url": "https://brightsmile.com",
  "priceRange": "$$"
}
```

7. Walk through where to place the code
8. Also generate FAQPage schema for their FAQ section
9. Guide through validation with Rich Results Test
10. Store plan to memory

### Example 2: E-commerce Product Pages

**User says:** "I want star ratings to show up next to my products in Google."

**You do:**
1. Check marketing context (found: online store selling handmade candles)
2. Browse site — Shopify store with basic Organization schema from theme, no Product schema
3. Ask: "Do your product pages have customer reviews?"
4. User: "Yes, we use Judge.me for reviews. Most products have 10-50 reviews."
5. Ask: "Can you show me a typical product page?"
6. Browse product page — find product name, price, description, reviews, images
7. Generate Product schema with AggregateRating:

```json
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Lavender Fields Soy Candle",
  "image": "https://example.com/images/lavender-candle.jpg",
  "description": "Hand-poured soy candle with natural lavender essential oil. 8oz, 50-hour burn time.",
  "brand": {
    "@type": "Brand",
    "name": "Glow & Co"
  },
  "offers": {
    "@type": "Offer",
    "price": "28.00",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock",
    "url": "https://example.com/products/lavender-fields-candle"
  },
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingValue": "4.8",
    "reviewCount": "42"
  }
}
```

8. Explain that Shopify may need an app or theme edit to add custom JSON-LD
9. Prioritize: Product schema on top 20 sellers first, then expand to all products
10. Guide validation and store plan

---

## Related Skills

- **product-marketing-context** — must exist before generating schema (provides business details, product info, and positioning)
- **seo-audit** — identifies existing schema issues and missing opportunities across the site
- **site-architecture** — provides the page hierarchy that informs BreadcrumbList schema
- **programmatic-seo** — scaled pages need schema templates applied automatically
- **page-cro** — rich results drive clicks, but the landing page must convert
- **local-seo** — LocalBusiness schema is critical for local search visibility
- **content-brief** — new content should include schema requirements from the start
- **seo-schema** — audit-focused companion skill that reviews existing schema for issues (this skill creates new schema)

**Always check product-marketing-context before starting.** Schema markup needs accurate business details — wrong information in structured data can hurt your search presence.
