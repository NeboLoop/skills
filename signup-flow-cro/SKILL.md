---
name: signup-flow-cro
description: Audit and optimize your registration or signup flow to reduce drop-off and get more people through the door. Use when signup completion rates are low or registration abandonment is high.
---

# Signup Flow CRO

Audit and optimize your registration or signup flow to reduce drop-off and get more people through the door.

---

## When to Use

Trigger this skill when you hear:
- "People aren't finishing signup"
- "Our registration drop-off is high"
- "Optimize our signup flow"
- "Too many people abandon during registration"
- "Simplify our signup process"
- "How do I get more signups?"
- "Review our signup page"
- "Our conversion rate on signup is low"

This skill focuses specifically on the registration/signup experience — from the moment someone clicks "Sign Up" to the moment they land inside your product. For broader landing page issues, use **page-cro**. For what happens after signup, use **onboarding-cro**.

---

## Context Gathering

Before diving in, you need to understand what you're working with. Ask these conversationally:

### The Basics
1. **What's the URL for your signup page?** (So we can walk through it together)
2. **What does someone get when they sign up?** (Free trial? Freemium access? Paid account? A quote?)
3. **Do you know where people are dropping off?** (Any analytics, heatmaps, or gut feel is fine)

### Current Setup
4. **How many steps does your signup have right now?** (Single page? Multi-step? Do you know?)
5. **Do you offer social login?** (Google, Apple, GitHub, etc.)
6. **What information do you collect during signup?** (Email, name, company, phone, credit card, etc.)
7. **Is there email verification or a confirmation step?**

### Goals
8. **What does a successful signup look like for you?** (Just an account created? Or do they need to complete a specific action?)
9. **Who is signing up?** (Developers? Small business owners? Consumers? This affects what "simple" means for them)
10. **What's your biggest frustration with the current flow?**

Don't ask all of these at once. Start with the URL (question 1), walk through the flow yourself, then ask smarter follow-ups based on what you see.

---

## Methodology

### Step 1: Check Product Marketing Context
Pull existing knowledge about the product so your recommendations fit the brand and audience:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. You need to understand who the customer is and what they care about before you can optimize a signup flow for them.

### Step 2: Load the Signup Page
Use Nebo's browser to visit the signup page and experience it firsthand:
```
web(action: "navigate", url: "their-signup-url.com/signup")
web(action: "read_page")
```

Look at the page as a first-time visitor would. Note your immediate impressions:
- Is it clear what I'm signing up for?
- Does it feel fast or heavy?
- Do I trust this page?

### Step 3: Map Every Step of the Current Flow
Walk through the entire signup process step by step using the browser:
```
web(action: "navigate", url: "their-signup-url.com/signup")
web(action: "screenshot")
```

Document each step:
1. **Step 1:** What does the user see first? What fields are on screen?
2. **Step 2:** What happens after they fill in the first screen? Another form? Email verification?
3. **Step 3:** Where do they land after completing signup? Dashboard? Welcome screen? Empty state?

Continue until you reach the end of the flow. Count the total number of steps and screens.

### Step 4: Identify Friction at Each Step
For every step you mapped, look for problems:

- **Unnecessary fields** — Do they really need a phone number right now? Company size? Job title? Every extra field reduces completion.
- **Confusing labels** — Is "Organization" clear? Would "Company name" be better? Are placeholders being used instead of labels?
- **Too many choices** — Is the user being asked to pick a plan before they've even tried the product?
- **Slow loading** — Did any step take noticeably long to load?
- **Unclear next action** — Is the button text vague ("Submit" instead of "Create my account")?
- **No value reinforcement** — Does the page remind people why they're signing up, or is it just a cold form?
- **Password requirements** — Are they overly strict? Are the rules shown before or after the user types?
- **Mobile experience** — Would this flow work well on a phone?

### Step 5: Check for Missing Best Practices
Look for these specific elements:

