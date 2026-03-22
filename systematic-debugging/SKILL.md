---
name: systematic-debugging
description: Find and fix problems methodically through investigation, hypothesis ranking, and controlled testing. Use when something is broken, a test fails unexpectedly, or you are tempted to make random changes instead of diagnosing the root cause first.
---

# Systematic Debugging

Find and fix problems methodically. No guessing, no random changes, no "try this and see." Diagnose first, then fix.

---

## When to Use

Trigger this skill when you hear:
- "It's broken"
- "This isn't working"
- "I'm getting an error"
- "It worked yesterday but not today"
- "I don't know what's wrong"
- "I've tried everything"
- "Can you figure out what happened?"
- When any test fails and the cause isn't immediately obvious
- When a fix doesn't work and you're tempted to try something else randomly

This skill replaces panic, guessing, and the urge to change things until something works. Every problem has a cause. Find the cause first. Then fix it.

---

## Context Gathering

Before debugging anything, understand the situation. Ask these questions:

### What's Happening?
1. **What's the actual problem?** (What do you see? Not what you think is wrong — what literally happens?)
2. **What should be happening instead?** (The expected behavior — what "working" looks like)
3. **When did it start?** (Was it always like this? Did it break recently? After a specific change?)

### What Changed?
4. **What was the last thing that changed?** (Code, settings, content, deployment, update — anything)
5. **Is there an error message?** (Exact text, not paraphrased — every word matters)
6. **Can you reproduce it?** (Does it happen every time? Only sometimes? Only for certain users?)

---

## Methodology

Debugging has four phases. Do them in order. Do not skip ahead to fixing before you've diagnosed.

### Phase 1: INVESTIGATE

**Goal:** Understand exactly what's happening. Not what you think is happening — what's actually happening.

**Step 1: Reproduce the problem.**
Do the thing that's broken. Watch it break. If you can't reproduce it, you can't debug it.
- Follow the exact steps the user described
- Use the same inputs, same browser, same environment
- Note exactly what happens — error messages, unexpected behavior, missing content

**Step 2: Read the error message. The whole thing.**
Error messages tell you what's wrong. Most people skim them. Don't.
- Read every line of the error
- Look for file names, line numbers, variable names
- Look for "caused by" or "root cause" sections buried deeper in the output
- If there's a stack trace, read from bottom to top — the earliest call is usually most relevant

**Step 3: Check the logs.**
The error you see on screen is often a symptom. The logs have the real story.
- Check browser console for client-side issues
- Check server logs for backend issues
- Check email delivery logs for email issues
- Look at the timestamps — what happened right before the error?

