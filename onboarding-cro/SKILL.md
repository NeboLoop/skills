---
name: onboarding-cro
description: Optimize your post-signup onboarding flow so new users reach their aha moment faster and stick around. Use when activation rates are low or users drop off after registration.
---

# Onboarding CRO

Optimize your post-signup onboarding flow so new users reach their "aha moment" faster and actually stick around.

---

## When to Use

Trigger this skill when you hear:
- "People sign up but never come back"
- "Our activation rate is low"
- "Users aren't completing onboarding"
- "How do I improve onboarding?"
- "New users seem lost after signing up"
- "We have a drop-off after registration"
- "Help me optimize the first-time user experience"
- "What should happen after someone signs up?"

This skill focuses on everything **after** signup and **before** the user becomes a regular, active customer. If the issue is the signup form itself, use **signup-flow-cro** instead.

---

## Context Gathering

Before diving in, understand the product and the current state. Ask these conversationally:

### About the Product
1. **What does your product do?** (One sentence a friend would understand)
2. **What's the single most valuable thing a new user can do?** (The action that proves your product works for them)
3. **How long should it take a new user to get value?** (Minutes? Hours? Days?)

### About Current Onboarding
4. **What happens right after someone signs up today?** (Welcome email? Dashboard? Setup wizard? Nothing?)
5. **Do you have any guided setup or tutorial?** (Walkthroughs, tooltips, checklists, videos?)
6. **What does your onboarding completion rate look like?** (Even a rough guess helps)

### About Drop-off
7. **Where do you think people get stuck?** (Is there a step where most people leave?)
8. **Do users need to set something up before getting value?** (Connect an account, import data, invite a teammate, configure settings?)
9. **Have you talked to users who dropped off?** (Any feedback on why they left?)