- **Progress indicators** — If there are multiple steps, can the user see where they are? (e.g., "Step 2 of 3")
- **Social login options** — Google, Apple, or other one-click signup options can dramatically reduce friction
- **Error handling** — What happens when someone enters an invalid email? Does the error message appear immediately and clearly, or after they submit?
- **Inline validation** — Do fields validate as you type, or do you have to submit the whole form to find out something's wrong?
- **Password visibility toggle** — Can users see what they're typing?
- **Terms and privacy** — Are they presented clearly without being intrusive?
- **Trust signals** — Security badges, customer counts, testimonials near the signup form

### Step 6: Benchmark Against Best Practices
Compare the current flow against what works best:

| Best Practice | Why It Works |
|---|---|
| **3 steps or fewer** | Every additional step loses 10-20% of users |
| **Email-first approach** | Start with just email — it's the lowest commitment and lets you follow up if they drop off |
| **Delay non-essential fields** | Ask for company size, job title, and phone number later — after they've seen value |
| **Social login available** | Cuts signup time from 60 seconds to 5 seconds |
| **Clear button text** | "Start my free trial" outperforms "Submit" every time |
| **Show what's next** | Tell them what happens after signup — "You'll get instant access to..." |
| **No credit card upfront** | Removing the card requirement can increase signups 2-4x for free trials |
| **Single-column layout** | Easier to scan and complete than multi-column forms |
| **Mobile-friendly** | 50%+ of signups may come from mobile devices |

### Step 7: Recommend Specific Changes
For each recommendation:
1. **What to change** — Be specific. "Change the button text from 'Submit' to 'Create my free account'"
2. **Why it matters** — Explain the friction it removes in plain language
3. **Expected impact** — Give a realistic sense of improvement (e.g., "This typically improves completion by 10-15%")
4. **Priority** — Label each as high, medium, or low priority based on likely impact and effort

Order recommendations from highest impact to lowest.

---

## Output Format

Present your findings in this structure:

```markdown
# Signup Flow Audit: [Product Name]
Date: [DATE]

## Current Flow Summary
**Total steps:** [Number]
**Fields collected:** [List all fields across all steps]
**Estimated completion time:** [Rough estimate]
**Social login available:** Yes/No
**Mobile-friendly:** Yes/No

## Flow Walkthrough
### Step 1: [Screen Name]
- What the user sees: [Description]
- Fields: [List]
- Friction points: [Issues found]

### Step 2: [Screen Name]
- What the user sees: [Description]
- Fields: [List]
- Friction points: [Issues found]

[Continue for each step...]

## Key Findings
1. [Most important finding]
2. [Second finding]
3. [Third finding]

## Recommendations

### High Priority
1. **[Change]**
   - Why: [Plain language explanation]
   - Expected impact: [Realistic estimate]

2. **[Change]**
   - Why: [Plain language explanation]
   - Expected impact: [Realistic estimate]

### Medium Priority
3. **[Change]**
   - Why: [Plain language explanation]
   - Expected impact: [Realistic estimate]

### Low Priority
4. **[Change]**
   - Why: [Plain language explanation]
   - Expected impact: [Realistic estimate]

## Optimized Flow (Proposed)
### Step 1: [What it should look like]
### Step 2: [What it should look like]
### Step 3: [What it should look like — if needed]

## Quick Wins
[Changes that can be made today with minimal effort]
```

---

## Quality Checks

Before delivering your audit, verify:

- [ ] **You walked the actual flow** — Don't guess. Use the browser to experience every step yourself.
- [ ] **Every recommendation is specific** — Not "simplify the form" but "remove the phone number field from step 1 and ask for it during onboarding instead"
- [ ] **Expected impact is realistic** — Don't promise "10x more signups." Be honest about what each change typically achieves.
- [ ] **Recommendations match the audience** — A developer signup can ask for a GitHub handle. A consumer signup probably shouldn't ask for a company name.
- [ ] **You considered mobile** — Many signups happen on phones. If the flow doesn't work on mobile, that's a major finding.
- [ ] **You checked error handling** — Bad error messages are invisible friction. Test what happens with wrong input.
- [ ] **No technical jargon** — Say "the button people click to sign up" not "the CTA element." Say "the page loads slowly" not "high TTFB."
- [ ] **Proposed flow has 3 steps or fewer** — If your recommended flow still has 5 steps, rethink it.
- [ ] **Quick wins are truly quick** — Changes labeled as quick should be doable in under an hour, not a redesign.

