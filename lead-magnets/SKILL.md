---
name: lead-magnets
description: Create lead magnets that attract qualified leads including checklists, templates, guides, calculators, swipe files, and mini-courses. Use when building your email list, creating a downloadable freebie, or designing a content offer with landing page copy and follow-up email sequence.
---

# Lead Magnets

Create lead magnets that attract qualified leads: checklists, templates, guides, calculators, and more.

---

## When to Use

Trigger this skill when you hear:
- "I need a lead magnet"
- "Create a freebie for my audience"
- "I want to build my email list"
- "Make a checklist for..."
- "I need a downloadable template"
- "Help me create a guide to attract leads"
- "I want to offer something free to capture emails"
- "Build a calculator for my website"
- "I need a PDF to give away"
- "What should I offer for free?"

This skill creates the actual content and supporting assets (landing page copy, follow-up emails). If you need to figure out what kind of lead magnet to create, start here — the methodology walks through format selection.

---

## Context Gathering

Before building anything, you need to understand the business and the audience. Ask these conversationally, not as a form.

### Audience & Problem
1. **Who is this lead magnet for?** (Specific persona — job title, situation, experience level)
2. **What immediate problem are they trying to solve?** (The thing they'd search for right now)
3. **Where are they in the buying journey?** (Just discovering the problem? Comparing solutions? Ready to buy?)

### Format & Scope
4. **Do you have a format in mind?** (Checklist, template, guide, calculator, swipe file, assessment, mini-course — or should I recommend one?)
5. **How much time should it take them to consume?** (5 minutes? 30 minutes? A week?)
6. **What specific result should they walk away with?** (Not "learn about X" — what can they DO after?)

### Business Goals
7. **What do you sell, and how does this lead magnet connect to it?** (The lead magnet should be the logical first step toward your paid offer)
8. **What happens after someone downloads it?** (Email sequence? Sales call? Free trial? Nothing yet?)
9. **Where will you promote this?** (Blog, ads, social, partnerships — affects how you position it)

If product-marketing-context exists in memory, skip questions that are already answered there. Reference what you know and confirm it's still accurate.

---

## Methodology

### Step 1: Check Product Marketing Context
Pull existing context from memory:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to pre-fill what you know about the product, customer, and positioning. If it doesn't exist, gather enough to proceed — but recommend running the product-marketing-context skill afterward for a complete foundation.

### Step 2: Identify Target Persona and Their Immediate Problem
Get specific about who this is for. A lead magnet that tries to appeal to everyone attracts nobody qualified.

**Good persona:** "First-time managers at tech companies who just inherited a team and have no idea how to run a 1:1"
**Bad persona:** "Managers" or "People interested in leadership"

The problem must be something they feel right now, not something they should care about. Urgency drives downloads.

### Step 3: Choose the Format
Match the format to the problem and the persona's situation:

| Format | Best When | Example |
|---|---|---|
| **Checklist** | They need to make sure they don't miss anything | "Launch Day Checklist: 47 Things Before You Go Live" |
| **Template** | They need a starting point they can customize | "Cold Email Templates That Book Meetings" |
| **Guide** | They need to understand a process end-to-end | "The Complete Guide to Running Your First Webinar" |
| **Calculator** | They need to quantify something (ROI, costs, sizing) | "How Much Is Employee Turnover Costing You?" |
| **Swipe file** | They want proven examples to model | "50 High-Converting Subject Lines (With Why They Work)" |
| **Assessment** | They want to benchmark themselves | "Rate Your Sales Process: Find Your Biggest Leak" |
| **Mini-course** | The topic requires sequential learning | "5-Day Email Course: Nail Your Positioning" |

**Decision criteria:**
- If the problem is "I don't know where to start" → Guide or Mini-course
- If the problem is "I keep forgetting steps" → Checklist
- If the problem is "I need to do this but don't want to start from scratch" → Template or Swipe file
- If the problem is "I need to justify a decision with numbers" → Calculator
- If the problem is "I don't know how I'm doing compared to others" → Assessment

### Step 4: Design the Content
Every lead magnet must promise a specific, achievable result. Not "learn about email marketing" but "write your first welcome sequence in 30 minutes."

**Structure the content with these principles:**
- **Title formula:** [Specific Result] + [Timeframe or Constraint] — e.g., "Write Your Homepage in 2 Hours (Even If You Hate Writing)"
- **Open with the pain:** One paragraph that makes them feel understood
- **Deliver the framework:** The actual content — steps, templates, formulas, examples
- **Show what good looks like:** Include at least one worked example
- **Bridge to your product:** End with a natural connection to what you sell, not a hard pitch

**Content length guidelines:**
- Checklist: 1-3 pages
- Template: 1-5 pages with instructions
- Guide: 5-15 pages (longer isn't better — tighter is)
- Calculator: functional spreadsheet or interactive tool spec
- Swipe file: 10-50 examples with annotations
- Assessment: 10-20 questions with scoring
- Mini-course: 3-7 emails, each under 500 words

### Step 5: Write the Content
Write the full lead magnet content. Match the brand voice from product-marketing-context. If no brand voice is defined, default to clear, direct, and conversational.

**Writing rules:**
- Every section must earn its place — cut anything that doesn't help them get the result
- Use specific numbers over vague claims ("47 items" not "comprehensive list")
- Include real examples, not hypothetical ones when possible
- Format for scanning: headers, bullets, bold key points, numbered steps
- End each major section with an action item or takeaway

### Step 6: Write the Landing Page Copy
Create the page that converts visitors into downloads. Structure:

1. **Headline:** States the specific result they'll get
2. **Subheadline:** Adds context — who it's for, how fast, or what's included
3. **Pain paragraph:** 2-3 sentences that describe their current frustration
4. **What's inside:** Bullet list of what's in the lead magnet (benefit-oriented, not feature-oriented)
5. **Social proof:** Where to place testimonials, download counts, or credibility markers
6. **CTA:** Clear button text (not "Submit" — something like "Get the Checklist" or "Send Me the Templates")
7. **Form fields:** Recommend minimum fields needed (usually just email, maybe first name)

### Step 7: Write the Follow-Up Email Sequence
Design 3-5 emails that run after download:

| Email | Timing | Purpose |
|---|---|---|
| **Email 1: Delivery** | Immediate | Deliver the lead magnet, set expectations for what's next |
| **Email 2: Quick win** | Day 1-2 | Help them get a quick result from the lead magnet |
| **Email 3: Deeper value** | Day 3-5 | Share a related insight or case study |
| **Email 4: Bridge** | Day 5-7 | Connect the problem to your paid solution |
| **Email 5: Soft offer** | Day 7-10 | Present your product as the next logical step |

Write actual email copy, not just descriptions. Each email should be under 300 words and have one clear CTA.

---

## Output Format

Deliver the complete lead magnet package in this structure:

```markdown
# Lead Magnet: [Title]

## Overview
**Format:** [Checklist / Template / Guide / Calculator / Swipe File / Assessment / Mini-Course]
**Target persona:** [Who this is for]
**Problem addressed:** [The immediate pain point]
**Promise:** [The specific result they'll achieve]
**Estimated consumption time:** [How long it takes to use]

## Lead Magnet Content
[Full content of the lead magnet — the actual deliverable]

## Landing Page Copy
**Headline:** [Main headline]
**Subheadline:** [Supporting line]
**Pain paragraph:** [2-3 sentences]
**What's inside:**
- [Benefit 1]
- [Benefit 2]
- [Benefit 3]
- [Benefit 4]
**CTA button text:** [Button copy]
**Recommended form fields:** [Fields needed]

## Follow-Up Email Sequence

### Email 1: Delivery (Send immediately)
**Subject line:** [Subject]
**Body:** [Full email copy]

### Email 2: Quick Win (Day 1-2)
**Subject line:** [Subject]
**Body:** [Full email copy]

### Email 3: Deeper Value (Day 3-5)
**Subject line:** [Subject]
**Body:** [Full email copy]

### Email 4: Bridge (Day 5-7)
**Subject line:** [Subject]
**Body:** [Full email copy]

### Email 5: Soft Offer (Day 7-10)
**Subject line:** [Subject]
**Body:** [Full email copy]
```

---

## Quality Checks

Before delivering, verify:

- [ ] **Title promises a specific result** — not vague ("improve your marketing") but concrete ("write 5 cold emails that book meetings")
- [ ] **Target persona is narrow enough** — a lead magnet for "everyone" attracts no one qualified
- [ ] **Format matches the problem** — a guide for something that needs a checklist wastes their time
- [ ] **Content delivers the promise** — if the title says "in 30 minutes," the content must be completable in 30 minutes
- [ ] **Every section earns its place** — no filler, no fluff, no padding to make it look longer
- [ ] **Bridge to paid offer is natural** — not a bait-and-switch, but a logical next step
- [ ] **Landing page has one clear CTA** — no competing actions, no navigation distractions
- [ ] **Email sequence builds momentum** — each email has a reason to exist and moves them forward
- [ ] **Brand voice is consistent** — matches product-marketing-context tone throughout
- [ ] **Content is actionable, not theoretical** — they can DO something after consuming it

---

## Examples

### Example 1: B2B SaaS — Checklist

**User says:** "I need a lead magnet for my project management tool. We target freelancers."

**You do:**
1. Check product-marketing-context (found: PM tool for freelancers, $19/mo, casual voice)
2. Ask: "What's the biggest headache freelancers have that your tool solves?"
3. User: "They lose track of deadlines and miss client deliverables."
4. Ask: "What result should they walk away with from this lead magnet?"
5. User: "A system for never missing a deadline again."
6. Recommend format: Checklist — it's a repeatable process they can follow
7. Create: "The Never-Miss-a-Deadline Checklist: 23 Steps to Bulletproof Your Freelance Projects"
8. Write the full checklist content with actionable items
9. Write landing page copy targeting freelancers who've missed deadlines
10. Write 5 follow-up emails bridging to the PM tool as the system that automates the checklist

**Delivered package includes:**
- 23-item checklist organized by project phase (kickoff, mid-project, delivery)
- Landing page with headline: "Stop Missing Deadlines. Start Keeping Clients."
- 5-email sequence ending with a free trial offer for the PM tool

### Example 2: Service Business — Guide

**User says:** "I run an accounting firm for e-commerce businesses. I want something to attract new clients."

**You do:**
1. Check product-marketing-context (found: accounting for Shopify sellers, $500-2k/mo, professional but approachable voice)
2. Ask: "What keeps your ideal client up at night about their finances?"
3. User: "They have no idea if they're actually profitable. Revenue looks good but they don't understand their margins."
4. Ask: "What's the one thing they could do this week to get clarity?"
5. User: "Calculate their true profit margin including all the hidden costs they forget about."
6. Recommend format: Calculator + short guide — they need numbers and context
7. Create: "The Real Profit Calculator for E-Commerce: Find Out What You Actually Keep"
8. Write a guide explaining the 7 hidden costs most Shopify sellers miss
9. Include a calculator template (spreadsheet spec) they can fill in
10. Write landing page and email sequence bridging to a free profit analysis call

### Example 3: Mini-Course for Complex Topic

**User says:** "I sell a hiring platform. Companies don't know how to write job posts that attract good candidates."

**You do:**
1. Check product-marketing-context (found: hiring platform for startups, $299/mo, direct and no-nonsense voice)
2. Ask: "How long does it typically take someone to learn to write a good job post?"
3. User: "If they follow a framework, they could do it in a week."
4. Recommend format: Mini-course — sequential learning with daily practice
5. Create: "5-Day Job Post Bootcamp: Write Listings That Attract A-Players"
6. Design 5 emails, each teaching one element (title, opener, requirements, culture, CTA)
7. Each email includes one exercise they complete that day
8. Landing page positions it as "the free course hiring managers at Stripe and Notion wish they had"
9. Follow-up sequence bridges to the platform as the tool that operationalizes what they learned

---

## Related Skills

- **copywriting** — writes the actual copy for any lead magnet surface
- **email-sequence** — designs more complex nurture flows beyond the basic 5-email post-download sequence
- **page-cro** — optimizes the landing page for higher conversion rates
- **form-cro** — optimizes the opt-in form to maximize completions
- **free-tool-strategy** — plans interactive lead magnets (calculators, assessments) as standalone tools
- **product-marketing-context** — provides the foundation every lead magnet builds on
