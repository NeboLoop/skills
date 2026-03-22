---
name: marketing-psychology
description: Apply behavioral science and persuasion principles to make your copy, pages, and flows more compelling. Use when reviewing a page or email for persuasion opportunities, choosing psychological principles like social proof or anchoring, or improving conversion through ethical influence techniques.
---

# Marketing Psychology

Apply behavioral science and persuasion principles to marketing — making your copy, pages, and flows more compelling by understanding how people actually make decisions.

---

## When to Use

Trigger this skill when you hear:
- "Why aren't people converting?"
- "How do I make this more persuasive?"
- "What psychological principles should I use?"
- "Review this page/email/ad for persuasion"
- "How do I get people to take action?"
- "Make this copy more compelling"
- "What's the psychology behind this?"
- "Help me influence the buying decision"
- "Is this manipulative or just good marketing?"

This skill works best when you already have copy, a page, or a flow to improve. If you're starting from scratch, use **copywriting** or **ad-creative** first, then layer psychology on top.

---

## Context Gathering

Before applying any principles, understand the situation. Ask conversationally:

### The Decision
1. **What action do you want someone to take?** (Buy, sign up, upgrade, book a call, share — be specific)
2. **Where does this happen?** (Landing page, email, checkout flow, ad, pricing page, onboarding)
3. **What's stopping them right now?** (Price concerns, trust issues, confusion, "I'll do it later," comparing alternatives)

### The Audience
4. **Who is the person making this decision?** (Role, mindset, awareness level)
5. **What do they already believe?** (About the problem, about solutions, about your brand)
6. **What's at stake for them?** (Low-risk impulse buy vs. career-defining software purchase)

### The Current State
7. **Show me what you have now** (The page, email, flow, or copy you want to improve)
8. **What data do you have?** (Conversion rates, drop-off points, heatmaps, user feedback)

If marketing context exists in memory, pull it:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

Use the customer profile, positioning, and voice to tailor recommendations.

---

## Methodology

### Step 1: Identify the Decision Being Influenced

Every piece of marketing asks someone to make a decision. Name it clearly:

- "Decide to start a free trial instead of leaving the page"
- "Choose the annual plan instead of monthly"
- "Reply to this cold email instead of ignoring it"
- "Add a second item to the cart before checkout"

Be specific. "Convert more" is not a decision. "Click the signup button after reading the hero section" is.

Map where this decision sits in the buyer's journey:
- **Unaware** — they don't know they have a problem yet
- **Problem-aware** — they know the pain but not the solution
- **Solution-aware** — they know solutions exist but not yours
- **Product-aware** — they know you but haven't decided
- **Most aware** — they know you and just need a push

The stage determines which principles apply most.

### Step 2: Select Applicable Principles

Not every principle applies to every situation. Choose the ones that fit. Here are the core principles, explained simply:

**Social Proof**
People look to others when uncertain. If others are doing it, it feels safer.
- Reviews, testimonials, customer counts, logos, case studies
- Works best when: the audience is uncertain or the product is unfamiliar
- Example: "Join 10,000 teams who switched this quarter"

**Scarcity**
When something is limited, it feels more valuable. We act faster when we might miss out.
- Limited time, limited quantity, limited access
- Works best when: there's genuine scarcity (real deadline, real inventory limit)
- Example: "Early pricing ends Friday — 40% off for founding members"

**Reciprocity**
When someone gives us something, we feel compelled to give back.
- Free tools, valuable content, free trials, helpful advice before the ask
- Works best when: the gift feels genuine and useful, not transactional
- Example: Free website audit before pitching SEO services

**Anchoring**
The first number or piece of information shapes how we evaluate everything after.
- Show the expensive plan first. Show the "before" cost. Compare to alternatives.
- Works best when: pricing or value needs context
- Example: "Agencies charge $5,000/month. Get the same results for $199."

**Loss Aversion**
Losing something hurts roughly twice as much as gaining something of equal value.
- Frame what they'll lose by not acting, not just what they'll gain
- Works best when: the audience is aware of the problem but procrastinating
- Example: "Every week without this, you're losing ~12 hours to manual work"

**Authority**
We trust experts and credible sources more than unknowns.
- Credentials, media mentions, expert endorsements, data, certifications
- Works best when: trust is the main barrier
- Example: "Recommended by [Industry Publication]" or "Built by engineers from Google and Stripe"

**Commitment and Consistency**
Once we take a small step, we want to stay consistent with that action.
- Start with a tiny ask. Get a micro-commitment. Build from there.
- Works best when: the main ask feels too big as a first step
- Example: "Start with our free plan" before asking for paid. Quiz before signup.

**Framing**
How you present information changes how people perceive it. Same facts, different story.
- "95% uptime" vs. "Only 5% downtime" — same number, different feeling
- Works best when: you need to shift perception without changing the facts
- Example: "$3.30/day" feels smaller than "$99/month" even though it's the same

**Endowment Effect**
Once we feel ownership over something, we value it more and don't want to give it up.
- Free trials, customization, "your dashboard," personalization
- Works best when: you can let people experience the product before paying
- Example: "Your custom report is ready" — it's already theirs

