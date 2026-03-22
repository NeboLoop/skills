---
name: seo-competitor-pages
description: Create high-quality "X vs Y" comparison pages and "alternatives to X" pages that rank for competitor search terms and honestly help buyers decide. Use when targeting competitor keywords, creating comparison or alternatives content, or capturing search traffic from people evaluating competitors.
---

# SEO Competitor Pages

Create "X vs Y" and "alternatives to X" comparison pages that rank in search and honestly help buyers make decisions.

---

## When to Use

Trigger this skill when you hear:
- "I want to rank for [competitor] alternative"
- "Can we create a comparison page?"
- "X vs Y page"
- "Alternatives to [competitor]"
- "People are searching for our competitor's name"
- "We need competitor comparison content"
- "How do we show up when people search for [competitor]?"
- "Competitor landing pages"

This skill is for creating hand-crafted, high-quality comparison pages. If you need templated comparison pages at scale (50+ competitors), use **seo-programmatic** instead.

---

## Context Gathering

Ask these questions conversationally. You need to understand both the user's product and their competitors.

### Your Product
1. **What's your product/service?** (If marketing context exists in memory, skip this)
2. **What are your genuine strengths?** (Features, pricing, support, ease of use — what you honestly do better)
3. **What are your weaknesses?** (Where competitors genuinely beat you — being honest here builds trust with readers)

### Target Competitors
4. **Which competitors do you want to create pages for?** (Name them specifically)
5. **Why do people choose each competitor over you?** (Understand their real appeal)
6. **Why do people choose you over each competitor?** (The switching trigger)

