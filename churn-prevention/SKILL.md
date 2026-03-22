---
name: churn-prevention
description: Reduce customer churn through smarter cancel flows, save offers, dunning sequences, and early warning signals. Use when customers keep canceling, failed payments are hurting revenue, you need a cancellation survey, or you want to identify and re-engage at-risk customers before they leave.
---

# Churn Prevention

Reduce customer churn through smarter cancel flows, save offers, payment recovery, and early warning signals that catch at-risk customers before they leave.

---

## When to Use

Trigger this skill when you hear:
- "People keep canceling"
- "How do I reduce churn?"
- "Our cancel flow needs work"
- "We're losing customers after [X] months"
- "Failed payments are killing our revenue"
- "How do I win back customers?"
- "Our retention rate is dropping"
- "I want to add a cancellation survey"
- "Should I offer a discount to keep people?"
- "People cancel and I don't know why"

This skill requires product-marketing-context. If that context doesn't exist in memory, run product-marketing-context first.

---

## Context Gathering

Ask these questions conversationally. Don't dump them all at once — have a real conversation.

### Current State
1. **What's your current monthly churn rate?** (Percentage of customers who cancel per month — even a rough guess helps)
2. **Do you know why people are canceling?** (Exit surveys, support tickets, gut feeling — anything)
3. **What happens when someone clicks "cancel" right now?** (Immediate cancellation? A form? A phone call? Nothing built yet?)

### Customer Patterns
4. **When do most cancellations happen?** (After the trial? Month 2? After a price increase? No pattern?)
5. **Are there warning signs before someone cancels?** (They stop logging in, they downgrade, they contact support more)
6. **What does an engaged, happy customer look like?** (How often do they log in? What features do they use?)

### Revenue Recovery
7. **How do you handle failed payments today?** (Automatic retry? Email? Nothing?)
8. **What percentage of your churn is involuntary?** (Failed payments vs. deliberate cancellations)

