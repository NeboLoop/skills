---
name: ai-seo
description: Optimize content so your brand appears in AI-powered search results, LLM citations, and AI Overviews from ChatGPT, Perplexity, and Google. Use when your brand is missing from AI search answers, competitors dominate AI results, you need an llms.txt file, or you want to build brand presence in AI training data sources.
---

# AI SEO

Optimize content so your brand shows up in AI-powered search results, LLM citations, and AI Overviews — not just traditional blue links.

---

## When to Use

Trigger this skill when you hear:
- "How do I show up in ChatGPT results?"
- "AI Overviews are stealing our traffic"
- "We don't appear in AI search"
- "LLM optimization" or "GEO" (generative engine optimization)
- "How do I get cited by AI?"
- "Perplexity doesn't mention us"
- "Google AI Overview doesn't include us"
- "What's llms.txt?"
- "AI search optimization"
- "We need to be in AI training data"

This skill is different from traditional SEO. Traditional SEO gets you ranked in search results. AI SEO gets your brand mentioned, cited, and recommended by AI systems when people ask questions.

---

## Context Gathering

Ask these questions conversationally. AI SEO is new territory for most people, so explain concepts as you go.

### Current Visibility
1. **Have you checked if AI tools mention your brand?** (Search for your product name in ChatGPT, Perplexity, Google AI Overview)
2. **What do AI tools say about your category?** (e.g., "What are the best project management tools?" — are you in that answer?)
3. **Do AI tools recommend your competitors?** (If yes, which ones and in what context?)

### Content Foundation
4. **What content do you currently have?** (Blog posts, documentation, guides, case studies, videos)
5. **Do you have authoritative, factual content that answers common questions in your space?** (AI loves clear, definitive answers)
6. **Is your content structured with clear headings and direct answers?** (Or is it buried in long narrative text?)

### Brand Presence
7. **Does your brand appear on Wikipedia?** (Even a mention on a related page helps)
8. **Are you discussed on Reddit, Quora, or forums?** (These are major LLM training data sources)
9. **Do you have a YouTube presence?** (YouTube transcripts are a significant AI training signal)
10. **Are you mentioned in industry publications or review sites?** (G2, Capterra, Product Hunt, industry blogs)

### Technical
11. **What platform is your site on?** (Determines what technical optimizations are possible)
12. **Do you have a llms.txt file?** (Most sites don't yet — this is an opportunity)
13. **Is your site's structured data (schema markup) up to date?** (AI systems use this heavily)

---

## Methodology

### Step 1: Check Marketing Context
Pull the user's product marketing context from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If none exists, run **product-marketing-context** first. You need to know the product, audience, and positioning before optimizing for AI.

### Step 2: Audit Current AI Visibility
Check how AI systems currently perceive the brand. Use the browser to test:

```
web(action: "navigate", url: "https://www.perplexity.ai")
web(action: "search", query: "best [category] tools")
```

Test these queries across AI tools (where accessible):
- "What is [product name]?"
- "Best [category] tools"
- "[Product] vs [competitor]"
- "What are alternatives to [competitor]?"
- Questions that the product answers

Document:
- **Mentioned:** Yes/No for each query
- **Accuracy:** Is the information correct and current?
- **Positioning:** How is the brand described? Favorably? Neutrally? Missing entirely?
- **Competitors mentioned:** Who shows up instead?

### Step 3: Structure Content for AI Extraction
AI systems extract information from content differently than humans read it. Optimize for extraction:

**Clear, direct answers at the top:**
- Start sections with a direct answer to the implied question
- Use the "inverted pyramid" — most important information first
- Avoid burying answers below lengthy introductions

**Factual, citation-worthy statements:**
- Include specific numbers, dates, and data points
- Make claims that are easy to quote: "X does Y, which reduces Z by N%"
- Cite sources for statistics — AI systems trust sourced claims more

**Structured formatting:**
- Use descriptive H2/H3 headings that match how people ask questions
- Use bullet points and numbered lists for multi-part answers
- Use definition-style formatting: **Term:** Definition
- Include comparison tables where relevant

**Entity clarity:**
- Clearly state what your product IS in the first paragraph of key pages
- Include category associations: "[Product] is a [category] tool that..."
- Mention related entities (integrations, platforms, use cases) to build entity connections

### Step 4: Optimize for Featured Snippets and AI Overviews
Google AI Overviews pull from the same content that wins featured snippets. Optimize for both:

**Paragraph snippets (for "what is" queries):**
- Write a 40-60 word definition paragraph directly under a question-style heading
- Start with "[Topic] is..." or "[Product] is..."

**List snippets (for "how to" or "best" queries):**
- Use numbered or bulleted lists with clear items
- Include 5-8 items (Google typically shows 4-8)
- Each item should be a complete thought, not just a word

**Table snippets (for comparison queries):**
- Use HTML tables with clear headers
- Include 3+ rows of data
- Make sure the table answers a specific question

### Step 5: Build Brand Presence in Training Data Sources
AI models learn about brands from their training data. Strengthen your presence in key sources:

**Wikipedia:**
- Don't create your own Wikipedia page (it'll get deleted)
- Look for existing pages in your category where your product could be mentioned as an example
- Contribute to category pages with cited, neutral information
- If your company has notable achievements (funding, awards, user milestones), these make Wikipedia mentions more likely

