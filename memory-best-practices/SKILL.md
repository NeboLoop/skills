---
name: memory-best-practices
description: Get the most out of persistent memory by learning what to save, what to skip, and how to keep it organized and current. Use when setting up memory for the first time, cleaning up cluttered memory, or learning how to stop repeating yourself across conversations.
---

# Memory Best Practices

Get the most out of Nebo's memory system.

---

## When to Use

Trigger this skill when you hear:
- "What should I save to memory?"
- "How does memory work?"
- "I keep repeating myself to the AI"
- "Can it remember things between conversations?"
- "How do I organize my saved information?"
- "My memory is getting cluttered"
- When a user is saving too much (or too little) to memory

Memory is what makes Nebo get smarter about your business over time. This skill teaches what to save, what to skip, and how to keep it useful.

---

## How Memory Works

Nebo's memory is persistent storage that carries across conversations. Unlike a regular chat where everything disappears when you close it, memory sticks around. When you save something to memory, any future conversation can pull it up.

Think of it like a shared notebook between you and the AI. You write down important things. The AI checks the notebook before helping you. Over time, the notebook becomes a rich picture of your business, your preferences, and your decisions.

Memory works best when it's:
- **Curated** — only the things that matter are saved
- **Current** — outdated information is updated or removed
- **Searchable** — organized so the AI can find what it needs
- **Specific** — detailed enough to be useful, not so broad it's meaningless

---

## Methodology

### Step 1: Understand What to Save

The best things to save to memory fall into five categories:

**1. Product Context**
What your product does, who it's for, how it's different. This is the foundation for almost everything the AI does for you.
- Product description and value proposition
- Ideal customer profile
- Competitive positioning
- Pricing model

**2. Preferences and Standards**
How you like things done. Once saved, you stop getting output that doesn't match your style.
- Brand voice and tone
- Content length preferences
- Formatting rules (bullet points vs. paragraphs, headers vs. no headers)
- Things you never want (no emojis, no exclamation marks, no buzzwords)

**3. Decisions and Strategies**
Important choices that affect future work. Saves you from relitigating the same questions.
- Campaign strategies and messaging decisions
- Target audience refinements
- Channel priorities
- Pricing decisions and rationale

**4. Research and Findings**
Insights you've uncovered that you'll reference later.
- Competitor analysis results
- Customer feedback themes
- Market research findings
- A/B test results and what you learned

**5. Project Status**
Where things stand on ongoing work. Lets you pick up where you left off across conversations.
- What's been completed
- What's in progress
- What's been approved vs. still in draft
- Key deadlines

### Step 2: Understand What NOT to Save

Saving everything is worse than saving nothing. It clutters the AI's view and makes it harder to find what matters.

**Don't save:**

- **Temporary information** — things that will change within days (today's to-do list, a draft you're still iterating on)
- **Obvious information** — things the AI already knows (what email marketing is, how SEO works)
- **Full documents** — save the key takeaways, not a 2,000-word article. Link to the full version if you need it later.
- **Conversation history** — the conversation itself is context. Only save the *conclusions* from conversations, not the back-and-forth.
- **Duplicate information** — check memory before saving something that might already be there

**Ask yourself:** "Will I need this in a future conversation, and will it still be true?" If no to either question, don't save it.

### Step 3: Organize with Clear Keys

Memory is searchable, but good organization makes it dramatically more useful. Use descriptive keys that follow a consistent pattern:

**Good key patterns:**
```
product/description
product/pricing
customer/profile
customer/pain-points
brand/voice
brand/avoid
strategy/email-campaign-q1
strategy/content-pillars
research/competitor-acme
research/customer-interviews-jan
project/website-redesign/status
project/website-redesign/decisions
```

**Bad key patterns:**
```
info1
stuff
marketing
notes
important
misc
```

The key should tell you what's inside without having to read it. Anyone (including future you) should be able to search and find it.

### Step 4: Search Effectively

When you need something from memory, search with intent:

```
agent(resource: "memory", action: "search", query: "brand voice tone")
```

**Tips for better searches:**
- Use the specific topic, not generic terms ("competitor pricing" not "info")
- Try multiple searches if the first doesn't find it ("customer profile" then "ideal customer" then "target audience")
- Search before saving to avoid duplicates
- Search at the start of new conversations to load relevant context

### Step 5: Keep Memory Current

Memory compounds in value over time, but only if you maintain it. Old information is worse than no information because the AI will use it confidently.