**Default Bias**
People tend to stick with the pre-selected option. The default wins most of the time.
- Pre-select the plan you want people to choose. Opt-in vs. opt-out matters.
- Works best when: there's a choice architecture you can influence
- Example: Annual billing pre-selected on pricing page. "Most popular" badge on middle tier.

### Step 3: Apply to the Specific Context

Take the principles you selected and apply them directly to the user's copy, page, or flow.

For each recommendation:
1. **Point to the specific element** — "Your hero headline," "The pricing toggle," "Step 3 of checkout"
2. **Name the principle** — "This is anchoring"
3. **Explain why it fits here** — "Your visitors are comparing you to agencies, so anchoring against agency pricing reframes your value"
4. **Give the specific change** — exact copy, layout suggestion, or flow modification

Don't just list principles. Show exactly where and how to apply them.

### Step 4: Show Before and After

Present changes as clear before/after comparisons with annotations explaining the psychology at work.

**Format:**

```
BEFORE:
"Start your free trial today."

AFTER:
"Join 2,400 marketing teams already using [Product]. Start free — no credit card needed."

WHY IT WORKS:
- Social proof ("2,400 marketing teams") reduces uncertainty
- Specificity ("marketing teams") makes it feel relevant to the reader
- Risk removal ("no credit card") eliminates loss aversion barrier
- "Already using" implies momentum — consistency principle
```

For pages or flows, walk through the experience step by step and annotate each change.

### Step 5: Distinguish Ethical Persuasion from Manipulation

This step is not optional. For every recommendation, check:

**Ethical persuasion:**
- Helps people make decisions they'll be happy with later
- Highlights real benefits and genuine value
- Uses true scarcity (actual deadlines, actual limits)
- Reduces friction for people who already want to act
- Respects the customer's ability to say no