**Reddit:**
- Participate genuinely in subreddits related to your category
- Answer questions where your product is a legitimate solution (disclose affiliation)
- Don't spam or self-promote — Reddit communities will flag this
- Create an AMA if your founder or team has genuine expertise to share

**YouTube:**
- Create product demos, tutorials, and comparison videos
- YouTube transcripts are a major training data source for AI models
- Optimize video titles and descriptions with category keywords
- Include clear product descriptions in the first 30 seconds (for transcript extraction)

**Review sites:**
- Maintain updated profiles on G2, Capterra, Product Hunt, TrustRadius
- Encourage genuine reviews — volume and recency both matter
- Respond to reviews (shows active engagement)

**Industry publications:**
- Guest posts on industry blogs with your product mentioned naturally
- Contribute expert quotes to journalists (HARO, Quoted, Featured)
- Publish original research that gets cited by others

### Step 6: Create an llms.txt File
The `llms.txt` file is an emerging standard that helps AI systems understand your site. It's like `robots.txt` but for LLMs.

**Create `/llms.txt` at your site root:**
```
# [Company Name]

> [One-sentence description of what you do]

## About
[2-3 paragraph description of the company, product, and what makes you different]

## Key Facts
- Founded: [year]
- Category: [product category]
- Customers: [number or type]
- Pricing: [model and starting price]

## Products/Services
- [Product 1]: [description]
- [Product 2]: [description]

## Links
- Documentation: [URL]
- Pricing: [URL]
- Blog: [URL]
- API: [URL]

## Contact
- Website: [URL]
- Support: [email/URL]
```

**Also create `/llms-full.txt`** with more comprehensive information:
- Full product descriptions
- Feature lists
- Use cases
- Integration details
- FAQ content

### Step 7: Implement Structured Data
Add or update schema markup to help AI systems understand your content:

**Organization schema:**
```json
{
  "@type": "Organization",
  "name": "Company Name",
  "description": "What the company does",
  "url": "https://example.com",
  "foundingDate": "2020",
  "sameAs": [
    "https://twitter.com/company",
    "https://linkedin.com/company/name",
    "https://youtube.com/@company"
  ]
}
```

**Product schema:**
```json
{
  "@type": "SoftwareApplication",
  "name": "Product Name",
  "applicationCategory": "Category",
  "offers": {
    "@type": "Offer",
    "price": "29",
    "priceCurrency": "USD"
  }
}
```

**FAQ schema on key pages:**
```json
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What does [Product] do?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Clear, citation-worthy answer"
      }
    }
  ]
}
```

### Step 8: Monitor and Iterate
AI SEO is a long game. Set up ongoing monitoring:

- **Monthly:** Test key queries in ChatGPT, Perplexity, and Google AI Overview
- **Track:** Which queries mention you, which don't, and what changed
- **Update:** Refresh content when AI systems have outdated information about you
- **Respond:** When competitors get mentioned and you don't, analyze why their content was chosen
- **Watch:** Industry developments in AI search — this space changes fast

---

## Output Format

Deliver the AI SEO strategy in this structure:

```markdown
# AI SEO Strategy
Date: [DATE]

## Current AI Visibility Audit
| Query | ChatGPT | Perplexity | Google AI Overview |
|-------|---------|------------|-------------------|
| "What is [product]?" | [result] | [result] | [result] |
| "Best [category] tools" | [result] | [result] | [result] |
| "[Product] vs [competitor]" | [result] | [result] | [result] |

## Content Optimization Plan

### Pages to Optimize
1. **[Page URL]** — [What to change and why]
2. **[Page URL]** — [What to change and why]

### New Content to Create
1. **[Topic]** — [Why this will help AI visibility]
2. **[Topic]** — [Why this will help AI visibility]

### Content Structure Improvements
- [Specific formatting changes across the site]

## Brand Presence Building

### Current Presence
| Platform | Status | Action Needed |
|----------|--------|---------------|
| Wikipedia | [status] | [action] |
| Reddit | [status] | [action] |
| YouTube | [status] | [action] |
| G2/Capterra | [status] | [action] |
| Industry blogs | [status] | [action] |

### Priority Actions (Next 30 Days)
1. [Action — specific and doable]
2. [Action — specific and doable]
3. [Action — specific and doable]

## Technical Implementation

### llms.txt
[Full content for the llms.txt file]

### llms-full.txt
[Full content for the llms-full.txt file]

### Schema Markup Updates
[Specific schema to add or update, with JSON-LD]

## Monitoring Plan
- Monthly check queries: [list]
- Tools to use: [list]
- Success metrics: [what "working" looks like]
```

