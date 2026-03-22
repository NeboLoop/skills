---
name: tdd
description: Define what "working" looks like before building anything, then build until tests pass. Use before building any new feature, form, page, email flow, or integration to ensure success criteria are clear, tested, and verified throughout development.
---

# Test-Driven Development

Write a test first, watch it fail, then build the thing. Before building anything, define what "working" looks like. Then build until it works.

---

## When to Use

Trigger this skill when you hear:
- "Build me a..."
- "I need a new feature"
- "Create a form / page / workflow"
- "How do I know this works?"
- "It should do X when Y happens"
- "Make sure this doesn't break"
- "Add a test for this"
- Before building ANY new feature, form, page, email flow, or integration

This skill applies to everything — not just code. Landing pages, email campaigns, form submissions, checkout flows, API integrations. If you're building something that needs to work, define "working" first.

---

## Context Gathering

Before writing any test, understand what you're building and what success looks like. Ask these questions conversationally:

### What Are We Building?
1. **What's the thing?** (A page, a form, an email, a feature, an integration — one sentence)
2. **What should it do when it's working?** (The happy path — what happens when everything goes right)
3. **What should it NOT do?** (Edge cases — what happens with bad input, missing data, wrong permissions)

### What Does "Done" Look Like?
4. **How will you know it works?** (What would you check manually? Load a page? Submit a form? Receive an email?)
5. **What's the most important thing to get right?** (If only one thing works perfectly, what should it be?)
6. **What broke last time?** (If this is a fix or rebuild, what went wrong before?)

---

## Methodology

### Step 1: Define Success Criteria

Before touching any code or content, write down exactly what "working" means. Be specific.

**Bad success criteria:**
- "The page loads"
- "The form works"
- "The email sends"

**Good success criteria:**
- "The page loads in under 3 seconds and shows the user's name in the header"
- "The form validates email format, rejects empty fields, and shows a confirmation message after submit"
- "The email arrives within 60 seconds, includes the user's first name, and the unsubscribe link works"

Write these as a checklist. Each item should be something you can observe and verify with a yes or no.

### Step 2: Write the Test First

Turn each success criterion into a check you can run. This looks different depending on what you're building:

**For code:** Write an automated test that checks the behavior.
```
// Example: testing that a registration endpoint works
test("registration creates a new user", func() {
    response = post("/register", {email: "test@example.com", name: "Test User"})
    expect(response.status).toBe(201)
    expect(response.body.user.email).toBe("test@example.com")
})
```

**For a landing page:** Write a manual test script.
```
1. Open the page in a browser
2. Verify the headline matches the copy doc
3. Click the CTA button
4. Verify it goes to the signup form
5. Submit the form with a test email
6. Verify the confirmation page appears
```

**For an email flow:** Write a verification checklist.
```
1. Trigger the email by completing the signup
2. Check inbox within 2 minutes
3. Verify subject line matches template
4. Verify first name personalization works
5. Click every link — verify none are broken
6. Check the unsubscribe link works
```

### Step 3: Watch It Fail

Run the test before building anything. This step is critical and non-negotiable.

**Why?** Because if a test passes before you've built the thing, the test is broken. It's not actually checking what you think it's checking. A test that always passes is worthless.

- For automated tests: run them, confirm they fail with the expected error
- For manual test scripts: walk through the steps, confirm the thing doesn't exist or work yet
- For verification checklists: attempt each step, confirm it can't be completed yet

If the test passes before you build — **stop**. Your test is wrong. Fix the test first.

### Step 4: Build the Minimum

Now build. But only build enough to make the test pass. Nothing extra.

- Don't add features that aren't in your success criteria
- Don't optimize before it works
- Don't style before it functions
- Don't add "nice to have" things

The goal is: make the test pass. That's it.

**Common mistake:** Building everything you can think of, then running the test at the end. This defeats the purpose. Build a little, test, build a little more, test.

### Step 5: Run the Test Again

Run your test. Does it pass?

- **Yes** — move to Step 6
- **No** — read the failure carefully. What exactly failed? Fix that specific thing. Run the test again. Repeat until it passes.

Do not skip failures. Do not comment out failing checks. Every success criterion must pass.

### Step 6: Clean Up

Now that it works, make it better:

- Remove any temporary workarounds
- Clean up the code or content
- Add any polish (styling, copy refinement, performance)
- Run the test one final time to make sure cleanup didn't break anything

