---
name: seo-content
description: Analyze page content quality through Google's E-E-A-T framework (Experience, Expertise, Authoritativeness, Trustworthiness) with readability scoring and AI-generated content detection. Use when evaluating whether content is good enough to rank, diagnosing why content isn't ranking, checking for thin content, or auditing E-E-A-T signals.
---

# SEO Content Quality (E-E-A-T)

Analyze any page's content quality through Google's E-E-A-T framework — Experience, Expertise, Authoritativeness, and Trustworthiness — and get clear recommendations to improve rankings.

---

## When to Use

Trigger this skill when you hear:
- "Is my content good enough?"
- "Check my page for E-E-A-T"
- "Why isn't my content ranking?"
- "Review my content quality"
- "Is this thin content?"
- "Does my page seem AI-generated?"
- "How can I improve my content for SEO?"
- "Audit my page content"

This skill focuses on content quality signals. For technical SEO issues (speed, crawlability, schema), use **seo-audit** instead. For local/geographic SEO, use **seo-geo**.

---

## Context Gathering

Before starting, you need:

1. **What URL do you want analyzed?** (The specific page, not just the domain)
2. **What keyword or topic is this page targeting?** (The primary search intent)
3. **Who is the target audience?** (Helps assess whether depth and language are appropriate)

If the user has product-marketing-context stored in memory, pull it automatically:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

Use that context to understand the brand voice and audience without asking again.

---

## Methodology

### Step 1: Load and Read the Page
Navigate to the URL and capture the full content:
```
web(action: "navigate", url: "target-page-url")
web(action: "read_page")
```

If the page requires JavaScript rendering, wait for it to load fully. Capture:
- Page title and meta description
- All heading tags (H1 through H6)
- Body content
- Author information (byline, bio, author page links)
- Publication and update dates
- Internal and external links
- Images and alt text
- Footer content (contact info, policies)

### Step 2: Assess Experience Signals
Check whether the content shows first-hand knowledge:

- **Original research or data** — Does the page include proprietary data, case studies, surveys, or experiments the author conducted?
- **Personal anecdotes** — Are there specific stories about actually doing the thing being discussed?
- **Unique media** — Original photos, screenshots, or videos (not stock images)?
- **Specific details** — Does the content include granular details that only someone with real experience would know?
- **Practical advice** — Are recommendations based on what actually worked, or are they generic?

Score each factor: Present / Weak / Missing

### Step 3: Assess Expertise Signals
Check whether the content demonstrates subject-matter depth:

- **Author credentials** — Is there a named author? Do they have relevant qualifications, certifications, or professional experience?
- **Author bio** — Is there a bio on the page or linked author page? Does it establish relevance to the topic?
- **Content depth** — Does the content go beyond surface-level? Does it cover nuances, edge cases, and trade-offs?
- **Technical accuracy** — Are claims accurate? Are industry terms used correctly?
- **Structured coverage** — Does the content logically progress through the topic, or does it jump around randomly?
- **Supporting evidence** — Are assertions backed by data, examples, or references?

Score each factor: Strong / Adequate / Weak / Missing

### Step 4: Assess Authoritativeness Signals
Check whether the page and site are recognized as credible sources:

- **Citations and references** — Does the content link to authoritative external sources (studies, official docs, industry reports)?
- **Outbound link quality** — Are external links pointing to trustworthy, relevant sites?
- **Site reputation signals** — Is the domain known in the industry? Are there "About" or "Our Team" pages?
- **Content attribution** — Is the content clearly attributed to a real person or organization?
- **Topic consistency** — Does the site regularly publish on this topic, or is this a one-off?
- **Industry recognition** — Any awards, certifications, media mentions, or partnerships visible on the site?

Score each factor: Strong / Adequate / Weak / Missing

### Step 5: Assess Trustworthiness Signals
Check whether users can trust this page and site:

