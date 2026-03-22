---
name: pricing-strategy
description: Design pricing, packaging, and monetization strategy that maximizes revenue while aligning with customer value. Use when setting prices, structuring tiers, choosing a value metric, deciding on free vs paid plans, designing a pricing page, or restructuring pricing for an existing product.
---

# Pricing Strategy

Design pricing, packaging, and monetization strategy that maximizes revenue while aligning with how customers perceive and receive value.

---

## When to Use

Trigger this skill when you hear:
- "How should I price this?"
- "What should my pricing look like?"
- "Help me with pricing tiers"
- "Should I offer a free plan?"
- "My pricing isn't working"
- "How do I package my product?"
- "Should I do monthly or annual?"
- "I need a pricing page"
- "What should I charge?"
- "Help me monetize"

This skill requires product-marketing-context. If it doesn't exist, run that skill first.

---

## Context Gathering

Ask these questions conversationally. Don't dump them all at once.

### Current State
1. **What do you charge today?** (If anything — free, one-time, subscription, usage-based)
2. **How did you arrive at that price?** (Gut feel? Copied a competitor? Cost-plus? Value-based?)
3. **What's your cost to serve one customer?** (Rough number — hosting, support, time)

### Value Understanding
4. **What's the most valuable thing your product does for customers?** (The thing they'd pay for even if everything else disappeared)
5. **How do customers measure the value they get?** (Revenue generated, time saved, deals closed, projects managed)
6. **What's the outcome worth to them?** (If you save them 10 hours/week and they bill $200/hr, that's $2k/week in value)

### Customer Segments
7. **Do different customers use your product differently?** (Power users vs casual, big teams vs individuals, different use cases)
8. **Which customers get the most value?** (Who would be devastated if you disappeared?)
9. **Are there customers you don't want?** (Too demanding, too small, wrong fit)

### Market Position
10. **What do competitors charge?** (If you know — we'll research this too)
11. **Where do you want to be positioned?** (Premium? Mid-market? Affordable alternative? Free-with-paid-tier?)
12. **What's your growth priority right now?** (Maximize users? Maximize revenue? Maximize retention?)

### Constraints
13. **Any pricing constraints?** (Existing contracts, promises made, market expectations, investor requirements)
14. **What's your billing infrastructure?** (Stripe? Custom? Nothing yet? Determines what's possible)

---

## Methodology

### Step 1: Check Product Marketing Context

Always start here. You need to understand the product, customer, and positioning before pricing.

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the product-marketing-context skill first. Pricing without positioning is guessing.

Pull these from memory:
- Product description and how it works
- Ideal customer profile and pain points
- Differentiation and competitive alternatives
- Current pricing model (if stored)
- Brand voice (pricing pages need copy too)

### Step 2: Understand the Value Metric

The value metric is the unit of measurement that aligns price with the value customers receive. This is the single most important pricing decision.

**Good value metrics:**
- Per user (Slack, Notion) — works when more users = more value
- Per transaction (Stripe, Shopify) — works when you help generate revenue
- Per unit of work (emails sent, projects, storage) — works when usage = value
- Flat rate (Basecamp) — works when simplicity is the brand

**Bad value metrics:**
- Per feature (punishes adoption)
- Per API call (unpredictable bills, creates anxiety)
- Per seat when seats don't correlate with value

Ask: *"When your customers get more value from your product, what number goes up?"* That number is your value metric candidate.

### Step 3: Research Competitor Pricing

Use Nebo's browser to research how competitors and alternatives price:

```
web(action: "navigate", url: "competitor-pricing-page.com/pricing")
web(action: "read_page")
```

