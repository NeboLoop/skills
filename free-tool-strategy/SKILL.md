---
name: free-tool-strategy
description: Design free tools, calculators, or interactive resources that attract leads and build authority for your business. Use when planning a free calculator, grader, generator, or assessment to drive organic traffic, capture emails, and create a natural path to your paid product.
---

# Free Tool Strategy

Design free tools, calculators, or resources that attract leads and build authority for your business.

---

## When to Use

Trigger this skill when you hear:
- "I want to create a free tool"
- "How do I build a calculator for my site?"
- "I need a lead magnet that actually works"
- "How do I attract more organic traffic?"
- "I want to create something interactive for my audience"
- "Can we build a free resource that generates leads?"
- "I need a top-of-funnel strategy"
- "How do I get backlinks naturally?"

This skill is for planning and designing the tool concept, layout, gating strategy, and distribution plan — not for writing the code.

---

## Context Gathering

Before designing a free tool, you need to understand the business. Ask these conversationally — don't dump them all at once.

### About the Audience
1. **Who are you trying to attract?** (Job title, role, or type of person)
2. **What repetitive task do they do manually today?** (Spreadsheets, guesswork, back-of-napkin math)
3. **What question do they keep Googling?** (The thing they search for before they know your product exists)

### About the Business
4. **What does your product or service do?** (If not already captured in memory)
5. **What's the connection between the free tool and your paid product?** (The tool should naturally lead people toward your offering)
6. **Do you have existing content or data that could power a tool?** (Benchmarks, formulas, frameworks, templates)

### About Distribution
7. **Where does your audience hang out online?** (Communities, social platforms, forums)
8. **Do you have an email list or social following already?** (Helps plan the launch)
9. **What's your timeline and budget for this?** (Keeps the scope realistic)

---

## Methodology

### Step 1: Check Product Marketing Context
Pull your existing marketing context from memory so the tool aligns with your brand, audience, and goals:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. You need to know who you're building for before you design anything.

### Step 2: Identify the Right Problem to Solve
The best free tools solve a problem your audience already deals with repeatedly. Look for these patterns:

- **They're using spreadsheets for it** — calculators, trackers, planners
- **They're guessing instead of knowing** — graders, analyzers, assessors
- **They're doing it by hand every time** — generators, builders, formatters

Use Nebo's browser to research what competitors offer as free tools:
```
web(action: "navigate", url: "competitor-website.com/resources")
web(action: "read_page")
```

Search for "[your industry] free tool" and "[your industry] calculator" to see what already exists. Your tool needs to be better, simpler, or cover a gap.

**Good free tool ideas share three traits:**
1. They deliver value in under 60 seconds
2. They require minimal input from the user (5-10 fields, not 50)
3. They naturally connect to your paid product

### Step 3: Design the Simplest Version
Start with the minimum version that still delivers real value. You can always add features later.

**Define these elements:**
- **Tool name** — clear, descriptive, includes the benefit (e.g., "ROI Calculator for Email Marketing")
- **What the user inputs** — keep it to 3-7 fields for version one
- **What the user gets back** — the result, score, recommendation, or output
- **How results are displayed** — numbers, charts, comparison tables, grades
- **One clear takeaway** — the user should walk away with something actionable

Don't over-engineer it. A simple calculator that works beats a fancy tool that confuses people.

### Step 4: Choose a Gating Strategy
Decide how much of the tool is free vs. gated. There are three main approaches:

**Option A: Fully ungated (recommended for most)**
- The tool is completely free to use, no email required
- Add an optional email capture: "Want to save your results? Enter your email."
- Best for: SEO traffic, social sharing, building trust
- Tradeoff: Lower email capture rate, but much higher usage

**Option B: Results-gated**
- The user fills in their inputs freely
- They must enter an email to see their results
- Best for: High-value outputs like audits or personalized reports
- Tradeoff: Higher email capture, but some people bounce before converting

**Option C: Fully gated**
- Email required before using the tool at all
- Best for: tools with expensive data or proprietary insights
- Tradeoff: Fewest users, but every user is a lead

**Rule of thumb:** Gate as little as possible. The more people who use your tool, the more who talk about it, link to it, and eventually buy from you.

### Step 5: Plan Distribution
A great tool with no distribution plan is just a page nobody visits. Plan for these three channels:

**Search (SEO)**
- Identify 3-5 keywords people search when they need this tool
- Examples: "[topic] calculator," "how to calculate [thing]," "[industry] ROI formula"
- Plan the page structure: tool at the top, educational content below (how to use it, what the numbers mean, benchmarks)

**Social and communities**
- Write a launch post showing the tool in action with real numbers
- Share in relevant communities where your audience asks questions
- Make the results easy to screenshot and share

**Backlinks**
- Reach out to bloggers who write about the topic — offer the tool as a resource for their readers
- Create a "powered by [your brand]" badge on shared results
- Write a companion blog post explaining the methodology behind the tool

### Step 6: Map the Conversion Path
The tool is not the end goal. It's the start of a relationship. Map out what happens after someone uses it:

```
Free tool → Email capture → Welcome email → Nurture sequence → Product offer
```

**Specifically:**
1. **Immediate:** User gets their result plus a tip on how to improve it
2. **Same day:** If they gave their email, send the results plus one bonus insight
3. **Days 2-5:** Send 2-3 educational emails related to the problem the tool solves
4. **Day 7:** Introduce your product as the next step — "You calculated X manually. Here's how to automate it."

