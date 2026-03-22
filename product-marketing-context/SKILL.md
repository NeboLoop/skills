---
name: product-marketing-context
description: Capture and maintain product positioning, audience, and differentiation context. Use when starting any marketing work, updating brand context, or before other marketing skills run.
---

# Product Marketing Context

Capture and maintain a clear picture of what you're marketing, who you're marketing to, and what makes you different.

---

## When to Use

Trigger this skill when you hear:
- "Help me with marketing"
- "Write copy for..."
- "My product is..."
- "Update my marketing context"
- "What do you know about my product?"
- "I need to market my business"
- Before ANY other marketing skill runs (this is always first)

This skill MUST run before any other marketing work. If marketing context doesn't exist in memory, create it. If it exists but seems outdated, offer to update it.

---

## Context Gathering

Ask these questions conversationally, one at a time. Don't overwhelm with a form. Have a conversation.

### Product/Service
1. **What does your product or service do?** (Ask for one clear sentence they'd tell a friend)
2. **What's the main problem it solves?** (The pain point people have before they find you)
3. **How does it work?** (High-level — SaaS? Service? Physical product? Marketplace?)

### Ideal Customer
4. **Who is your ideal customer?** (Job title, industry, company size if B2B; or demographics/psychographics if B2C)
5. **What specific pain are they experiencing?** (The thing that makes them search for a solution)
6. **What does success look like for them?** (The outcome they want to achieve)

### Differentiation
7. **What makes you different from alternatives?** (Not "better" — different. What's your unique angle?)
8. **Who are your main competitors or alternatives?** (Named competitors or the alternative behavior like "spreadsheets" or "hiring someone")
9. **Why would someone choose you over them?** (The real reason, not generic claims)

### Pricing
10. **What's your pricing model?** (Free/freemium/paid, one-time/subscription, rough price points)
11. **What's your average deal size or customer value?** (Helps understand what marketing tactics are viable)

### Brand Voice
12. **How do you talk to customers?** (Formal or casual? Playful or serious? Expert or friendly? Technical or simple?)
13. **Show me an example of your current marketing** (website, email, social post — helps calibrate voice)

### Goals
14. **What are your top 3 marketing goals right now?** (More signups? Higher conversion? More traffic? Better SEO? Launch a new thing?)
15. **What's worked well so far?** (Channels, tactics, campaigns that got results)
16. **What's been frustrating or hasn't worked?** (Learn what to avoid)

---

## Methodology

### Step 1: Check for Existing Context
Use Nebo's memory to search for existing product marketing context:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, read it and ask: *"I have your marketing context from [date]. Has anything changed since then?"*

If yes → update mode (ask only what changed)
If no → proceed to next task
If no context exists → proceed to Step 2

### Step 2: Gather Context
Ask the 16 questions above conversationally. Group them naturally:
- Start with product (questions 1-3)
- Move to customer (questions 4-6)
- Explore differentiation (questions 7-9)
- Cover pricing (questions 10-11)
- Understand voice (questions 12-13)
- Align on goals (questions 14-16)

**Don't ask all at once.** Ask 2-3, get answers, synthesize, ask the next batch. Show that you're listening.

If the user provides a website, use Nebo's browser to read it:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

Use what you learn from the site to ask smarter follow-up questions.

### Step 3: Synthesize the Context
Create a structured context document (see Output Format below) that captures:
- The core positioning (one-sentence value prop)
- Customer profile (who they are, what they need, what success looks like)
- Differentiation (the unique angle, competitive alternatives, why people choose them)
- Pricing and economics (model, rough numbers, deal size)
- Brand voice (tone, style, examples)
- Current goals and constraints

### Step 4: Confirm with the User
Present the synthesized context and ask: *"Does this capture your product and positioning accurately?"*

If no → ask what's wrong, revise
If yes → proceed to Step 5

### Step 5: Store in Memory
Save the context to Nebo's persistent memory using these keys:

```
agent(resource: "memory", action: "store", key: "product/name", value: "Product Name", layer: "tacit")
agent(resource: "memory", action: "store", key: "product/description", value: "One-sentence description", layer: "tacit")
agent(resource: "memory", action: "store", key: "product/problem", value: "Main problem solved", layer: "tacit")
agent(resource: "memory", action: "store", key: "customer/profile", value: "Ideal customer description", layer: "tacit")
agent(resource: "memory", action: "store", key: "customer/pain", value: "Specific pain point", layer: "tacit")
agent(resource: "memory", action: "store", key: "customer/success", value: "Success outcome", layer: "tacit")
agent(resource: "memory", action: "store", key: "positioning/differentiation", value: "Unique angle", layer: "tacit")
agent(resource: "memory", action: "store", key: "positioning/competitors", value: "Main alternatives", layer: "tacit")
agent(resource: "memory", action: "store", key: "positioning/why_us", value: "Why choose us", layer: "tacit")
agent(resource: "memory", action: "store", key: "pricing/model", value: "Pricing model", layer: "tacit")
agent(resource: "memory", action: "store", key: "pricing/deal_size", value: "Average deal size", layer: "tacit")
agent(resource: "memory", action: "store", key: "brand/voice", value: "Voice description", layer: "tacit")
agent(resource: "memory", action: "store", key: "brand/examples", value: "Example copy", layer: "tacit")
agent(resource: "memory", action: "store", key: "marketing/goals", value: "Top 3 goals", layer: "tacit")
agent(resource: "memory", action: "store", key: "marketing/what_works", value: "Successful tactics", layer: "tacit")
agent(resource: "memory", action: "store", key: "marketing/context_date", value: "YYYY-MM-DD", layer: "tacit")
```

Also save a full-text version for reference:
```
agent(resource: "memory", action: "store", key: "product/marketing_context_full", value: "Full context document", layer: "tacit")
```

### Step 6: Confirm Next Steps
Tell the user: *"I've saved your marketing context. I'll reference this whenever I'm helping with marketing — copy, strategy, SEO, ads, anything. If things change, just say 'update my marketing context' and we'll revise it."*

Then ask: *"What marketing help do you need right now?"*

---

## Output Format

The full context document follows this structure:

```markdown
# Product Marketing Context
Last updated: [DATE]

## Product
**Name:** [Product/Company Name]
**What it does:** [One-sentence description]
**Problem solved:** [The main pain point]
**How it works:** [High-level mechanism]

## Ideal Customer
**Profile:** [Job title, industry, company size OR demographics/psychographics]
**Pain point:** [The specific pain they experience]
**Success outcome:** [What success looks like for them]

## Positioning
**Differentiation:** [Unique angle — what makes this different]
**Competitors/Alternatives:** [Named competitors or alternative behaviors]
**Why choose us:** [The real reason people choose this over alternatives]

## Pricing & Economics
**Model:** [Free/freemium/paid, one-time/subscription]
**Price points:** [Rough numbers if available]
**Deal size:** [Average customer value]

## Brand Voice
**Tone:** [Formal/casual, playful/serious, expert/friendly, technical/simple]
**Style notes:** [Specific guidance]
**Example:** [Sample copy that represents the voice]

## Current Goals
1. [Goal 1]
2. [Goal 2]
3. [Goal 3]

## What's Working
[Channels, tactics, or campaigns that have delivered results]

## What Hasn't Worked
[Things that were tried and didn't work — to avoid repeating]
```

---

## Quality Checks

Before storing the context, verify:

- [ ] **One-sentence product description is clear** — someone unfamiliar could understand what it does
- [ ] **Ideal customer is specific** — not "everyone" or "small businesses" (too vague)
- [ ] **Differentiation is meaningful** — not just "better" or "faster" (that's not differentiation)
- [ ] **Competitors/alternatives are named** — helps understand the competitive landscape
- [ ] **Brand voice has examples** — not just adjectives, actual copy to match
- [ ] **Goals are specific** — not "grow" but "increase trial signups by 30%"
- [ ] **No jargon unless the user used it first** — speak their language
- [ ] **All required fields are populated** — no [TBD] or [Unknown]

If any check fails, ask clarifying questions before storing.

---

## Examples

### Example 1: B2B SaaS

**User says:** "I need help with marketing. I run a SaaS product."

**You do:**
1. Search memory for existing context (none found)
2. Ask: "Great! Tell me in one sentence — what does your SaaS do?"
3. User: "It's a tool that helps sales teams automate their outbound emails."
4. Ask: "Who's your ideal customer? Job title, company size?"
5. User: "Sales managers at B2B companies with 10-50 employees. They don't have big budgets for enterprise tools."
6. Ask: "What's the main pain point they have before they find you?"
7. User: "Their sales reps are spending hours manually sending the same emails. It's not scalable."
8. Ask: "What makes you different from alternatives like Outreach or SalesLoft?"
9. User: "We're way simpler and cheaper. Those are enterprise tools that take weeks to set up. We get you started in 10 minutes."
10. Continue through pricing, voice, goals...
11. Synthesize and present context
12. User confirms
13. Store to memory with all keys
14. Ask: "What marketing help do you need right now?"

**Stored context includes:**
```
Product: Email automation for sales teams
Customer: Sales managers at 10-50 person B2B companies
Differentiation: Simple and affordable vs enterprise complexity
Pricing: $49/mo per user, freemium model
Voice: Friendly and straightforward, not corporate
Goals: 1) Increase free trial signups, 2) Improve trial-to-paid conversion, 3) Rank for "sales email automation"
```

### Example 2: Service Business

**User says:** "Update my marketing context — we changed our pricing."

**You do:**
1. Search memory, find existing context
2. Say: "I have your context from Jan 15. What changed with pricing?"
3. User: "We moved from project-based to monthly retainers."
4. Ask: "What's the new pricing structure?"
5. User: "$3k/month for ongoing work, 3-month minimum."
6. Update only the pricing section in memory
7. Confirm: "Updated. Anything else changed?"
8. User: "Nope, that's it."
9. Say: "Got it. Your marketing context is current."

---

## Related Skills

- **copywriting** — uses context to write on-brand copy for any surface
- **copy-editing** — checks copy against brand voice from context
- **cold-email** — personalizes outreach using customer pain points
- **email-sequence** — designs nurture flows aligned with customer journey
- **social-content** — creates posts that match brand voice
- **page-cro** — audits landing pages against value prop and differentiation
- **paid-ads** — targets ads to ideal customer profile
- **ad-creative** — writes ad copy using positioning and pain points
- **pricing-strategy** — evolves pricing model based on market position
- **marketing-ideas** — generates ideas specific to product stage and goals
- **launch-strategy** — plans launches around product positioning
- **sales-enablement** — creates materials using positioning and differentiation

**Every marketing skill starts by checking for product-marketing-context.** If it doesn't exist, this skill runs first.