### Search Context
7. **Are people already searching for "[your product] vs [competitor]"?** (Check if there's existing search demand)
8. **Do your competitors have comparison pages targeting you?** (If yes, you need to respond)
9. **What does someone searching for "[competitor] alternative" actually want?** (Price? Features? Different approach entirely?)

### Content Resources
10. **Have you used the competitor products yourself?** (First-hand experience makes better content)
11. **Do you have customer testimonials from people who switched?** (Powerful social proof for comparison pages)
12. **Can you provide accurate feature and pricing data for competitors?** (Needs to be current and honest)

---

## Methodology

### Step 1: Check Marketing Context
Pull the user's product marketing context from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If none exists, run **product-marketing-context** first. You cannot write honest comparison content without deeply understanding the user's own product and positioning.

### Step 2: Research Each Target Competitor
For each competitor, use the browser to gather current information:

```
web(action: "navigate", url: "competitor-website.com")
web(action: "read_page")
```

Gather:
- **Positioning:** How do they describe themselves? What's their tagline?
- **Features:** What do they offer? What are their key capabilities?
- **Pricing:** What are their current plans and prices? (Screenshot or note the date — pricing changes)
- **Target audience:** Who are they built for?
- **Strengths:** What do they genuinely do well?
- **Weaknesses:** Where do users complain? (Check review sites, Reddit, G2, Capterra)

**Important:** Be thorough and fair. If you misrepresent a competitor, readers will notice and trust is lost.

### Step 3: Build the Feature Comparison Matrix
Create an honest, detailed comparison table:

| Feature | Your Product | Competitor |
|---------|-------------|------------|
| Feature A | Yes | Yes |
| Feature B | Yes | No |
| Feature C | No | Yes |
| Pricing | $X/mo | $Y/mo |
| Free trial | 14 days | 7 days |

**Rules for the comparison matrix:**
- Include features where the competitor wins — this builds trust
- Use specific details, not vague claims ("100 integrations" vs "many integrations")
- Mark pricing with a "last checked" date
- If a feature is partial, say "Partial" or "Limited" not just Yes/No
- Don't cherry-pick only categories where you win

### Step 4: Determine the Page Type
Choose the right format based on search intent:

**"X vs Y" page** — for when people are comparing two specific products:
- Direct head-to-head comparison
- Feature matrix is the centerpiece
- Both products get fair treatment
- Verdict section explains who each product is best for

**"Alternatives to X" page** — for when people want to leave a competitor:
- Position your product as the primary alternative
- Briefly mention 3-5 other alternatives for credibility
- Focus on the pain points that drive people away from X
- Emphasize your switching story (how easy it is to move)

**"X vs Y vs Z" page** — for when three or more products are commonly compared:
- Three-way comparison table
- Category-by-category breakdown
- "Best for" verdict for each product

### Step 5: Write Balanced Comparison Copy
The key to comparison pages that rank AND convert is honesty. Follow these writing rules:

**Do:**
- Acknowledge where the competitor is strong
- Lead with the buyer's decision criteria, not your features
- Include a "Who should choose [competitor]" section — readers respect this
- Use real data, screenshots, and specific details
- Include a verdict that's nuanced, not just "pick us"

**Don't:**
- Trash the competitor — it makes you look insecure
- Use biased language ("unlike our inferior competitor...")
- Omit features where the competitor wins
- Make claims you can't back up
- Use outdated competitor information

**Tone:** Helpful buying guide, not sales pitch. You're the knowledgeable friend helping someone make a decision.

### Step 6: Add Schema Markup
Add structured data to help the page appear in rich results:

**FAQ Schema** — for common comparison questions:
```json
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What's the difference between X and Y?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "The main difference is..."
      }
    }
  ]
}
```

**AggregateRating Schema** — if you have genuine user ratings:
```json
{
  "@type": "Product",
  "name": "Your Product",
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingValue": "4.7",
    "reviewCount": "234"
  }
}
```

Only use AggregateRating if you have real, verifiable reviews. Don't fabricate ratings.

### Step 7: Optimize for Target Keywords
Each comparison page should target specific keywords:

**Primary keywords:**
- "[Your product] vs [Competitor]"
- "[Competitor] alternative"
- "[Competitor] vs [Your product]"

**Secondary keywords:**
- "best [competitor] alternative"
- "[competitor] replacement"
- "switch from [competitor]"
- "[competitor] pricing" (if you're more affordable)
- "[competitor] review" (use carefully — focus on comparison, not review)

**On-page optimization:**
- H1 includes primary keyword naturally
- Meta title: "[Your Product] vs [Competitor]: Honest Comparison [Year]"
- Meta description: Summarize the key difference and who each is best for
- Use competitor name naturally throughout (don't keyword-stuff)

### Step 8: Plan Updates and Maintenance
Competitor pages go stale fast. Plan for ongoing updates:

- **Review pricing quarterly** — competitor pricing changes frequently
- **Update features when competitors ship new ones** — stay current
- **Add new customer testimonials from switchers** — fresh social proof
- **Update the "last reviewed" date** — signals freshness to Google and readers
- **Monitor ranking and traffic monthly** — adjust if pages are slipping

---

## Output Format

Deliver each competitor page in this structure:

```markdown
# Competitor Page: [Your Product] vs [Competitor]
Date: [DATE]

## Target Keywords
- Primary: [keyword]
- Secondary: [keyword], [keyword], [keyword]
- Monthly search volume: [volume if known]

## Page Metadata
**URL:** /compare/[your-product]-vs-[competitor]
**Meta title:** [Title — under 60 characters]
**Meta description:** [Description — under 155 characters]

## Page Content

### H1: [Your Product] vs [Competitor]: [Value statement]

### Introduction (150-200 words)
[What both products do, who this comparison is for, what you'll cover]

### Quick Summary Table
| | [Your Product] | [Competitor] |
|---|---|---|
| Best for | [audience] | [audience] |
| Starting price | [price] | [price] |
| Key strength | [strength] | [strength] |
| Key limitation | [limitation] | [limitation] |
| Rating | [rating] | [rating] |

### Detailed Comparison by Category

#### [Category 1: e.g., Features]
[Honest breakdown — 150-200 words]

#### [Category 2: e.g., Pricing]
[Honest breakdown — 150-200 words]

#### [Category 3: e.g., Ease of Use]
[Honest breakdown — 150-200 words]

#### [Category 4: e.g., Support]
[Honest breakdown — 150-200 words]

### Who Should Choose [Competitor]
[Honest assessment of when the competitor is the better choice — 100 words]

### Who Should Choose [Your Product]
[When your product is the better choice — 100 words]

### What Customers Say (Switching Stories)
[1-2 testimonials from people who switched from competitor — with attribution]

### Verdict
[Balanced summary — who each product is best for, final recommendation]

### FAQ Section
[3-5 common questions with answers — also used for FAQ schema]

## Schema Markup
[Full JSON-LD for FAQ and AggregateRating if applicable]

## Internal Links
- Link FROM: [existing pages that should link to this comparison]
- Link TO: [other comparison pages, pricing page, signup page]

## Update Schedule
- Next review date: [date — typically 3 months out]
- Monitor for: [competitor pricing changes, new feature launches]
```

---

## Quality Checks

Before delivering the comparison page, verify:

- [ ] **Comparison is honest** — includes areas where the competitor wins
- [ ] **Feature data is current** — checked the competitor's website today, not from memory
- [ ] **Pricing is accurate and dated** — includes "last checked" date
- [ ] **"Who should choose [competitor]" section exists** — builds trust and credibility
- [ ] **No biased or disparaging language** — reads like a helpful guide, not an attack
- [ ] **Schema markup is valid** — only uses real, verifiable ratings
- [ ] **Target keywords appear naturally** — not stuffed
- [ ] **Update schedule is defined** — competitor pages need regular maintenance

---

## Examples

### Example 1: Direct Competitor Comparison

**User says:** "We want to create a page for 'Slack vs [our product]' — we're a team messaging tool."

**You do:**
1. Check marketing context (found: team messaging tool focused on async communication)
2. Ask: "What do you genuinely do better than Slack?"
3. User: "We're built for async-first teams. Slack is noisy — we organize conversations by topic with built-in summaries."
4. Ask: "Where does Slack beat you honestly?"
5. User: "They have way more integrations and everyone already knows how to use Slack."
6. Browse Slack's website for current features and pricing
7. Build comparison matrix — include integrations (Slack wins) and async features (user wins)
8. Write balanced copy: "If your team thrives on real-time chat, Slack is excellent. If notification fatigue is killing your team's focus, [product] was built for that problem."
9. Include "Who should choose Slack" section
10. Add FAQ schema for common questions
11. Deliver full page with update schedule

### Example 2: "Alternatives to" Page

**User says:** "People search for 'alternatives to Mailchimp' and we should show up."

**You do:**
1. Check marketing context (found: email marketing platform for e-commerce)
2. Ask: "Why do people leave Mailchimp?"
3. User: "Price increases, complicated pricing tiers, and they removed the free plan."
4. Ask: "What's your advantage for those specific pain points?"
5. User: "Simple flat-rate pricing and a generous free tier."
6. Research Mailchimp's current pricing and 4-5 other alternatives
7. Write page: Lead with Mailchimp pain points, position user's product as primary alternative
8. Include 4 other alternatives briefly (Constant Contact, ConvertKit, Brevo, MailerLite) for credibility
9. Focus on switching ease: "Import your Mailchimp list in 2 minutes"
10. Target keywords: "mailchimp alternative," "best mailchimp alternative," "mailchimp replacement"

### Example 3: Multiple Competitor Pages

**User says:** "We want comparison pages against our top 5 competitors."

**You do:**
1. Ask: "Who are the 5 competitors?"
2. User lists them with context on each
3. Research all 5 competitor websites
4. Prioritize by search volume: Which "[competitor] alternative" keywords get the most searches?
5. Create pages in order of search demand
6. Cross-link all comparison pages to each other
7. Create a hub page: "How [Product] Compares" linking to all 5 comparison pages
8. Deliver first page as a template, confirm quality, then produce the remaining 4
9. Set quarterly review calendar for all 5 pages

---

## Related Skills

- **product-marketing-context** — must understand your own positioning before comparing to competitors
- **seo-programmatic** — for scaling comparison pages beyond 10+ competitors using templates
- **copywriting** — for refining comparison copy tone and persuasion
- **page-cro** — optimize comparison pages for conversion (signups, demos)
- **ai-seo** — ensure comparison content appears in AI search results
- **seo-hreflang** — if comparison pages need international versions

**Honest comparison pages build trust. Biased comparison pages lose readers.** Always prioritize being genuinely helpful over being promotional.