### About Success
10. **What does an activated user look like?** (What actions did they take that predict they'll stay?)
11. **How long does it typically take to get there?** (First session? First week? Longer?)

---

## Methodology

### Step 1: Check Product Marketing Context

Pull the stored context so you understand what the product does, who it's for, and what success looks like for customers:

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. You need to understand the product before you can optimize onboarding.

Pay special attention to:
- **What the product does** — defines what "getting value" means
- **Customer pain point** — this is what onboarding should address immediately
- **Success outcome** — this is the destination onboarding should lead to

### Step 2: Define the Activation Metric

Work with the user to pin down: **What is the one action that proves a new user got value?**

This is the activation metric. It should be:
- **Specific** — not "used the product" but "sent their first campaign" or "created their first project"
- **Measurable** — something you can track
- **Meaningful** — users who do this action are significantly more likely to stay

Examples of activation metrics:
- Project management tool: "Created a project and added at least one task"
- Email marketing tool: "Sent their first email to a real list"
- Analytics tool: "Connected a data source and viewed their first report"
- Design tool: "Created and exported their first design"

If the user isn't sure, help them think through it: *"What's the moment where a new user would say 'oh, this is useful'? That's your activation metric."*

### Step 3: Map the Current Onboarding Flow

Walk through the actual onboarding experience step by step using Nebo's browser:

```
web(action: "navigate", url: "the-product-url.com/signup")
web(action: "read_page")
```

Document every step from signup to activation:
1. What does the user see right after signing up?
2. What are they asked to do first?
3. What's the second step? Third?
4. How many steps before they reach real value?
5. Are there any dead ends, confusing choices, or unnecessary steps?

Create a simple map: **Step 1 → Step 2 → Step 3 → ... → Activation moment**

Note the total number of steps and estimated time to complete.

### Step 4: Identify the Shortest Path to the "Aha Moment"

Compare what the onboarding does today vs. the fastest possible route to activation:

- **What steps are truly necessary?** (Some setup is unavoidable)
- **What steps could be skipped or deferred?** (Profile details, preferences, optional integrations)
- **What steps could be automated?** (Pre-fill defaults, use smart defaults, skip configuration)
- **What's blocking users from getting value right away?** (Long forms, complex setup, information overload)

The goal: reduce the number of steps and the time between "I just signed up" and "This product is useful to me."

### Step 5: Audit Onboarding Elements

Check for these six critical onboarding components:

**1. Welcome Message**
- Is there a clear welcome that tells the user what to do first?
- Does it feel personal or generic?
- Does it set expectations for what's coming?

**2. Guided Setup**
- Is there a clear path forward (wizard, checklist, tutorial)?
- Or is the user dropped into a blank dashboard with no direction?
- Is the guidance skippable for experienced users?

**3. Progress Indicators**
- Can users see how far along they are in setup?
- Do they know how many steps remain?
- Is there a sense of momentum (progress bar, checkmarks, completion percentage)?

**4. Empty States**
- When a page has no data yet, does it explain what goes there?
- Do empty states include a clear action to fill them?
- Or are they just blank and confusing?

**5. Help Access**
- Can users get help without leaving the onboarding flow?
- Is there a chat widget, tooltip, or help link when they might get stuck?
- Are common questions answered proactively?

**6. Quick Win**
- Does the user accomplish something meaningful early?
- Is there a moment of "this is working" within the first session?
- Or does all the value come later, after a lot of setup?

### Step 6: Recommend Changes to Reduce Time-to-Value

Based on everything above, create prioritized recommendations:

**Priority 1 — Remove friction (do first):**
- Steps that can be eliminated entirely
- Steps that can be deferred until later
- Form fields that can be skipped or auto-filled

**Priority 2 — Add guidance (do next):**
- Missing welcome message or unclear first step
- Blank screens that need empty states
- Steps where users commonly get confused

**Priority 3 — Optimize the experience (do after):**
- Better progress indicators
- Smarter defaults to reduce decisions
- Personalized onboarding based on user type or goal

**Priority 4 — Measure and iterate (ongoing):**
- Track completion rate at each onboarding step
- Measure time from signup to activation
- Follow up with users who drop off at specific steps

---

## Output Format

Present findings in this structure:

```markdown
# Onboarding Audit: [Product Name]
Date: [DATE]

## Activation Metric
**What success looks like:** [The specific action that means a user "got it"]
**Current time to activation:** [Estimated time from signup to activation]
**Target time to activation:** [What it should be]

## Current Onboarding Flow
1. [Step 1 — what happens, estimated time]
2. [Step 2 — what happens, estimated time]
3. [Step 3 — what happens, estimated time]
...
**Total steps:** [X]
**Estimated total time:** [X minutes]

## Onboarding Elements Checklist
- [x/missing] Welcome message
- [x/missing] Guided setup
- [x/missing] Progress indicators
- [x/missing] Empty states
- [x/missing] Help access
- [x/missing] Quick win moment

## Recommended Changes

### Remove Friction (Priority 1)
- [Specific change and why it helps]

### Add Guidance (Priority 2)
- [Specific change and why it helps]

### Optimize Experience (Priority 3)
- [Specific change and why it helps]

### Measure and Iterate (Priority 4)
- [What to track and why]

## Optimized Flow
1. [Simplified Step 1]
2. [Simplified Step 2]
...
**Target steps:** [X — fewer than current]
**Target time:** [X minutes — shorter than current]
```

---

## Quality Checks

Before delivering the audit, verify:

- [ ] **Activation metric is defined** — not vague, a specific user action
- [ ] **Every onboarding step is documented** — nothing skipped or assumed
- [ ] **All six onboarding elements are checked** — welcome, guided setup, progress, empty states, help, quick win
- [ ] **Recommendations are specific** — not "improve onboarding" but "move the team invite step to after the user creates their first project"
- [ ] **Priorities are clear** — the user knows what to fix first
- [ ] **Time-to-value is addressed** — recommendations reduce the gap between signup and activation
- [ ] **No technical jargon** — a non-technical founder could read and act on this
- [ ] **Product context was used** — recommendations align with the product's value prop and target customer

---

## Examples

### Example: Auditing a SaaS Project Management Tool

**User says:** "People sign up for our project management app but most never create a project. What's wrong?"

**You do:**

1. Check memory for product marketing context:
   - Product: Simple project management for freelancers
   - Customer: Solo freelancers juggling multiple clients
   - Pain: Losing track of tasks and deadlines across clients
   - Differentiation: Built for one-person businesses, not teams

2. Define activation metric together:
   - You ask: *"What's the one thing a user needs to do to realize your tool works for them?"*
   - User: "Create a project, add a few tasks, and see them on the calendar view."
   - Activation metric: **Created a project with at least 2 tasks and viewed the calendar**

3. Walk through current onboarding via browser:
   - Step 1: Sign up form (name, email, password) — 30 seconds
   - Step 2: Email verification — 1-3 minutes waiting
   - Step 3: Landing on dashboard with a big empty screen, no guidance
   - Step 4: User has to find the "New Project" button on their own
   - Step 5: Project creation form asks for name, description, color, client name, start date, end date, tags, and budget — 8 fields
   - Step 6: Empty project page, no guidance on adding tasks
   - Step 7: User has to find and click "Add Task"
   - Step 8: Task form asks for name, description, priority, due date, estimated hours, and tags — 6 fields
   - **Total: 8 steps, estimated 6-8 minutes to activation**

4. Identify the shortest path:
   - Email verification could be deferred (let them start using the tool immediately)
   - Project creation only needs a name, everything else can come later
   - First task only needs a name and due date
   - Calendar view should be highlighted immediately after first tasks are added

5. Audit onboarding elements:
   - Welcome message: Missing. User lands on empty dashboard.
   - Guided setup: Missing. No wizard, no checklist, no direction.
   - Progress indicators: Missing. No sense of "you're almost there."
   - Empty states: Weak. Dashboard just says "No projects yet" with no guidance.
   - Help access: A help link in the footer, easy to miss.
   - Quick win: Missing. The calendar view is the "wow" moment but nothing leads the user there.

6. Recommend changes:

   **Priority 1 — Remove friction:**
   - Defer email verification until after the user creates their first project
   - Reduce project creation to just a name (move other fields to project settings)
   - Reduce task creation to name only, with an optional due date
   - Skip the general dashboard and go straight to project creation

   **Priority 2 — Add guidance:**
   - Add a welcome screen: "Let's set up your first project. It takes about 60 seconds."
   - Show an empty state on the project page that says: "Add your first task — just type a name and hit enter"
   - After 2-3 tasks are added, prompt: "Nice! Check out your calendar view to see everything laid out"

   **Priority 3 — Optimize:**
   - Add a simple 3-step progress indicator: "Create project → Add tasks → View calendar"
   - Offer a sample project template: "Start with a sample project to see how it works"
   - Show a chat widget during onboarding for users who get stuck

   **Optimized flow:**
   - Step 1: Sign up (name, email, password) — 30 seconds
   - Step 2: Welcome screen with "Create your first project" — 10 seconds
   - Step 3: Enter project name, land in project — 10 seconds
   - Step 4: Add a couple tasks (just names) — 30 seconds
   - Step 5: Prompted to view calendar — instant "aha moment"
   - **Total: 5 steps, under 2 minutes to activation**

**Result:** Reduced from 8 steps and 6-8 minutes down to 5 steps and under 2 minutes. The user hits the calendar view — the "aha moment" — with minimal setup.

---

## Related Skills

- **signup-flow-cro** — optimizes the signup form and process that happens before onboarding begins
- **page-cro** — audits and improves any page, including onboarding screens and empty states
- **form-cro** — reduces friction in forms within the onboarding flow (project setup, profile completion)
- **product-marketing-context** — provides the product understanding that onboarding recommendations are built on

**This skill always checks product-marketing-context first.** Understanding what the product does and who it's for is essential to knowing what good onboarding looks like.
