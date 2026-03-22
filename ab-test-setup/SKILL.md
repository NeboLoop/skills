---
name: ab-test-setup
description: Design and analyze A/B tests to make data-driven decisions. Use when planning split tests, calculating sample sizes, defining hypotheses, choosing test metrics, or interpreting experiment results to know if a change actually works.
---

# A/B Test Setup

Design and analyze A/B tests to make data-driven decisions. Know what to test, how long to run it, and whether the results actually mean something.

---

## When to Use

Trigger this skill when you hear:
- "I want to test..."
- "Which version is better?"
- "Should I A/B test this?"
- "How do I know if this change works?"
- "Set up a split test"
- "My conversion rate changed — is it real?"
- "How long should I run this test?"
- "Help me design an experiment"

This skill is for anyone who wants to make decisions based on evidence instead of gut feeling. No stats background required.

---

## Context Gathering

Before designing any test, you need to understand the situation. Ask these conversationally:

### The Change
1. **What do you want to test?** (A headline, button color, pricing page layout, email subject line — be specific)
2. **Why do you think this change might work?** (What's the reasoning? What made you think of it?)
3. **What page or experience does this affect?** (URL, email, ad — where does the test live?)

### The Goal
4. **What does success look like?** (More signups? Higher click-through? More purchases? Pick one primary metric)
5. **Is there anything you don't want to hurt?** (Sometimes boosting one metric tanks another — these are guardrail metrics)
6. **How much improvement would be meaningful to you?** (A 1% lift? 10%? This helps determine how long the test needs to run)

### The Traffic
7. **How much traffic does this page or experience get?** (Daily visitors, weekly emails sent, monthly ad impressions)
8. **Do you have a tool for running the test?** (Google Optimize, VWO, Optimizely, custom code, or do you need help choosing?)

---

## Methodology

### Step 1: Check for Product Context

Before designing a test, understand what you're working with:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If product context exists, use it to inform the hypothesis. Good tests are grounded in an understanding of the customer and the value prop.

If no context exists, suggest running **product-marketing-context** first: *"Before we test anything, let's make sure I understand your product and customers. That way we design tests that actually matter."*

### Step 2: Check the Current Experience

If the user provides a URL, look at what's there now:
```
web(action: "navigate", url: "their-page.com")
web(action: "read_page")
```

Understanding the current experience helps you design a meaningful variant, not just a random change.

### Step 3: Define the Hypothesis

Every good test starts with a hypothesis. Structure it like this:

**"Changing [specific thing] will improve [specific metric] because [reasoning based on customer insight]."**

Bad hypothesis: "A new headline will get more conversions."
Good hypothesis: "Changing the headline from feature-focused ('AI-powered analytics') to outcome-focused ('See what's driving your revenue') will improve signups because our customers care about results, not technology."

The hypothesis forces you to think about *why* the change should work. If you can't explain why, you probably shouldn't test it.

### Step 4: Calculate Sample Size

This answers the question: **"How many visitors do I need before the results mean something?"**

The key inputs:
- **Current conversion rate** — what's happening now (e.g., 3% of visitors sign up)
- **Minimum detectable effect** — the smallest improvement you care about (e.g., a 20% relative lift, meaning 3% going to 3.6%)
- **Confidence level** — how sure you want to be that the result is real, not random (use 95% — this means if you ran the same test 20 times, the result would be wrong only once)

**Rule of thumb for sample sizes:**

| Current Rate | Detect 10% Lift | Detect 20% Lift | Detect 50% Lift |
|---|---|---|---|
| 1% | ~300,000 per variant | ~80,000 per variant | ~13,000 per variant |
| 3% | ~95,000 per variant | ~25,000 per variant | ~4,200 per variant |
| 5% | ~55,000 per variant | ~15,000 per variant | ~2,500 per variant |
| 10% | ~26,000 per variant | ~7,000 per variant | ~1,200 per variant |
| 20% | ~12,000 per variant | ~3,200 per variant | ~550 per variant |

These are approximate. The point is: small improvements on low-conversion pages need a LOT of traffic to detect.

If the required sample size is unrealistic for the user's traffic, say so clearly: *"With 500 visitors per week and a 2% conversion rate, you'd need to run this test for about 6 months to detect a 20% lift. That's not practical. Let's either test something with a bigger expected impact, or test on a higher-traffic page."*

### Step 5: Define Metrics

**Primary metric** — the one number that determines the winner. Pick exactly one.
- Examples: signup rate, click-through rate, purchase rate, revenue per visitor

**Guardrail metrics** — things you monitor to make sure you're not breaking something else.
- Examples: bounce rate, page load time, support ticket volume, refund rate
- These don't determine the winner, but if they go sideways, you pause and investigate.

**Don't track too many metrics.** If you look at 20 metrics, at least one will show a "significant" result by pure chance. Stick to 1 primary and 2-3 guardrails.

### Step 6: Design the Variants

**The golden rule: change one thing at a time.**

If you change the headline AND the button color AND the layout, and the variant wins, you have no idea which change caused it.

- **Control (A):** the current experience, unchanged
- **Variant (B):** the current experience with ONE specific change

If you want to test multiple changes, run them as separate sequential tests. Yes, it takes longer. But you actually learn something.

**Exceptions:** A full page redesign can be tested as one variant, but treat it as a "does this overall approach work better?" test, not a "what specifically made it better?" test.

### Step 7: Set Test Duration

Calculate minimum duration based on traffic:
- **Minimum visitors needed** = sample size from Step 4 (both variants combined)
- **Daily traffic** = visitors per day to the test page
- **Minimum days** = visitors needed / daily traffic

**Important rules for duration:**
- Always run for at least **7 full days** (even if you hit sample size sooner) to account for day-of-week patterns
- Ideally run for **2 full weeks** to smooth out weekly cycles
- **Never peek and stop early** when results look good. This is the most common mistake. Early results are unreliable. Commit to the full duration before you start.
- Avoid launching during unusual periods (holidays, big sales, product launches) unless you're specifically testing for those periods

### Step 8: Analyze Results

When the test is done, answer these questions:

**1. Is the difference real or random?**
Look at the confidence level (sometimes called "statistical significance"):
- **95% or higher** — the difference is almost certainly real, not just random noise. Safe to act on it.
- **90-95%** — probably real, but there's some uncertainty. Consider running longer or treating as directional.
- **Below 90%** — you can't tell if the difference is real. It might just be random variation.

In plain English: "95% confidence" means if there were actually no difference between the variants, there's only a 5% chance you'd see results this extreme by luck alone. So it's very likely the difference is real.

**2. Is the difference meaningful?**
A "statistically significant" 0.1% improvement is real but probably not worth the effort. Consider:
- Is the lift big enough to matter for the business?
- Does the improvement justify the cost of implementing the change permanently?

**3. Did any guardrail metrics move in a bad direction?**
If the variant wins on the primary metric but tanks a guardrail (e.g., signups went up but bounce rate doubled), dig deeper before shipping.

### Step 9: Make a Recommendation

Based on the analysis, recommend one of three outcomes:

**Ship the winner:** The variant clearly beat the control with high confidence and meaningful lift. No guardrail issues. Implement the change permanently.

**Iterate and retest:** Results are promising but not conclusive, or the lift is smaller than expected. Design a new variant building on what you learned.

**Call it inconclusive:** Not enough data, no meaningful difference, or mixed results. Document what you learned and move on to a different test. Not every test has a winner, and that's fine — you still learned something.

---

## Output Format

Present the test plan in this structure:

```markdown
# A/B Test Plan: [What You're Testing]
Date: [DATE]

## Hypothesis
Changing [specific thing] will improve [metric] because [reasoning].

## Variants
- **Control (A):** [Description of current experience]
- **Variant (B):** [Description of changed experience — one specific change]

## Metrics
- **Primary:** [The one metric that determines the winner]
- **Guardrails:** [2-3 metrics to monitor for negative side effects]

## Sample Size & Duration
- **Current conversion rate:** [X%]
- **Minimum detectable effect:** [Y% relative lift]
- **Required visitors per variant:** [Number]
- **Estimated daily traffic:** [Number]
- **Minimum test duration:** [X days/weeks]
- **Planned end date:** [DATE — commit to this before starting]

## Setup
- **Tool:** [Testing platform]
- **Traffic split:** [Usually 50/50]
- **Page/experience:** [URL or description]

## Decision Rules (set these BEFORE the test starts)
- Ship variant if: [primary metric improves by X% with 95%+ confidence]
- Ship control if: [variant shows no improvement or hurts guardrails]
- Extend test if: [results are trending but haven't hit confidence threshold]
- Stop test early if: [guardrail metric drops by more than X%]
```

For results analysis, use this structure:

```markdown
# A/B Test Results: [What Was Tested]
Test period: [Start date] to [End date]

## Results
| Metric | Control (A) | Variant (B) | Difference | Confidence |
|---|---|---|---|---|
| [Primary metric] | [Value] | [Value] | [+/- %] | [X%] |
| [Guardrail 1] | [Value] | [Value] | [+/- %] | [X%] |
| [Guardrail 2] | [Value] | [Value] | [+/- %] | [X%] |

## What This Means
[Plain-English interpretation. Is the difference real? Is it meaningful?]

## Recommendation
[Ship winner / Iterate / Inconclusive — with reasoning]

## What We Learned
[Key takeaway that informs future tests, regardless of outcome]
```

---

## Quality Checks

Before finalizing a test plan, verify:

- [ ] **Hypothesis is specific** — includes what changes, what metric improves, and why
- [ ] **Only one thing changes** — variant differs from control in exactly one way
- [ ] **Primary metric is singular** — one metric determines the winner, not several
- [ ] **Sample size is realistic** — the test can reach required visitors within a reasonable timeframe
- [ ] **Duration includes full weeks** — at least 7 days, ideally 14, to capture weekly patterns
- [ ] **Decision rules are set in advance** — what constitutes a win, loss, or inconclusive result is defined before the test starts
- [ ] **Guardrail metrics are identified** — know what you're watching to make sure nothing breaks
- [ ] **No peeking plan** — the team agrees not to call the test early based on incomplete data
- [ ] **Product context is loaded** — the test is grounded in understanding of the customer and value prop
- [ ] **The test is worth running** — the potential impact justifies the time and traffic cost

---

## Examples

### Example 1: Testing Landing Page Headlines

**User says:** "I want to test a new headline on our landing page."

**You do:**
1. Check memory for product marketing context
2. Found context: B2B project management tool for remote teams. Customers care about visibility into what everyone's working on.
3. Ask: "What's the current headline?"
4. User: "The All-in-One Project Management Platform"
5. Ask: "What headline do you want to test against it?"
6. User: "I'm not sure, something better."
7. Use the product context to suggest: "Based on your customers' main pain point — not knowing what their remote team is working on — how about: 'See What Your Team Is Working On. Right Now.' It speaks directly to the problem."
8. User: "I like that. Let's test it."
9. Ask: "What's your current signup rate on this page, and how many visitors do you get per day?"
10. User: "About 4% sign up, and we get around 800 visitors a day."
11. Calculate: To detect a 20% relative lift (4% to 4.8%), you need about 15,000 visitors per variant, so 30,000 total. At 800/day, that's about 38 days. Round up to 6 weeks to capture full weekly cycles.

**Test plan delivered:**

```markdown
# A/B Test Plan: Landing Page Headline
Date: 2026-03-21

## Hypothesis
Changing the headline from feature-focused ("The All-in-One Project Management
Platform") to outcome-focused ("See What Your Team Is Working On. Right Now.")
will improve the signup rate because our customers' primary pain point is lack
of visibility into remote team activity, not a need for project management
features.

## Variants
- **Control (A):** "The All-in-One Project Management Platform"
- **Variant (B):** "See What Your Team Is Working On. Right Now."

## Metrics
- **Primary:** Signup rate (visitors who complete the signup form)
- **Guardrails:** Bounce rate, time on page, trial activation rate (7-day)

## Sample Size & Duration
- **Current conversion rate:** 4%
- **Minimum detectable effect:** 20% relative lift (4% to 4.8%)
- **Required visitors per variant:** ~15,000
- **Estimated daily traffic:** 800 visitors
- **Minimum test duration:** 38 days
- **Planned end date:** 2026-05-02 (6 full weeks)

## Setup
- **Tool:** [User's testing platform]
- **Traffic split:** 50/50
- **Page:** Homepage / landing page

## Decision Rules
- Ship variant if: signup rate improves with 95%+ confidence
- Ship control if: variant shows no improvement after full test duration
- Extend test if: confidence is between 85-95% at planned end date
- Stop test early if: bounce rate increases by more than 25%
```

**After the test runs, results come back:**

| Metric | Control (A) | Variant (B) | Difference | Confidence |
|---|---|---|---|---|
| Signup rate | 4.1% | 4.9% | +19.5% | 97% |
| Bounce rate | 42% | 40% | -4.8% | 78% |
| Time on page | 2m 15s | 2m 28s | +9.6% | 82% |

**Your analysis:** "The outcome-focused headline increased signups by about 20%, and we're 97% confident this is a real improvement — not random noise. Bounce rate actually went down slightly (good sign), and people spent a bit more time on the page. Both guardrails look healthy. Recommendation: ship the new headline."

### Example 2: When a Test Isn't Worth Running

**User says:** "I want to test changing the button from blue to green."

**You do:**
1. Ask: "What's the reasoning behind green? Is there something about the current blue button that you think is causing problems?"
2. User: "I read that green buttons convert better."
3. Explain: "That's a common myth. Button color tests almost never produce meaningful results because the color itself rarely changes someone's decision to buy or sign up. The words on the button, the offer itself, and the context around it matter much more."
4. Ask: "What's the actual conversion problem you're trying to solve? Is the button hard to see? Is the conversion rate lower than you'd expect?"
5. User: "Our signup rate is only 2% and I feel like it should be higher."
6. Suggest: "Let's look at higher-impact things to test. Can I see the page?"
7. Use browser to review the page, then suggest tests that address real friction points — like unclear value props, too many form fields, or mismatched messaging.

### Example 3: Not Enough Traffic

**User says:** "I want to run an A/B test on our pricing page."

**You do:**
1. Gather context about current conversion rate (1.5%) and traffic (200 visitors/week)
2. Calculate: To detect a 20% lift on a 1.5% conversion rate, you need roughly 55,000 visitors per variant. At 200/week, that's over 10 years.
3. Be honest: "With your current traffic, a traditional A/B test isn't practical here. You'd need to run it for years to get a reliable result. Here are some alternatives:"
   - **Test a bigger change** — instead of tweaking the pricing page, try a completely different pricing structure. Bigger changes need less traffic to detect.
   - **Use qualitative methods** — run 5-10 user interviews watching people interact with the pricing page. You'll learn more, faster.
   - **Test on a higher-traffic page** — your blog gets 5,000 visitors/week. Could test there first.
   - **Just ship it** — if you have strong reasons to believe the change is better, sometimes the right call is to make the change and monitor metrics closely.

---

## Related Skills

- **page-cro** — audits pages for conversion issues and identifies what to test
- **analytics-tracking** — sets up proper tracking so your test metrics are reliable
- **ad-creative** — tests different ad variations using similar experimental thinking
- **product-marketing-context** — provides the customer understanding that makes good hypotheses

**Before designing a test, check that product-marketing-context exists.** Good tests come from understanding your customers, not from random ideas.
