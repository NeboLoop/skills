---
name: competitor-alternatives
description: Create honest comparison pages ("X vs Y") and alternatives roundup pages that capture competitor search traffic and help buyers decide. Use when ranking for competitor keywords, writing comparison content, creating "best alternatives to X" pages, or responding to competitor comparison pages targeting your brand.
---

# Competitor Alternatives

Create comparison and alternatives pages that capture competitor search traffic — honestly and effectively.

---

## When to Use

Trigger this skill when you hear:
- "Help me rank for competitor keywords"
- "Write a comparison page"
- "Create an alternatives page for [competitor]"
- "We need a vs page"
- "How do we capture competitor search traffic?"
- "People are searching for [competitor] alternatives"
- "Write an X vs Y page"

This skill creates two types of content: "[Your Product] vs [Competitor]" comparison pages and "Best [Competitor] Alternatives" roundup pages. Both are designed to rank for high-intent search traffic.

---

## Context Gathering

Before writing anything, you need to understand both sides of the comparison. Ask conversationally:

### Your Product
1. **Which competitors do you want to target?** (Pick 1-3 to start — the ones your prospects compare you to most)
2. **What do you honestly do better than each one?** (Specific features, pricing, experience — not vague claims)
3. **Where are they stronger than you?** (Being honest here builds trust with readers and improves conversion)
4. **What do customers say when they switch from a competitor to you?** (The real reason they moved)

### The Competitor
5. **What's their pricing?** (Or rough range — readers always want to know)
6. **What are their known weaknesses?** (From reviews, from customers who switched, from your own research)
7. **What audience do they serve best?** (Enterprise? SMB? A specific industry?)

### Your Goals
8. **Which keywords matter most?** (e.g., "[competitor] alternative" vs "[competitor] vs [you]" vs "best [category] tools")
9. **Where will these pages live on your site?** (Blog? Dedicated comparison section? Landing pages?)
10. **What's the conversion action?** (Free trial? Demo? Contact sales?)

If product-marketing-context exists in memory, pull questions 2-4 from there and skip to competitor-specific questions.

---

## Methodology

