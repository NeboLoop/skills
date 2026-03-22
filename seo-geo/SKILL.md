---
name: seo-geo
description: Optimize content for AI-powered search engines like Google AI Overviews, ChatGPT, and Perplexity through Generative Engine Optimization (GEO). Use when your site doesn't appear in AI search results, you want to be cited by AI, need an llms.txt file, or want to audit AI crawler access and content citability.
---

# SEO GEO (Generative Engine Optimization)

Optimize your content for AI-powered search — Google AI Overviews, ChatGPT, Perplexity, and other AI engines that generate answers instead of listing links.

---

## When to Use

Trigger this skill when you hear:
- "How do I show up in AI search results?"
- "My site doesn't appear in Google AI Overviews"
- "Optimize for ChatGPT" or "Optimize for Perplexity"
- "What is GEO?"
- "AI is stealing my traffic"
- "How do I get cited by AI?"
- "Help with generative engine optimization"
- "Do I need an llms.txt file?"
- After an seo-audit reveals declining organic traffic that may be AI-related

This is the new frontier of SEO. Traditional SEO gets you ranked in link results. GEO gets your content cited in AI-generated answers — which is where a growing share of searches end up.

---

## Context Gathering

Ask these questions conversationally. This is a newer topic, so many users won't know what they need.

### The Basics
1. **What's your website URL?** (Need to check current AI crawler access and content structure)
2. **Have you noticed changes in your organic traffic recently?** (AI Overviews can reduce click-through from traditional results)
3. **Do you know if AI tools mention your brand?** (Try asking ChatGPT or Perplexity about their industry and see if they come up)

### Content Strategy
4. **What topics do you want to be known for?** (AI models cite authoritative sources on specific topics)
5. **Do you publish original research, data, or unique insights?** (AI models strongly prefer citing primary sources)
6. **How is your content structured?** (Long-form articles, short posts, product pages, documentation?)

### Technical Awareness
7. **Do you know what your robots.txt says?** (Many site owners don't realize they're blocking AI crawlers)
8. **Have you heard of llms.txt?** (Newer standard — most people haven't, and that's fine)

---

## Methodology

### Step 1: Check Product Marketing Context
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

Understand the business, their target audience, and what topics they want to be the authority on.

### Step 2: Check AI Crawler Access
Fetch the site's robots.txt:
```
web(action: "navigate", url: "their-site.com/robots.txt")
web(action: "read_page")
```

Check for these AI crawlers:
- **GPTBot** — OpenAI's crawler (powers ChatGPT search)
- **ClaudeBot** — Anthropic's crawler
- **PerplexityBot** — Perplexity's crawler
- **Google-Extended** — Google's AI training crawler (separate from Googlebot)
- **CCBot** — Common Crawl (feeds many AI training datasets)
- **Bytespider** — ByteDance's crawler

**What you want to see:**
- These crawlers are NOT blocked. If they're blocked, AI tools can't access the content to cite it.
- Many sites copied robots.txt rules that block AI crawlers during the 2023-2024 panic. If the user wants AI visibility, those blocks need to come down.

**Important nuance:** Blocking Google-Extended doesn't block Google AI Overviews — those use regular Googlebot. Google-Extended only controls AI training data usage. Explain this distinction clearly.

### Step 3: Assess Content Citability
AI models cite content that is structured, specific, and self-contained. Browse key pages:
```
web(action: "navigate", url: "their-site.com/key-page")
web(action: "read_page")
```

**Check for citable passages:**
- The ideal citable passage is 134-167 words — long enough to be substantive, short enough to be a clean quote
- Each passage should make a complete point without requiring context from the rest of the page
- Look for clear, factual statements that an AI would confidently attribute
- Statistics, original data points, and specific how-to steps are highly citable