**Maintenance habits:**
- When something changes (pricing, positioning, strategy), update it immediately
- After major projects, save learnings and remove outdated project status
- Quarterly, review what's in memory and prune anything stale
- When you notice the AI using outdated information, that's your cue to update

**To update existing memory:**
```
agent(resource: "memory", action: "store", key: "product/pricing", value: "Updated pricing information", layer: "tacit")
```

This overwrites the old value for that key.

### Step 6: Watch Memory Compound

Over time, well-maintained memory creates a flywheel:

1. **Week 1:** You save your product description and customer profile. The AI writes better copy.
2. **Month 1:** You've saved voice guidelines, competitor research, and campaign strategy. The AI produces work that sounds like you and accounts for your market position.
3. **Month 3:** You've saved A/B test results, customer feedback themes, and refined positioning. The AI makes recommendations based on what's actually worked for your specific business.
4. **Month 6:** The AI knows your business almost as well as a team member. New tasks require minimal briefing because the background is already there.

The key is consistency. Save a little after each meaningful conversation, not everything all at once.

---

## Output Format

When helping a user organize their memory, present a memory map:

```markdown
## Your Memory Map

### Saved (and current)
- product/description — "Your product description" (saved Jan 15)
- customer/profile — "Your customer profile" (saved Jan 15)
- brand/voice — "Your voice guidelines" (saved Jan 20)

### Should Save
- product/pricing — you mentioned pricing but it's not saved yet
- research/competitor-analysis — we did research last week, save the key findings
- strategy/content-pillars — we decided on 3 pillars, worth preserving

### Should Update
- product/description — you changed your positioning last month, memory still has the old version

### Should Remove
- project/launch-v1/status — this project is done, status is no longer relevant
```

---

## Quality Checks

Before finishing this skill, verify:

- [ ] **The user knows the five categories** — product, preferences, decisions, research, project status
- [ ] **They know what NOT to save** — temporary, obvious, full documents, conversation history, duplicates
- [ ] **They use clear keys** — organized by category with descriptive names
- [ ] **They know how to search** — specific queries, multiple attempts, search before saving
- [ ] **They have a maintenance plan** — update when things change, prune quarterly
- [ ] **At least three things are saved** — they left this conversation with something useful in memory
- [ ] **No jargon was used** — "key-value store," "namespace," and "retrieval" were avoided
- [ ] **They understand compounding** — they see why consistent saving pays off over time

---

## Examples

### Example 1: Getting Started with Memory

**User says:** "I keep having to re-explain my business every conversation. How do I fix that?"

**You do:**
1. Say: "That's exactly what memory is for. Let's save the key things about your business so the AI knows them automatically."
2. Ask about their product, customer, and voice (keep it to 5-6 questions)
3. Save each piece with clear keys:
```
agent(resource: "memory", action: "store", key: "product/description", value: "Project management tool for freelancers who want simplicity", layer: "tacit")
agent(resource: "memory", action: "store", key: "customer/profile", value: "Solo freelancers and consultants, non-technical, tired of complex tools", layer: "tacit")
agent(resource: "memory", action: "store", key: "brand/voice", value: "Casual, friendly, no corporate speak. Talk like a helpful friend, not a software company", layer: "tacit")
```
4. Start a test: ask the AI to write something and show how it pulls the saved context
5. Say: "Now every conversation starts with this knowledge. You'll never have to explain your product again — just add the specifics of what you need."

### Example 2: Cleaning Up Cluttered Memory

**User says:** "I've been saving a lot of stuff and I think the AI is getting confused."

**You do:**
1. Search memory to see what's there:
```
agent(resource: "memory", action: "search", query: "product")
agent(resource: "memory", action: "search", query: "customer")
agent(resource: "memory", action: "search", query: "strategy")
agent(resource: "memory", action: "search", query: "project")
```
2. Present a memory map showing what's saved
3. Identify duplicates, outdated items, and things too vague to be useful
4. Say: "Let's clean this up. I see a few things that are outdated and some duplicates. Here's what I'd recommend keeping, updating, and removing."
5. Walk through the cleanup together
6. Say: "Going forward, ask yourself 'Will I need this in a future conversation, and will it still be true?' before saving. Quality over quantity."

---

## Related Skills

- **context-fundamentals** — understand what context is before diving into memory
- **context-compression** — save key decisions from long conversations before starting fresh
- **product-marketing-context** — the most important things to save for marketing work
- **multi-agent-patterns** — how agents access and share memory
- **evaluation** — use saved criteria and past results to evaluate new output
