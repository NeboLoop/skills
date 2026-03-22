---
name: revops
description: Set up and optimize the lead-to-customer pipeline so no lead falls through the cracks. Use when building a sales pipeline, creating a lead scoring model, defining lifecycle stages, setting up lead routing and handoff processes, or fixing conversion bottlenecks in your revenue operations.
---

# RevOps

Set up and optimize the lead-to-customer pipeline so no lead falls through the cracks.

---

## When to Use

Trigger this skill when you hear:
- "How do I track leads?"
- "Leads are falling through the cracks"
- "Set up my sales pipeline"
- "I need a lead scoring model"
- "How do I hand off leads to sales?"
- "What should my pipeline stages be?"
- "My conversion rates are terrible"
- "I don't know which leads to prioritize"
- "Help me with revenue operations"
- "Build me a lead funnel"

This skill helps non-technical founders, marketers, and ops people build a repeatable system for turning strangers into customers. No CRM expertise required.

---

## Context Gathering

Ask these questions conversationally, not as a checklist. Group them naturally.

### Current State
1. **How do leads come in today?** (Website form? Email? DMs? Word of mouth? All of the above?)
2. **What happens after someone shows interest?** (Walk me through what happens right now, even if it's messy)
3. **Where do leads get stuck or lost?** (The gap where people disappear)

### Volume & Capacity
4. **How many leads do you get per month?** (Rough number is fine)
5. **How many of those become customers?** (Even a guess helps)
6. **Who's responsible for following up?** (One person? A team? Nobody consistently?)

### Sales Process
7. **How long does it take from first contact to closed deal?** (Days? Weeks? Months?)
8. **What's the buying process look like?** (Demo → trial → close? One call close? Self-serve?)
9. **What tools are you using?** (CRM, spreadsheet, email, sticky notes — no judgment)

### Qualification
10. **What makes a lead "good" vs "bad"?** (Industry? Company size? Budget? Behavior?)
11. **Have you ever defined when a lead is "ready for sales"?** (Or does everyone just wing it?)

### Goals
12. **What's the one pipeline problem you want to fix first?** (More leads? Better conversion? Faster close? Less chaos?)

---

## Methodology

### Step 1: Check Product Marketing Context
Before building the pipeline, check if product marketing context exists:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to inform customer profile, pricing, and deal size for the pipeline design.

If no context exists, ask the user: *"Before we build your pipeline, I need to understand your product and customer. Let me ask a few quick questions."* Then run the **product-marketing-context** skill first.

### Step 2: Define Lifecycle Stages
Map out the journey from stranger to customer. Start with the standard framework, then customize based on their business:

**Default stages:**
1. **Visitor** — Someone who lands on your site or content. Anonymous. No contact info yet.
2. **Lead** — They gave you contact info. Form fill, signup, download, email reply.
3. **Qualified Lead** — They match your ideal customer profile AND have shown real interest.
4. **Opportunity** — Active sales conversation. They're evaluating your solution.
5. **Customer** — They paid. Deal closed.

**Customize for their business:**
- Simple businesses (freelancers, small services) may only need: Lead → Qualified → Customer
- Complex B2B may need sub-stages: Marketing Qualified Lead (MQL) → Sales Accepted Lead (SAL) → Sales Qualified Lead (SQL) → Opportunity → Customer
- Product-led growth may need: Visitor → Free User → Activated User → Qualified → Customer

Ask: *"Here's a standard pipeline. What feels right for your business, and what feels unnecessary?"*

### Step 3: Define Transition Criteria
For each stage transition, define the specific criteria that move a lead forward. No ambiguity.

**Visitor → Lead:**
- What action converts them? (Form submit, signup, email reply, chat message)
- What information do you capture? (Name, email, company, phone — minimum viable)

**Lead → Qualified Lead:**
- Demographic fit: Do they match your ideal customer? (Industry, size, role, budget)
- Behavioral signal: Have they shown real interest? (Visited pricing page, opened 3+ emails, requested demo, used free trial for 7+ days)
- Negative signals: What disqualifies? (Wrong industry, too small, competitor, student)

**Qualified → Opportunity:**
- Human conversation happened (call, demo, meeting)
- Budget confirmed or discussable
- Decision-maker identified
- Timeline exists (not "someday")

**Opportunity → Customer:**
- Proposal or quote sent
- Terms agreed
- Payment received or contract signed

Present these as a simple table and ask: *"Does this match how your deals actually work?"*

### Step 4: Build Lead Scoring Model
Create a simple scoring system the user can actually maintain. Two dimensions:

**Demographic Score (Who they are) — 0 to 50 points:**

| Factor | High Fit (full points) | Medium Fit (half points) | Low Fit (0 points) |
|--------|----------------------|------------------------|-------------------|
| Job title/role | Decision maker | Influencer | Researcher/student |
| Company size | Sweet spot range | Adjacent range | Too big or too small |
| Industry | Target industry | Related industry | Unrelated |
| Budget | In range | Uncertain | Confirmed too low |
| Location | Target market | Adjacent market | Outside market |

**Behavioral Score (What they do) — 0 to 50 points:**

| Action | Points |
|--------|--------|
| Visited pricing page | +10 |
| Downloaded resource | +5 |
| Attended webinar/event | +10 |
| Requested demo | +20 |
| Opened 3+ emails | +5 |
| Replied to email | +15 |
| Started free trial | +15 |
| Used product 3+ times | +10 |
| Visited site 5+ times | +5 |

**Score thresholds:**
- 0-30: Cold — nurture with content
- 31-60: Warm — increase engagement, targeted content
- 61-80: Hot — sales ready, route to closer
- 81-100: On fire — priority follow-up within 24 hours

Customize the factors and point values based on what the user said matters most in Context Gathering. Ask: *"Which of these signals actually predict a deal for you?"*

### Step 5: Define Lead Routing Rules
Determine who gets which leads, and when:

**By volume (solo or small team):**
- One person handles everything → simple, just define follow-up timing
- Round-robin → leads distributed evenly across reps

**By criteria (larger teams):**
- By territory (geographic region)
- By deal size (enterprise vs SMB)
- By product line
- By lead source (inbound vs outbound vs partner)

**Response time rules:**
- Hot leads (score 61+): Follow up within 1 hour during business hours
- Warm leads (score 31-60): Follow up within 24 hours
- Cold leads (score 0-30): Enter nurture sequence, no direct outreach

**Escalation rules:**
- No response in [X hours] → reassign or escalate
- Lead revisits after going cold → re-route as warm

Ask: *"Who should get leads, and how fast do they need to respond?"*

### Step 6: Design the Handoff Process
The marketing-to-sales handoff is where most leads die. Define it clearly:

**What marketing passes to sales:**
- Contact info (name, email, company, role)
- Lead score and breakdown (why they scored high)
- Activity history (pages visited, content downloaded, emails opened)
- Source (how they found you)
- Notes (anything relevant from prior interactions)

**How the handoff happens:**
- Automated: CRM moves lead to sales queue when score hits threshold
- Semi-automated: Marketing reviews and manually approves before routing
- Manual: Marketing sends a Slack message or email to sales with context

**Sales accepts or rejects:**
- Sales has 24 hours to accept the lead
- If rejected, sales must provide a reason (wrong fit, not ready, bad data)
- Rejected leads return to marketing nurture with the feedback

**Feedback loop:**
- Sales reports back: Did the lead close? Why or why not?
- Marketing uses this to improve scoring and targeting
- Monthly review: Are the right leads getting through?

Ask: *"What's realistic for your team right now? Automated, semi-automated, or manual?"*

### Step 7: Set Up Pipeline Reporting
Define the numbers that tell you if the pipeline is healthy:

**Core metrics (track weekly):**
- New leads this week
- Leads by source
- Lead-to-qualified conversion rate
- Qualified-to-opportunity conversion rate
- Opportunity-to-customer conversion rate
- Average time in each stage
- Pipeline value (total potential revenue in active opportunities)

**Health indicators:**
- Stage velocity: How long do leads sit in each stage? (Stuck leads = problem)
- Conversion by source: Which channels produce leads that actually close?
- Score accuracy: Do high-scoring leads actually convert more? (If not, fix scoring)
- Leakage points: Where are leads dropping out? (The biggest hole to fix first)

**Simple dashboard setup:**
If they use a CRM → show them what reports to build
If they use spreadsheets → give them a template structure
If they use nothing → recommend a starting tool based on their volume and budget

Ask: *"How do you want to see these numbers? Dashboard, weekly email, spreadsheet?"*

---

## Output Format

Deliver the complete pipeline plan in this structure:

```markdown
# Revenue Operations Pipeline Plan
Created: [DATE]

## Pipeline Overview
**Business:** [Name]
**Model:** [B2B/B2C/B2B2C]
**Avg deal size:** [Amount]
**Avg sales cycle:** [Duration]
**Current monthly leads:** [Volume]
**Current conversion rate:** [Percentage]

## Lifecycle Stages

| Stage | Definition | Entry Criteria | Exit Criteria |
|-------|-----------|---------------|--------------|
| Visitor | [Description] | [How they enter] | [What moves them forward] |
| Lead | [Description] | [How they enter] | [What moves them forward] |
| Qualified | [Description] | [How they enter] | [What moves them forward] |
| Opportunity | [Description] | [How they enter] | [What moves them forward] |
| Customer | [Description] | [How they enter] | [What moves them forward] |

## Lead Scoring Model

### Demographic Score (0-50)
| Factor | High Fit | Medium Fit | Low Fit | Points |
|--------|----------|------------|---------|--------|
| [Factor] | [Criteria] | [Criteria] | [Criteria] | [Max] |

### Behavioral Score (0-50)
| Action | Points |
|--------|--------|
| [Action] | [Points] |

### Score Thresholds
- **Cold (0-30):** [Action taken]
- **Warm (31-60):** [Action taken]
- **Hot (61-80):** [Action taken]
- **On fire (81-100):** [Action taken]

## Lead Routing
**Method:** [Round-robin / Territory / Deal size / etc.]
**Response times:**
- Hot: [Timeframe]
- Warm: [Timeframe]
- Cold: [Timeframe]

**Escalation:** [What happens when leads aren't followed up]

## Handoff Process
**From:** Marketing
**To:** [Sales person/team]
**Trigger:** [Score threshold or action]
**Information passed:** [What sales receives]
**Acceptance window:** [Timeframe]
**Rejection process:** [What happens]

## Reporting
**Frequency:** [Weekly/monthly]
**Key metrics:**
1. [Metric]
2. [Metric]
3. [Metric]

**Review cadence:** [When and who reviews pipeline health]

## Immediate Next Steps
1. [First thing to do today]
2. [Second thing to do this week]
3. [Third thing to do this month]
```

---

## Quality Checks

Before delivering the pipeline plan, verify:

- [ ] **Stages match the business model** — a solo consultant doesn't need MQL/SAL/SQL; an enterprise SaaS might
- [ ] **Transition criteria are specific and observable** — not "seems interested" but "visited pricing page + opened 3 emails"
- [ ] **Lead scoring reflects real buying signals** — based on what actually predicts deals, not theory
- [ ] **Routing rules are realistic for team size** — don't design for 10 reps if they have 2
- [ ] **Response time commitments are achievable** — 1-hour response is pointless if nobody can do it
- [ ] **Handoff includes enough context** — sales shouldn't have to ask "who is this person?"
- [ ] **Feedback loop exists** — sales tells marketing what happened, marketing adjusts
- [ ] **Metrics are trackable with current tools** — don't require Salesforce reports if they use a spreadsheet
- [ ] **Next steps are immediately actionable** — not "implement a CRM" but "create these 5 columns in your spreadsheet"
- [ ] **No jargon unless the user used it** — say "follow-up rules" not "SLA-based routing protocols"

If any check fails, simplify before delivering.

---

## Examples

### Example 1: Solo Founder, Early Stage

**User says:** "Leads are coming in from my website but I don't know who to follow up with first."

**You do:**
1. Check for product marketing context (found: B2B SaaS, $99/mo, targeting small marketing teams)
2. Ask about current volume: "About 30 leads a month from the signup form"
3. Ask about follow-up: "I try to email everyone but I run out of time by Wednesday"
4. Ask about what makes a good lead: "Marketing managers at agencies, they close fastest"

**You deliver:**
- 3 simplified stages: Lead → Qualified → Customer (skip the complexity)
- Scoring focused on two factors: Is it an agency? Did they use the free trial?
- No routing needed (solo founder) — just a priority order
- Response rules: Agencies who used the trial → same day. Everyone else → within 48 hours.
- Tracking in a simple spreadsheet with 4 columns: Name, Score, Stage, Last Contact

**Key insight you share:** *"You don't need a CRM yet. You need a scoring shortcut. Every morning, check for agency signups who used the trial — those are your calls for the day. Everyone else gets a nurture email."*

### Example 2: Growing Team, Pipeline Chaos

**User says:** "We have 3 sales reps and nobody knows who's supposed to follow up on what."

**You do:**
1. Check for product marketing context (found: B2B service, $5k/mo retainer, targeting mid-market SaaS companies)
2. Ask about current process: "Marketing runs ads, leads go into HubSpot, reps fight over them"
3. Ask about volume: "About 200 leads a month, maybe 20 become calls, 3-5 close"
4. Ask about what's broken: "Two reps called the same lead last week. Some leads sit for days untouched."

**You deliver:**
- 5 stages: Lead → MQL → SQL → Opportunity → Customer
- Scoring model with 10 factors (they have the volume to justify complexity)
- Round-robin routing with territory override (West Coast, East Coast, International)
- 4-hour SLA for hot leads with auto-reassignment if missed
- HubSpot workflow recommendations (they already use it)
- Weekly pipeline review meeting agenda
- Dashboard with 6 key metrics

**Key insight you share:** *"Your problem isn't lead volume — it's that 200 leads come in and nobody owns the sorting. Set up auto-scoring in HubSpot and auto-assign by territory. Your reps should wake up to a prioritized list, not a pile."*

### Example 3: Fixing a Leaky Pipeline

**User says:** "We get plenty of demos booked but deals stall after the first call."

**You do:**
1. Check for product marketing context (found: B2B SaaS, $500/mo, targeting HR teams)
2. Focus on the Opportunity → Customer transition (where they're stuck)
3. Ask about the demo process: "Rep does a 30-min demo, sends a follow-up email, then... silence"
4. Ask about close rate: "Maybe 10% of demos become customers. It used to be 25%."

**You deliver:**
- Tighter qualification before the demo (add a pre-demo questionnaire: budget, timeline, decision process)
- Post-demo follow-up sequence (not one email — a 5-touch sequence over 14 days)
- Deal stage sub-steps: Demo Done → Proposal Sent → Negotiation → Verbal Yes → Closed
- Stale deal rules: No activity in 7 days → rep gets a nudge. 14 days → manager reviews. 21 days → moved to nurture.
- Win/loss analysis template for every closed-lost deal

**Key insight you share:** *"Demos aren't closing because unqualified leads are getting on calls. Add 3 qualifying questions before the demo gets booked: What's your budget? When do you need a solution? Who else is involved in the decision? This alone will cut your demo volume in half but double your close rate."*

---

## Related Skills

- **analytics-tracking** — instruments the pipeline so you can measure conversion at every stage
- **lead-magnets** — creates the content that turns visitors into leads at the top of the funnel
- **cold-email** — writes outbound sequences for leads that match your ideal profile
- **sales-enablement** — builds the decks, one-pagers, and battle cards your sales team needs to close
- **product-marketing-context** — defines the customer profile and positioning that informs lead scoring and qualification

**This skill depends on product-marketing-context.** Always check for it first. You can't score leads if you don't know who your ideal customer is.
