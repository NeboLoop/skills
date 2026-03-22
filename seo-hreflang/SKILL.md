---
name: seo-hreflang
description: Audit and implement hreflang tags for international SEO so search engines serve the correct language and regional page versions to users. Use when managing a multilingual or multi-region site, fixing hreflang errors in Search Console, expanding to new countries, or users are landing on the wrong language version.
---

# SEO Hreflang

Audit and implement international SEO with hreflang tags so search engines serve the right language and regional version of your pages to the right users.

---

## When to Use

Trigger this skill when you hear:
- "We have a site in multiple languages"
- "Our international pages aren't ranking"
- "Users are landing on the wrong language version"
- "Hreflang tags" or "hreflang audit"
- "We're expanding to a new country"
- "Google is showing our French page to English users"
- "International SEO"
- "Multilingual website" or "multi-region site"
- "We need to add a new language to our site"

This skill covers hreflang implementation, auditing, and troubleshooting. If the user needs help with translation quality or content localization strategy, pair this with **copywriting**.

---

## Context Gathering

Ask these questions conversationally. International SEO has many edge cases, so get the full picture first.

### Current Setup
1. **What languages and regions does your site support?** (e.g., English US, English UK, French, Spanish Mexico — need both language and region)
2. **How are your URLs structured?** (Subdomains like fr.example.com, subdirectories like example.com/fr/, or separate domains like example.fr)
3. **Do you already have hreflang tags in place?** (If yes, where — HTML head, HTTP headers, or XML sitemap?)

### Content Coverage
4. **Is every page translated into every language?** (Or do some pages only exist in certain languages?)
5. **How are translations created?** (Human translators, machine translation, a mix?)
6. **Do you have regional content differences?** (e.g., different pricing for US vs UK, different product availability)

### Technical Details
7. **What CMS or platform are you on?** (WordPress with WPML, Shopify, custom, headless CMS)
8. **Who manages the technical implementation?** (Developer in-house, agency, the user themselves)
9. **Do you have access to Google Search Console?** (Needed to check for hreflang errors)

### Problems
10. **What's going wrong right now?** (Wrong language showing in search results, duplicate content issues, specific countries not ranking)

---

## Methodology

### Step 1: Check Marketing Context
Pull the user's product marketing context from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If none exists and this is a new engagement, run **product-marketing-context** first.

### Step 2: Map All Language/Region Versions
Create a complete map of every page and its language/region variants:

| Page | en-us | en-gb | fr-fr | es-mx | de-de |
|------|-------|-------|-------|-------|-------|
| Homepage | /  | /en-gb/ | /fr/ | /es-mx/ | /de/ |
| Pricing | /pricing | /en-gb/pricing | /fr/tarifs | /es-mx/precios | /de/preise |
| About | /about | /en-gb/about | /fr/a-propos | -- | /de/ueber-uns |

Mark missing translations with `--`. These gaps matter for hreflang implementation.

### Step 3: Validate Language and Region Codes
Every hreflang tag uses a specific format. Verify all codes are correct:

**Language codes (ISO 639-1):**
- `en` for English, `fr` for French, `es` for Spanish, `de` for German, `pt` for Portuguese, `ja` for Japanese, `zh` for Chinese, etc.
- Must be lowercase
- Must be a valid ISO 639-1 code (2 letters)

**Region codes (ISO 3166-1 Alpha-2) — optional but recommended:**
- `US` for United States, `GB` for Great Britain, `FR` for France, `MX` for Mexico, `BR` for Brazil, etc.
- Must be uppercase when used
- Combined format: `en-US`, `fr-FR`, `es-MX`, `pt-BR`

**Common mistakes to catch:**
- `en-UK` is wrong. It's `en-GB` (Great Britain, not United Kingdom)
- `zh-CN` for Simplified Chinese, `zh-TW` for Traditional Chinese
- Using country codes alone without language (e.g., `US` instead of `en-US`)
- Using 3-letter codes (e.g., `eng` instead of `en`)

