---
name: spec-writing
description: Turn an approved design into a clear specification document with user flows, testable requirements, risks, and success criteria. Use after brainstorming produces an approved design, or when someone needs to document exactly what gets built before development begins.
---

# Spec Writing

Turn an approved design into a clear specification document anyone can follow.

---

## When to Use

Trigger this skill when you hear:
- "Write a spec for..."
- "Let's spec this out"
- "Turn this design into a spec"
- "We need a specification document"
- "Document what we're building"
- "What exactly are we building?"
- After a brainstorming session produces an approved design
- When a `design.approved` event is received from the brainstorming skill

This skill takes the output of brainstorming and makes it concrete. No code, no architecture — just a clear description of what gets built, who it's for, and how it should work.

---

## Context Gathering

Before writing anything, you need the design. Get it from one of these sources:

### From Memory
```
agent(resource: "memory", action: "search", query: "approved design brainstorming")
```

### From a File
Ask: *"Where's the design document? Share a file path or paste it in."*

### From Conversation
If the user describes the design live, capture it first. Ask:
1. **What are we building?** (The thing that came out of brainstorming)
2. **What problem does it solve?** (The user pain it addresses)
3. **Who approved this direction?** (Confirm it's been agreed on, not just an idea)

If the design hasn't been approved yet, push back: *"This sounds like it's still in the idea phase. Want to run brainstorming first to explore and lock in the direction?"*

---

## Methodology

### Step 1: Read the Design
Pull in the approved design from brainstorming — whether it's stored in memory, a file, or provided in conversation.

Read it carefully. Understand:
- What was the original problem?
- What solution direction was chosen?
- What alternatives were considered and rejected?
- What constraints were mentioned?

If the design is vague or incomplete, ask clarifying questions before proceeding. Don't guess.

### Step 2: Define the Scope
This is the most important step. Write down explicitly:

**What's included:**
- List every feature, screen, or behavior that's part of this work
- Be specific — "user can filter by date" not "filtering"

**What's NOT included:**
- List things people might assume are included but aren't
- Things that were discussed in brainstorming but deferred
- Adjacent features that are tempting but out of scope

Present the scope to the user: *"Here's what I think is in and out. Does this match your expectations?"*

If they disagree, adjust before moving forward.

### Step 3: Describe the User Experience
Walk through what the user sees and does, step by step. No technical language. Write it like you're explaining to someone who's never seen the product.

For each flow:
1. **Where does the user start?** (What page, what state, what triggers this?)
2. **What do they see?** (Describe the screen, the elements, the content)
3. **What do they do?** (Click, type, choose, drag — the actions)
4. **What happens next?** (The response, the feedback, the next screen)
5. **What could go wrong?** (Error states, edge cases, empty states)

Use plain language:
- "The user sees a list of their projects" not "The UI renders a collection component"
- "They click 'Create New' and a form appears" not "A modal is instantiated on CTA interaction"

### Step 4: List All Requirements
Split requirements into two clear categories:

**Must-Have (ship blockers):**
- Things that MUST work for this to be usable
- Number them (R1, R2, R3...) so they can be referenced later
- Each requirement should be testable — you can look at it and say "yes, this works" or "no, it doesn't"

**Nice-to-Have (can ship without):**
- Things that would make it better but aren't essential
- Things that can be added in a follow-up
- Number them (N1, N2, N3...)

For each requirement, write it as a clear statement:
- "R1: A user can create a new project by providing a name and description"
- "R2: Project names must be unique within an organization"
- "R3: The project list shows the 20 most recent projects, sorted by last modified"

Avoid vague requirements:
- "The page should be fast" (how fast?)
- "It should look good" (what does good mean?)
- "Users should be able to manage their settings" (which settings?)

### Step 5: Identify Unknowns and Risks
Be honest about what you don't know. List:

**Open Questions:**
- Things that need answers before building can start
- Decisions that haven't been made yet
- Dependencies on other people, teams, or systems

**Risks:**
- Things that could go wrong or take longer than expected
- Technical uncertainties
- User experience concerns
- Business risks

For each risk, note the impact (what happens if it comes true) and any mitigation ideas.

### Step 6: Write the Spec Document
Assemble everything into the output format below. Keep the language simple and direct. Write for the person who has to build this AND the person who has to approve it — neither should need a translator.

Read through the full document yourself. Check:
- Does it flow logically?
- Could someone who wasn't in the brainstorming session understand it?
- Are there contradictions?
- Are there gaps?

### Step 7: Present for Approval
Share the complete spec with the user: *"Here's the spec document. Read through it and let me know what needs to change."*

Walk them through each section briefly:
- *"The scope covers X, Y, and Z — but explicitly excludes A and B."*
- *"There are 12 must-have requirements and 4 nice-to-haves."*
- *"I flagged 3 open questions that need answers before building starts."*

If they request changes, make them and present again.

### Step 8: Approve and Hand Off
When the user approves the spec, store it and signal the next skill:

```
agent(resource: "memory", action: "store", key: "spec/current", value: "Full spec document", layer: "tacit")
agent(resource: "memory", action: "store", key: "spec/status", value: "approved", layer: "tacit")
agent(resource: "memory", action: "store", key: "spec/approved_date", value: "YYYY-MM-DD", layer: "tacit")
agent(resource: "memory", action: "store", key: "spec/requirements_count", value: "X must-have, Y nice-to-have", layer: "tacit")
```

Emit the handoff event:
```
event: spec.approved
```

Tell the user: *"Spec is approved and saved. The plan-writing skill can now pick this up and break it into buildable tasks. Want to start planning?"*

---

## Output Format

The spec document follows this structure:

```markdown
# Spec: [Feature/Project Name]
Status: Draft | Approved
Date: [DATE]
Author: [User name or team]

## Problem
[2-3 sentences describing the problem this solves. Written from the user's perspective — what pain are they experiencing?]

## Solution
[2-3 sentences describing the chosen approach. Reference the brainstorming session if applicable. Explain WHY this approach was chosen over alternatives.]

## User Experience
### Flow 1: [Name]
1. User starts at [location/state]
2. User sees [what's on screen]
3. User does [action]
4. System responds with [result]
5. ...

### Flow 2: [Name]
1. ...

### Error States
- [What happens when X goes wrong]
- [What the user sees when Y fails]
- [Empty state when there's no data]

## Requirements

### Must-Have
- R1: [Clear, testable requirement]
- R2: [Clear, testable requirement]
- R3: [Clear, testable requirement]
- ...

### Nice-to-Have
- N1: [Clear, testable requirement]
- N2: [Clear, testable requirement]
- ...

## Out of Scope
- [Thing that's explicitly NOT part of this work]
- [Thing that was discussed but deferred]
- [Adjacent feature that might be assumed but isn't included]

## Risks
| Risk | Impact | Mitigation |
|------|--------|------------|
| [What could go wrong] | [What happens if it does] | [How to reduce the risk] |

## Open Questions
1. [Question that needs an answer before building]
2. [Decision that hasn't been made]
3. ...

## Success Criteria
- [How do we know this worked?]
- [Measurable outcome tied to the original problem]
- [What does "done" look like?]
```

---

## Quality Checks

Before presenting the spec, verify:

- [ ] **Problem is stated from the user's perspective** — not a technical problem, a human problem
- [ ] **Solution explains the "why"** — not just what, but why this approach
- [ ] **User experience is walkable** — someone could follow the steps and understand the flow
- [ ] **Every must-have requirement is testable** — you can verify pass/fail
- [ ] **Out of scope is explicit** — no ambiguity about what's not included
- [ ] **Risks are honest** — not just "everything will go fine"
- [ ] **Success criteria are measurable** — not "users like it" but "signup rate increases 15%"
- [ ] **No jargon** — a non-technical stakeholder can read and understand the entire document
- [ ] **No contradictions** — requirements don't conflict with each other
- [ ] **Scope matches the approved design** — nothing added that wasn't in the brainstorming output

If any check fails, fix it before presenting.

---

## Examples

### Example 1: New Feature Spec

**User says:** "We approved the design for project templates. Let's write the spec."

**You do:**
1. Search memory for the approved design
2. Find the brainstorming output: "Users want to save project setups as reusable templates"
3. Ask: "I found the design. Let me confirm scope — are we talking about creating templates, using templates, or both?"
4. User: "Both. Create and use. But not sharing templates between organizations — that's later."
5. Define scope: creating templates from existing projects, applying templates to new projects, managing template list
6. Out of scope: sharing across orgs, template marketplace, template versioning
7. Walk through the UX: how a user creates a template, how they apply one, how they manage their list
8. List requirements: R1-R8 must-haves, N1-N3 nice-to-haves
9. Flag risks: "Template data could get large if projects have many settings"
10. Present the full spec
11. User requests one change: "Add a requirement that templates can be renamed"
12. Add R9, present again
13. User approves
14. Store to memory, emit `spec.approved`
15. Ask: "Want to start planning the build?"

### Example 2: Spec from Conversation

**User says:** "I want to spec out a notification system."

**You do:**
1. Search memory for an approved design (none found)
2. Ask: "Has this been through a brainstorming session? I want to make sure we're speccing an agreed-upon direction, not still exploring."
3. User: "We talked about it last week and decided on in-app notifications with an inbox. No email or push for now."
4. Ask: "Got it. Who's the user here — what role, what are they doing when they need notifications?"
5. User: "Project managers. They need to know when tasks are assigned to their team or when deadlines change."
6. Proceed to define scope, UX, requirements
7. Write spec with clear boundaries: in-app only, specific trigger events, inbox UI
8. Present and iterate until approved

---

## Related Skills

- **brainstorming** — produces the approved design that this skill turns into a spec
- **plan-writing** — picks up the approved spec and breaks it into buildable tasks
