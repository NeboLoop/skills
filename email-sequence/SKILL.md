---
name: email-sequence
description: Design multi-email automated sequences that guide people from sign-up to purchase, first use to power user, or going quiet to coming back. Use when building welcome, nurture, onboarding, re-engagement, or drip campaigns.
---

# Email Sequence

Design multi-email automated sequences that guide people from sign-up to purchase, from first use to power user, or from going quiet to coming back.

---

## When to Use

Trigger this skill when you hear:
- "Create a welcome sequence"
- "I need a nurture sequence"
- "Build me an onboarding email flow"
- "Help me re-engage inactive users"
- "Design an abandoned cart sequence"
- "Set up a drip campaign"
- "I want to automate my follow-up emails"
- "Map out my email flow"
- "What emails should I send after someone signs up?"

---

## Context Gathering

Before designing any sequence, you need to understand the business and the specific situation. Ask conversationally — don't dump a form.

### About the Sequence
1. **What kind of sequence do you need?** (Welcome new subscribers? Onboard new customers? Win back people who stopped engaging? Nurture leads who aren't ready to buy?)
2. **What's the goal?** (Get them to buy? Get them using the product? Get them to book a call? Just build trust?)
3. **What triggers this sequence?** (Someone signs up? Makes a purchase? Hasn't logged in for 30 days? Downloads a lead magnet?)

### About the Audience
4. **Who's receiving these emails?** (New leads? Paying customers? Free trial users? People who went quiet?)
5. **What do they know about you at the start of this sequence?** (Nothing? They just signed up? They've been a customer for months?)
6. **What objections or hesitations might they have?** (Price? Trust? Not understanding the product? Already using a competitor?)

### About the Business
7. **What's your current email setup?** (What tool do you use? Do you have any sequences running now?)
8. **Do you have existing content you want to reference?** (Blog posts, case studies, testimonials, videos?)

If product-marketing-context exists in memory, skip questions about the product, customer, and voice — you already know those.

---

## Methodology

### Step 1: Check Product Marketing Context

Pull the stored context so every email matches your brand:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. Every sequence needs to know: what you sell, who you sell to, what makes you different, and how you talk to people.

### Step 2: Identify Sequence Type

Determine which type of sequence fits the situation:

| Type | Trigger | Goal |
|------|---------|------|
| **Welcome** | New subscriber or sign-up | Build trust, introduce your product, get first action |
| **Nurture** | Lead not ready to buy | Educate, build authority, move toward purchase |
| **Onboarding** | New customer or trial user | Get them using the product, show value fast |
| **Re-engagement** | Inactive for X days | Bring them back or clean your list |
| **Abandoned cart** | Started checkout, didn't finish | Recover the sale |

If the user isn't sure, ask: *"What just happened that makes someone enter this sequence?"* The trigger tells you the type.

### Step 3: Map the Customer Journey

Before writing a single email, map where people are mentally at each stage:

- **Entry point** — What just happened? What are they thinking and feeling?
- **Early stage** — What do they need to know first? What's their biggest question?
- **Middle stage** — What builds confidence? What proof do they need?
- **Late stage** — What's the final push? What's holding them back?
- **Exit point** — What action signals they're done with this sequence?

Write this map out. It guides everything.

### Step 4: Define Email Count and Timing

Set the number of emails and the spacing between them. General guidelines:

- **Welcome sequences:** 5-7 emails over 2 weeks
- **Nurture sequences:** 6-10 emails over 3-6 weeks
- **Onboarding sequences:** 4-7 emails over 10 days
- **Re-engagement sequences:** 3-4 emails over 10 days
- **Abandoned cart sequences:** 3 emails over 3 days

Typical timing pattern:
- Day 0 (immediate): First email
- Day 1-2: Second email
- Day 3-5: Third email
- Day 7: Fourth email
- Day 10-14: Fifth email

Spacing gets wider as the sequence goes on. Early emails come fast because motivation is high. Later emails give people breathing room.

### Step 5: Write Each Email

For every email in the sequence, define:

1. **Goal** — The one thing this email should accomplish
2. **Subject line** — Short, specific, curiosity-driven (write 2-3 options)
3. **Body copy** — Conversational, focused on the reader, not the product
4. **Call to action** — One clear next step (not three)
5. **Connection to previous email** — How this builds on what came before

Keep emails short. 150-300 words for most emails. People scan — make it scannable.

### Step 6: Build Progressive Value

Each email must build on the last. Never repeat the same message with different words. The progression looks like:

1. **Welcome/Set expectations** — Here's what you'll get from us
2. **Quick win** — Here's something useful right now
3. **Deeper value** — Here's how to get more from this
4. **Social proof** — Here's someone like you who got results
5. **Ask** — Here's the next step when you're ready

Every email earns the right to send the next one. If someone could skip an email and not miss anything, that email shouldn't exist.

### Step 7: Define Exit Conditions

People should leave the sequence when:
- **They completed the goal** — Purchased, activated, booked a call
- **They unsubscribed** — Respect it immediately
- **They replied** — Move to personal conversation
- **They entered a higher-priority sequence** — A purchase sequence beats a nurture sequence
- **The sequence ended** — Move them to your regular newsletter or a different sequence

Map these exit conditions explicitly. Nobody should get a "buy now" email the day after they already bought.

### Step 8: Save the Sequence

Save the complete sequence using Nebo's file tools:
```
file(action: "write", path: "email-sequences/[sequence-name].md", content: "Full sequence document")
```

Also store a reference in memory:
```
agent(resource: "memory", action: "store", key: "email/sequences/[name]", value: "Summary of sequence purpose and status", layer: "tacit")
```

---

## Output Format

Deliver the sequence in this structure:

```markdown
# [Sequence Name] Email Sequence
Created: [DATE]

## Overview
**Type:** [Welcome / Nurture / Onboarding / Re-engagement / Abandoned Cart]
**Trigger:** [What starts this sequence]
**Goal:** [What success looks like]
**Emails:** [Number]
**Duration:** [Total days]
**Exit conditions:**
- [Condition 1]
- [Condition 2]
- [Condition 3]

## Customer Journey Map
**Entry mindset:** [What they're thinking when they enter]
**Exit mindset:** [What they should think/feel/do when they leave]

---

## Email 1: [Name]
**Send:** [Day 0 / Immediately]
**Goal:** [One sentence]
**Subject line options:**
1. [Option A]
2. [Option B]

**Body:**
[Full email copy]

**CTA:** [Button text or link text]

---

## Email 2: [Name]
**Send:** [Day X]
**Goal:** [One sentence]
**Subject line options:**
1. [Option A]
2. [Option B]

**Body:**
[Full email copy]

**CTA:** [Button text or link text]

[Continue for each email...]

---

## Notes
- [Implementation notes, tool-specific settings, segmentation tips]
```

---

## Quality Checks

Before delivering the sequence, verify:

- [ ] **Every email has exactly one goal** — not two things crammed together
- [ ] **Subject lines are under 50 characters** — they get cut off on mobile otherwise
- [ ] **Each email builds on the previous one** — reading them in order tells a story
- [ ] **The first email delivers immediate value** — not just "thanks for signing up"
- [ ] **CTAs are specific actions** — "Start your first project" not "Click here"
- [ ] **Exit conditions are defined** — nobody gets irrelevant emails
- [ ] **Timing makes sense** — not too fast (annoying), not too slow (forgotten)
- [ ] **Tone matches the brand voice** — pulled from product-marketing-context
- [ ] **No email is filler** — every email earns the right to send the next one
- [ ] **The sequence has a clear end** — people don't stay in it forever

---

## Examples

### Example: 5-Email Welcome Sequence for a Project Management Tool

**Context:** A SaaS tool that helps freelancers manage client projects. $29/month. Friendly, straightforward voice. Customer is a freelance designer who just signed up for a free trial.

---

**Overview**
- **Type:** Welcome / Onboarding
- **Trigger:** New free trial sign-up
- **Goal:** Get the user to create their first project and invite a client within 7 days
- **Emails:** 5
- **Duration:** 10 days
- **Exit conditions:** Created a project and invited a client, upgraded to paid, unsubscribed

---

**Email 1: The Welcome**
- **Send:** Day 0 (immediately)
- **Goal:** Set expectations and get them to create their first project
- **Subject lines:**
  1. Your first project starts here
  2. You're in — here's your one next step

**Body:**

> Hey [First Name],
>
> Welcome to [Product]. You signed up because managing client projects was getting messy — scattered emails, lost files, awkward status updates. We're going to fix that.
>
> Here's the only thing I'd suggest doing today:
>
> **Create your first project.** It takes about 2 minutes. Just give it a name, add a deadline, and you're set.
>
> [Create Your First Project]
>
> Over the next week, I'll send you a few short emails showing you the features that freelancers tell us save them the most time. No fluff — just the stuff that actually matters.
>
> Talk soon,
> [Name] from [Product]

**CTA:** Create Your First Project

---

**Email 2: The Quick Win**
- **Send:** Day 2
- **Goal:** Show them the client portal feature — the thing that makes people stay
- **Subject lines:**
  1. The feature freelancers love most
  2. Stop sending status update emails

**Body:**

> Hey [First Name],
>
> The number one thing freelancers tell us changed their workflow: the client portal.
>
> Instead of sending your client emails like "here's the latest version" or "where are we on feedback?" — you give them a link. They see progress, leave comments, and approve work. All in one place.
>
> No more digging through email threads. No more "did you see my last message?"
>
> **Set up your client portal in about 60 seconds:**
>
> [Set Up Client Portal]
>
> If you already created a project, the portal is ready — you just need to invite your client.
>
> [Name]

**CTA:** Set Up Client Portal

---

**Email 3: The Social Proof**
- **Send:** Day 5
- **Goal:** Build confidence through a real customer story
- **Subject lines:**
  1. How Sara went from 5 clients to 12
  2. "I got 10 hours back every week"

**Body:**

> Hey [First Name],
>
> Sara is a freelance brand designer in Austin. A year ago, she was juggling 5 clients with spreadsheets and email. She was working late most nights — not on design, but on project admin.
>
> She started using [Product] and within a month:
> - Cut her project admin time from 10 hours/week to under 2
> - Took on 7 more clients without working longer hours
> - Stopped losing track of client feedback
>
> The thing she says made the biggest difference? "My clients stopped emailing me for updates. They just check the portal."
>
> If you haven't invited a client yet, that's the move:
>
> [Invite Your First Client]
>
> [Name]

**CTA:** Invite Your First Client

---

**Email 4: The Objection Handler**
- **Send:** Day 7
- **Goal:** Address the biggest hesitation — "I don't have time to learn a new tool"
- **Subject lines:**
  1. "I don't have time for another tool"
  2. 10 minutes is all it takes

**Body:**

> Hey [First Name],
>
> The most common thing I hear from freelancers: "I know I need a better system, but I don't have time to set one up."
>
> Fair. So here's what "set up" actually looks like:
>
> - **2 minutes:** Create a project, add deadlines
> - **1 minute:** Invite your client to the portal
> - **5 minutes:** Move your current tasks from wherever they are now
>
> That's it. Under 10 minutes and your next project is running in [Product].
>
> You don't need to move everything over at once. Start with one project. If it saves you time (it will), move the rest.
>
> [Start With One Project]
>
> [Name]

**CTA:** Start With One Project

---

**Email 5: The Nudge**
- **Send:** Day 10
- **Goal:** Create gentle urgency around the trial ending, offer help
- **Subject lines:**
  1. Your trial wraps up soon
  2. Quick question before your trial ends

**Body:**

> Hey [First Name],
>
> Your free trial is winding down in a few days. Two quick things:
>
> **1. If you've been using [Product]** and it's working for you — great. Your projects, clients, and files all carry over when you upgrade. Nothing changes except you keep access.
>
> [Upgrade and Keep Your Projects]
>
> **2. If you haven't had a chance to try it yet** — I totally get it. Freelancer life is unpredictable. If you want, I can extend your trial. Just reply to this email and say "more time" and I'll add another week.
>
> Either way, I'm here if you have questions. Just hit reply.
>
> [Name]

**CTA:** Upgrade and Keep Your Projects

---

### Example Interaction

**User says:** "I need a re-engagement sequence for people who stopped opening our emails."

**You do:**
1. Check memory for product-marketing-context (found: B2B SaaS, HR software, professional tone)
2. Ask: "How long has it been since they last engaged? And roughly how many people are we talking about?"
3. User: "30+ days of no opens. About 2,000 people."
4. Ask: "Before they went quiet, what were they getting from you? Newsletter? Product updates?"
5. User: "Weekly newsletter with HR tips."
6. Ask: "What's the goal — get them reading again, or clean your list?"
7. User: "Both. Re-engage who we can, remove who we can't."
8. Design a 3-email re-engagement sequence:
   - Email 1 (Day 0): "We noticed you've been quiet" — share your single best piece of content
   - Email 2 (Day 4): "Is this still useful?" — offer to change frequency or topics
   - Email 3 (Day 8): "Last email unless you say stay" — clear stay/leave choice, auto-remove non-responders
9. Present the sequence with full copy
10. Save to file and memory

---

## Related Skills

- **copywriting** — write the actual email copy once you have the sequence structure
- **cold-email** — for outbound prospecting emails (different from automated sequences)
- **product-marketing-context** — must exist before building any sequence — provides voice, positioning, and customer details
- **churn-prevention** — re-engagement sequences often connect to broader churn strategies
- **lead-magnets** — what brings people into the sequence in the first place

**Always check product-marketing-context before building a sequence.** The emails need to sound like your brand.