### Step 4: Audit Existing Implementation
If hreflang tags already exist, check all three possible locations:

**Location 1: HTML `<head>` tags**
Use the browser to check each page:
```
web(action: "navigate", url: "example.com")
web(action: "read_page")
```
Look for: `<link rel="alternate" hreflang="en-us" href="https://example.com/" />`

**Location 2: HTTP Headers**
Check for `Link` headers in the HTTP response (used for non-HTML files like PDFs):
```
Link: <https://example.com/>; rel="alternate"; hreflang="en-us"
```

**Location 3: XML Sitemap**
Check the sitemap for hreflang entries:
```
web(action: "navigate", url: "example.com/sitemap.xml")
web(action: "read_page")
```
Look for `<xhtml:link rel="alternate" hreflang="en-us" href="..."/>` within `<url>` entries.

**Important:** Only use ONE method. Mixing methods (e.g., HTML tags AND sitemap entries) can cause conflicts.

### Step 5: Check Self-Referencing Tags
Every page must include a hreflang tag that points to itself. This is the most commonly missed requirement.

**Correct (English US page includes itself):**
```html
<link rel="alternate" hreflang="en-us" href="https://example.com/" />
<link rel="alternate" hreflang="fr-fr" href="https://example.com/fr/" />
<link rel="alternate" hreflang="es-mx" href="https://example.com/es-mx/" />
```

**Wrong (English US page is missing from its own hreflang set):**
```html
<link rel="alternate" hreflang="fr-fr" href="https://example.com/fr/" />
<link rel="alternate" hreflang="es-mx" href="https://example.com/es-mx/" />
```

### Step 6: Check Return Tags (Bidirectional Confirmation)
If Page A says "my French version is Page B," then Page B must say "my English version is Page A." This is called return tag validation.

For every hreflang relationship, verify both directions:
- English page points to French page? Check that French page points back to English page.
- Do this for EVERY language pair. A missing return tag tells Google to ignore the hreflang.

### Step 7: Handle the x-default Tag
The `x-default` tag tells search engines which page to show users who don't match any specific language/region. Every hreflang set should include one:

```html
<link rel="alternate" hreflang="x-default" href="https://example.com/" />
```

**Best practices for x-default:**
- Point it to your language selector page, or
- Point it to your most common language version (usually English), or
- Point it to a page that auto-detects language

### Step 8: Identify Missing Translations
For pages that don't exist in all languages:

- **Don't create a hreflang tag pointing to a non-existent page**
- **Don't point to the homepage as a fallback** (this confuses search engines)
- **Do use x-default** for the fallback behavior
- **Do plan which pages to translate next** based on search demand in each market

### Step 9: Generate the Implementation
Based on the audit, create the corrected hreflang tags. Deliver them in the format that matches their implementation method (HTML, HTTP headers, or sitemap).

---

## Output Format

Deliver the hreflang audit and implementation in this structure:

```markdown
# Hreflang Audit & Implementation
Date: [DATE]

## Language/Region Map
| Language | Code | URL Pattern | Status |
|----------|------|-------------|--------|
| English (US) | en-us | example.com/ | Active |
| French (France) | fr-fr | example.com/fr/ | Active |
| Spanish (Mexico) | es-mx | example.com/es-mx/ | Missing 5 pages |

## Implementation Method
**Method:** [HTML head tags / HTTP headers / XML sitemap]
**Reason:** [Why this method was chosen]

## Audit Findings

### Issues Found
1. **[Issue]** — [Description and which pages are affected]
2. **[Issue]** — [Description and which pages are affected]

### What's Working
- [Things that are correctly implemented]

## Corrected Hreflang Tags

### Homepage
[Full set of corrected hreflang tags for the homepage]

### [Page Name]
[Full set of corrected hreflang tags]

(Repeat for each page that needs changes)

## Missing Translations
| Page | Missing Languages | Priority | Search Volume |
|------|-------------------|----------|---------------|
| /pricing | de-de, ja-jp | High | 2,400/mo (DE) |

## Implementation Checklist
- [ ] [Step 1]
- [ ] [Step 2]
- [ ] Verify in Google Search Console after implementation
```