---

## Quality Checks

Before delivering the strategy, verify:

- [ ] **AI visibility audit is current** — tested queries in at least 2 AI systems today
- [ ] **Content recommendations are specific** — not vague ("improve content"), but actionable ("add a 50-word definition paragraph under the H1 on /features")
- [ ] **Brand presence actions are realistic** — not "get on Wikipedia tomorrow" but achievable steps
- [ ] **llms.txt content is accurate and complete** — matches current product and positioning
- [ ] **Schema markup is valid JSON-LD** — no syntax errors, uses correct types
- [ ] **Strategy doesn't rely on gaming AI systems** — focuses on genuine authority and helpful content
- [ ] **Monitoring plan is practical** — user can actually do this monthly
- [ ] **Recommendations prioritize high-impact actions first** — not a list of 50 things, but the top 5-10 that matter most

---

## Examples

### Example 1: Not Showing Up in AI Results

**User says:** "When I ask ChatGPT about project management tools, it mentions Asana, Monday, and Notion but never us."

**You do:**
1. Check marketing context (found: project management tool for creative agencies)
2. Test queries in ChatGPT, Perplexity, Google AI Overview: "best project management tools," "project management for agencies," "best tools for creative teams"
3. Find: User's product doesn't appear in any AI results for broad queries. Appears in Perplexity for "[product name]" direct search but with outdated info.
4. Diagnose: Limited online presence — no Wikipedia mention, minimal Reddit discussion, no YouTube content, 12 G2 reviews vs. competitors with 1,000+
5. Recommend:
   - Create an llms.txt file with clear product positioning
   - Restructure the homepage to lead with a citation-worthy definition
   - Start a YouTube channel with weekly tutorials (transcripts = training data)
   - Launch a Reddit presence in r/projectmanagement and r/agencies
   - Increase G2 reviews to 50+ over next 3 months
   - Write 5 definitive guide posts answering common agency PM questions
6. Deliver strategy with 30-day priority actions

### Example 2: AI Has Wrong Information

**User says:** "Perplexity says we're a freemium tool but we switched to paid-only 6 months ago."

**You do:**
1. Verify the issue by checking Perplexity
2. Find the source: Old blog post still says "free plan available," pricing page was updated but not the marketing pages
3. Recommend:
   - Update ALL pages that mention pricing (not just the pricing page)
   - Update llms.txt (or create one) with current pricing
   - Update G2, Capterra, and Product Hunt profiles with current pricing
   - Publish a blog post about the pricing change (creates a clear signal)
   - Update schema markup with current pricing
4. Explain: "AI models pull from multiple sources. If 5 pages say 'free' and 1 says 'paid,' the AI will likely say 'free.' You need consistent information everywhere."

### Example 3: Competitor Dominates AI Results

**User says:** "Every time someone asks an AI about our category, our main competitor gets mentioned first."

**You do:**
1. Audit: Test 10 category-related queries across AI platforms
2. Find: Competitor appears in 9/10 queries, user appears in 2/10
3. Analyze why: Competitor has 400+ YouTube videos (transcripts), active Reddit presence with 50+ upvoted recommendations, Wikipedia mention in the category page, 2,000 G2 reviews, and comprehensive structured data
4. Build a realistic plan: "You can't match their presence overnight, but here's a 6-month plan to close the gap"
5. Priority actions:
   - Month 1: llms.txt, schema markup, restructure top 5 pages for AI extraction
   - Month 2: Launch YouTube with 8 tutorial videos, start Reddit engagement
   - Month 3: Publish 3 definitive guides that directly answer common AI queries
   - Month 4-6: Continue content and presence building, track progress monthly
6. Set expectations: "AI training data updates on different schedules. ChatGPT may take months to reflect changes. Perplexity and Google AI Overview update faster because they search the live web."

---

## Related Skills

- **product-marketing-context** — must understand your positioning before optimizing for AI
- **seo-competitor-pages** — comparison pages are high-value content for AI citations
- **seo-programmatic** — programmatic pages can build topical authority that AI systems recognize
- **copywriting** — for writing content structured for AI extraction
- **social-content** — Reddit and social presence feeds into AI training data
- **seo-hreflang** — ensure AI search shows the right language version of your brand

**AI SEO is a long-term strategy, not a quick fix.** Building genuine authority and presence takes months, but the payoff is being recommended by AI systems to millions of users.