---

## Examples

### Example: B2B Project Management Tool

**User says:** "People are starting our signup but not finishing. Can you take a look?"

**You do:**
1. Check memory for product marketing context — found: project management tool for small agencies, friendly brand voice, freemium model
2. Ask: "What's the URL for your signup page?"
3. User shares: `app.projecttool.com/signup`
4. Load the page in browser and walk through the entire flow

**What you find:**

The signup has 5 steps:
- Step 1: Email, password, full name
- Step 2: Company name, company size (dropdown), industry (dropdown)
- Step 3: Pick a plan (Free, Pro, Enterprise) with a comparison table
- Step 4: Invite team members (email input with "add more" button)
- Step 5: Email verification — must click link in email, then gets redirected back

**Friction identified:**
- 5 steps is too many — industry standard is 2-3
- Company size and industry are not needed to start using the product
- Asking users to pick a plan before they've tried anything causes decision paralysis
- "Invite team members" during signup is premature — they haven't even seen the product yet
- Email verification interrupts the flow — user has to leave, open email, find the link, click it, and get redirected back. Many never return.
- No social login option (Google login would skip the entire first step)
- No progress indicator — users don't know they're on step 2 of 5
- Button says "Next" on every step instead of something motivating

**Your recommendations:**

**High Priority:**
1. **Cut it to 2 steps: email/password, then company name**
   - Why: Every extra step loses 10-20% of people. Going from 5 steps to 2 could recover a large chunk of drop-off.
   - Expected impact: 25-40% more completed signups

2. **Add Google sign-in**
   - Why: One click instead of typing email and creating a password. Most people have Google accounts.
   - Expected impact: 15-20% more completions, especially on mobile

3. **Move plan selection to inside the product**
   - Why: Let people try the free version first. Asking them to pick a plan before they've seen anything creates doubt and confusion.
   - Expected impact: Removes a major decision point that causes people to leave and "think about it"

**Medium Priority:**
4. **Switch to delayed email verification**
   - Why: Let people into the product immediately and verify email later (show a banner inside the app). Requiring verification before access loses 15-25% of signups who never open the email.
   - Expected impact: 15-25% more people actually reaching the product

5. **Move team invites to onboarding**
   - Why: People want to see the product before inviting their team. Asking too early feels pushy.
   - Expected impact: Removes one full step from signup

**Low Priority:**
6. **Change button text from "Next" to "Create my account" on the final step**
   - Why: Specific button text tells people what's happening. "Next" gives no sense of progress or completion.
   - Expected impact: Small lift (2-5%) but takes 5 minutes to implement

7. **Add a progress bar**
   - Why: If you keep more than one step, show people where they are. "Step 1 of 2" reduces anxiety.
   - Expected impact: Reduces mid-flow abandonment by 5-10%

**Proposed optimized flow:**
- **Step 1:** Email and password (or Google sign-in). Button: "Create my free account"
- **Step 2:** Company name (one field). Button: "Take me to my workspace"
- **Done:** Land directly in the product. Show a welcome banner with a quick tour option. Ask for company size, industry, and team invites later — once they've seen the value.

---

## Related Skills

- **page-cro** — audits the landing page that leads to the signup flow (the page before the signup form)
- **onboarding-cro** — optimizes what happens after signup (first-run experience, activation, getting to the "aha" moment)
- **form-cro** — general form optimization for any type of form (contact, lead gen, checkout — not just signup)
- **product-marketing-context** — captures who your customer is and what they care about, so signup flow recommendations match your audience