---

## Quality Checks

Before delivering the audit or implementation, verify:

- [ ] **All language codes are valid ISO 639-1** — two lowercase letters
- [ ] **All region codes are valid ISO 3166-1 Alpha-2** — two uppercase letters
- [ ] **Every page has a self-referencing hreflang tag** — points to itself
- [ ] **All return tags are present** — every relationship is bidirectional
- [ ] **x-default is set** — one page is designated as the default
- [ ] **Only one implementation method is used** — not mixing HTML and sitemap
- [ ] **No hreflang tags point to non-existent pages** — every URL actually resolves
- [ ] **URLs are absolute, not relative** — hreflang requires full URLs including https://

---

## Examples

### Example 1: New Hreflang Setup

**User says:** "We're launching our site in French and Spanish. We need hreflang tags."

**You do:**
1. Ask: "What regions are you targeting? French for France only, or also Canada? Spanish for Spain, Mexico, or Latin America?"
2. User: "French for France and Canada. Spanish for Mexico."
3. Ask: "How are your URLs structured?"
4. User: "Subdirectories. /fr/ for French France, /fr-ca/ for French Canada, /es-mx/ for Spanish Mexico."
5. Ask: "Is every page translated into every language?"
6. User: "Most pages, but our blog is English only."
7. Map all pages and their translations
8. Generate hreflang tags with proper codes: `fr-fr`, `fr-ca`, `es-mx`, `en-us`, `x-default`
9. Include self-referencing tags on every page
10. Note: Blog pages only get `en-us` and `x-default` (no other translations exist)

**Key output:**
```html
<!-- On the English homepage -->
<link rel="alternate" hreflang="en-us" href="https://example.com/" />
<link rel="alternate" hreflang="fr-fr" href="https://example.com/fr/" />
<link rel="alternate" hreflang="fr-ca" href="https://example.com/fr-ca/" />
<link rel="alternate" hreflang="es-mx" href="https://example.com/es-mx/" />
<link rel="alternate" hreflang="x-default" href="https://example.com/" />
```

### Example 2: Fixing Broken Hreflang

**User says:** "Google Search Console says we have hreflang errors."

**You do:**
1. Ask: "Can you describe the errors? Common ones are 'no return tag' or 'unknown language code.'"
2. User: "It says 'no return tag' on a bunch of pages."
3. Ask for the site URL and browse the pages
4. Find: The English pages point to French, but French pages don't point back to English
5. Explain: "Your French pages are missing the return tags. If English says 'my French version is X,' then French page X needs to say 'my English version is Y.' Both sides need to agree."
6. Generate corrected tags for all French pages
7. Also find: French pages use `en-UK` instead of `en-GB`
8. Fix the code error
9. Deliver corrected implementation with checklist

### Example 3: Regional Variants of Same Language

**User says:** "We have English pages for US, UK, and Australia. Do we need hreflang?"

**You do:**
1. Ask: "Is the content identical across all three, or are there differences?"
2. User: "Pricing is different, we use local currency, and some spelling differences."
3. Explain: "Yes, you need hreflang. Without it, Google might see these as duplicate content and pick one version for all English searchers."
4. Recommend: `en-us`, `en-gb`, `en-au` hreflang tags
5. Set `x-default` to the US version (or a language selector page)
6. Generate tags for all pages
7. Note: "Make sure each regional page has enough unique content beyond just currency — local testimonials, local compliance info, region-specific features."

---

## Related Skills

- **product-marketing-context** — understand the business before planning international SEO
- **seo-programmatic** — if creating programmatic pages in multiple languages (city pages per country)
- **copywriting** — for writing or reviewing translated content quality
- **seo-competitor-pages** — comparison pages may need international versions
- **ai-seo** — ensure AI search results show the right language version

**Always verify that translated content is genuinely localized, not just machine-translated.** Hreflang tags tell Google which version to show, but the content still needs to be high quality in each language.