**Manipulation (avoid):**
- Creates false urgency ("Only 2 left!" when there are thousands)
- Uses dark patterns (hiding unsubscribe, pre-checked add-ons people don't want)
- Exploits fear beyond the real problem
- Makes it hard to cancel, return, or change your mind
- Tricks people into actions they'll regret

If a recommendation feels borderline, flag it: *"This uses scarcity. Make sure the deadline is real — fake urgency erodes trust fast and damages your brand long-term."*

**The test:** Would you be comfortable if your customer knew exactly what psychological principle you were using and why? If yes, it's persuasion. If no, it's manipulation.

---

## Output Format

Structure your analysis like this:

```markdown
# Psychology Review: [Page/Email/Flow Name]

## Decision Being Influenced
[One clear sentence describing the action and the audience]

## Buyer Awareness Stage
[Where they are in the journey and what that means for persuasion]

## Current State
[Brief assessment of what's working and what's missing psychologically]

## Recommendations

### 1. [Element] — [Principle Name]
**Before:** [Current version]
**After:** [Improved version]
**Why:** [1-2 sentences explaining the psychology]

### 2. [Element] — [Principle Name]
**Before:** [Current version]
**After:** [Improved version]
**Why:** [1-2 sentences explaining the psychology]

[Continue for each recommendation]

## Priority Order
1. [Highest-impact change] — [Expected effect]
2. [Second-highest] — [Expected effect]
3. [Third] — [Expected effect]

## Ethics Check
[Note any recommendations that need careful implementation to stay ethical]
```

---

## Quality Checks

Before delivering recommendations, verify:

- [ ] **The decision is clearly identified** — not vague "convert more" but a specific action
- [ ] **Principles match the situation** — not every principle applies everywhere; only recommend what fits
- [ ] **Recommendations are specific** — exact copy, exact placement, exact changes (not "add social proof somewhere")
- [ ] **Before/after is shown** — the user can see exactly what changes and why
- [ ] **Psychology is explained simply** — no academic jargon; the user understands why each change works
- [ ] **Ethics are addressed** — every recommendation passes the transparency test
- [ ] **Brand voice is preserved** — persuasion techniques are adapted to match the user's tone and style
- [ ] **Recommendations are prioritized** — the user knows what to change first for maximum impact
- [ ] **No dark patterns** — nothing that tricks, traps, or deceives the customer
- [ ] **Context is considered** — B2B vs. B2C, price point, audience sophistication all affect which principles work

---

## Examples

### Example 1: SaaS Pricing Page

**User says:** "People visit our pricing page but don't convert. Can you look at it through a psychology lens?"

**You do:**
1. Ask to see the pricing page (URL or screenshot)
2. Ask: "What's your current conversion rate on this page, and what action are you optimizing for?"
3. User: "About 2% start a trial. We want more trial starts, especially on the Pro plan."
4. Review the page and identify applicable principles

**Your analysis includes:**

```
Decision: Choose the Pro plan and start a free trial
Awareness: Product-aware (they're on the pricing page, they know you)

Recommendations:

1. Plan Order — Anchoring
Before: Plans shown cheapest to most expensive (Basic → Pro → Enterprise)
After: Show Enterprise first, then Pro (highlighted), then Basic
Why: Seeing the $299 Enterprise price first makes $79 Pro feel reasonable.

2. Pro Plan — Default Bias
Before: No plan pre-selected
After: Pro plan pre-selected with "Most Popular" badge
Why: People tend to stick with the default. Labeling it "Most Popular"
adds social proof on top of default bias — double effect.

3. Feature Comparison — Loss Aversion
Before: Features listed as what each plan includes
After: Show what Pro includes that Basic doesn't, framed as what you'd miss
Why: "Basic plan doesn't include analytics, priority support, or API access"
makes the upgrade feel like avoiding a loss rather than gaining extras.

4. Trial CTA — Commitment/Consistency + Risk Removal
Before: "Buy Now"
After: "Start Free Trial — No Credit Card Needed"
Why: Lower commitment (free trial vs. buy) gets the foot in the door.
No credit card removes the loss aversion of entering payment info.

5. Social Proof — Below Pricing Cards
Before: No social proof on pricing page
After: "Trusted by 1,200 companies including [3 recognizable logos]"
Why: At the moment of decision, seeing others who chose builds confidence.

Priority: Default bias (#2) first — it's the easiest change with outsized impact.
Then anchoring (#1), then social proof (#5).
```

### Example 2: Email Subject Lines

**User says:** "My email open rates are terrible. Help me write better subject lines using psychology."

**You do:**
1. Ask: "What kind of emails? Newsletter, cold outreach, product updates?"
2. User: "Product update emails to existing users. We get like 15% open rates."
3. Ask: "Who are these users and what do they care about most?"
4. User: "Marketing managers. They care about saving time and looking good to their boss."
5. Apply principles to subject line writing

**Your recommendations:**

```
Decision: Open the email (that's it — just the open)
Awareness: Most aware — they're already customers

Current subject lines (examples from user):
- "March Product Update"
- "New Features Available"
- "What's New in [Product]"

Problem: These are descriptive but create no motivation to open.

Recommended subject lines with psychology:

1. Loss Aversion:
"You're missing a feature that saves 3hrs/week"
Why: Framing the new feature as something they're losing by not knowing about it.

2. Curiosity Gap:
"We just fixed your #1 complaint"
Why: They need to open to find out which complaint. Specific enough to feel real.

3. Social Proof + FOMO:
"The feature 400 teams activated this week"
Why: Others are using it — am I missing out?

4. Endowment:
"Your new dashboard is ready"
Why: It's already "theirs" — they just need to claim it.

5. Authority + Specificity:
"[Customer Name]'s team cut reporting time by 60% with this"
Why: Named example adds authority. Specific number adds credibility.

Ethics note: All subject lines must accurately reflect the email content.
"We just fixed your #1 complaint" only works if you actually did. Clickbait
subject lines destroy trust and increase unsubscribes.
```

### Example 3: Checkout Flow Optimization

**User says:** "People add items to cart but 70% abandon at checkout. What psychology can help?"

**You do:**
1. Ask to see the checkout flow
2. Ask: "What's the average cart value? And do people need to create an account?"
3. User: "$85 average. Yes, account required before payment."
4. Walk through the flow step by step

**Your recommendations:**

```
Decision: Complete the purchase after adding items to cart
Awareness: Most aware — they chose products, they're ready (something is stopping them)

Key issues identified:

1. Forced Account Creation — Commitment Escalation Problem
Before: Step 1 is "Create an Account" with 6 form fields
After: Guest checkout option, or collect email only and create account after purchase
Why: The ask is too big too early. They came to buy, not to join.
Each form field is a moment where abandonment can happen.

2. Cart Summary — Endowment Effect
Before: Small text summary at top of checkout
After: Visual thumbnails of items with "Your items" language throughout
Why: Seeing the products visually reinforces ownership. They already
feel like these items are theirs. "Your" language deepens that.

3. Shipping Costs — Anchoring + Loss Aversion
Before: Shipping cost revealed at final step
After: Show shipping estimate on the cart page. Offer free shipping threshold.
Why: Surprise costs trigger loss aversion. If they're at $85 and free shipping
is at $100, anchoring against that threshold encourages adding one more item.

4. Trust Signals — Authority at Point of Payment
Before: No trust signals near the payment form
After: Security badges, "256-bit encryption," return policy summary next to payment fields
Why: The moment of entering credit card info is peak anxiety.
Authority signals reduce perceived risk exactly when it matters most.

5. Progress Indicator — Commitment/Consistency
Before: No progress bar
After: 3-step progress bar showing they're on step 2 of 3
Why: Seeing progress creates commitment. They've already done step 1 —
consistency drives them to complete what they started.

Priority: Guest checkout (#1) will have the biggest impact by far.
Surprise shipping costs (#3) is the second-biggest abandonment driver.
```

---

## Related Skills

- **copywriting** — write the persuasive copy that psychology principles inform
- **page-cro** — optimize full pages for conversion, using psychology as one lens
- **ad-creative** — apply psychology to ad copy and creative for higher click-through
- **pricing-strategy** — structure pricing using anchoring, framing, and default bias
- **product-marketing-context** — understand the audience and positioning before applying psychology
