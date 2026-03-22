---
name: paywall-upgrade-cro
description: Optimize in-app upgrade prompts and paywall screens to increase paid conversions — turning free users into paying customers without being pushy. Use when free-to-paid conversion is low or upgrade screens need improvement.
---

# Paywall & Upgrade CRO

Optimize in-app upgrade prompts and paywall screens to increase paid conversions — turning free users into paying customers without being pushy.

---

## When to Use

Trigger this skill when you hear:
- "How do I get more people to upgrade?"
- "My paywall isn't converting"
- "Help me design an upgrade screen"
- "People hit the limit but don't upgrade"
- "My free-to-paid conversion is low"
- "I need a better pricing page inside the app"
- "Users cancel before they even start paying"
- "How should I show my plans to users?"

This skill focuses on **in-app** upgrade moments — the screens, prompts, and flows users see when they hit a paywall or are invited to upgrade. For your public pricing page, use **page-cro** instead.

---

## Context Gathering

Before designing or improving a paywall, understand the situation. Ask these conversationally:

### Current Setup
1. **What's your pricing model?** (Free trial? Freemium? Usage-based? How many plans?)
2. **Where do users currently see the upgrade prompt?** (Feature gate, usage limit, settings page, banner?)
3. **What does the current paywall look like?** (Share a screenshot or URL if possible)

### User Behavior
4. **What's your current free-to-paid conversion rate?** (Even a rough number helps)
5. **At what point do users typically upgrade?** (After a specific action? A certain number of days?)
6. **Why do users say no?** (Too expensive? Not enough value yet? Confusing plans? Do you have any feedback?)

