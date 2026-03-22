---
name: sales-enablement
description: Create sales support materials that help your team close deals. Use when building a sales deck, writing a one-pager, creating an objection handling doc, designing a competitive battle card, or preparing pitch materials and talking points for sales conversations.
---

# Sales Enablement

Create sales support materials that help your team close deals — decks, one-pagers, objection handling docs, and battle cards.

---

## When to Use

Trigger this skill when you hear:
- "I need a sales deck"
- "Create a one-pager for..."
- "Help me handle objections about..."
- "Build a battle card against [competitor]"
- "My sales team needs materials"
- "We're losing deals to [competitor]"
- "Help me with sales collateral"
- "I need talking points for..."
- "Put together a pitch for..."
- "How do we position against [competitor]?"

This skill creates the materials your sales team uses in conversations with prospects. It's not strategy — it's the actual documents they carry into calls and meetings.

---

## Context Gathering

Before creating any material, you need to understand the situation. Ask conversationally, not as a checklist.

### What Are We Building?
1. **What type of material do you need?** (Sales deck, one-pager, objection doc, battle card, or something else?)
2. **Who will use this?** (The sales rep persona — AE, SDR, founder doing sales, partner?)
3. **Who's the audience?** (The prospect — job title, seniority, technical or non-technical?)

### The Sales Situation
4. **Where does this fit in the sales process?** (First call, demo follow-up, proposal stage, competitive evaluation?)
5. **What's the prospect's main pain point?** (The problem that got them on the call)
6. **What objections do you hear most often?** (Price, switching cost, trust, feature gaps, timing?)

### Competitive Landscape
7. **Who are you competing against in this deal?** (Named competitors or "do nothing")
8. **What do they say about you?** (The FUD competitors spread)
9. **Where do you genuinely win vs. lose?** (Honest assessment — your materials should lean into real strengths)

### Proof Points
10. **What customer results can you reference?** (Case studies, metrics, logos, quotes)
11. **Do you have specific numbers?** (ROI, time saved, revenue increased, adoption rates)
12. **Any analyst or third-party validation?** (G2, Forrester, awards, press)

---

## Methodology

### Step 1: Check Product Marketing Context

Always start by pulling the existing product marketing context:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. You need positioning, differentiation, customer profile, and brand voice before creating any sales material.

If context exists, review it and confirm it's current: *"I have your marketing context from [date]. Is this still accurate for what we're building today?"*

### Step 2: Identify Material Type and Audience

Based on the user's request, determine:
- **Material type:** Sales deck, one-pager, objection handling doc, battle card, or custom format
- **Primary audience:** The prospect persona who will read/see this
- **Sales stage:** Where this gets used in the buying process
- **Tone calibration:** Match the audience (C-suite wants outcomes, practitioners want details, technical buyers want specs)

If the user isn't sure what they need, ask: *"Tell me about the deal situation. What conversation are you preparing for?"* Then recommend the right format.

### Step 3: Structure by Format

Each format has a proven structure. Follow these frameworks.

#### Sales Deck (10-15 slides)

1. **The World Has Changed** — name the shift happening in their industry
2. **The Problem** — what this shift means for the prospect (pain they feel today)
3. **The Cost of Inaction** — what happens if they do nothing (make the status quo uncomfortable)
4. **The Solution Exists** — introduce the new way (not your product yet — the category or approach)
5. **How It Works** — now introduce your product and show how it delivers the solution
6. **Proof** — customer stories, metrics, logos (make it specific and relevant to their industry)
7. **Differentiation** — why you vs. alternatives (honest comparison, not trash talk)
8. **Pricing / Packaging** — how to buy, what they get, investment required
9. **Next Steps** — clear call to action (pilot, trial, deeper demo, proposal)
10. **Appendix** — detailed features, integrations, security, compliance (for follow-up questions)

**Slide rules:**
- One idea per slide
- Headlines tell the story — someone scanning just headlines should get the full pitch
- Minimize text — slides are visual aids, not documents
- Suggest speaker notes for what the rep should say

#### One-Pager

Structure it as a single-page document the prospect can share internally:

