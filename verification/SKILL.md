---
name: verification
description: Verify that finished work actually works by checking every claim with fresh, firsthand evidence. Use after any build, fix, deployment, or change to confirm all success criteria pass before calling anything done.
---

# Verification

Never claim something is done without fresh proof. Trust nothing — verify everything with your own eyes, right now.

---

## When to Use

Trigger this skill when you hear:
- "Is it working?"
- "Can you check that?"
- "It should be live now"
- "I just deployed / published / sent"
- "Did that fix it?"
- "Are we good?"
- "Ship it"
- Before claiming ANY task is complete
- After ANY build, fix, deployment, or change

This skill is the final gate before anything gets called "done." No exceptions. If you haven't verified it yourself with fresh evidence, it's not done — it's a guess.

---

## Context Gathering

Before verifying, understand what you're checking and what "correct" looks like. Ask these questions:

### What Was Changed?
1. **What did we just build, fix, or deploy?** (A page, an email, a feature, a fix — one sentence)
2. **What was it supposed to do?** (The expected behavior — what should happen now?)
3. **What was the problem before?** (If this was a fix, what was broken?)

### What Does Success Look Like?
4. **How do we know it's working?** (Load a page? Check an inbox? Submit a form? Run a test?)
5. **What specifically should we see?** (Exact text, behavior, response, or result)
6. **Are there related things that could have broken?** (Other pages, flows, or features that touch the same thing)

---

## Methodology

### Step 1: Identify What to Verify

List every claim you're about to make. Be honest about what you actually know vs. what you assume.

**Common traps:**
- "I changed the code, so it must work now" — **No.** Did you run it?
- "The deployment succeeded, so the site is updated" — **No.** Did you load the page?
- "I sent the email, so it arrived" — **No.** Did you check the inbox?
- "The test passed last time" — **No.** Did it pass right now, after your changes?

Write down each thing that needs to be true for the work to be "done." These are your verification targets.

### Step 2: Run the Verification

For each verification target, actually do the check. Not in your head — in reality.

**For a webpage:**
```
1. Open the actual URL in a browser (not a preview, not a cached version)
2. Hard refresh to clear any cache
3. Look at the page with your own eyes
4. Check on mobile too if applicable
```

**For an email:**
```
1. Trigger the email (sign up, submit form, whatever sends it)
2. Check the actual inbox (not the sending dashboard)
3. Open the email
4. Click every link
5. Check on mobile too
```

**For a code change:**
```
1. Run the actual test suite
2. Load the actual feature in a browser or API client
3. Try the exact scenario that was broken
4. Try a related scenario that should still work
```

**For a form:**
```
1. Go to the page with the form
2. Submit with valid data — verify the correct response
3. Submit with invalid data — verify errors appear
4. Check that the submission arrived where it should (inbox, database, CRM)
```

### Step 3: Read the FULL Result

Don't skim. Don't glance. Read the entire result.

- If a test output 50 lines, read all 50 lines
- If a page loaded, scroll through the whole page
- If an email arrived, read the entire email
- If an API returned data, examine every field

**Common mistake:** Seeing "200 OK" and assuming everything is fine. The status code can be 200 while the response body is completely wrong. Read the actual content.

### Step 4: Compare Against Success Criteria

Take each verification target from Step 1 and compare it against what you actually observed in Steps 2-3.

Use this format:

| What should happen | What actually happened | Pass/Fail |
|---|---|---|
| Page shows user's name | Page shows "undefined" | FAIL |
| Form rejects empty email | Form rejects empty email | PASS |
| Email arrives within 60s | Email arrived in 45s | PASS |
| Links in email work | "Get Started" link returns 404 | FAIL |

Be ruthlessly honest. "Close enough" is not passing. If the criterion says "shows the user's name" and it shows "null," that's a failure.

### Step 5: Claim Done (or Report Failures)

**If everything passes:** Now — and only now — you can say "It's done. Here's the evidence." Share what you verified, what you observed, and that it matches the success criteria.