### Product Value
7. **What's the "wow moment" in your product?** (The action where users first feel the value)
8. **Which paid features do free users ask about most?** (Tells us what to highlight)
9. **Do you offer annual pricing?** (And if so, what's the discount?)

---

## Methodology

### Step 1: Check Product Marketing Context

Pull in what you already know about the product, customer, and positioning:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

This gives you the value proposition, ideal customer, and brand voice — all of which shape how the paywall should read and feel.

If no context exists, run **product-marketing-context** first.

### Step 2: Review the Current Paywall

If the user shares a URL or screenshot, review it:
```
web(action: "navigate", url: "their-app.com/upgrade")
web(action: "read_page")
```

Look for these common problems:
- **Too many choices** — more than 3-4 plans creates decision paralysis
- **No recommended plan** — users don't know which one is right for them
- **Feature lists without context** — listing features without explaining the benefit
- **No urgency or incentive** — nothing that encourages acting now vs. later
- **Missing trust signals** — no money-back guarantee, no social proof, no "cancel anytime"
- **Bad timing** — the prompt appears before the user has experienced enough value
- **Aggressive tone** — makes users feel punished for being on the free plan

### Step 3: Identify the Trigger Moment

Every paywall needs the right trigger — the moment it appears matters as much as the design. Map out the trigger:

**Feature gate** — user tries a paid-only feature
- Best when: the feature is clearly valuable and the user already uses the product regularly
- Tip: show a preview of what they'd get, not just a locked icon

**Usage limit** — user hits a cap (storage, team members, projects, etc.)
- Best when: the limit is generous enough that users have already gotten real value
- Tip: warn them before they hit the limit, not just after

**Trial expiry** — free trial period is ending
- Best when: the trial was long enough for them to build a habit
- Tip: remind them what they've accomplished during the trial

**Proactive prompt** — nudge at a natural pause point
- Best when: the user just completed something meaningful (shipped a project, closed a deal)
- Tip: tie the upgrade to doing more of what they just succeeded at

### Step 4: Confirm Value Before the Ask

This is the most important step. Users who haven't felt the product's value will never convert, no matter how good the paywall looks.

Check:
- Has the user completed the "wow moment" (the action that demonstrates core value)?
- Has the user returned to the product at least a few times?
- Has the user invested effort (created content, set up workflows, invited teammates)?

If the answer to any of these is no, the problem isn't the paywall — it's onboarding. Recommend improving activation before optimizing the upgrade screen.

### Step 5: Design the Paywall

Build the paywall with these elements:

**1. Headline that speaks to the outcome**
- Not: "Upgrade to Pro"
- Better: "Get unlimited projects and team collaboration"
- Best: "Keep your momentum going" (tied to what they just did)

**2. Clear plan comparison**
- Show 2-3 plans side by side (rarely more)
- Highlight the recommended plan visually (border, badge, slightly larger)
- List 5-7 key differences, not every feature
- Use plain language: "Up to 5 projects" not "5x project allocation"

**3. Pricing that's easy to scan**
- Show monthly price prominently
- If you offer annual billing, show the per-month cost with "billed annually"
- Display the savings: "Save 20%" or "Save $48/year"
- For higher-tier plans, consider showing daily cost: "Less than $2/day"

**4. Handle objections inline**
- Add "Cancel anytime" near the price
- Add "30-day money-back guarantee" if applicable
- Show a short testimonial or stat: "Trusted by 10,000+ teams"
- If there's a free trial of the paid plan: "Try free for 14 days"

**5. Smart defaults and actions**
- Pre-select the recommended plan
- Make the primary button action clear: "Start Pro Plan" not just "Continue"
- Offer a secondary action: "Compare all features" or "Talk to sales" for enterprise
- Always provide a way to dismiss: "Maybe later" (not "No thanks, I don't want to grow my business")

**6. Context-aware copy**
- Reference what the user was trying to do: "To add your 6th project, upgrade to Pro"
- Reference their usage: "You've created 5 projects this month — nice work!"
- Reference their team if applicable: "Your team of 4 is growing. Pro supports unlimited members"

### Step 6: Suggest A/B Test Variations

Always recommend testing. Suggest 2-3 variations:

**Common tests that move the needle:**
- Annual vs. monthly as the default view
- Showing 2 plans vs. 3 plans
- Different headline approaches (outcome-focused vs. feature-focused)
- Adding vs. removing a testimonial
- Different discount amounts on annual plans
- Urgency elements (limited-time offer vs. always available)
- Different button copy ("Start free trial" vs. "Upgrade now" vs. "See what you're missing")

**How to structure the test:**
- Change only one thing at a time
- Run the test for at least 2 weeks or 200 conversions (whichever comes first)
- Measure: click-through rate on the upgrade button, completed purchases, and plan choice distribution

---

## Output Format

Deliver the paywall recommendation in this structure:

```markdown
# Paywall Optimization: [Product Name]
Date: [DATE]

## Current State
**Trigger:** [When the paywall appears]
**Conversion rate:** [Current rate if known]
**Key issues:** [Top 2-3 problems with current approach]

## Recommended Trigger Strategy
**When to show:** [Trigger moment and why]
**Value checkpoint:** [What the user should have experienced first]

## Paywall Design

### Headline
[Recommended headline]

### Plan Comparison
| | [Plan 1] | [Plan 2 - Recommended] | [Plan 3] |
|---|---|---|---|
| Price | $X/mo | $Y/mo | $Z/mo |
| [Feature 1] | ... | ... | ... |
| [Feature 2] | ... | ... | ... |
| [Feature 3] | ... | ... | ... |

### Objection Handlers
- [Trust signal 1]
- [Trust signal 2]

### Copy
**Button:** [Primary CTA text]
**Dismiss:** [Secondary action text]
**Context line:** [Usage-aware message]

## A/B Test Plan
**Test 1:** [What to test] — [Hypothesis]
**Test 2:** [What to test] — [Hypothesis]
**Success metric:** [What to measure]
**Duration:** [How long to run]
```

---

## Quality Checks

Before delivering the recommendation, verify:

- [ ] **Product-marketing-context was referenced** — the paywall reflects the product's value proposition and voice
- [ ] **Trigger timing makes sense** — users have experienced enough value before seeing the upgrade prompt
- [ ] **Plan comparison is scannable** — a user can understand the difference in under 10 seconds
- [ ] **Recommended plan is clear** — there's an obvious "best choice" for most users
- [ ] **Annual savings are shown** — if annual billing exists, the discount is visible and compelling
- [ ] **Objections are handled** — money-back guarantee, cancel anytime, or free trial is mentioned
- [ ] **Copy is specific, not generic** — references the user's actual usage or goal, not just "Upgrade now"
- [ ] **Dismiss option isn't guilt-tripping** — "Maybe later" not "No, I don't want more features"
- [ ] **A/B tests are actionable** — each test changes one variable with a clear hypothesis
- [ ] **Language is plain** — no marketing jargon, no technical terms, no corporate speak

---

## Examples

### Example: Freemium Project Management SaaS

**User says:** "Our free-to-paid conversion is 2%. Users love the free plan but never upgrade. Help me fix the paywall."

**You do:**
1. Check memory for product-marketing-context (found: project management tool for freelancers, $12/mo Pro, $24/mo Team)
2. Review the current paywall with browser
3. Find issues: paywall only appears in settings, no recommended plan, 12 features listed with checkmarks

**Your recommendation:**

**Trigger change:** Instead of hiding the upgrade in settings, show it when the user tries to create their 4th project (free plan allows 3). They've already proven they use the product.

**Headline:** "You're on a roll — unlock unlimited projects"

**Plan comparison:**

| | Free | Pro (Most Popular) | Team |
|---|---|---|---|
| Price | $0 | $12/mo ($9/mo annually) | $24/mo ($19/mo annually) |
| Projects | 3 | Unlimited | Unlimited |
| File storage | 500 MB | 10 GB | 50 GB |
| Team members | Just you | Up to 3 | Unlimited |
| Priority support | — | Included | Included |
| Custom branding | — | — | Included |

**Objection handlers:**
- "14-day free trial of Pro — no credit card needed"
- "Cancel anytime, no questions asked"
- "Join 8,000+ freelancers who upgraded this year"

**Context-aware copy:**
- Button: "Start my Pro trial"
- Dismiss: "Maybe later"
- Context line: "You've completed 12 tasks across 3 projects this month. Imagine what you could do with unlimited projects."

**A/B tests:**
1. **Headline test:** "Unlock unlimited projects" vs. "You've completed 12 tasks — keep going with Pro" — testing feature-focused vs. accomplishment-focused messaging
2. **Trial vs. direct purchase:** "Start free trial" vs. "Upgrade to Pro — $12/mo" — testing whether a trial reduces friction or just delays revenue
3. **Annual default:** Show monthly pricing first vs. annual pricing first — testing whether the lower annual price increases clicks

**Expected outcome:** Moving the trigger from settings to the feature gate alone should increase upgrade visibility significantly. Combined with the clearer plan comparison and trial offer, target 4-5% free-to-paid conversion within 60 days.

---

## Related Skills

- **pricing-strategy** — set the right price points and plan structure before building the paywall
- **page-cro** — optimize the public-facing pricing page (this skill handles in-app upgrade flows)
- **copywriting** — write compelling upgrade copy that matches brand voice
- **churn-prevention** — keep paying customers after they upgrade (retention is half the equation)
- **product-marketing-context** — always referenced first to align the paywall with positioning and voice

**This skill works best when product-marketing-context exists in memory.** If it doesn't, that skill runs first so the paywall reflects your actual value proposition and customer language.