### Step 7: Add Edge Case Tests

Now that the happy path works, think about what could go wrong:

- What happens with empty input?
- What happens with really long input?
- What happens when the network is slow?
- What happens when the user does something unexpected?
- What happens on mobile vs desktop?

Write tests for the important edge cases. Run them. Fix what fails.

### Step 8: Confirm Done

Run all tests — happy path and edge cases. Everything passes? You're done.

If any test fails, go back to Step 4 for that specific test. Don't ship with known failures.

---

## Output Format

When applying TDD, document the work in this structure:

```markdown
# TDD: [What You're Building]
Date: [DATE]

## Success Criteria
1. [Specific, observable criterion]
2. [Specific, observable criterion]
3. [Specific, observable criterion]

## Tests Written
- [ ] [Test name / check description] — STATUS
- [ ] [Test name / check description] — STATUS
- [ ] [Test name / check description] — STATUS

## Initial Run (All Should Fail)
- Test 1: FAILED (expected)
- Test 2: FAILED (expected)
- Test 3: FAILED (expected)

## Build Log
- Built [thing] to satisfy test 1 — PASSED
- Built [thing] to satisfy test 2 — PASSED
- Built [thing] to satisfy test 3 — PASSED

## Edge Cases
- [Edge case 1]: TESTED — PASSED/FAILED
- [Edge case 2]: TESTED — PASSED/FAILED

## Final Run
All tests passing: YES/NO
```

---

## Quality Checks

Before calling anything "done," verify:

- [ ] **Success criteria are specific** — each one can be answered yes or no
- [ ] **Tests were written before building** — not after
- [ ] **Tests failed initially** — proving they actually check something real
- [ ] **Only the minimum was built** — no feature creep during the build
- [ ] **All tests pass** — no skipped or ignored failures
- [ ] **Edge cases are covered** — not just the happy path
- [ ] **Cleanup didn't break anything** — final test run after polish
- [ ] **No assumptions** — every claim of "working" has a test proving it

If any check fails, go back and fix it before moving on.

---

## Examples

### Example 1: Building a Contact Form

**User says:** "Add a contact form to the website."

**You do:**
1. Ask: "What should happen when someone submits the form?"
2. User: "I get an email with their name, email, and message."
3. Define success criteria:
   - Form displays with name, email, and message fields
   - Empty submissions are rejected with error messages
   - Valid submission shows a "Thank you" confirmation
   - Email arrives in the inbox within 2 minutes with all three fields
4. Write tests for each criterion
5. Run tests — all fail (form doesn't exist yet)
6. Build the form — run tests after each piece
7. All tests pass — clean up and confirm done

### Example 2: Verifying an Email Campaign

**User says:** "Set up a welcome email for new signups."

**You do:**
1. Ask: "What should the welcome email include? And when should it send?"
2. User: "Send immediately after signup. Include their first name, a link to get started, and our support email."
3. Define success criteria:
   - Email sends within 60 seconds of signup
   - Subject line: "Welcome to [Product], [First Name]!"
   - Body includes first name, getting-started link, support email
   - Getting-started link goes to the correct page
   - Email renders correctly on mobile
4. Write a test checklist for each criterion
5. Attempt the checklist — fails (email not set up yet)
6. Build the email flow — test after each step
7. All checks pass — confirm done

### Example 3: Building an API Endpoint

**User says:** "I need an endpoint that returns a user's profile."

**You do:**
1. Ask: "What fields should the profile include? And who should be able to access it?"
2. User: "Name, email, and plan type. Only the user themselves, authenticated."
3. Define success criteria:
   - GET /api/profile returns 200 with name, email, plan for authenticated users
   - Returns 401 for unauthenticated requests
   - Returns 404 for non-existent users
   - Response time under 200ms
4. Write automated tests for each criterion
5. Run tests — all fail (endpoint doesn't exist)
6. Build the endpoint — run tests after each piece
7. All tests pass — clean up and confirm done

---

## Related Skills

- **verification** — the companion to TDD; verifies that finished work actually works with fresh evidence
- **systematic-debugging** — when a test fails and you can't figure out why, debug methodically
- **product-marketing-context** — when building marketing assets, define what "working" means for your audience

**Every build starts with TDD.** Define success, test for it, then build until the tests pass.