- **Contact information** — Is there a real business address, phone number, or contact form?
- **Privacy policy** — Is there a privacy policy? Is it accessible from the page?
- **Terms of service** — Are terms available?
- **HTTPS** — Is the site served over a secure connection?
- **Accuracy of claims** — Are factual claims verifiable? Are statistics sourced?
- **Transparency** — If the page contains reviews or recommendations, is there an affiliate or sponsorship disclosure?
- **Recency** — Is the content dated? Has it been updated recently? Are the facts still current?
- **Error-free content** — Is the content free of factual errors, broken links, and contradictions?

Score each factor: Present / Missing

### Step 6: Evaluate Readability
Analyze the content's readability for the target audience:

- **Reading level** — Estimate the grade level. Is it appropriate for the audience?
- **Sentence structure** — Are sentences clear and varied, or are they overly complex or monotonously simple?
- **Paragraph length** — Are paragraphs scannable (3-4 sentences) or walls of text?
- **Formatting** — Are headings, bullet points, bold text, and whitespace used effectively?
- **Jargon** — Is technical language appropriate for the audience, or is it alienating?
- **Scannability** — Can a reader quickly find what they need?

Rate overall readability: Excellent / Good / Needs Work / Poor

### Step 7: Check for Thin Content
Determine whether the page provides sufficient value:

- **Word count** — How much content is on the page? (Not a direct ranking factor, but thin pages under 300 words rarely rank well for competitive terms)
- **Depth vs. competitors** — Does this page cover the topic as thoroughly as top-ranking pages?
- **Unique value** — Does the page offer something you can't find on 10 other sites?
- **Content-to-noise ratio** — How much of the page is actual content vs. ads, navigation, boilerplate?
- **User intent match** — Does the content fully satisfy the likely search intent for the target keyword?
- **Duplicate content** — Does the page reuse large blocks of text from other pages on the same site?

Rate: Comprehensive / Adequate / Thin / Very Thin

### Step 8: Detect AI-Generated Content Issues
Check for patterns that suggest low-quality AI-generated content:

- **Generic phrasing** — Overuse of filler phrases like "In today's fast-paced world," "It's important to note that," or "Let's dive in"
- **Lack of specificity** — Vague claims without concrete examples, numbers, or real-world references
- **Repetitive structure** — Every section follows the exact same pattern (intro sentence, three bullet points, summary sentence)
- **Missing perspective** — No opinions, preferences, or point of view — just neutral summaries
- **Absence of experience** — No "I did this" or "We found that" — only "One should consider"
- **Perfect but empty** — Grammatically flawless but says nothing original or useful
- **Keyword stuffing** — Unnatural repetition of the target keyword in a way that feels forced

Note: AI-generated content is not inherently bad. The issue is when AI content lacks the experience, expertise, and unique value that humans bring. Flag problems, not the tool.

### Step 9: Score and Recommend
Compile a final assessment with actionable improvements.

Calculate an overall E-E-A-T score (see Output Format) and prioritize recommendations by impact:
- **High impact** — Changes that directly address missing E-E-A-T signals
- **Medium impact** — Improvements that strengthen existing signals
- **Low impact** — Nice-to-haves that polish the content

---

## Output Format

Present the analysis in this structure:

```markdown
# E-E-A-T Content Analysis
**URL:** [analyzed URL]
**Target keyword:** [keyword]
**Date:** [analysis date]

## Overall Score: [X/100]

| Category         | Score | Summary                    |
|------------------|-------|----------------------------|
| Experience       | X/25  | [one-line summary]         |
| Expertise        | X/25  | [one-line summary]         |
| Authoritativeness| X/25  | [one-line summary]         |
| Trustworthiness  | X/25  | [one-line summary]         |

## Experience ([X/25])
[Detailed findings from Step 2]

**What's working:**
- [signals found]

**What's missing:**
- [gaps identified]

## Expertise ([X/25])
[Detailed findings from Step 3]

**What's working:**
- [signals found]

**What's missing:**
- [gaps identified]

## Authoritativeness ([X/25])
[Detailed findings from Step 4]

**What's working:**
- [signals found]

**What's missing:**
- [gaps identified]

## Trustworthiness ([X/25])
[Detailed findings from Step 5]

**What's working:**
- [signals found]

**What's missing:**
- [gaps identified]

## Readability
**Level:** [rating]
[Key findings from Step 6]

## Content Depth
**Rating:** [Comprehensive / Adequate / Thin / Very Thin]
**Word count:** [approximate]
[Key findings from Step 7]

## AI Content Flags
[Findings from Step 8 — or "No significant issues detected"]

## Recommendations

### High Impact
1. [specific action with explanation]
2. [specific action with explanation]
3. [specific action with explanation]

### Medium Impact
1. [specific action with explanation]
2. [specific action with explanation]

### Low Impact
1. [specific action with explanation]
```