**If anything fails:** Report exactly what failed, what you observed vs. what was expected, and what needs to be fixed. Do not claim partial completion as done. Do not say "it mostly works." Either it all passes or it's not done.

---

## Output Format

When verifying work, document it in this structure:

```markdown
# Verification: [What Was Verified]
Date: [DATE]

## What Was Changed
[One-sentence description of the change]

## Success Criteria
1. [Criterion] — PASS/FAIL
2. [Criterion] — PASS/FAIL
3. [Criterion] — PASS/FAIL

## Evidence
### Criterion 1: [Description]
- Checked by: [What you did — loaded URL, ran test, checked inbox]
- Observed: [What you actually saw]
- Result: PASS/FAIL

### Criterion 2: [Description]
- Checked by: [What you did]
- Observed: [What you actually saw]
- Result: PASS/FAIL

## Verdict
ALL PASSING / [N] FAILURES — [list what needs fixing]
```

---

## Quality Checks

Before signing off on any verification:

- [ ] **Every criterion was checked** — no skipped items, no "I'm sure that one's fine"
- [ ] **Evidence is fresh** — checked right now, not from a previous run or memory
- [ ] **Full results were read** — not skimmed, not assumed from a status code
- [ ] **Related features still work** — the fix didn't break something else
- [ ] **Verification matched the actual environment** — checked production if it's a deploy, not just staging
- [ ] **No assumptions were made** — every claim has a corresponding observation
- [ ] **Failures are reported honestly** — nothing was rounded up to "pass"
- [ ] **Evidence could convince a skeptic** — if someone asked "how do you know?" you have a real answer

If any check fails, go back and verify properly.

---

## Examples

### Example 1: Verifying a Bug Fix

**User says:** "I fixed the login error. Users should be able to log in now."

**You do:**
1. Identify targets: login works with valid credentials, login rejects invalid credentials, error message is clear
2. Go to the login page — load it fresh
3. Enter valid credentials, submit — logged in successfully, dashboard loads with correct user name
4. Log out, enter invalid credentials — "Invalid email or password" error appears
5. Check: no console errors, page loads quickly, session persists after refresh
6. Report: "Verified. Login works with valid credentials (tested with test@example.com). Invalid credentials show proper error. Session persists after refresh. All passing."

### Example 2: Verifying an Email Campaign

**User says:** "The welcome email is set up and ready to go."

**You do:**
1. Identify targets: email triggers on signup, arrives within 60 seconds, personalization works, links work, renders on mobile
2. Sign up with a test account
3. Check inbox — email arrived in 38 seconds
4. Open email — subject line says "Welcome to Acme, Test!" (personalization works)
5. Click "Get Started" link — goes to the onboarding page (works)
6. Click "Unsubscribe" link — goes to preference center (works)
7. Open on mobile — layout is readable, buttons are tappable
8. Report: "Verified. Welcome email arrives within 60 seconds, personalization works, all links functional, mobile rendering is clean. All passing."

### Example 3: Verifying a Page Deployment

**User says:** "New pricing page is live."

**You do:**
1. Identify targets: page loads at /pricing, shows correct plans, prices are right, CTA buttons link to signup
2. Navigate to the actual production URL
3. Page loads — but the "Enterprise" plan shows "$0/month" instead of "Contact Us"
4. CTA for "Starter" plan links to /signup?plan=starter (correct)
5. CTA for "Pro" plan links to /signup?plan=pro (correct)
6. CTA for "Enterprise" links to /signup?plan=enterprise (should link to /contact)
7. Report: "Two failures found. Enterprise plan shows '$0/month' instead of 'Contact Us.' Enterprise CTA links to signup instead of contact page. Starter and Pro plans are correct. Not done — two items need fixing."

---

## Related Skills

- **tdd** — defines success criteria before building; verification confirms them after
- **systematic-debugging** — when verification reveals a failure, debug it methodically instead of guessing
- **product-marketing-context** — when verifying marketing assets, check them against your positioning and brand voice

**Nothing is done until it's verified.** Fresh evidence, every time.
