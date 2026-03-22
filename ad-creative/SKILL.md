---
name: ad-creative
description: Generate ad copy and creative concepts for paid campaigns across Google, Meta, LinkedIn, and other platforms. Use when you need ad headlines, copy variations, or creative direction for paid campaigns.
---

# Ad Creative

Generate ad copy and creative concepts for paid campaigns across Google, Meta, LinkedIn, and other platforms.

---

## When to Use

Trigger this skill when you hear:
- "Write ad copy for..."
- "I need ads for my campaign"
- "Create Google ads"
- "Help me with Facebook/Meta ads"
- "I need ad headlines"
- "Write some ad variations"
- "Help me with my ad creative"
- "I'm running a paid campaign and need copy"

This skill focuses on the words and creative direction — not the campaign setup, bidding, or targeting. For campaign structure, see **paid-ads**.

---

## Context Gathering

Before writing a single headline, you need to understand these things. Ask conversationally — don't dump a form on the user.

### Campaign Basics
1. **What platform are you running ads on?** (Google Search, Google Display, Meta/Facebook, Instagram, LinkedIn, TikTok, YouTube, etc.)
2. **What's the goal of this campaign?** (Drive traffic, get signups, sell a product, book demos, build awareness, retarget visitors, etc.)
3. **What's the specific action you want people to take?** (Click to landing page, fill out a form, buy something, download something, watch a video, etc.)

### Audience
4. **Who are you targeting?** (If product-marketing-context exists, confirm the audience matches. If not, ask who these ads are for.)
5. **Where are they in the buying journey?** (Never heard of you? Visited your site? Already considering you? Past customer?)

