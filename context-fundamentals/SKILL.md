---
name: context-fundamentals
description: Understand what context is and why it matters for getting good AI results. Use when someone is new to AI, frustrated with output quality, or needs to learn how conversation context, memory, and tools work together to produce better answers.
---

# Context Fundamentals

Understand what context is and why it matters for getting good AI results.

---

## When to Use

Trigger this skill when you hear:
- "Why doesn't the AI understand what I want?"
- "How do I get better answers?"
- "What does context mean?"
- "The AI keeps getting it wrong"
- "How do I talk to AI effectively?"
- "I'm new to using AI"
- When someone is frustrated with AI output quality

This is a foundational skill. If someone is struggling with AI results, context is almost always the reason. Start here before suggesting anything else.

---

## What Context Actually Is

Context is everything the AI knows when it answers your question. Think of it like talking to a new coworker on their first day. If you say "fix the homepage," they'll stare at you blankly. They don't know what your homepage looks like, what's wrong with it, or what "fixed" means to you.

AI works the same way. The more relevant information it has, the better its answers get. Not more information in general — more *relevant* information.

### The Three Types of Context

**1. What You Tell It (Conversation Context)**
This is what you type in the current conversation. It's the most direct and powerful form of context. If you say "write me an email," the AI has almost nothing to work with. If you say "write a follow-up email to a customer who signed up for a free trial 3 days ago but hasn't logged in yet — keep it friendly and short," the AI has a clear picture.

**2. What It Remembers (Memory Context)**
In Nebo, the AI can store and recall things you've told it before — your product details, your preferences, past decisions, research you've done. This means you don't have to repeat yourself every time. The AI builds up knowledge about you and your business over time.

**3. What Tools It Has Access To (Tool Context)**
The AI can use tools — browse the web, search your memory, run sub-agents for specialized tasks. Each tool gives the AI access to information it wouldn't otherwise have. When the AI searches the web for your competitor's pricing page, that becomes context it can use in its answer.

---

## Methodology

### Step 1: Diagnose the Context Gap

When someone gets a bad result from AI, the problem is almost always one of these:

- **Too vague:** "Write me some copy" vs. "Write a headline for my landing page targeting freelance designers who struggle with invoicing"
- **Missing background:** The AI doesn't know your product, audience, or goals
- **Wrong assumptions:** The AI filled in gaps with its own guesses, and those guesses were wrong
- **Stale information:** The context the AI has is outdated

Ask the user: *"When you got that result, what exactly did you ask the AI? Let's look at what information it had to work with."*

### Step 2: Teach the Context Formula

Good context follows a simple pattern:

**Who** — Who is this for? (your audience, your customer, yourself)
**What** — What do you need? (the specific deliverable)
**Why** — Why does it matter? (the goal behind the request)
**How** — How should it look/sound/feel? (style, format, tone)

You don't need all four every time, but the more you provide, the better the result. One detailed sentence beats five vague ones.

### Step 3: Show the Difference

Walk the user through a before/after example using their own situation. Take their vague request, add context using the formula from Step 2, and show how the output changes.

**Before:** "Help me with my website"
**After:** "Review my homepage headline and suggest 3 alternatives. My product is a project management tool for freelancers. My customers care about simplicity — they left complex tools like Asana. Keep it under 10 words."

The second version will produce dramatically better results because the AI knows the audience, the product positioning, the competitive angle, and the constraints.

### Step 4: Introduce Memory as a Shortcut

Explain that Nebo's memory system means they don't have to provide all this context every time. Once they save their product details, customer profile, and brand voice to memory, the AI can pull that information automatically.

Say: *"You can save things like your product description, your ideal customer, and how you like to talk to customers. Then every time you ask for help, the AI already knows the background. You only need to provide the specifics of what you need right now."*

### Step 5: Set Up Their First Context

Help the user save their most important context to memory right now. Start with:

1. What their product or service does (one sentence)
2. Who their ideal customer is
3. How they want to sound (casual, professional, technical, friendly)

```
agent(resource: "memory", action: "store", key: "product/description", value: "One-sentence description", layer: "tacit")
agent(resource: "memory", action: "store", key: "customer/profile", value: "Ideal customer description", layer: "tacit")
agent(resource: "memory", action: "store", key: "brand/voice", value: "Preferred tone and style", layer: "tacit")
```

### Step 6: Build the Habit

Tell the user: *"Going forward, before you ask the AI for something, take 5 seconds to add one specific detail. Instead of 'write an email,' say 'write a short email to trial users who haven't logged in.' That one detail changes everything."*

---

## Output Format

When explaining context to a user, keep it conversational. Don't lecture. Use their specific situation as the example.

Structure your explanation like this:

```
1. Acknowledge what went wrong (validate their frustration)
2. Explain WHY it went wrong (missing context, not user error)
3. Show the fix (add specific context to their actual request)
4. Demonstrate the difference (run both versions so they can see)
5. Set up memory so it gets easier over time
```

---

## Quality Checks

Before finishing this skill, verify:

- [ ] **The user understands what context is** — they can explain it in their own words
- [ ] **They see the difference** — they've seen a vague vs. specific prompt produce different results
- [ ] **They know about memory** — they understand they can save context so they don't repeat themselves
- [ ] **They have at least one thing saved** — product description, customer profile, or brand voice is in memory
- [ ] **They know the quick fix** — add one specific detail before asking for anything
- [ ] **No jargon was used** — "context window," "tokens," and "system prompt" were avoided
- [ ] **They feel empowered, not lectured** — the tone was collaborative, not instructional
- [ ] **Their original problem was solved** — don't just teach theory, fix what they came for

---

## Examples

### Example 1: Frustrated with Bad Copy

**User says:** "The AI keeps writing terrible marketing copy. It sounds generic."

**You do:**
1. Ask: "Can you show me what you asked for and what it gave you?"
2. User shares: "I said 'write marketing copy for my product.'"
3. Say: "That makes total sense — the AI just didn't have enough to work with. It doesn't know what your product does, who it's for, or how you like to sound. Let me show you the difference."
4. Ask a few questions about their product and audience
5. Re-run the request with context: "Write a landing page headline for [product] targeting [audience] who struggle with [pain point]. Sound [voice]. Keep it under 12 words."
6. Show them the dramatically better result
7. Save their product info and voice to memory
8. Say: "Now the AI knows this. Next time you can just say 'write a headline for my landing page' and it'll pull your product and voice automatically."

### Example 2: New to AI

**User says:** "I'm new to this. How do I use AI effectively?"

**You do:**
1. Say: "The single biggest thing is giving it context — the background it needs to help you well. Think of it like briefing a smart assistant who knows nothing about your business yet."
2. Ask: "What are you hoping to use AI for?"
3. User: "Marketing my consulting business."
4. Say: "Perfect. Let's start by telling the AI about your business so it can help effectively."
5. Walk through saving product description, ideal customer, and voice
6. Run a sample task with the saved context so they see it working
7. Say: "That's it. The more context it has, the better it gets. And it remembers what you tell it, so it gets smarter about your business over time."

---

## Related Skills

- **context-compression** — when conversations get long, learn to keep context focused
- **memory-best-practices** — go deeper on what to save and how to organize it
- **product-marketing-context** — the most important context to set up for marketing work
- **multi-agent-patterns** — how context flows between agents working together
- **evaluation** — judge whether AI output is good, and improve context when it's not
