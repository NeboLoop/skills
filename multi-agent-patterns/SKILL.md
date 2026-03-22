---
name: multi-agent-patterns
description: Know when and how to use sub-agents for parallel or pipelined work. Use when a task has multiple independent parts, needs different expertise for each piece, or could be completed faster by delegating to specialized agents working simultaneously.
---

# Multi-Agent Patterns

Know when to use sub-agents and how they work together.

---

## When to Use

Trigger this skill when you hear:
- "Can the AI do multiple things at once?"
- "This task has a lot of parts"
- "How do sub-agents work?"
- "Should I use agents for this?"
- "Can different agents work on different pieces?"
- "This is taking too long — can we parallelize?"
- When a task naturally breaks into independent pieces

Sub-agents are one of Nebo's most powerful features. This skill teaches when to use them, when not to, and how to set them up for success.

---

## What Sub-Agents Are

A sub-agent is a separate AI worker that handles a specific piece of a bigger task. Think of it like delegating to a team member. Instead of one person doing everything sequentially, you assign parts of the work to specialists who work in parallel.

In Nebo, you can spin up sub-agents that:
- Work on their own piece of the task independently
- Have their own focused context (no clutter from other tasks)
- Can access memory to pull in relevant background
- Report their results back to the main conversation

The main conversation acts as the coordinator — it defines what each agent does, sends them off, and assembles the results.

---

## Methodology

### Step 1: Decide If You Need Multiple Agents

Not every task benefits from sub-agents. Use this simple test:

**Use sub-agents when:**
- The task has 3+ independent parts that don't depend on each other
- Different parts need different expertise (research vs. writing vs. analysis)
- You want to save time by running things in parallel
- One part of the task requires deep focus and you don't want to pollute the context

**Use a single conversation when:**
- The parts are sequential (each step depends on the previous one)
- The task is small enough that splitting it adds complexity without saving time
- You need tight coordination between all the parts (the tone of the email depends on the landing page copy)
- It's a simple, straightforward request

Ask the user: *"Let's look at your task. Can the parts be done independently, or does each piece depend on the others?"*

### Step 2: Choose the Right Pattern

There are three main patterns for how agents work together:

**Pattern 1: Supervisor**
One main conversation coordinates everything. It defines tasks, assigns them to sub-agents, reviews results, and assembles the final output.

Best for: Marketing campaigns, content creation, research projects.

How it works:
1. You describe the full project to the main agent
2. The main agent breaks it into sub-tasks
3. Each sub-task goes to a sub-agent with specific instructions
4. Results come back to the main agent for review and assembly

Example: "Create a product launch plan" becomes:
- Sub-agent 1: Research competitor launches
- Sub-agent 2: Draft announcement email
- Sub-agent 3: Write social media posts
- Main agent: Review everything, ensure consistency, assemble the plan

**Pattern 2: Pipeline**
Each agent handles one stage, and the output of one becomes the input of the next. Like an assembly line.

Best for: Content workflows, data processing, multi-stage analysis.

How it works:
1. Agent 1 does research and produces findings
2. Agent 2 takes the findings and creates a strategy
3. Agent 3 takes the strategy and writes the content
4. Agent 4 takes the content and edits it

Example: "Write a blog post about our industry" becomes:
- Stage 1: Research trending topics and competitor content
- Stage 2: Create an outline based on research
- Stage 3: Write the full draft from the outline
- Stage 4: Edit for voice, clarity, and SEO

**Pattern 3: Peer-to-Peer**
Multiple agents work on the same type of task in parallel. No hierarchy — they're equals working on different pieces.

Best for: Batch work, A/B variations, covering multiple segments.

How it works:
1. Define the task template
2. Spin up multiple agents with different inputs
3. Collect all results

Example: "Write onboarding emails for different customer segments" becomes:
- Agent 1: Onboarding email for freelancers
- Agent 2: Onboarding email for small teams
- Agent 3: Onboarding email for enterprise

### Step 3: Set Up Each Agent With Clear Context

The most common mistake with sub-agents is giving them vague instructions. Each agent needs:

1. **A clear task** — exactly what to produce
2. **Relevant background** — pulled from memory or provided directly
3. **Constraints** — length, tone, format, things to avoid
4. **What success looks like** — how you'll judge the output

Example agent brief:
```
Task: Write a 3-email welcome sequence for new trial users
Background: Pull product description and customer profile from memory
Constraints: Each email under 150 words, casual tone, no hard sell
Success: Emails feel helpful, each has a clear CTA, they build on each other
```

### Step 4: Give Agents Access to Memory