### Constraints
6. **Any specific offers or promotions to highlight?** (Discounts, free trials, limited-time deals, bonuses)
7. **Any words or claims you need to avoid?** (Compliance restrictions, brand guidelines, competitor mentions)
8. **Do you have existing ads to improve on?** (Share what's running now so we can beat it)

---

## Methodology

### Step 1: Check Product Marketing Context

Pull your stored product and brand information from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
agent(resource: "memory", action: "search", query: "brand voice")
```

If no context exists, tell the user: *"Before writing ads, I need to understand your product and who you're selling to. Let's set that up first."* Then run **product-marketing-context**.

If context exists, use it to ground every headline and description in real positioning — not generic filler.

### Step 2: Define Platform, Format, and Objective

Each platform has different rules and character limits. Confirm these before writing:

**Google Search Ads:**
- Headlines: up to 30 characters each (up to 15 headlines)
- Descriptions: up to 90 characters each (up to 4 descriptions)
- No emojis, limited punctuation rules

**Meta (Facebook/Instagram) Ads:**
- Primary text: 125 characters visible before "See more" (can be longer)
- Headline: 40 characters recommended
- Description: 30 characters recommended
- Emojis and casual tone usually perform well

**LinkedIn Ads:**
- Headline: 70 characters max
- Introductory text: 150 characters visible before "See more"
- Professional tone expected

**YouTube/Display:**
- Short headline: 30 characters
- Long headline: 90 characters
- Description: 90 characters

Lock in the platform and format before writing.

### Step 3: Generate 5 Headline Variants

Write 5 headlines, each using a different psychological angle:

1. **Pain angle** — Call out the problem they're experiencing right now
2. **Gain angle** — Promise the positive outcome they want
3. **Social proof angle** — Reference results, numbers, or other customers
4. **Urgency angle** — Create a reason to act now (real urgency, not fake scarcity)
5. **Curiosity angle** — Make them want to click to learn more

Each headline should:
- Stay within the platform's character limits
- Use the language your customers actually use (pull from product-marketing-context)
- Avoid generic phrases like "Best Solution" or "Industry-Leading"
- Include a keyword if it's a search ad

### Step 4: Generate 3 Description Variants

Write 3 descriptions that support the headlines:

1. **Feature-benefit description** — What it does and why that matters
2. **Proof description** — Back up the headline with a specific result or fact
3. **Action description** — Tell them exactly what happens next when they click

Descriptions should:
- Complement the headlines (don't repeat the same words)
- Include a clear call to action
- Address a potential objection if there's room

### Step 5: Suggest Image or Video Direction

For visual platforms (Meta, LinkedIn, Display, YouTube), suggest creative direction:

- **What to show:** Product in action, before/after, customer outcome, or attention-grabbing visual
- **Text overlay:** One key phrase to put on the image (if applicable)
- **Style guidance:** Photo vs illustration, colors, mood
- **Video hook (if applicable):** What happens in the first 3 seconds to stop the scroll

Use Nebo's browser to check what competitors are running for inspiration:
```
web(action: "navigate", url: "https://www.facebook.com/ads/library/")
web(action: "read_page")
```

Don't copy competitors — use their ads to find gaps and angles they're missing.

### Step 6: A/B Testing Recommendations

For every ad set, recommend a testing plan:

- **What to test first:** The single variable most likely to improve results (usually the headline angle)
- **How to structure the test:** Which variants to pit against each other and why
- **What "winning" looks like:** The metric to watch (click-through rate for awareness, conversion rate for signups, cost per acquisition for sales)
- **When to call it:** Minimum impressions or spend before making a decision (typically 1,000+ impressions per variant or 100+ clicks)
- **What to test next:** After you find a winning headline, what to test second (description, image, audience, landing page)

---

## Output Format

Present ad creative in this structure:

```markdown
# Ad Creative: [Campaign Name]
Platform: [Google Search / Meta / LinkedIn / etc.]
Objective: [What this campaign is trying to achieve]
Audience: [Who these ads target]

## Headlines

### 1. Pain Angle
[Headline text] ([character count])

### 2. Gain Angle
[Headline text] ([character count])

### 3. Social Proof Angle
[Headline text] ([character count])

### 4. Urgency Angle
[Headline text] ([character count])

### 5. Curiosity Angle
[Headline text] ([character count])

## Descriptions

### 1. Feature-Benefit
[Description text] ([character count])

### 2. Proof
[Description text] ([character count])

### 3. Action
[Description text] ([character count])

## Creative Direction
[Image/video suggestions for visual platforms]

## A/B Testing Plan
- **Test first:** [What to test and why]
- **Structure:** [Which variants to compare]
- **Winning metric:** [What to measure]
- **Minimum data:** [When to make a call]
- **Test next:** [Second variable to test]
```

---

## Quality Checks

Before delivering ad creative, verify:

- [ ] **Every headline is within character limits** — don't guess, count the characters
- [ ] **Headlines use 5 different angles** — not 5 variations of the same message
- [ ] **Copy matches brand voice** — check against the voice stored in product-marketing-context
- [ ] **No generic filler phrases** — "Best in class," "World-class," "Innovative" say nothing
- [ ] **Descriptions complement, not repeat, headlines** — they should add new information
- [ ] **Call to action is clear** — the reader knows exactly what happens when they click
- [ ] **Keyword is included for search ads** — headlines should contain the target keyword naturally
- [ ] **Claims are supportable** — don't write "#1 rated" unless the user actually has that claim
- [ ] **Urgency is real** — don't manufacture fake deadlines or scarcity
- [ ] **A/B test plan is actionable** — not vague advice, specific pairings and metrics

---

## Examples

### Example 1: Google Search Ads — B2B SaaS

**User says:** "I need Google Search ads for my email automation tool. We're targeting the keyword 'sales email automation.'"

**You do:**
1. Check memory — find product-marketing-context (email automation for sales teams, $49/mo, simple vs enterprise competitors)
2. Confirm: platform is Google Search, goal is free trial signups, audience is sales managers at small B2B companies
3. Generate creative:

**Headlines:**

| # | Angle | Headline | Characters |
|---|-------|----------|------------|
| 1 | Pain | Stop Sending Sales Emails by Hand | 30 |
| 2 | Gain | Automate Outreach in 10 Minutes | 28 |
| 3 | Social Proof | 2,400 Sales Teams Use This Daily | 29 |
| 4 | Urgency | Free Trial — Start Sending Today | 29 |
| 5 | Curiosity | Why Top Reps Send 3x More Emails | 30 |

**Descriptions:**

| # | Type | Description | Characters |
|---|------|-------------|------------|
| 1 | Feature-Benefit | Automate personalized outbound emails. Set up sequences in minutes, not weeks. | 75 |
| 2 | Proof | Sales teams send 3x more outbound emails in their first week. No setup fees. | 73 |
| 3 | Action | Start your free trial — create your first email sequence in under 10 minutes. | 74 |

**A/B Testing Plan:**
- Test first: Pain headline vs Gain headline — these test fundamentally different motivations
- Winning metric: Click-through rate (both drive to the same free trial page)
- Minimum data: 1,000 impressions per headline before comparing
- Test next: Once the winning headline is found, test Feature-Benefit vs Proof descriptions

### Example 2: Meta Ads — E-commerce Product

**User says:** "I need Facebook and Instagram ads for my skincare brand. We're launching a new moisturizer."

**You do:**
1. Check memory — find product-marketing-context (clean skincare brand, targets women 25-40, differentiator is dermatologist-formulated with no synthetic fragrances)
2. Confirm: platform is Meta (Facebook + Instagram), goal is product sales, audience is women 25-40 interested in clean beauty
3. Generate creative:

**Primary Text Variants:**

**Pain angle:**
Your moisturizer shouldn't come with ingredients you can't pronounce. We kept it simple — 12 clean ingredients, dermatologist-formulated, zero synthetic fragrances. Your skin knows the difference.

**Gain angle:**
Hydrated skin that actually lasts all day. Our new Daily Glow Moisturizer absorbs in seconds, works for 24 hours, and is made with only 12 clean ingredients your dermatologist would approve.

**Social proof angle:**
Over 5,000 women switched to Daily Glow in our pre-launch. Here's what they keep saying: "My skin has never felt this hydrated without feeling greasy." Now available to everyone.

**Headlines:**

| # | Angle | Headline | Characters |
|---|-------|----------|------------|
| 1 | Pain | Your Skin Deserves Better | 25 |
| 2 | Gain | All-Day Hydration, 12 Ingredients | 34 |
| 3 | Social Proof | 5,000 Women Made the Switch | 28 |
| 4 | Urgency | Launch Week — 20% Off Ends Sunday | 35 |
| 5 | Curiosity | The Moisturizer Derms Keep Recommending | 40 |

**Creative Direction:**
- Image option A: Close-up of the moisturizer being applied, natural lighting, dewy skin visible — no heavy editing
- Image option B: Flat lay of the product with the 12 ingredients displayed as real, recognizable items (aloe leaf, shea butter, etc.)
- Video hook (first 3 seconds): A hand flipping over a competitor product to show a long ingredient list, then flipping over Daily Glow to show a short, clean one
- Text overlay: "12 ingredients. That's it."
- Style: Clean, bright, minimal — matches the "clean beauty" positioning

**A/B Testing Plan:**
- Test first: Pain primary text vs Social proof primary text — these are the strongest emotional drivers
- Structure: Run both with the same image (option A) so the only variable is the copy
- Winning metric: Cost per purchase (since the goal is sales, not just clicks)
- Minimum data: $50-100 spend per variant or 50+ link clicks before comparing
- Test next: After finding the winning copy, test image option A vs image option B

---

## Related Skills

- **paid-ads** — campaign setup, targeting, bidding, and budget strategy (this skill writes the creative, that skill builds the campaign)
- **copywriting** — longer-form copy for landing pages, emails, and websites
- **marketing-psychology** — deeper frameworks for persuasion and buyer motivation
- **product-marketing-context** — the foundation every ad is built on (always check this first)
- **ab-test-setup** — detailed testing frameworks beyond simple ad variant testing