### Retention Efforts
9. **Have you tried anything to reduce churn?** (Discounts, outreach, product changes — what worked, what didn't)
10. **Do you have different pricing tiers or a free plan?** (Downgrade options matter for save flows)
11. **Is pausing an option you'd consider?** (Letting customers freeze their account instead of canceling)

---

## Methodology

### Step 1: Check Product Marketing Context

Pull existing context from memory to understand the product, pricing, and customer profile:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

You need to understand:
- What the product does and who it serves
- Current pricing model and tiers
- Customer pain points and success outcomes
- Brand voice (for writing cancel flow copy and emails)

If no context exists, tell the user: *"Before we work on churn prevention, I need to understand your product and customers. Let's start there."* Then run the product-marketing-context skill.

### Step 2: Map the Current Cancellation Flow

If the user has a live product, use Nebo's browser to walk through the actual cancel experience:
```
web(action: "navigate", url: "their-app.com/settings")
web(action: "read_page")
```

Document each step:
- Where does the cancel button live? (Buried in settings? Easy to find?)
- What happens after clicking cancel? (Immediate? Confirmation screen? Survey?)
- Is there any attempt to save the customer? (Offer? Pause option? Human outreach?)
- What's the tone of the cancellation page? (Guilt trip? Neutral? Supportive?)

If there's no live product to review, ask the user to describe the current flow or confirm there isn't one yet.

### Step 3: Add Strategic Friction to the Cancel Flow

Design a cancellation experience that respects the customer while creating opportunities to save them. This is NOT about making it hard to cancel — it's about understanding why and offering real alternatives.

**The cancel flow should have these steps, in order:**

**A. Cancellation Survey (Required)**
Ask why they're leaving. Use multiple choice with an "other" option. Common reasons:
- Too expensive
- Not using it enough
- Missing a feature I need
- Switching to a competitor
- Just need a break
- My situation changed
- Other (free text)

This data is gold. Every response helps you improve the product and tailor the save offer.

**B. Save Offer (Based on Survey Response)**
Match the offer to the reason:

| Reason | Save Offer |
|---|---|
| Too expensive | Discount (20-30% for 2-3 months) or downgrade to cheaper plan |
| Not using it enough | Pause account for 1-3 months |
| Missing a feature | Share roadmap timeline if the feature is planned |
| Switching to competitor | Ask what the competitor offers that you don't (research input) |
| Just need a break | Pause account, keep data safe |
| My situation changed | Pause option or downgrade to free tier |

**C. Pause Option**
Always offer a pause before a full cancel. "Take a break" feels less permanent than "cancel." Suggest 1, 2, or 3 month pause periods. Their data stays intact. They can come back with one click.

**D. Downgrade Option**
If you have multiple tiers, offer a step down instead of a full exit. A customer paying $10/month is better than a lost customer.

**E. Confirm Cancellation**
If none of the save options work, make cancellation clean and respectful. Confirm the date their access ends. Explain what happens to their data. Leave the door open — "You can reactivate anytime."

### Step 4: Design the Dunning Sequence for Failed Payments

Failed payments cause involuntary churn — customers who didn't mean to leave. A dunning sequence recovers this revenue.

**Email 1: Friendly Heads-Up (Day 1 of failed payment)**
- Subject: "Quick heads up — your payment didn't go through"
- Tone: Helpful, not alarming
- Include a direct link to update payment info
- Mention what they'll lose access to if not resolved

**Email 2: Gentle Reminder (Day 3)**
- Subject: "Your [Product] account needs attention"
- Remind them what they've built or accomplished in the product (if you can pull usage data)
- Another direct link to update payment
- Mention the timeline — "We'll keep trying for [X] more days"

**Email 3: Final Notice (Day 7)**
- Subject: "Last chance to keep your [Product] account"
- Clear deadline: "Your account will be downgraded/paused on [date]"
- One-click payment update link
- Reassure them their data is safe even if the account pauses

**In-App Notification (Ongoing until resolved)**
- Banner at the top of the app: "Your payment method needs updating. [Update now]"
- Keep it visible but not blocking — they should still be able to use the product while sorting it out

**Behind the scenes:**
- Retry the payment automatically on days 1, 3, 5, and 7
- Try on different days of the week (some cards decline on weekends)
- If the card expires, check if your payment processor supports automatic card updates

### Step 5: Identify Leading Indicators of Churn

Don't wait for someone to hit the cancel button. Watch for signals that a customer is becoming at-risk.

**Common leading indicators:**
- **Usage drop:** Logins decrease by 50% or more compared to their average
- **Feature abandonment:** They stop using the core feature they signed up for
- **Support tickets:** Increase in complaints or frustrated support interactions
- **Billing friction:** Failed payment attempts, downgrades, or requests for refunds
- **Engagement silence:** No email opens, no in-app activity for 14+ days
- **Negative feedback:** Low NPS scores, bad reviews, or negative survey responses

Work with the user to identify which 3-5 signals are most meaningful for their specific product.

### Step 6: Create Re-Engagement Triggers

For each at-risk signal, design an automatic response:

| Signal | Trigger Action |
|---|---|
| Login drop (50%+ decline over 2 weeks) | Send "We miss you" email with a reason to come back (new feature, tip, content) |
| No activity for 14 days | In-app message on next login: "Welcome back! Here's what's new" |
| Support complaint | Personal outreach from success team within 24 hours |
| Failed payment | Start dunning sequence (Step 4) |
| Downgrade | Check in 2 weeks later: "How's the new plan working?" |
| Low NPS score | Personal follow-up: "Thanks for the feedback. What would make this better?" |

**Key principle:** Re-engagement should provide value, not just ask for attention. Send them something useful — a tip, a new feature announcement, content relevant to their use case.

Store the churn prevention plan in memory:
```
agent(resource: "memory", action: "store", key: "retention/cancel_flow", value: "Cancel flow design", layer: "tacit")
agent(resource: "memory", action: "store", key: "retention/dunning_sequence", value: "Dunning email sequence", layer: "tacit")
agent(resource: "memory", action: "store", key: "retention/churn_signals", value: "Leading indicators list", layer: "tacit")
agent(resource: "memory", action: "store", key: "retention/reengagement_triggers", value: "Trigger-action pairs", layer: "tacit")
agent(resource: "memory", action: "store", key: "retention/plan_date", value: "YYYY-MM-DD", layer: "tacit")
```

---

## Output Format

Deliver the churn prevention plan in this structure:

```markdown
# Churn Prevention Plan
Created: [DATE]

## Current State
**Churn rate:** [X%/month]
**Primary reasons:** [Top 3 reasons people leave]
**Involuntary churn:** [X% from failed payments]

## Cancel Flow Design

### Step 1: Survey
[The cancellation survey questions]

### Step 2: Save Offers
[Table of reason-to-offer matches]

### Step 3: Pause Option
[Pause terms — duration, data retention, reactivation]

### Step 4: Downgrade Path
[Available downgrade options]

### Step 5: Clean Exit
[What happens on cancellation — access end date, data policy, reactivation path]

## Dunning Sequence
**Email 1 (Day 1):** [Subject line + key message]
**Email 2 (Day 3):** [Subject line + key message]
**Email 3 (Day 7):** [Subject line + key message]
**In-app:** [Notification design]
**Retry schedule:** [Automatic retry days]

## Early Warning Signals
1. [Signal] — [How to detect] — [Threshold]
2. [Signal] — [How to detect] — [Threshold]
3. [Signal] — [How to detect] — [Threshold]

## Re-Engagement Triggers
| Signal | Action | Timing |
|---|---|---|
| [Signal] | [What to do] | [When] |

## Metrics to Track
- [Metric 1]
- [Metric 2]
- [Metric 3]
```

---

## Quality Checks

Before finalizing the churn prevention plan, verify:

- [ ] **Cancel flow respects the customer** — no dark patterns, no guilt trips, no hiding the cancel button
- [ ] **Survey covers the real reasons** — not just generic options, tailored to this product
- [ ] **Save offers match survey reasons** — each reason has a specific, relevant counter-offer
- [ ] **Pause option is included** — always give people a way to take a break without fully leaving
- [ ] **Dunning emails are helpful, not threatening** — tone is friendly, includes a clear action link
- [ ] **Dunning sequence has 3 emails plus in-app** — not too many, not too few
- [ ] **Leading indicators are specific to this product** — not generic churn signals, but ones relevant to how their customers use the product
- [ ] **Re-engagement triggers provide value** — not just "please come back" but a real reason to return
- [ ] **All copy matches brand voice** — checked against product-marketing-context
- [ ] **Plan is actionable** — the user can implement this without hiring a consultant

---

## Examples

### Example: Redesigning a Cancel Flow for a Project Management SaaS

**User says:** "People keep canceling after the second month. I think our cancel flow is just a button that immediately cancels — no survey, no offer, nothing."

**You do:**

1. Check memory for product-marketing-context. Found: Project management tool for freelancers, $19/month, casual brand voice. Free tier available but limited to 3 projects.

2. Use browser to walk through the current cancel flow:
   ```
   web(action: "navigate", url: "app.example.com/settings/billing")
   web(action: "read_page")
   ```
   Confirmed: Settings > Billing > "Cancel Subscription" button > immediate cancellation. No survey. No offer. No confirmation screen.

3. Design the new cancel flow:

   **Step 1 — Survey screen:**
   *"Before you go — mind telling us why? It genuinely helps us get better."*
   - Too expensive for what I'm getting
   - I'm not using it enough
   - Missing a feature I need (which one?)
   - Found something that fits better
   - Just need a break from projects
   - Something else (tell us)

   **Step 2 — Save offer (based on response):**

   If "too expensive":
   *"Totally fair. How about 30% off for the next 3 months? That brings it to $13/month. Or you can switch to our Free plan and keep your 3 most recent projects."*
   [Accept discount] [Switch to Free] [Continue canceling]

   If "not using it enough":
   *"No worries — want to pause your account instead? Your projects and data stay safe, and you can jump back in whenever. No charge while paused."*
   [Pause for 1 month] [Pause for 3 months] [Continue canceling]

   If "missing a feature":
   *"Ugh, sorry about that. What feature would have made the difference? We're shipping updates every 2 weeks and this helps us prioritize."*
   [Text field + Submit] then → *"Thanks. Want to pause while we work on it? We'll email you when it ships."*

   If "found something better":
   *"No hard feelings. Mind sharing what you're switching to? It helps us understand what we're missing."*
   [Text field + Submit] → proceed to confirmation

   If "need a break":
   *"Totally get it. You can pause your account instead of canceling — everything stays right where you left it."*
   [Pause for 1 month] [Pause for 3 months] [Continue canceling]

   **Step 3 — Confirmation screen (if they continue canceling):**
   *"Got it. Your Pro access ends on [date]. Your projects will move to the Free plan (3 project limit). You can reactivate anytime from Settings."*
   [Confirm cancellation] [Go back]

4. Design the dunning sequence:

   **Email 1 (Day 1):**
   Subject: "Heads up — your payment didn't go through"
   Body: "Hey [Name], your card ending in [XXXX] was declined for your [Product] subscription. No worries, these things happen. Hit the button below to update your payment method and keep your projects running. [Update payment info]"

   **Email 2 (Day 3):**
   Subject: "Your [Product] projects need you"
   Body: "You've got [X] active projects and [Y] tasks completed this month — nice work. Your payment still needs updating though. Takes 30 seconds. [Update payment info]"

   **Email 3 (Day 7):**
   Subject: "Your [Product] account will be paused Friday"
   Body: "Quick heads up — if we can't process your payment by [date], your account will switch to the Free plan. Your data is safe, but you'll only have access to 3 projects. Update your card to keep everything running. [Update payment info]"

5. Identify leading indicators for this product:
   - Login frequency drops below 2x per week (their average is 5x)
   - No new tasks created in 10 days
   - Stopped inviting team collaborators
   - Support ticket about a missing feature
   - Hasn't opened the last 3 product emails

6. Set up re-engagement triggers:
   - Login drop → email with "3 ways to get more out of [Product]" (tips they haven't tried)
   - No new tasks in 10 days → in-app: "Need help organizing your next project? Try our templates"
   - Support complaint → personal check-in from founder within 24 hours
   - Email disengagement → switch to in-app messaging only (they might have email fatigue)

**Result:** User has a complete cancel flow design, dunning sequence, and early warning system they can hand to their developer to build.

---

## Related Skills

- **email-sequence** — write the actual dunning emails and re-engagement sequences
- **paywall-upgrade-cro** — optimize upgrade flows (the flip side of churn prevention)
- **pricing-strategy** — restructure pricing tiers to create natural downgrade paths
- **onboarding-cro** — fix early churn by improving the first-run experience
- **product-marketing-context** — provides the product, customer, and voice foundation this skill builds on