**Content structure for AI citation:**
- Clear headings that match how people phrase questions
- Direct answers near the top (don't bury the answer after 500 words of introduction)
- Lists and structured data that AI can extract cleanly
- Definitions and explanations that stand alone as complete thoughts

**What makes content NOT citable:**
- Vague, generic statements anyone could write
- Content that requires reading the whole page to understand any single section
- Heavy jargon without explanation
- Opinions without supporting evidence or expertise signals

### Step 4: Check Brand Mention Signals
AI models learn about brands from multiple sources. Assess the brand's AI visibility:

**Search for the brand in AI tools:**
- Ask ChatGPT: "What is [brand name]?" and "What are the best [their category] tools/services?"
- Ask Perplexity the same questions
- Check if Google AI Overviews mention them for relevant queries

**Signals that improve brand mentions:**
- Consistent NAP (Name, Address, Phone) across directories (especially for local businesses)
- Mentions on authoritative third-party sites (press, reviews, industry publications)
- Structured data (Schema.org markup) on the website
- Wikipedia or Wikidata presence (if notable enough)
- Active presence in industry forums, communities, and social platforms
- Customer reviews on major platforms

### Step 5: Check llms.txt Compliance
llms.txt is an emerging standard that helps AI models understand your site. Check for it:
```
web(action: "navigate", url: "their-site.com/llms.txt")
web(action: "read_page")
```

**If it doesn't exist, recommend creating one.** An llms.txt file tells AI crawlers:
- What your site/product/service is about (in plain language)
- What content is available and how it's organized
- Key pages the AI should prioritize
- Contact and attribution preferences

**Basic llms.txt structure:**
```
# [Site Name]

> [One-paragraph description of what the site/business does]

## Docs
- [Title]: [URL]
- [Title]: [URL]

## Blog
- [Title]: [URL]

## Contact
- [Contact info or URL]
```

Also check for `llms-full.txt` — a more detailed version some sites provide.

### Step 6: Evaluate Content for AI Answer Optimization
For each key topic the business wants to own, check if their content is structured to appear in AI-generated answers:

**Question-answer format:**
- Does the content directly answer common questions in the first 1-2 sentences?
- Are headings phrased as questions people actually ask?
- Example: "How much does [service] cost?" as a heading with a direct answer immediately below

**Authority signals in content:**
- Author bylines with credentials
- Citations to primary sources
- "Last updated" dates showing freshness
- Experience-based content (first-hand knowledge, case studies, original data)

**Comparison and "best of" content:**
- If they want to appear when people ask "What's the best [category]?", they need comparison content that's balanced and informative
- AI models prefer sources that acknowledge alternatives rather than only promoting themselves

### Step 7: Review Structured Data
Check if the site uses Schema.org markup that helps AI understand content:
```
web(action: "navigate", url: "their-site.com")
web(action: "read_page")
```

Look for:
- `Organization` or `LocalBusiness` schema
- `Article` schema on blog posts with `author`, `datePublished`, `dateModified`
- `FAQPage` schema on FAQ sections
- `HowTo` schema on tutorial content
- `Product` schema on product pages with reviews and pricing
- `BreadcrumbList` for site structure

### Step 8: Compile Recommendations
Prioritize by impact:
1. **Critical** — AI crawlers are blocked, no citable content structure, brand is invisible to AI tools
2. **Important** — No llms.txt, weak authority signals, content not optimized for question-answer format
3. **Nice to have** — Schema markup improvements, llms-full.txt, comparison content creation

---

## Output Format

```markdown
# GEO Audit (Generative Engine Optimization)
**Site:** [URL]
**Date:** [DATE]

## Summary
- **AI crawler access:** Allowed / Partially blocked / Fully blocked
- **Brand visibility in AI tools:** Strong / Moderate / Weak / Not found
- **Content citability:** High / Medium / Low
- **llms.txt present:** Yes / No
- **Structured data:** Present / Partial / Missing

## AI Crawler Access
| Crawler | Status | Notes |
|---------|--------|-------|
| GPTBot | Allowed/Blocked | [details] |
| ClaudeBot | Allowed/Blocked | [details] |
| PerplexityBot | Allowed/Blocked | [details] |
| Google-Extended | Allowed/Blocked | [details] |

### Recommended robots.txt changes:
[Specific lines to add or remove]

## Content Citability
[Analysis of how well content is structured for AI citation]

### Strong pages:
[Pages with good citable structure and why]

### Pages needing improvement:
[Pages that need restructuring with specific suggestions]

### Citable passage examples:
[Show before/after examples of how to restructure content]

## Brand Visibility
[Results from checking AI tools]

### ChatGPT mentions: [Yes/No — with details]
### Perplexity mentions: [Yes/No — with details]
### Google AI Overviews: [Yes/No — with details]

### How to improve brand signals:
[Specific recommendations]

## llms.txt
[Whether it exists, and if not, a recommended version]

## Structured Data
[What's present and what's missing]

## Next Steps
1. [Highest priority action]
2. [Second priority]
3. [Third priority]
```

---

## Quality Checks

Before delivering your audit, verify:

- [ ] **Actually tested AI tools** — don't guess whether the brand appears; check ChatGPT and Perplexity
- [ ] **robots.txt analysis is accurate** — distinguish between blocking training vs. blocking search features
- [ ] **Citable passage advice is specific** — don't just say "write better content"; show exact examples
- [ ] **Word count guidance is practical** — 134-167 words is a target range, not a rigid rule
- [ ] **llms.txt recommendation follows the actual standard** — don't invent a format
- [ ] **Structured data suggestions match the content type** — don't recommend Product schema on a blog
- [ ] **Google-Extended vs. Googlebot distinction is explained** — this confuses almost everyone
- [ ] **Recommendations are actionable for a non-technical user** — no jargon without explanation

---

## Examples

### Example 1: SaaS Company Not Appearing in AI Answers

**User says:** "People ask ChatGPT for project management tools and we never come up. What can I do?"

**You do:**
1. Check marketing context — learn they're a project management SaaS for creative teams
2. Check robots.txt — find `Disallow: /` for GPTBot and ClaudeBot (blocking everything)
3. Check their content — mostly feature-focused marketing copy, not informational content
4. Search ChatGPT for "best project management tools for creative teams" — they don't appear
5. Check for llms.txt — doesn't exist

**Key recommendations:**
- Remove GPTBot and ClaudeBot blocks from robots.txt immediately
- Create an llms.txt file describing their product and key pages
- Write comparison content: "Project Management for Creative Teams: How to Choose"
- Restructure product pages with citable passages: "Unlike general project management tools, [Product] includes built-in creative review workflows that reduce revision cycles by 40%." (That's a passage AI can cite.)
- Add Organization and SoftwareApplication schema
- Get listed on G2, Capterra, and industry roundup posts (third-party mentions improve AI brand awareness)

### Example 2: Local Business Wanting AI Visibility

**User says:** "When I ask Perplexity for plumbers in my area, my competitors show up but I don't."

**You do:**
1. Check context — local plumbing company in Austin, TX
2. Check robots.txt — no AI crawler blocks (good)
3. Check their site — thin content, just a homepage and contact page
4. Ask Perplexity "best plumbers in Austin TX" — competitors with strong review profiles appear
5. Check Google Business Profile, Yelp, Angi — minimal presence

**Key recommendations:**
- Build out service pages with citable content: "Emergency drain cleaning in Austin typically costs $150-300 depending on the blockage location and severity." (Specific, factual, citable.)
- Create FAQ pages answering common plumbing questions for Austin residents
- Build up reviews on Google, Yelp, and Angi — AI tools pull from these
- Ensure consistent NAP across all directories
- Add LocalBusiness schema with service area, hours, and reviews
- Create content about Austin-specific plumbing issues (hard water, older pipe materials in certain neighborhoods)

---

## Related Skills

- **seo-audit** — comprehensive SEO check that covers traditional ranking factors alongside AI visibility
- **seo-plan** — strategic roadmap that incorporates GEO into the overall SEO strategy
- **seo-sitemap** — ensures AI crawlers can discover and access all important content
- **seo-content** — creates content optimized for both traditional and AI search
- **seo-technical** — structured data and technical foundations that support AI discoverability
- **product-marketing-context** — defines the positioning and topics the brand wants to own in AI results
- **seo-local** — local SEO signals that also drive local AI search visibility