**Step 4: Narrow the scope.**
Figure out where the problem lives. Is it:
- The frontend? (Page doesn't render correctly)
- The backend? (API returns wrong data)
- The database? (Data is missing or wrong)
- The deployment? (Old code is still running)
- The configuration? (Settings are wrong)
- External? (Third-party service is down)

Test each layer independently. If the API returns correct data but the page shows wrong data, the problem is in the frontend.

### Phase 2: HYPOTHESIZE

**Goal:** Form a short list of what could be causing this, ranked by likelihood.

**Step 1: List possible causes.**
Based on what you found in Phase 1, what could explain the behavior?
- Start with the most recent change — that's the most likely culprit
- Consider the simplest explanations first (typo, wrong variable, missing file)
- Consider environmental differences (works locally but not in production?)

**Step 2: Rank them.**
Put the most likely cause at the top. Consider:
- What changed most recently?
- What's the simplest explanation?
- What matches the error message?
- What has gone wrong before in similar situations?

**Step 3: Pick the top hypothesis.**
You're going to test this one first. If it's wrong, move to the next.

### Phase 3: TEST

**Goal:** Confirm or eliminate your hypothesis. One at a time.

**The rule: Change one thing at a time.** If you change three things and the problem goes away, you don't know which change fixed it. And you might have introduced new problems with the other two changes.

**Step 1: Design a test for your hypothesis.**
If your hypothesis is "the API key expired," the test is: check the API key's expiration date. Not "generate a new key and see if it works" — that's a fix, not a test.

**Step 2: Run the test.**
- If hypothesis is confirmed — move to Phase 4 (fix it)
- If hypothesis is eliminated — go back to Phase 2, test the next hypothesis
- If the result is ambiguous — get more information, refine the test

**Step 3: Document what you tested and what you found.**
Even failed hypotheses are valuable. They narrow the search.

### Phase 4: VERIFY

**Goal:** Confirm the fix works AND nothing else broke.

**Step 1: Fix the specific problem.**
Apply the minimum fix for the confirmed cause. Don't fix other things at the same time — you'll confuse what actually solved the problem.

**Step 2: Verify the fix.**
Reproduce the original problem. Does it still happen?
- If yes — your fix didn't work. Go back to Phase 3.
- If no — proceed to Step 3.

**Step 3: Check for side effects.**
The fix might have broken something else. Check:
- Related features that touch the same area
- Other pages or flows that use the same component
- Tests that were passing before — do they still pass?
- Edge cases the original thing handled — does it still handle them?

**Step 4: Run the full verification.**
Use the verification skill to formally confirm everything works.

---

## Output Format

When debugging, document the work in this structure:

```markdown
# Debug: [What Was Broken]
Date: [DATE]

## Problem
**What happened:** [What the user saw]
**What should happen:** [Expected behavior]
**When it started:** [Timeline]

## Investigation
- Reproduced: YES/NO
- Error message: [Exact text]
- Logs showed: [Key findings]
- Scope: [Where the problem lives — frontend, backend, config, etc.]

## Hypotheses (Ranked)
1. [Most likely cause] — TESTED — CONFIRMED/ELIMINATED
2. [Second most likely] — TESTED — CONFIRMED/ELIMINATED
3. [Third possibility] — NOT YET TESTED

## Root Cause
[The confirmed cause of the problem]

## Fix Applied
[Exactly what was changed and why]

## Verification
- Original problem resolved: YES/NO
- Related features still working: YES/NO
- All tests passing: YES/NO
```

---

## Quality Checks

Before closing any debugging session:

- [ ] **Problem was reproduced** — not guessed at from a description
- [ ] **Error messages were read completely** — not skimmed
- [ ] **Root cause was identified** — not just symptoms treated
- [ ] **Only one thing was changed at a time** — so you know what fixed it
- [ ] **Fix addresses the root cause** — not a workaround for a symptom
- [ ] **Original problem is resolved** — verified with fresh evidence
- [ ] **Nothing else broke** — related features checked
- [ ] **The fix is documented** — someone else could understand what happened and why

If any check fails, go back and do it properly.

---

## Examples

### Example 1: Page Not Loading

**User says:** "The pricing page is broken — it just shows a blank white screen."

**You do:**

**Investigate:**
1. Load /pricing — blank white screen confirmed
2. Open browser console — JavaScript error: "Cannot read property 'map' of undefined" at PricingTable.js line 42
3. Check the API that feeds the pricing page — returns empty response, status 200
4. Check the database — pricing data exists
5. Scope: backend API is returning empty data despite database having records

**Hypothesize:**
1. API query was changed recently and has a bug (most likely — a deploy happened yesterday)
2. Database connection issue (less likely — other pages work)
3. Caching issue (possible — might be serving stale empty response)

**Test:**
1. Check the git log for recent changes to the pricing API — found a query change yesterday that added a filter for "active" plans, but the column name is "status" not "active"

**Verify:**
1. Fix the column name in the query
2. Reload /pricing — plans appear, page renders correctly
3. Check other pages that use plan data — all working
4. Run test suite — all passing
5. Report: "Root cause was a typo in a database query — filtering on 'active' column that doesn't exist. Fixed the column name to 'status.' Pricing page and all related pages verified working."

### Example 2: Email Not Arriving

**User says:** "Customers say they're not getting the confirmation email."

**You do:**

**Investigate:**
1. Sign up with a test account — no email after 5 minutes
2. Check email service dashboard — emails are queued but not sent
3. Check sending logs — "API rate limit exceeded" error starting 3 days ago
4. Scope: external email service is blocking sends due to rate limiting

**Hypothesize:**
1. Hit the email service's rate limit (matches the error message exactly)
2. Email service account suspended (possible, but dashboard loads fine)
3. DNS records changed (unlikely — was working 3 days ago)

**Test:**
1. Check rate limit status in email service dashboard — confirmed, daily limit reached every day for 3 days running
2. Check what's sending so many emails — a new signup notification was accidentally set to send to all existing users, not just the new signup

**Verify:**
1. Fix the notification to only send to the new signup (not all users)
2. Reset the rate limit by contacting the email service
3. Sign up with a new test account — confirmation email arrives in 30 seconds
4. Check that existing notification flows still work — all arriving correctly
5. Monitor sending volume for 24 hours — back to normal levels
6. Report: "Root cause was a misconfigured notification sending to all users on each signup, exhausting the daily rate limit. Fixed the recipient to be only the new user. Email delivery confirmed working and volume back to normal."

### Example 3: Form Submission Error

**User says:** "People are filling out the contact form but I'm not getting the messages."

**You do:**

**Investigate:**
1. Submit the contact form with test data — "Thank you" confirmation appears
2. Check inbox — no message received
3. Check form handling service — submissions are being received
4. Check the forwarding email address in settings — it's set to an old email address that was deactivated

**Hypothesize:**
1. Forwarding email address is wrong (matches what the settings show)

**Test:**
1. Confirmed: the email address in the form handler is the old deactivated address

**Verify:**
1. Update the forwarding address to the current email
2. Submit the form again with test data — message arrives in inbox within 1 minute
3. Check that the "Thank you" confirmation still shows (it does)
4. Check that the form still validates properly — rejects empty fields, shows error for invalid email
5. Report: "Root cause was the form forwarding to a deactivated email address. Updated to the current address. Form submissions now arriving correctly, validation still working."

---

## Related Skills

- **tdd** — defines what "working" looks like before building; gives you clear criteria to debug against
- **verification** — the final step of debugging is always verification with fresh evidence
- **product-marketing-context** — when debugging marketing assets, check them against your brand and positioning requirements

**No fixes without diagnosis.** Find the cause first. Then fix it. Then verify it. Every time.