Sub-agents can search Nebo's memory for context they need. This means you don't have to copy-paste product descriptions, voice guidelines, or strategy documents into every agent's instructions.

When setting up a sub-agent, tell it what to look for:
```
agent(resource: "memory", action: "search", query: "product description")
agent(resource: "memory", action: "search", query: "brand voice")
agent(resource: "memory", action: "search", query: "customer profile")
```

This is why the memory-best-practices skill matters. Well-organized memory makes multi-agent work dramatically easier.

### Step 5: Review and Assemble

When sub-agents return their results:

1. **Check each piece individually** — does it meet the brief?
2. **Check consistency across pieces** — do they sound like they came from the same brand?
3. **Fill gaps** — did any agent miss something important?
4. **Resolve conflicts** — if two agents made different assumptions, decide which is right
5. **Assemble the final output** — put the pieces together in a coherent way

If something isn't right, you can re-run just that sub-agent with better instructions. You don't have to redo everything.

### Step 6: Save What You Learned

After a multi-agent project, save the pattern that worked:

```
agent(resource: "memory", action: "store", key: "workflow/product-launch", value: "Used supervisor pattern with 3 sub-agents: research, email copy, social. Worked well. Next time also add landing page agent.", layer: "tacit")
```

Next time you do something similar, you (or the AI) can pull up the pattern and improve on it.

---

## Output Format

When helping a user set up a multi-agent workflow, present the plan clearly:

```markdown
## Multi-Agent Plan: [Project Name]

**Pattern:** [Supervisor / Pipeline / Peer-to-Peer]

### Agent 1: [Role]
- **Task:** [What it does]
- **Context needed:** [What to pull from memory]
- **Output:** [What it produces]

### Agent 2: [Role]
- **Task:** [What it does]
- **Context needed:** [What to pull from memory]
- **Output:** [What it produces]

### Agent 3: [Role]
- **Task:** [What it does]
- **Context needed:** [What to pull from memory]
- **Output:** [What it produces]

### Assembly
[How the pieces come together]

### Estimated time saved
[Compared to doing it sequentially]
```

---

## Quality Checks

Before finishing this skill, verify:

- [ ] **The user knows when to use sub-agents** — and when a single conversation is better
- [ ] **They understand the three patterns** — supervisor, pipeline, peer-to-peer
- [ ] **Each agent has a clear brief** — task, background, constraints, and success criteria
- [ ] **Agents have access to memory** — not working from scratch with vague instructions
- [ ] **There's a plan for assembly** — someone reviews, checks consistency, fills gaps
- [ ] **The pattern matches the task** — not forcing multi-agent on a simple task
- [ ] **No technical jargon was used** — "orchestration," "DAG," and "fan-out" were avoided
- [ ] **The user feels confident** — they could set up a multi-agent workflow on their own

---

## Examples

### Example 1: Marketing Campaign

**User says:** "I need to create a full marketing campaign for our new feature — emails, blog post, social posts, and a landing page section."

**You do:**
1. Say: "That's a great candidate for sub-agents. These are all independent pieces that can be created at the same time."
2. Recommend the Supervisor pattern:
   - Sub-agent 1: Blog post (pulls product context and voice from memory)
   - Sub-agent 2: 3-email announcement sequence (pulls customer segments from memory)
   - Sub-agent 3: Social posts for LinkedIn, Twitter, and newsletter (pulls brand voice from memory)
   - Sub-agent 4: Landing page section (pulls positioning and feature details from memory)
   - Main agent: Reviews all output, ensures consistent messaging, assembles into a campaign brief
3. Set up the agents with clear briefs
4. Review results together
5. Say: "All four pieces were created in parallel. Let's review each one and make sure they tell a consistent story."

### Example 2: Content Research

**User says:** "I want to research what our competitors are doing with content marketing."

**You do:**
1. Say: "We could research them one by one, but it'll be faster to have separate agents look at each competitor simultaneously."
2. Recommend the Peer-to-Peer pattern:
   - Agent 1: Research Competitor A's content strategy
   - Agent 2: Research Competitor B's content strategy
   - Agent 3: Research Competitor C's content strategy
   - Main agent: Compare findings and identify gaps and opportunities
3. Each agent browses the competitor's blog, social channels, and newsletter
4. Main agent synthesizes into a competitive content analysis
5. Save findings to memory for future reference

---

## Related Skills

- **context-fundamentals** — understand how context works before splitting it across agents
- **context-compression** — keep each agent's context focused and clean
- **memory-best-practices** — well-organized memory makes multi-agent work much easier
- **evaluation** — check the quality of each agent's output and the assembled result
- **plan-writing** — structure complex projects that benefit from multi-agent execution