---

## Quality Checks

Before delivering the analysis, verify:

- [ ] **Every score has evidence** — no scores without specific examples from the page
- [ ] **Recommendations are actionable** — not "improve expertise" but "add an author bio with credentials in [field]"
- [ ] **Target keyword context is considered** — a medical page needs higher E-E-A-T than a recipe page
- [ ] **Competitor context is mentioned** — what top-ranking pages do that this one doesn't
- [ ] **No false positives on AI detection** — professional writing is not the same as AI writing
- [ ] **Readability matches audience** — a developer docs page can be technical; a consumer blog should be simple
- [ ] **Recommendations are prioritized** — the user knows what to fix first
- [ ] **YMYL topics get stricter scoring** — Your Money or Your Life content (health, finance, legal, safety) is held to a higher standard

---

## Examples

### Example 1: Blog Post Audit

**User says:** "Check if my blog post is good enough to rank."

**You do:**
1. Ask: "What's the URL and what keyword are you targeting?"
2. User: "https://example.com/best-crm-for-startups — targeting 'best CRM for startups'"
3. Load the page, read all content
4. Run through all 9 methodology steps
5. Find: No author byline, no original research, generic comparisons that match 20 other listicles, stock images throughout, no disclosure of testing methodology
6. Score: Experience 5/25, Expertise 10/25, Authoritativeness 12/25, Trustworthiness 18/25 = 45/100
7. Recommend: "Add a byline with your credentials. Include a section on your testing methodology — how you actually evaluated each CRM. Replace stock images with real screenshots from your testing. Add specific numbers: how long you tested, how many contacts you imported, what workflows you set up."

### Example 2: Product Page Review

**User says:** "Why isn't my product page ranking for 'email automation tool'?"

**You do:**
1. Load the page
2. Find: Page is only 200 words, no social proof, no use cases, no comparison to alternatives, no author, published 2 years ago with no updates
3. Score: 28/100 overall — thin content with missing trust signals
4. Recommend: "This page needs substantially more content. Add: customer testimonials with real names and companies, detailed feature explanations with screenshots, a comparison table against alternatives (Mailchimp, ActiveCampaign), use cases by industry, and an FAQ section addressing common buyer questions. Update the page date after making changes."

### Example 3: YMYL Content

**User says:** "Audit my financial advice article."

**You do:**
1. Recognize this is YMYL content — apply stricter standards
2. Load the page
3. Find: Author has no financial credentials listed, no disclaimers, specific investment advice without sourcing, no update date
4. Score: 22/100 — critically low for YMYL
5. Recommend: "YMYL content like financial advice requires the highest E-E-A-T standards. Immediately add: author credentials (CFA, CFP, or relevant experience), a disclaimer that this is not financial advice, sources for every claim about returns or performance, a clear publication and last-updated date, and links to authoritative sources like SEC.gov or Federal Reserve data."

---

## Related Skills

- **seo-audit** — technical SEO analysis (speed, crawlability, schema, site structure)
- **seo-geo** — local and geographic SEO optimization
- **copywriting** — rewrite content to improve quality based on E-E-A-T findings
- **product-marketing-context** — provides brand and audience context for better analysis
