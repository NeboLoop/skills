---
name: seo-schema
description: Validate existing Schema.org structured data and generate new JSON-LD markup for rich search results like star ratings, FAQ dropdowns, breadcrumbs, and product pricing. Use when checking structured data for errors, adding schema to a page, troubleshooting missing rich snippets, or fixing Google Search Console structured data warnings.
---

# SEO Schema

Validate and generate Schema.org structured data (JSON-LD) so search engines understand your pages.

---

## When to Use

Trigger this skill when you hear:
- "Add schema to my page"
- "Check my structured data"
- "My site doesn't show rich snippets"
- "Generate JSON-LD for my page"
- "What schema does my page need?"
- "Help with structured data"
- "Why doesn't my page show stars/FAQ/breadcrumbs in Google?"
- Any mention of Schema.org, JSON-LD, rich results, or structured data

---

## Context Gathering

Before diving in, ask a few questions conversationally:

### The Page
1. **What page do you want to work on?** (Get the URL — you need to load it)
2. **What type of content is on this page?** (Product, article, local business, FAQ, recipe, event, person, service — helps prioritize which schemas matter)

### The Goal
3. **Are you trying to fix existing schema or add new schema?** (Determines whether you start with validation or generation)
4. **Have you seen any errors in Google Search Console?** (If yes, focus on fixing those first)
5. **What rich results do you want to appear in search?** (Stars, FAQ dropdowns, breadcrumbs, product pricing, event dates — sets expectations on what's achievable)

---

## Methodology

### Step 1: Load the Page

Use Nebo's browser to visit the page:
```
web(action: "navigate", url: "their-page-url.com")
web(action: "read_page")
```

Read the full HTML source. You need the raw markup, not just visible text.

### Step 2: Detect Existing Structured Data

Search the page source for all three formats:

**JSON-LD** — Look for `<script type="application/ld+json">` blocks. This is the preferred format.

**Microdata** — Look for `itemscope`, `itemtype`, and `itemprop` attributes in HTML elements.

**RDFa** — Look for `vocab`, `typeof`, and `property` attributes.

Report what you find:
- Which schema types are present (e.g., Organization, Product, Article)
- Which format they use (JSON-LD, Microdata, RDFa)
- Whether they're complete or missing required properties

If no structured data exists, say so clearly and move to Step 4.

### Step 3: Validate Existing Schema

For each schema block found, check against the Schema.org spec:

**Required properties** — Does each type have its mandatory fields?
- `Product` needs: name, description, image, offers (with price, priceCurrency, availability)
- `Article` needs: headline, image, datePublished, author
- `Organization` needs: name, url, logo
- `LocalBusiness` needs: name, address, telephone
- `FAQ` needs: mainEntity with Question/Answer pairs
- `BreadcrumbList` needs: itemListElement with position, name, item
- `Event` needs: name, startDate, location
- `Recipe` needs: name, image, author, datePublished, description

**Common errors to flag:**
- Missing `@context` (must be `https://schema.org`)
- Wrong `@type` for the content
- Invalid date formats (must be ISO 8601)
- Missing image URLs (Google requires images for most rich results)
- Price without currency
- Nested types without proper `@type` declarations
- Duplicate schema blocks that conflict
- URLs that are relative instead of absolute

**Present findings as a simple list:**
- What's correct (green light)
- What's missing (needs adding)
- What's wrong (needs fixing)

### Step 4: Identify Missing Schema Opportunities

Based on the page content, recommend which schema types should be added. Match content to schema:

| Page Content | Schema Type | Rich Result |
|---|---|---|
| Company/brand info | `Organization` | Knowledge panel, logo in search |
| Physical business with address | `LocalBusiness` | Map pack, business info panel |
| Product with pricing | `Product` | Price, availability, stars in search |
| Blog post or news | `Article` | Article carousel, publish date |
| FAQ section | `FAQPage` | Expandable Q&A in search results |
| How-to instructions | `HowTo` | Step-by-step in search results |
| Navigation breadcrumbs | `BreadcrumbList` | Breadcrumb trail in search results |
| Customer reviews | `AggregateRating` | Star rating in search results |
| Events with dates | `Event` | Event details in search results |
| Recipe content | `Recipe` | Recipe card in search results |
| Video content | `VideoObject` | Video thumbnail in search results |
| Job postings | `JobPosting` | Job listing in search results |
| Software/app | `SoftwareApplication` | App info in search results |
| Course content | `Course` | Course details in search results |

**Only recommend schemas that match actual page content.** Don't suggest Product schema for a blog post. Don't suggest FAQ schema if there's no FAQ section.

Tell the user which schemas are worth adding and why — what they'll see in search results.

### Step 5: Generate JSON-LD Code

For each recommended schema type, generate complete, valid JSON-LD that the user can paste into their page.

**Always use JSON-LD format.** Google recommends it. It goes in a `<script>` tag and doesn't touch the HTML structure.

**Rules for generating JSON-LD:**
- Always include `"@context": "https://schema.org"`
- Use the most specific `@type` available (e.g., `LocalBusiness` > `Organization`)
- Fill in real values from the page content — don't leave placeholders if the data is on the page
- Mark fields you couldn't fill with `"TODO: [what to add here]"` so the user knows what to complete
- Use absolute URLs for all links and images
- Use ISO 8601 for all dates
- Nest related types properly (e.g., `offers` inside `Product`, `address` inside `LocalBusiness`)
- If multiple schema types are needed, combine them in a single `<script>` block using `@graph`

**Example output for a product page:**
```json
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Wireless Bluetooth Headphones",
  "image": "https://example.com/photos/headphones.jpg",
  "description": "Noise-cancelling wireless headphones with 30-hour battery life.",
  "brand": {
    "@type": "Brand",
    "name": "AudioCo"
  },
  "offers": {
    "@type": "Offer",
    "price": "79.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock",
    "url": "https://example.com/headphones"
  },
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingValue": "4.5",
    "reviewCount": "127"
  }
}
</script>
```

**Example output for multiple schemas using @graph:**
```json
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "Organization",
      "name": "AudioCo",
      "url": "https://example.com",
      "logo": "https://example.com/logo.png"
    },
    {
      "@type": "BreadcrumbList",
      "itemListElement": [
        {
          "@type": "ListItem",
          "position": 1,
          "name": "Home",
          "item": "https://example.com"
        },
        {
          "@type": "ListItem",
          "position": 2,
          "name": "Headphones",
          "item": "https://example.com/headphones"
        }
      ]
    }
  ]
}
</script>
```

### Step 6: Explain the Impact

For each schema type generated, explain in plain language:

- **What it does** — how search engines use this data
- **What the user will see** — the specific rich result (stars, price, FAQ accordion, breadcrumbs, etc.)
- **How long it takes** — Google can take days to weeks to pick up new schema; it's not instant
- **No guarantees** — valid schema makes you eligible for rich results, but Google decides whether to show them

Tell the user where to paste the code (just before `</head>` or before `</body>`) and how to test it using Google's Rich Results Test (https://search.google.com/test/rich-results).

---

## Output Format

Present results in this structure:

```markdown
# Schema Audit: [Page URL]

## Existing Structured Data
[List what was found, or "None detected"]

## Validation Issues
[List of errors/warnings, or "No issues found"]

## Recommended Schema
[List of schema types to add, with one-line explanation of why]

## Generated JSON-LD
[Complete code blocks ready to paste]

## What This Means for Search
[Plain-language explanation of expected rich results]

## Next Steps
1. [Paste the code into your page]
2. [Test with Google Rich Results Test]
3. [Submit URL for re-indexing in Google Search Console]
```

---

## Quality Checks

Before delivering the schema code, verify:

- [ ] **Every JSON-LD block is valid JSON** — no trailing commas, no unquoted keys, proper nesting
- [ ] **`@context` is present** — set to `https://schema.org`
- [ ] **`@type` matches the content** — don't mark a blog post as a Product
- [ ] **Required properties are filled** — check Google's requirements for each type
- [ ] **URLs are absolute** — no relative paths like `/images/logo.png`
- [ ] **Dates are ISO 8601** — format `YYYY-MM-DD` or `YYYY-MM-DDTHH:MM:SS+00:00`
- [ ] **No placeholder text left untagged** — all TODOs are clearly marked
- [ ] **Schema matches visible page content** — structured data must reflect what users actually see (Google penalizes mismatches)
- [ ] **Recommendations are practical** — only suggest schemas the user can actually implement

---

## Examples

### Example 1: Product Page Missing Schema

**User says:** "Can you check the schema on my product page?"

**You do:**
1. Load the page with Nebo's browser
2. Find one JSON-LD block with Organization schema only
3. Report: "Your page has Organization schema, but no Product schema. For a product page, you're missing the most important one."
4. Ask: "I can see the product name, price, and reviews on your page. Want me to generate the Product schema?"
5. User: "Yes please"
6. Generate complete Product JSON-LD with data pulled from the page
7. Explain: "This makes you eligible for product rich results — the price, availability, and star rating that show directly in Google search results. Paste this before your closing `</head>` tag."
8. Provide testing link

**Generated code includes:**
```
Product schema with: name, image, description, brand, offers (price, currency, availability), aggregateRating
```

### Example 2: Local Business Needs Schema

**User says:** "My business doesn't show up well in Google. Can schema help?"

**You do:**
1. Ask for their website URL
2. Load the page — it's a local plumber's website with address, phone, hours
3. Find no structured data at all
4. Report: "Your site has no structured data. For a local business, this is a big missed opportunity."
5. Recommend: LocalBusiness, BreadcrumbList, and FAQ (they have a FAQ section)
6. Generate all three schema types using @graph
7. Explain: "LocalBusiness schema helps Google show your business info in the map pack and knowledge panel — your address, phone number, hours, and service area. The FAQ schema can give you expandable Q&A right in search results, which takes up more space and gets more clicks."

### Example 3: Blog with Existing Errors

**User says:** "Google Search Console says I have structured data errors."

**You do:**
1. Ask for the page URL and what errors they're seeing
2. Load the page
3. Find Article schema with issues: missing `image`, `datePublished` in wrong format, `author` is a string instead of a Person object
4. Report each error clearly:
   - "Missing `image` — Google requires at least one image for Article rich results"
   - "`datePublished` is `March 15, 2025` — needs to be `2025-03-15` (ISO 8601 format)"
   - "`author` is just a name string — needs to be a `Person` object with `name` and optionally `url`"
5. Generate the corrected Article JSON-LD
6. Explain what to replace and why each fix matters

---

## Related Skills

- **seo-audit** — full SEO audit including technical, content, and backlink factors
- **seo-technical** — crawlability, indexing, site speed, and technical SEO issues
- **schema-markup** — deeper schema implementation for complex multi-page sites