1. **Headline** — the value proposition in one sentence (not your product name — the outcome)
2. **The Problem** — 2-3 sentences describing the pain (use the prospect's language)
3. **The Solution** — what your product does and how it solves the problem (keep it high-level)
4. **Key Benefits** — 3-4 bullet points, each with a specific outcome (not features — results)
5. **Proof Point** — one strong customer result with a real number
6. **How It Works** — 3 steps showing the path from "today" to "success" (simplify the process)
7. **Call to Action** — one clear next step with contact info

**One-pager rules:**
- Must fit on a single page when printed
- Write for the person who wasn't on the call — this gets forwarded to decision makers
- Lead with outcomes, not features
- One proof point is better than five weak ones

#### Objection Handling Doc

Structure as a reference the sales team uses during or after calls:

1. **Introduction** — brief context on when and how to use this doc
2. **Top 10 Objections** — ranked by frequency, each with:
   - **The objection** — written exactly as the prospect says it (not sanitized)
   - **What they really mean** — the underlying concern behind the words
   - **The response** — a natural, conversational reply (not a script — a framework)
   - **Proof point** — evidence that backs up the response (customer quote, data, case study)
   - **If they push back** — a follow-up response if the first answer doesn't land
3. **Red Flags** — signals that this deal might not close (helps reps qualify out early)
4. **Competitive Objections** — specific objections that come from competitor influence

**Objection doc rules:**
- Write responses as conversation, not corporate speak
- Include the actual words prospects use — reps need to recognize the objection in real time
- Be honest — if a competitor is genuinely better at something, acknowledge it and redirect
- Keep responses under 3 sentences — reps need to be concise on calls

#### Battle Card

Structure as a quick-reference competitive comparison:

1. **Overview** — who the competitor is, their positioning, their ideal customer
2. **Quick Comparison Table** — side-by-side on 5-8 key dimensions that matter to buyers
3. **Where We Win** — 3-4 areas with specific proof (not opinions — evidence)
4. **Where They Win** — 1-2 areas where they're genuinely stronger (builds credibility with reps)
5. **Their Pitch** — what the competitor says about themselves and about you
6. **Our Counter** — how to respond to their claims (with talk tracks)
7. **Landmine Questions** — questions your rep can ask the prospect that expose competitor weaknesses (without being obvious)
8. **Customer Proof** — quotes or stories from customers who evaluated both and chose you
9. **Quick Reference** — pricing comparison, key differentiators at a glance

**Battle card rules:**
- Keep it to 1-2 pages — reps won't read a novel
- Update regularly — competitive landscape changes fast
- Be honest about weaknesses — reps lose credibility if they can't acknowledge trade-offs
- Focus on what matters to the buyer, not what matters to your product team

### Step 4: Write in Customer Language

Every piece of sales collateral must pass this test: *Would the prospect say this sentence to their boss?*

- Replace internal jargon with words the prospect uses
- Lead with their problem, not your solution
- Quantify where possible — "saves 10 hours per week" beats "increases efficiency"
- Use their industry terms, not yours
- Write at the level of the decision maker, not the evaluator (unless it's a technical doc)

Pull voice and tone from the product marketing context. Sales materials should sound like a helpful expert, not a brochure.

---

## Output Format

Deliver the material in a format the user can immediately use. Structure depends on the type:

### Sales Deck Output
```markdown
# [Deck Title]

## Slide 1: [Headline]
**Visual:** [Description of what should appear on the slide]
**Speaker Notes:** [What the rep says while this slide is up]

## Slide 2: [Headline]
**Visual:** [Description]
**Speaker Notes:** [Script]

[Continue for all slides...]

---
**Design Notes:** [Guidance on visual style, colors, imagery]
**Customization Guide:** [What to change per prospect — company name, industry examples, relevant case study]
```

### One-Pager Output
```markdown
# [Headline — Value Proposition]

[Problem statement — 2-3 sentences]

## How [Product] Helps
[Solution description — 2-3 sentences]

### Key Results
- [Benefit 1 with specific outcome]
- [Benefit 2 with specific outcome]
- [Benefit 3 with specific outcome]

> "[Customer quote with real metric]" — [Name, Title, Company]

### How It Works
1. [Step 1]
2. [Step 2]
3. [Step 3]

**[CTA]** — [Contact info or next step]
```

### Objection Doc Output
```markdown
# Objection Handling Guide
Last updated: [DATE]

## How to Use This Doc
[Brief context]

---

### Objection 1: "[Exact words the prospect says]"
**What they mean:** [Underlying concern]
**Response:** [Conversational reply]
**Proof:** [Supporting evidence]
**If they push back:** [Follow-up]

---

### Objection 2: "[Exact words]"
[Same structure...]

[Continue for all objections...]

---

## Red Flags
- [Signal 1 — what it means]
- [Signal 2 — what it means]
```

### Battle Card Output
```markdown
# Battle Card: [Your Product] vs. [Competitor]
Last updated: [DATE]

## Competitor Overview
[2-3 sentences on who they are and their positioning]

## Quick Comparison
| Dimension | Us | Them |
|-----------|-------|------|
| [Key area 1] | [Our position] | [Their position] |
| [Key area 2] | [Our position] | [Their position] |
[Continue...]

## Where We Win
1. **[Strength 1]** — [Evidence]
2. **[Strength 2]** — [Evidence]
3. **[Strength 3]** — [Evidence]

## Where They Win
1. **[Their strength]** — [How to handle it]

## Their Pitch & Our Counter
**They say:** "[Their claim]"
**We say:** "[Our response]"

**They say:** "[Their claim about us]"
**We say:** "[Our counter]"

## Landmine Questions
- "[Question that exposes their weakness]"
- "[Question that highlights our strength]"

## Customer Proof
> "[Quote from customer who chose us over them]" — [Name, Title, Company]
```

---

## Quality Checks

Before delivering any sales material, verify:

- [ ] **Passes the "would the prospect say this?" test** — language matches how buyers actually talk
- [ ] **Leads with the problem, not the product** — the prospect should feel understood before they see a solution
- [ ] **Proof points are specific** — real numbers, named customers, concrete outcomes (not "improved efficiency")
- [ ] **Differentiation is honest** — acknowledges trade-offs, doesn't trash competitors
- [ ] **Call to action is clear** — the prospect knows exactly what to do next
- [ ] **Appropriate for the sales stage** — early-stage materials don't discuss pricing; late-stage materials do
- [ ] **Scannable** — busy people should get the key message in 30 seconds
- [ ] **Customizable** — clearly marks what the rep should personalize per prospect
- [ ] **Consistent with product marketing context** — positioning, voice, and claims match the stored context
- [ ] **No unsubstantiated claims** — every bold statement has evidence behind it

If any check fails, revise before delivering.

---

## Examples

### Example 1: Sales Deck Request

**User says:** "I need a sales deck for enterprise prospects."

**You do:**
1. Search memory for product marketing context (found)
2. Ask: "Who specifically will you be presenting to? CTO? VP of Engineering? Head of Operations?"
3. User: "VP of Operations at logistics companies. 500+ employees."
4. Ask: "What's the main problem they're dealing with that gets them on a call with you?"
5. User: "They're losing packages and can't track shipments in real time. It costs them millions in claims."
6. Ask: "What competitors do they usually mention?"
7. User: "They're comparing us to their existing TMS or building something in-house."
8. Ask: "Do you have a customer story in logistics with real numbers?"
9. User: "Yeah, FastFreight reduced lost packages by 73% and saved $2.1M in claims in their first year."
10. Build the 10-15 slide deck following the framework
11. Include speaker notes for every slide
12. Add customization guide for swapping in prospect-specific details
13. Run quality checks
14. Deliver with: *"Here's your deck. The speaker notes tell you what to say on each slide. Swap in the prospect's company name and industry examples before each meeting."*

### Example 2: Battle Card Request

**User says:** "We keep losing deals to Acme Corp. I need a battle card."

**You do:**
1. Search memory for product marketing context (found)
2. Ask: "Tell me about the last 2-3 deals you lost to Acme. What did the prospect say was the reason?"
3. User: "They said Acme had better integrations and a bigger customer base. But honestly, our product is faster and cheaper."
4. Ask: "What does Acme say about you when they're in a deal against you?"
5. User: "They say we're too small and might not be around in 2 years."
6. Ask: "How do you counter that today?"
7. User: "We mention our funding and growth rate, but it doesn't always work."
8. Build the battle card with honest comparisons
9. Include landmine questions that expose integration limitations
10. Add talk tracks for the "too small" objection with proof points
11. Run quality checks — verify honesty about where Acme genuinely wins
12. Deliver with: *"Here's your Acme battle card. Two things to note: I've been honest about where they're stronger (integrations) and included specific counters for their 'too small' FUD. The landmine questions should help your reps steer the conversation."*

### Example 3: Objection Handling Doc

**User says:** "My team keeps hearing the same objections. Can you help?"

**You do:**
1. Search memory for product marketing context (found)
2. Ask: "What are the top 3 objections your reps hear most often? Give me the exact words prospects use."
3. User: "We hear 'it's too expensive,' 'we already have something that works,' and 'can you just send me a proposal?'"
4. Ask: "When they say 'too expensive,' what are they comparing you to?"
5. User: "Usually their current manual process. They don't realize how much that costs them."
6. Ask: "When they say they 'already have something,' what are they using?"
7. User: "Spreadsheets and email, usually. Sometimes a competitor tool they set up years ago."
8. Explore each objection, understand the real concern behind it
9. Build the objection doc with natural language responses
10. Include proof points for each response
11. Add red flags section for deals that won't close
12. Run quality checks — make sure responses sound conversational, not scripted
13. Deliver with: *"Here's your objection handling guide. The responses are frameworks, not scripts — your reps should say these in their own words. I've also included red flags to help them qualify out of bad deals earlier."*

---

## Related Skills

- **copywriting** — writes the actual copy for sales materials in brand voice
- **competitor-alternatives** — deep competitive analysis that feeds into battle cards
- **pricing-strategy** — develops the pricing frameworks referenced in decks and one-pagers
- **revops** — aligns sales materials with the revenue operations process
- **product-marketing-context** — the foundation for all sales materials (always check first)