For each competitor, capture:
- **Tier names and prices** (what they call each plan)
- **Value metric** (per user, per project, flat rate)
- **Feature gating** (what's in each tier)
- **Free tier** (if any, what's included)
- **Annual discount** (typically 15-20%)
- **Enterprise tier** (custom pricing, what triggers it)

Build a comparison matrix:

| Feature | Competitor A | Competitor B | Competitor C |
|---------|-------------|-------------|-------------|
| Pricing model | Per user | Flat rate | Usage-based |
| Entry price | $12/mo | $49/mo | $0.01/email |
| Mid-tier | $25/mo | $99/mo | $0.005/email |
| Free plan | Yes (3 users) | No (14-day trial) | Yes (100/mo) |
| Annual discount | 17% | 20% | None |
| Value metric | Users | Features | Volume |

### Step 4: Design Tier Structure

Most products need 3-4 tiers. Each tier serves a specific purpose:

**Tier 1 — Free or Entry (Acquisition)**
- Purpose: remove friction, build habit, create upgrade pressure
- Include: enough to experience core value, but hit natural limits
- Limit by: volume, features, team size, or time
- Name it something inviting: Free, Starter, Personal

**Tier 2 — Core (Revenue)**
- Purpose: where most customers land, your bread and butter
- Include: everything a typical customer needs
- Price at: where value clearly exceeds cost for the ideal customer
- Name it something aspirational: Pro, Growth, Team

**Tier 3 — Premium (Expansion)**
- Purpose: capture more value from power users and bigger teams
- Include: advanced features, higher limits, priority support
- Price at: 2-3x the core tier (justifiable by expanded value)
- Name it something elevated: Business, Scale, Premium

**Tier 4 — Enterprise (Optional, High-value)**
- Purpose: land large accounts with custom needs
- Include: SSO, SLA, dedicated support, custom integrations, admin controls
- Price at: custom/contact us (allows deal-by-deal negotiation)
- Name it: Enterprise

**Tier design rules:**
- Each tier should have a clear "who is this for"
- Features should flow logically (every tier includes all features from the tier below)
- The jump between tiers should feel natural, not punitive
- Avoid more than 5 tiers (decision paralysis)

### Step 5: Apply Price Anchoring

Price anchoring makes the recommended tier feel like the best deal.

**Anchoring tactics:**
1. **Put the most expensive tier first** (left to right on pricing page) — makes everything else feel reasonable
2. **Or highlight the middle tier** — the "recommended" badge draws attention, surrounding tiers make it feel balanced
3. **Show the enterprise/custom tier** — even if few people buy it, it anchors the perception of value
4. **Display monthly price even for annual billing** — "$25/mo billed annually" feels smaller than "$300/year"
5. **Show savings on annual plans** — "Save $60/year" or "2 months free" makes annual feel like a win
6. **Use charm pricing selectively** — $49 vs $50 works for SMB; round numbers ($100, $500) work for enterprise (signals confidence)

**Decoy pricing:**
If you want to push people to a specific tier, create a tier nearby that's slightly worse value. Example:
- Basic: $29/mo (5 users)
- Pro: $49/mo (15 users) ← best value, recommended
- Plus: $39/mo (7 users) ← decoy, makes Pro look like an obvious upgrade

### Step 6: Structure Annual vs Monthly Discounts

Annual plans improve cash flow and reduce churn. Standard discount is 15-20%.

**Calculating the discount:**
- 15% discount = ~2 months free ("Get 2 months free with annual billing")
- 17% discount = ~2 months free (cleaner math)
- 20% discount = ~2.4 months free ("Save 20%")

**Presenting the discount:**
- Default to annual pricing on the pricing page (toggle to monthly)
- Show the monthly equivalent: "$25/mo billed annually" not "$300/year"
- Frame as savings: "Save $120/year" or "2 months free"
- Don't go above 25% discount — it devalues the monthly price

**Monthly pricing role:**
- Monthly exists for customers who aren't sure yet
- Monthly is the "try before you commit" option
- Monthly price should feel fair, not punitive — don't make it 2x the annual equivalent

### Step 7: Design the Pricing Page

The pricing page is a conversion page. It needs to sell, not just list prices.

**Page structure (top to bottom):**

1. **Headline** — state the value, not "Pricing"
   - Good: "Plans that grow with your team"
   - Bad: "Pricing" or "Our Plans"

2. **Billing toggle** — Annual (default, highlighted) / Monthly
   - Show savings: "Save 20%" badge on annual

3. **Tier comparison table** — the core of the page
   - Recommended tier gets a badge ("Most Popular" or "Best Value")
   - Recommended tier is visually elevated (different color, slightly larger, border)
   - Each tier shows: name, price, one-sentence description, feature list, CTA button
   - CTA copy varies by tier: "Start Free" / "Start Trial" / "Get Started" / "Contact Sales"

4. **Feature comparison grid** — expandable or below the tiers
   - Group features by category
   - Use checkmarks and dashes, not text
   - Highlight where tiers differ

5. **Social proof** — logos, testimonials, or stats between sections
   - "Trusted by 5,000+ teams"
   - Customer quote about value/ROI

6. **FAQ section** — handle objections
   - "Can I switch plans?" (Yes, anytime)
   - "What happens if I go over my limit?" (Upgrade prompt, not cutoff)
   - "Do you offer discounts for nonprofits/education?" (If applicable)
   - "Can I cancel anytime?" (Yes, no long-term contracts)
   - "What payment methods do you accept?" (Cards, invoicing for enterprise)

7. **Final CTA** — repeat the recommended plan's call-to-action

---

## Output Format

Deliver the pricing strategy as a structured document:

```markdown
# Pricing Strategy: [Product Name]
Created: [DATE]

## Value Metric
**Primary metric:** [What you charge by — per user, per project, flat rate]
**Why this metric:** [How it aligns price with customer value]

## Competitive Landscape
| | Competitor A | Competitor B | You (Proposed) |
|---|---|---|---|
| Model | [Their model] | [Their model] | [Your model] |
| Entry price | [Price] | [Price] | [Price] |
| Mid-tier | [Price] | [Price] | [Price] |
| Free plan | [Y/N] | [Y/N] | [Y/N] |
| Annual discount | [%] | [%] | [%] |

## Positioning
[Where you sit relative to competitors — cheaper, premium, different model entirely]

## Tier Structure

### [Tier 1 Name] — [Free / $X/mo]
**Who it's for:** [Persona]
**Includes:**
- [Feature 1]
- [Feature 2]
- [Feature 3]
**Limits:** [What's capped]
**Goal:** [Acquisition / activation / habit]

### [Tier 2 Name] — $X/mo (annual) / $X/mo (monthly)
**Who it's for:** [Persona]
**Includes:** Everything in [Tier 1], plus:
- [Feature 1]
- [Feature 2]
- [Feature 3]
**Limits:** [What's capped or unlimited]
**Goal:** [Core revenue]
**Recommended:** Yes — badge this as "Most Popular" or "Best Value"

### [Tier 3 Name] — $X/mo (annual) / $X/mo (monthly)
**Who it's for:** [Persona]
**Includes:** Everything in [Tier 2], plus:
- [Feature 1]
- [Feature 2]
- [Feature 3]
**Limits:** [Higher or unlimited]
**Goal:** [Expansion revenue]

### [Tier 4 Name] — Custom
**Who it's for:** [Persona]
**Includes:** Everything in [Tier 3], plus:
- [Custom feature 1]
- [Custom feature 2]
**Goal:** [Land large accounts]

## Annual vs Monthly
- **Annual discount:** [X]% ([framing — "2 months free" or "Save $X"])
- **Default display:** Annual
- **Monthly positioning:** [Try before you commit]

## Price Anchoring
- **Recommended tier:** [Tier name]
- **Anchoring approach:** [How the other tiers make this one feel right]
- **Decoy consideration:** [If applicable]

## Pricing Page Wireframe
[Describe the layout, headline, toggle, tier cards, comparison grid, FAQ topics, and CTA]

## Migration Plan (if changing pricing)
- **Existing customers:** [Grandfathered? Migrated? Timeline?]
- **Communication:** [How to announce the change]
- **Rollout:** [Phased or all-at-once]
```

---

## Quality Checks

Before delivering the pricing strategy, verify:

- [ ] **Value metric aligns with customer value** — price goes up as value goes up
- [ ] **Each tier has a clear persona** — you can name who each tier is for
- [ ] **Tier progression is logical** — features flow up, no weird gaps or overlaps
- [ ] **Prices are defensible** — you can explain why each number was chosen
- [ ] **Competitive positioning is intentional** — you know where you sit and why
- [ ] **Annual discount is 15-20%** — not too low (no incentive) or too high (devalues monthly)
- [ ] **No more than 4-5 tiers** — simplicity wins, decision fatigue kills conversion
- [ ] **Pricing page has a recommended tier** — guide the customer, don't make them guess
- [ ] **FAQ handles top objections** — switching, canceling, overage, payment methods
- [ ] **Free tier has upgrade pressure** — if you offer free, there must be a natural reason to upgrade
- [ ] **No made-up numbers** — prices are grounded in competitor research, value analysis, or cost-plus reasoning
- [ ] **Strategy accounts for existing customers** — if changing prices, migration plan exists

---

## Examples

### Example 1: B2B SaaS — Project Management Tool

**User says:** "How should I price my project management tool?"

**You do:**
1. Check memory for product-marketing-context (found: PM tool for small agencies, competing with Asana/Monday, differentiation is simplicity)
2. Ask: "What's your value metric — what goes up as customers get more value? More users? More projects? More storage?"
3. User: "More users on the team."
4. Ask: "What do you charge now?"
5. User: "Nothing yet, still in beta."
6. Research competitor pricing pages for Asana, Monday, Basecamp, ClickUp
7. Ask: "Where do you want to sit — cheaper than Monday, similar, or premium?"
8. User: "Cheaper and simpler. We're the anti-complexity option."

**Delivered strategy:**
- **Value metric:** Per user/month
- **Free:** Up to 3 users, 5 projects (enough for a freelancer to get hooked)
- **Pro:** $8/user/mo annual, $10/user/mo monthly (small agencies, unlimited projects)
- **Business:** $16/user/mo annual, $20/user/mo monthly (larger teams, reporting, permissions)
- **Enterprise:** Custom (SSO, SLA, dedicated support)
- **Anchoring:** Pro is recommended, positioned as "Most Popular"
- **Annual discount:** 20% ("2 months free")
- **Pricing page:** "Simple plans for teams that ship" headline, annual default, comparison grid with checkmarks

### Example 2: Creator Tool — Moving from Free to Paid

**User says:** "I have a free tool with 10,000 users. How do I start charging?"

**You do:**
1. Check memory for product-marketing-context (found: design tool for content creators, free with no revenue)
2. Ask: "What features do your most active users rely on most?"
3. User: "Templates and the AI writing assistant."
4. Ask: "What would users be most willing to pay for vs what feels like it should stay free?"
5. User: "The AI stuff costs me money, so that needs to be paid. Basic templates should stay free."
6. Research Canva, Figma, and similar creator tool pricing
7. Ask: "Do you want to keep a generous free tier or push toward paid?"
8. User: "Keep free generous. I want the community. But I need revenue."

**Delivered strategy:**
- **Value metric:** Flat rate with AI usage limits
- **Free:** Core editor, 50 templates, 10 AI generations/month (enough to experience value)
- **Pro:** $12/mo annual, $15/mo monthly (all templates, 200 AI generations/month, premium exports)
- **Team:** $29/mo annual, $35/mo monthly (collaboration, shared brand kit, 1000 AI generations/month)
- **Migration:** Email existing users about new Pro features, give power users 30 days free Pro, don't remove any currently-free features
- **Anchoring:** Pro is recommended, free tier creates natural upgrade pressure when AI limit hits
- **Annual discount:** 20% ("Save $36/year")

### Example 3: Service Business — Restructuring Pricing

**User says:** "I'm a consultant and I want to move from hourly to value-based pricing."

**You do:**
1. Check memory for product-marketing-context (found: marketing consultant for ecommerce brands)
2. Ask: "What outcomes do your clients typically get? Revenue increase, conversion lift, something else?"
3. User: "I usually help them increase email revenue by 30-50%."
4. Ask: "What's the average revenue impact in dollars?"
5. User: "Most clients see an extra $10-20k/month in email revenue."
6. Ask: "And what do you charge now per hour?"
7. User: "$150/hr, usually 20 hours a month."

**Delivered strategy:**
- **Current:** $3,000/mo ($150 x 20hrs) for $10-20k/mo in value (capturing 15-30% of value created)
- **Proposed packages:**
  - **Email Audit + Strategy:** $2,500 one-time (entry point, leads to retainer)
  - **Growth Retainer:** $5,000/mo, 3-month minimum (core offer: full email program management)
  - **Scale Partner:** $8,000/mo + 5% of incremental email revenue (aligned incentives, premium)
- **Anchoring:** Growth Retainer is recommended, Scale Partner makes it look reasonable
- **Why it works:** You're capturing 25-50% of value created, justified by measurable ROI

---

## Related Skills

- **paywall-upgrade-cro** — optimizes the upgrade flow and paywall experience once tiers are defined
- **page-cro** — audits and improves the pricing page conversion rate
- **competitor-alternatives** — deep competitive analysis to inform pricing position
- **product-marketing-context** — foundational context this skill depends on (always runs first)
- **sales-enablement** — creates sales materials that justify pricing to buyers