### Step 1: Check Product Marketing Context
Pull your positioning, differentiation, and competitor list from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
agent(resource: "memory", action: "search", query: "positioning/competitors")
agent(resource: "memory", action: "search", query: "positioning/differentiation")
```

If no context exists, run the **product-marketing-context** skill first. You need solid positioning before writing comparison content.

### Step 2: Research Each Competitor
Use the browser to gather current information on each competitor:
```
web(action: "navigate", url: "competitor-website.com")
web(action: "read_page")
web(action: "navigate", url: "competitor-website.com/pricing")
web(action: "read_page")
```

For each competitor, document:
- **Core product description** — what they actually do
- **Key features** — their main capabilities
- **Pricing** — plans, tiers, and costs
- **Target audience** — who they serve best
- **Strengths** — what they genuinely do well
- **Weaknesses** — where they fall short (use review sites like G2, Capterra, TrustPilot)

Also check review sites:
```
web(action: "navigate", url: "g2.com/products/[competitor]/reviews")
web(action: "read_page")
```

### Step 3: Build the Feature Comparison Matrix
Create an honest, side-by-side feature comparison. Structure it as:

| Feature | Your Product | Competitor |
|---------|-------------|------------|
| Feature 1 | What you offer | What they offer |
| Feature 2 | What you offer | What they offer |
| Pricing | Your pricing | Their pricing |

Rules for the matrix:
- Use checkmarks, X marks, and descriptive text — not just yes/no
- Include features where the competitor wins (this builds credibility)
- Group features by category (Core Features, Integrations, Support, Pricing)
- Include pricing comparison — readers expect it
- Note when features require higher-tier plans

### Step 4: Write the "X vs [Your Product]" Page
Structure the comparison page:

1. **Title:** "[Competitor] vs [Your Product]: [Honest differentiator]"
2. **Opening:** Acknowledge why someone is comparing — validate their search
3. **Quick summary:** A 2-3 sentence TL;DR of the key differences
4. **Feature comparison table:** The matrix from Step 3
5. **Detailed breakdown:** 3-5 sections covering the biggest differences
6. **Who should choose [Competitor]:** Be honest about when they're the better fit
7. **Who should choose [Your Product]:** Your ideal customer profile
8. **Pricing comparison:** Side-by-side pricing with links to both pricing pages
9. **CTA:** Clear next step (trial, demo, etc.)

Write in the brand voice from product-marketing-context. Keep the tone respectful — never trash the competitor.

### Step 5: Write the "Best [Competitor] Alternatives" Page
Structure the alternatives page:

1. **Title:** "Best [Competitor] Alternatives in [Year]"
2. **Opening:** Why someone might be looking for alternatives (common pain points)
3. **What to look for:** Criteria for evaluating alternatives (sets up your strengths)
4. **Your product (listed first):** Honest description, key features, pricing, best for
5. **Other alternatives (3-5):** Brief descriptions of other real alternatives
6. **Comparison table:** All alternatives side by side
7. **Verdict:** When each alternative makes sense
8. **CTA:** Why your product is worth trying

Include real alternatives, not just your product. Readers expect a genuine roundup.

### Step 6: Add Structured Data
Include schema markup for the comparison content:

```json
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "[Competitor] vs [Your Product]",
  "description": "Detailed comparison of [Competitor] and [Your Product]...",
  "author": {
    "@type": "Organization",
    "name": "[Your Company]"
  },
  "datePublished": "[DATE]",
  "dateModified": "[DATE]"
}
```

For alternatives pages, consider FAQ schema for common questions:
```json
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is the best alternative to [Competitor]?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "The best alternative depends on your needs..."
      }
    }
  ]
}
```

### Step 7: Optimize for Target Keywords
Primary keywords to target:
- `[competitor] alternative` / `[competitor] alternatives`
- `[competitor] vs [your product]`
- `[your product] vs [competitor]`
- `best [competitor] alternatives [year]`
- `[competitor] vs` (broad — captures all comparison searches)
- `switch from [competitor]`
- `[competitor] pricing` (if your pricing is more competitive)

On-page SEO:
- Use the primary keyword in the H1, meta title, and first paragraph
- Include related keywords naturally throughout
- Add internal links to your feature pages, pricing page, and case studies
- Include a meta description that mentions both products
- Use descriptive alt text on comparison images and tables

### Step 8: Keep It Honest
Review everything against these honesty rules:

- **Never make false claims** about competitors — readers will check and you lose trust
- **Acknowledge competitor strengths** — say "If you need X, [Competitor] does this well"
- **Use real data** — actual pricing, actual features, not guesses
- **Date the content** — pricing and features change, readers need to know when this was written
- **Update regularly** — set a reminder to review quarterly as competitors update their products
- **Link to competitor sites** — shows confidence and helps readers verify your claims

Being honest about where you're weaker makes you more credible where you're stronger.

---

## Output Format

### Comparison Page
```markdown
# [Competitor] vs [Your Product]: [Key Differentiator]
Last updated: [DATE]

## TL;DR
[2-3 sentences summarizing the key differences and who each product is best for.]

## Feature Comparison

| Feature | [Your Product] | [Competitor] |
|---------|---------------|-------------|
| [Feature 1] | [Detail] | [Detail] |
| [Feature 2] | [Detail] | [Detail] |
| [Feature 3] | [Detail] | [Detail] |
| Pricing starts at | [Price] | [Price] |
| Best for | [Audience] | [Audience] |

## [Difference 1 - Biggest Differentiator]
[3-4 paragraphs explaining this difference in detail, with examples]

## [Difference 2]
[3-4 paragraphs]

## [Difference 3]
[3-4 paragraphs]

## Who Should Choose [Competitor]
[Honest assessment of when the competitor is the better choice]

## Who Should Choose [Your Product]
[Your ideal customer and why this is the right fit]

## Pricing Comparison
[Side-by-side pricing breakdown with links]

## Try [Your Product]
[CTA — what to do next, how to get started]
```

### Alternatives Page
```markdown
# Best [Competitor] Alternatives in [Year]

## Why Look for [Competitor] Alternatives?
[Common reasons people switch — based on real review data]

## What to Look for in a [Competitor] Alternative
[Evaluation criteria that highlight your strengths without being obvious about it]

## Top [Competitor] Alternatives

