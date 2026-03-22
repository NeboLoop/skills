---
name: page-cro
description: Audit any marketing page for conversion rate optimization — find what helps visitors take action and what gets in the way. Use when a landing page, homepage, or marketing page isn't converting well.
---

# Page CRO

Audit any marketing page for conversion rate optimization — find what's helping visitors take action and what's getting in the way.

---

## When to Use

Trigger this skill when you hear:
- "Review my landing page"
- "Why isn't my page converting?"
- "Can you audit this page?"
- "What's wrong with my website?"
- "How can I get more signups from this page?"
- "Check my homepage for conversion issues"
- "My page isn't working"
- Any request to improve a specific marketing page's performance

This skill requires a URL or screenshot to audit. If the user doesn't provide one, ask for it.

---

## Context Gathering

Before auditing, you need:

1. **The page URL** — so you can load and review it
2. **What the page should accomplish** — signups, demo requests, purchases, downloads, email captures?
3. **Who the page is for** — what kind of visitor lands here and what are they looking for?
4. **How visitors arrive** — paid ads, organic search, social media, email? (This affects expectations)
5. **Any known problems** — "people leave after 3 seconds" or "nobody clicks the button" helps you focus

Don't ask all five at once. Start with the URL and goal, then ask follow-ups as needed.

---

## Methodology

### Step 1: Check Product Marketing Context

Before looking at the page, understand the business:

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to evaluate whether the page matches the product's positioning, target customer, and brand voice.

If no context exists, tell the user: *"I don't have your marketing context saved yet. I can still audit your page, but the feedback will be more useful if I understand your product and customer first. Want to set that up now, or jump straight to the audit?"*

If they want context first, run the **product-marketing-context** skill. Otherwise, proceed and gather what you need from the page itself.

### Step 2: Load the Page

Use Nebo's browser to visit and capture the page:

```
web(action: "navigate", url: "the-page-url.com")
web(action: "screenshot")
web(action: "read_page")
```

Take screenshots at multiple scroll positions to see the full page. Pay attention to:
- What loads first (above the fold)
- How the page flows from top to bottom
- What the page looks like on mobile vs desktop

### Step 3: Check Above the Fold

The area visitors see before scrolling is the most important real estate on the page. Check:

- **Headline clarity** — Can a stranger understand what this product does within 5 seconds? Is it specific or vague?
- **Value proposition** — Does the page answer "why should I care?" immediately? Is the benefit to the visitor clear?
- **Primary call to action** — Is there a button or action visible without scrolling? Is it obvious what happens when you click it?
- **Visual hierarchy** — Does the eye naturally go to the most important elements first? Or is the page cluttered and confusing?
- **Supporting image or visual** — Does the image help explain the product, or is it generic stock photography?

Score this area 1-5 and note specific fixes.

### Step 4: Check Social Proof

People look for evidence that others trust this product. Check for:

- **Testimonials** — Are there quotes from real customers with names, titles, or photos? Or are they anonymous and generic?
- **Company logos** — Are recognizable brands displayed? Do they actually use the product?
- **Numbers** — User counts, results achieved, years in business? Are they specific ("12,847 teams") or vague ("thousands")?
- **Case studies** — Are there detailed examples of customer results?
- **Trust badges** — Security certifications, press mentions, awards, ratings?
- **Placement** — Is social proof positioned where visitors need reassurance (near the call to action, near pricing)?

Score this area 1-5 and note specific fixes.

### Step 5: Check Friction

Every extra step or moment of confusion reduces conversions. Check:

- **Form length** — How many fields are required? Can any be removed or asked later?
- **Required fields** — Are you asking for information that isn't necessary at this stage (phone number on a free trial signup)?
- **Loading speed** — Does the page feel fast? Do images take time to appear? Is there layout shifting?
- **Mobile layout** — Does the page work well on a phone? Are buttons big enough to tap? Is text readable without zooming?
- **Navigation distractions** — Are there too many links pulling visitors away from the main action?
- **Confusing steps** — Is it clear what happens after the visitor takes action? "Start free trial" is better than "Submit"

Score this area 1-5 and note specific fixes.

### Step 6: Check Objection Handling

Visitors have doubts. Good pages address them before they become reasons to leave. Check:

- **Pricing clarity** — Can visitors understand the cost without hunting for it? Are there hidden fees or confusing tiers?
- **Risk reversal** — Is there a free trial, money-back guarantee, or "no credit card required" message?
- **FAQ section** — Does it answer the real objections people have, not just easy questions?
- **Comparison content** — Is there anything that helps visitors understand how this differs from alternatives?
- **Privacy and security** — If you're asking for personal information, do you address data safety concerns?
- **Support access** — Can visitors easily find help if they have questions before buying?

Score this area 1-5 and note specific fixes.

### Step 7: Score and Prioritize

Create the final scorecard (see Output Format below). For each area:
- Give a score from 1 to 5
- List what's working well
- List specific improvements with expected impact
- Prioritize fixes by impact and effort:
  - **Quick wins** — high impact, low effort (do these first)
  - **Big projects** — high impact, high effort (plan these next)
  - **Small tweaks** — low impact, low effort (batch these together)
  - **Deprioritize** — low impact, high effort (skip for now)

### Step 8: Deliver Recommendations

Present findings conversationally. Lead with the biggest opportunities, not a laundry list. Tell the user:
- The single most important thing to fix
- The 3 quick wins they can do today
- What's already working well (so they don't break it)

---

## Output Format

The audit report follows this structure:

```markdown
# Page CRO Audit
**Page:** [URL]
**Goal:** [What the page should accomplish]
**Audited:** [DATE]

## Overall Score: [X/5]

## Above the Fold — [X/5]
**What's working:**
- [Positive observations]

**What needs improvement:**
- [Issue] — [Specific recommendation]
- [Issue] — [Specific recommendation]

## Social Proof — [X/5]
**What's working:**
- [Positive observations]

**What needs improvement:**
- [Issue] — [Specific recommendation]
- [Issue] — [Specific recommendation]

## Friction — [X/5]
**What's working:**
- [Positive observations]

**What needs improvement:**
- [Issue] — [Specific recommendation]
- [Issue] — [Specific recommendation]

## Objection Handling — [X/5]
**What's working:**
- [Positive observations]

**What needs improvement:**
- [Issue] — [Specific recommendation]
- [Issue] — [Specific recommendation]

## Priority Fixes

### Quick Wins (do this week)
1. [Fix] — [Why it matters]
2. [Fix] — [Why it matters]
3. [Fix] — [Why it matters]

### Big Projects (plan next)
1. [Project] — [Expected impact]

### Already Working Well (don't break these)
- [Thing that's good]
- [Thing that's good]
```

---

## Quality Checks

Before delivering the audit, verify:

- [ ] **You actually loaded the page** — don't guess based on the URL alone
- [ ] **Every score has specific evidence** — not just "this could be better" but exactly what's wrong and how to fix it
- [ ] **Recommendations are actionable** — the user should know exactly what to change, not just what's wrong
- [ ] **Fixes include expected impact** — tell them why each change matters
- [ ] **Priority order makes sense** — quick wins before big projects
- [ ] **You checked mobile layout** — not just desktop
- [ ] **You referenced product context** — if available, recommendations align with their positioning and target customer
- [ ] **Language is plain and clear** — no "CTA optimization" or "value proposition hierarchy" — say "make the button clearer" or "lead with the benefit"
- [ ] **You noted what's working** — not just a list of problems, but what they should keep doing
- [ ] **Recommendations are realistic** — don't suggest "hire a design team" to a solo founder

---

## Examples

### Example: SaaS Landing Page Audit

**User says:** "Can you check my landing page? It's getflowboard.com. We get decent traffic from Google but only 2% are signing up for the free trial."

**You do:**

1. Search memory for product marketing context — found:
   - Product: Project management tool for creative teams
   - Customer: Creative directors at agencies with 10-30 people
   - Differentiation: Visual-first boards designed for creative workflows, not generic task lists
   - Goal: Increase free trial signups

2. Load the page with Nebo's browser, take screenshots at top, middle, and bottom

3. **Above the Fold — 2/5**
   - Headline says "The Future of Project Management" — too vague, doesn't tell creative directors this is built for them
   - No subheadline explaining what the product actually does
   - Call to action says "Get Started" but it's the same color as the background, hard to spot
   - Hero image is an abstract illustration instead of showing the actual product
   - **Fixes:**
     - Change headline to something specific: "Project management built for creative teams" — tells visitors immediately if this is for them
     - Add a subheadline: "Visual boards that match how your team actually works — not another spreadsheet disguised as a project tool"
     - Make the signup button a contrasting color and change text to "Start free trial — no credit card"
     - Replace illustration with a screenshot of the product showing a real creative project board

4. **Social Proof — 3/5**
   - Has three customer logos but they're tiny and hard to recognize
   - One testimonial at the bottom but it's generic: "Great tool!" — no specifics
   - No numbers (how many teams use it? What results do they get?)
   - **Fixes:**
     - Make logos larger, add "Trusted by 400+ creative teams" above them
     - Replace the testimonial with a specific result: "We cut our project kickoff time from 2 weeks to 3 days" with the person's name, title, and company
     - Add a small case study section: "How Studio Nine reduced revision rounds by 40%"

5. **Friction — 4/5**
   - Signup form only asks for email — that's good
   - Page loads quickly
   - Mobile layout works but the signup button is below the fold on phones
   - **Fixes:**
     - Add a sticky signup button on mobile so it's always visible
     - The navigation has 8 links competing for attention — simplify to just "Features," "Pricing," and the signup button

6. **Objection Handling — 2/5**
   - No pricing information anywhere on this page — visitors have to click through to find it
   - No mention of a free trial length (is it 7 days? 30 days?)
   - No FAQ section
   - No "cancel anytime" or guarantee language
   - **Fixes:**
     - Add "Free for 14 days, no credit card required" near the signup button
     - Add a brief FAQ addressing: "What happens after the trial?", "Can I import from Asana/Trello?", "Is my data secure?"
     - Add a small pricing preview: "Plans start at $12/user/month" so visitors know the range before signing up

7. **Priority Fixes:**

   **Quick wins (this week):**
   1. Change the headline to speak directly to creative teams — vague headlines are the number one reason visitors leave without reading further
   2. Add "Free for 14 days, no credit card required" next to the signup button — removes the biggest hesitation
   3. Make the signup button a contrasting color — it blends in right now and visitors can't find it

   **Big projects (plan next):**
   1. Replace the hero illustration with a product screenshot — visitors need to see what they're signing up for
   2. Build a mini case study section with real customer results — specific numbers are more convincing than any amount of marketing copy

   **Already working well:**
   - One-field signup form (email only) — keep this, low friction is an advantage
   - Fast page load time — don't add heavy animations or videos that slow it down
   - Clean, uncluttered design — the layout is good, it just needs stronger content

---

## Related Skills

- **copywriting** — rewrite headlines, buttons, and page copy based on audit findings
- **signup-flow-cro** — audit what happens after the visitor clicks the signup button
- **form-cro** — optimize form fields, layout, and validation for higher completion
- **popup-cro** — audit exit-intent popups, slide-ins, and other overlays
- **product-marketing-context** — establish the positioning and customer profile that informs every audit
- **marketing-psychology** — apply behavioral principles to increase persuasion on the page