The key is that the nurture sequence connects the free tool's problem to your product's solution.

---

## Output Format

Deliver the free tool strategy as a structured plan:

```markdown
# Free Tool Strategy: [Tool Name]

## Overview
**Tool type:** [Calculator / Grader / Generator / Analyzer / Template]
**Target user:** [Who this is for]
**Problem solved:** [The manual task or question this replaces]
**Connection to product:** [How this leads people toward your paid offering]

## Tool Design
**User inputs:**
1. [Field 1 — what it is and why you need it]
2. [Field 2]
3. [Field 3]

**Output/Results:**
- [What the user sees after submitting]
- [How results are formatted — numbers, grades, charts]
- [The one actionable takeaway]

## Gating Strategy
**Approach:** [Ungated / Results-gated / Fully gated]
**Email capture point:** [Where and how you ask for the email]
**What they get for sharing their email:** [Saved results, PDF report, bonus insights]

## Distribution Plan
**SEO keywords:**
1. [Keyword 1] — [estimated search intent]
2. [Keyword 2]
3. [Keyword 3]

**Launch plan:**
- [Where to share first]
- [Communities to post in]
- [People to reach out to]

**Backlink strategy:**
- [How to earn links naturally]

## Conversion Path
1. **Use tool** → [what happens]
2. **Capture email** → [what they get]
3. **Nurture sequence** → [email plan]
4. **Product introduction** → [how and when]

## Success Metrics
- [Metric 1 — e.g., monthly tool users]
- [Metric 2 — e.g., email capture rate]
- [Metric 3 — e.g., tool-to-trial conversion rate]
```

---

## Quality Checks

Before delivering the strategy, verify:

- [ ] **The tool solves a real, repeated problem** — not a one-time curiosity
- [ ] **It delivers value in under 60 seconds** — fast input, instant output
- [ ] **Inputs are minimal** — no more than 7 fields for version one
- [ ] **The connection to your product is natural** — not forced or confusing
- [ ] **Gating strategy matches the goal** — if you want traffic, don't over-gate
- [ ] **SEO keywords are realistic** — you can actually rank for them
- [ ] **Distribution plan has specific actions** — not just "share on social media"
- [ ] **Conversion path is mapped end to end** — from first visit to product introduction
- [ ] **The tool name is clear and descriptive** — someone should know what it does from the name alone
- [ ] **You checked what competitors offer** — your tool fills a gap or does it better

---

## Examples

### Example: ROI Calculator for Email Marketing Software

**User says:** "I want to create a free tool that brings in leads. We sell email marketing software for small businesses."

**You do:**
1. Search memory for product marketing context — found: email marketing platform, targets small business owners, $29/month starting price
2. Ask: "What manual task do your potential customers do before they find you?"
3. User: "They try to figure out if email marketing is even worth the investment. They don't know what kind of return to expect."
4. Ask: "So a calculator that shows them the potential ROI of email marketing would be useful?"
5. User: "Exactly. Something simple where they plug in their numbers and see what they could earn."
6. Research competitor tools — find that most ROI calculators are either too complicated (20+ fields) or too generic (not specific to email marketing)

**Strategy delivered:**

**Tool name:** Email Marketing ROI Calculator

**User inputs (5 fields):**
1. Monthly email list size
2. Average email open rate (provide industry default if they don't know)
3. Average order value or deal size
4. Number of emails sent per month
5. Current email marketing cost (or $0 if not doing it yet)

**Output:**
- Estimated monthly revenue from email marketing
- Projected annual ROI as a percentage
- Comparison: "Your list of 5,000 subscribers could generate $12,000/month based on industry benchmarks"
- One tip: "Your open rate is below industry average. Improving subject lines could increase revenue by 25%."

**Gating strategy:** Ungated tool with optional email capture
- Anyone can use the calculator freely
- After seeing results: "Want a detailed breakdown emailed to you with tips to improve each metric? Enter your email."
- The emailed report includes 3 personalized recommendations

**Distribution plan:**
- SEO targets: "email marketing ROI calculator," "is email marketing worth it," "email marketing revenue calculator"
- Launch in small business communities on Reddit and Facebook groups
- Reach out to 10 marketing bloggers who write "is email marketing worth it" posts — offer the calculator as an embedded resource

**Conversion path:**
1. User calculates their ROI and sees the potential revenue
2. Optional: enters email for the detailed report
3. Day 1 email: detailed report with personalized tips
4. Day 3 email: "3 ways small businesses are doubling their email ROI" (educational)
5. Day 5 email: "Here's how [product name] automates what you calculated" — introduces the product with a free trial offer

**Why this works:**
- Solves a real question small business owners have before buying any email tool
- Takes 30 seconds to use
- The results naturally make the case for investing in email marketing (and therefore in your tool)
- Ungated approach maximizes traffic and backlinks
- The optional email captures people who are most interested — warmer leads

---

## Related Skills

- **lead-magnets** — designs downloadable resources that pair well with free tools
- **page-cro** — optimizes the landing page where the tool lives for maximum conversions
- **analytics-tracking** — sets up tracking to measure tool usage, email captures, and conversion rates
- **product-marketing-context** — provides the audience and positioning foundation every tool strategy needs
- **seo-audit** — identifies keyword opportunities and technical requirements for the tool page

**Always check product-marketing-context before designing a free tool.** You need to know who you're building for and what problem your product solves.