### 1. [Your Product]
**Best for:** [Audience]
**Pricing:** [Range]
**Key features:** [Bullet list]
**Why consider it:** [Honest pitch]

### 2. [Alternative 2]
**Best for:** [Audience]
**Pricing:** [Range]
**Key features:** [Bullet list]

### 3. [Alternative 3]
[Same structure]

## Side-by-Side Comparison
[Table comparing all alternatives]

## Which [Competitor] Alternative Is Right for You?
[Decision framework based on use case, budget, team size]

## Get Started with [Your Product]
[CTA]
```

---

## Quality Checks

Before publishing, verify:

- [ ] **All competitor information is accurate** — pricing, features, and claims are current and verifiable
- [ ] **Honesty check passed** — you acknowledged at least one area where the competitor is stronger
- [ ] **Feature comparison is fair** — not cherry-picking only features where you win
- [ ] **Pricing is current** — includes dates and links to source
- [ ] **Structured data is valid** — test with Google's Rich Results Test
- [ ] **Target keywords appear naturally** — in H1, meta title, first paragraph, and throughout
- [ ] **CTA is clear** — reader knows exactly what to do next
- [ ] **Tone is respectful** — no competitor bashing, sarcasm, or exaggeration
- [ ] **Page is dated** — readers can see when the comparison was last updated
- [ ] **Internal links included** — to your pricing, features, and case study pages
- [ ] **Brand voice matches** — consistent with product-marketing-context

---

## Examples

### Example 1: SaaS Comparison Page

**User says:** "We need a comparison page — people keep searching for Mailchimp vs us."

**You do:**
1. Check product-marketing-context (found: email marketing tool for creators)
2. Research Mailchimp: features, pricing tiers, G2 reviews
3. Build comparison matrix covering features, pricing, ease of use, support
4. Write the page:

**Title:** "Mailchimp vs CreatorMail: Which Email Tool Is Right for Creators?"

**TL;DR:** "Mailchimp is the industry standard with powerful automation and a massive template library. CreatorMail is purpose-built for creators with simpler pricing and built-in monetization tools. Choose Mailchimp if you need enterprise-grade automation. Choose CreatorMail if you want email tools designed specifically for your creator business."

**Honesty moment:** "Mailchimp has a larger template library and more third-party integrations. If you rely on complex automation workflows with dozens of triggers, Mailchimp gives you more options."

**Your angle:** "CreatorMail was built from the ground up for creators. Paid newsletters, digital product delivery, and audience analytics are built in — not bolted on."

### Example 2: Alternatives Roundup

**User says:** "Write a 'best Salesforce alternatives' page — we're targeting SMBs who find Salesforce too complex."

**You do:**
1. Check product-marketing-context (found: simple CRM for small teams)
2. Research Salesforce: common complaints from SMB users on G2 and Reddit
3. Research 4-5 legitimate alternatives (HubSpot, Pipedrive, Zoho, Close)
4. Write the page:

**Title:** "Best Salesforce Alternatives for Small Teams in 2026"

**Opening pain points:**
- "Salesforce is powerful, but it's built for enterprise sales orgs with dedicated admins"
- "If you're a team of 5-20 and spending more time configuring your CRM than selling, you're not alone"

**Evaluation criteria (subtly favoring your strengths):**
- Setup time (you win here)
- Per-user pricing (you win here)
- Core CRM features (Salesforce wins, acknowledged)
- Learning curve (you win here)

**Your product listed first** with honest positioning, followed by real alternatives with fair descriptions.

### Example 3: Updating Existing Comparison Content

**User says:** "Our competitor just changed their pricing — update the comparison page."

**You do:**
1. Pull existing comparison content from memory or site
2. Research competitor's new pricing via browser
3. Update only the pricing sections and the "last updated" date
4. Flag any new features or changes noticed during research
5. Ask: "Their pricing changed. I also noticed they launched a new feature — want me to add that to the comparison?"

---

## Related Skills

- **seo-competitor-pages** — broader SEO strategy for ranking on competitor terms
- **copywriting** — writes the actual page copy in brand voice
- **pricing-strategy** — informs how to position pricing in comparisons
- **product-marketing-context** — provides the positioning and differentiation for your side of every comparison
- **sales-enablement** — creates battle cards and objection-handling docs using the same competitive research
