---
name: plan-writing
description: Break an approved spec into a step-by-step action plan with bite-sized tasks anyone can execute. Use when you have an approved spec and need to turn it into ordered, dependency-aware phases with clear verification steps and effort estimates.
---

# Plan Writing

Break an approved spec into a step-by-step plan with bite-sized tasks that anyone can execute — no technical background required.

---

## When to Use

Trigger this skill when you hear:
- "Write me a plan for..."
- "Break this spec into tasks"
- "How do I get this done?"
- "Create an action plan"
- "What are the steps to do this?"
- "Turn this into actionable tasks"
- "Plan out the work for..."
- "I have a spec, now what?"

This skill requires an approved spec. If there's no approved spec, run **spec-writing** first. Don't plan against loose ideas — plan against decisions that have been made.

---

## Context Gathering

Before writing a plan, you need to understand:

### The Spec
1. **Where is the approved spec?** (In memory, in a file, or pasted directly)
2. **Has it been approved?** (Look for a `spec.approved` event or explicit confirmation)

### Constraints
3. **Who is doing the work?** (One person? A small team? What's their skill level?)
4. **What tools are available?** (Software, platforms, services they already use)
5. **What already exists?** (Starting from scratch or building on something?)
6. **Are there hard deadlines?** (Time constraints change how you scope tasks)

### Dependencies
7. **What external resources are needed?** (Third-party services, accounts, permissions, other people's input)
8. **Are there blockers?** (Waiting on access, approvals, or someone else's work)

Don't ask all of these if the answers are obvious from the spec. Use judgment. If the spec says "add a FAQ section to our existing website," you already know there's an existing site and the work is content + web updates.

---

## Methodology

### Step 1: Read the Approved Spec

Pull the spec from memory or file:
```
agent(resource: "memory", action: "search", query: "approved spec")
```

If found, read it thoroughly. If not found, ask the user to provide it or point to a file.

**Do not proceed without a spec.** A plan without a spec is just guessing.

### Step 2: Identify All the Work

Read the spec end to end and list every piece of work needed. Think about:
- **Content:** What needs to be written, designed, or created?
- **Setup:** What accounts, tools, or platforms need to be configured?
- **Building:** What needs to be assembled, connected, or constructed?
- **Data:** What information needs to be gathered, organized, or migrated?
- **Communication:** What needs to be sent, shared, or announced?
- **Review:** What needs to be checked, tested, or approved by someone?

Don't filter yet. Just get everything on the list.

### Step 3: Break Into Bite-Sized Tasks

Take each piece of work and break it down until every task is **2-5 minutes of work**. A task is too big if someone would need to make multiple decisions to complete it.

**Too big:** "Set up our email marketing"
**Right size:** "Create a free Mailchimp account using the company email address"

**Too big:** "Write the landing page"
**Right size:** "Write a one-sentence headline that describes the main benefit of the product"

Each task gets three things:
1. **What to do** — a clear, specific description. No ambiguity. Someone with zero context reads this and knows exactly what to do.
2. **Done looks like** — the observable result when the task is finished. A page that exists, a file that's saved, a message that's sent, a setting that's toggled on.
3. **How to verify** — the exact check that proves it worked. Not "make sure it looks good." Something like "open the page in your browser and confirm the headline reads 'Get paid faster with instant invoicing.'"

### Step 4: Order by Dependency

Arrange tasks so that each task's prerequisites come before it. Ask:
- What has to exist before this task can start?
- What would break if I did these out of order?

Common dependency patterns:
- Account creation before using the account
- Content writing before publishing the content
- Design decisions before building what was designed
- Gathering information before using that information
- Getting access or permissions before doing the work that requires them

If two tasks have no dependency between them, they can happen in either order — but still pick a sequence. Plans are linear. Parallel execution is an optimization the reader can figure out.

### Step 5: Group Into Phases

Organize ordered tasks into logical phases. Each phase should produce something you can check — a visible milestone you can verify before moving on.

Good phases:
- **Phase 1: Setup** — accounts, tools, access, permissions
- **Phase 2: Foundation** — core content, data, or structure
- **Phase 3: Assembly** — putting the pieces together
- **Phase 4: Details** — refinements, edge cases, polish
- **Phase 5: Review** — checking everything, getting approvals
- **Phase 6: Launch** — publishing, sending, going live

Don't force every plan into these exact phases. Let the work dictate the grouping. A plan for a single email campaign might only have 3 phases. A plan for a product launch might have 8.

### Step 6: Estimate Total Effort

Add up the task count per phase. Multiply by 3-5 minutes per task (accounting for context-switching and small hiccups). Present as a range.

Example: "32 tasks across 4 phases. Estimated effort: 1.5 to 2.5 hours."

Don't pretend precision. Estimates are for setting expectations, not making promises.

### Step 7: Present the Plan

Show the full plan to the user in the output format below. Walk through the phases briefly and highlight:
- Any tasks that seem risky or uncertain
- Any assumptions you made
- Any places where the spec was ambiguous and you made a judgment call

Ask: *"Does this plan look right? Any tasks missing, out of order, or scoped wrong?"*

### Step 8: Revise if Needed

If the user has feedback:
- Add missing tasks
- Reorder if dependencies were wrong
- Split tasks that are still too big
- Remove tasks that aren't needed
- Adjust estimates

Re-present the revised plan and confirm again.

### Step 9: Approve and Store

When the user approves the plan, store it in memory and emit the approval event:

```
agent(resource: "memory", action: "store", key: "plan/current", value: "Full plan document", layer: "tacit")
agent(resource: "memory", action: "store", key: "plan/spec_reference", value: "Reference to the spec this plan implements", layer: "tacit")
agent(resource: "memory", action: "store", key: "plan/task_count", value: "Total number of tasks", layer: "tacit")
agent(resource: "memory", action: "store", key: "plan/phase_count", value: "Number of phases", layer: "tacit")
agent(resource: "memory", action: "store", key: "plan/estimate", value: "Effort estimate range", layer: "tacit")
agent(resource: "memory", action: "store", key: "plan/approved_date", value: "YYYY-MM-DD", layer: "tacit")
```

Emit the approval event:
```
agent(resource: "event", action: "emit", event: "plan.approved")
```

Tell the user: *"Plan is locked in. Ready to start? I can walk you through task by task."*

---

## Output Format

The plan document follows this structure:

```markdown
# Action Plan
**Spec:** [Name or reference to the approved spec]
**Created:** [DATE]
**Estimated effort:** [X to Y hours] ([N] tasks across [M] phases)

---

## Phase 1: [Phase Name]
*[One sentence describing what this phase accomplishes]*

### Task 1.1: [Short task title]
**Do:** [Specific, unambiguous description of the work]
**Done looks like:** [Observable result]
**Verify:** [Exact check to confirm it worked]

### Task 1.2: [Short task title]
**Do:** [Specific, unambiguous description of the work]
**Done looks like:** [Observable result]
**Verify:** [Exact check to confirm it worked]

---

## Phase 2: [Phase Name]
*[One sentence describing what this phase accomplishes]*

### Task 2.1: [Short task title]
**Do:** [Specific, unambiguous description of the work]
**Done looks like:** [Observable result]
**Verify:** [Exact check to confirm it worked]

(continue for all phases and tasks)

---

## Summary
- **Total tasks:** [N]
- **Phases:** [M]
- **Estimated effort:** [X to Y hours]
- **Risks:** [Anything uncertain or potentially tricky]
- **Assumptions:** [Decisions you made where the spec was silent]
```

---

## Quality Checks

Before presenting the plan, verify:

- [ ] **Every task is 2-5 minutes of work** — if you can't do it in 5 minutes, split it
- [ ] **Every task has a clear "done" state** — no tasks that end with "should work" or "looks good"
- [ ] **Every task has a verification step** — not "test it" but the specific thing to check
- [ ] **Tasks are ordered by dependency** — nothing references something that hasn't been created yet
- [ ] **No vague tasks** — nothing like "set things up" or "handle the details" or "finalize everything"
- [ ] **No skipped work** — reviews, approvals, and cleanup are explicit tasks, not afterthoughts
- [ ] **Phases produce checkable results** — after each phase, you can see that something concrete exists
- [ ] **Estimate is realistic** — not optimistic, not padded, just honest

If any check fails, fix it before presenting.

---

## Examples

### Example 1: Launching an Email Newsletter

**Spec says:** "Launch a weekly email newsletter for our existing customers. Content: product tips, one customer story, and a CTA to book a demo. Send every Tuesday at 9am. Use our existing brand colors and logo."

**You produce:**

```markdown
# Action Plan
**Spec:** Weekly Customer Newsletter Launch
**Created:** 2026-03-21
**Estimated effort:** 2 to 3 hours (18 tasks across 4 phases)

---

## Phase 1: Setup
*Get the email platform ready and connected to our customer list.*

### Task 1.1: Create Mailchimp account
**Do:** Go to mailchimp.com and create a free account using the company email address (hello@ourcompany.com).
**Done looks like:** You're logged into the Mailchimp dashboard.
**Verify:** You can see the Mailchimp dashboard with the company email in the top-right corner.

### Task 1.2: Import customer email list
**Do:** Export customer emails from the CRM as a CSV file. In Mailchimp, go to Audience > Import Contacts and upload the CSV.
**Done looks like:** Mailchimp shows the imported contacts in your audience.
**Verify:** The audience count in Mailchimp matches the number of rows in your CSV (minus any duplicates or invalid emails Mailchimp flags).

### Task 1.3: Upload brand assets
**Do:** In Mailchimp, go to Content Studio and upload the company logo and note the brand colors (hex codes from the style guide).
**Done looks like:** Logo appears in Content Studio. Brand colors are saved.
**Verify:** Open Content Studio and confirm the logo file is there and the colors are recorded somewhere you can reference them.

---

## Phase 2: Content
*Write the first issue's content.*

### Task 2.1: Write the newsletter headline
**Do:** Write a short, specific subject line for the first issue. Focus on the main product tip. Example format: "3-minute trick to [specific benefit]."
**Done looks like:** A subject line under 50 characters saved in a doc.
**Verify:** Read it aloud. Does it make you want to open the email? Is it under 50 characters?

### Task 2.2: Write the product tip section
**Do:** Write 150-200 words explaining one specific product tip. Include a step-by-step walkthrough a customer could follow.
**Done looks like:** A clear, actionable tip with numbered steps.
**Verify:** Hand it to someone unfamiliar with the product. Can they follow the steps?

### Task 2.3: Write the customer story
**Do:** Pick one customer who got a great result. Write 100 words: who they are, what problem they had, what they did, what happened.
**Done looks like:** A short story with a specific, measurable result.
**Verify:** Does the story include a concrete number or outcome? ("Saved 5 hours a week" not "improved efficiency")

### Task 2.4: Write the CTA
**Do:** Write a one-sentence call to action that links to the demo booking page. Make it specific to the tip in this issue.
**Done looks like:** A CTA sentence with the booking page URL.
**Verify:** Click the URL. Does it go to the correct booking page?

(remaining tasks follow the same pattern...)
```

### Example 2: Spec Has Ambiguity

**Spec says:** "Create a referral program for our customers."

**You do:**
1. Read the spec — it doesn't specify: what's the reward? For the referrer, the referee, or both? How do people refer? Is there a limit?
2. Note the ambiguity in your plan:
   - *"The spec says 'referral program' but doesn't specify the reward structure. I'm planning for a simple 'give $10, get $10' model with a unique referral link per customer, since that's the most common approach. If you want a different structure, we'll adjust the plan."*
3. Plan for the simple model
4. List the assumption in the Summary section
5. Present and ask for confirmation before proceeding

---

## Related Skills

- **brainstorming** — generates the ideas that lead to specs and then plans
- **spec-writing** — produces the approved spec this skill consumes
