---
name: form-cro
description: Optimize lead capture and contact forms for higher conversion rates — fewer fields, clearer labels, stronger calls to action. Use when form completion or lead capture rates are low.
---

# Form CRO

Optimize lead capture and contact forms for higher conversion rates. Fewer fields, clearer labels, stronger calls to action.

---

## When to Use

Trigger this skill when you hear:
- "My form isn't converting"
- "How do I get more leads from my form?"
- "Should I reduce form fields?"
- "Optimize my contact form"
- "Review my signup form"
- "People start my form but don't finish"
- "My form abandonment rate is high"
- "Help me improve my lead capture"

This skill focuses specifically on form elements — fields, labels, buttons, validation, and layout. For full-page conversion optimization, use **page-cro** instead.

---

## Context Gathering

Before diving into the form, you need a few things:

1. **What's the form URL?** (So you can load and examine it directly)
2. **What's the form's goal?** (Lead capture? Contact request? Demo booking? Free trial signup?)
3. **What happens after someone submits?** (Email reply? Redirect? Nothing?)
4. **Do you know your current conversion rate?** (How many visitors vs. how many submissions)
5. **What are you offering in exchange for their info?** (Nothing? A consultation? A download? A quote?)

Don't ask all five at once. Start with the URL and goal, then ask follow-ups based on what you see.

---

## Methodology

### Step 1: Check Product Marketing Context

Before analyzing the form, understand what the business does and who they're targeting:

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run **product-marketing-context** first. You need to understand the customer and value proposition to evaluate whether the form speaks to the right audience.

### Step 2: Load the Form

Use the browser to view the actual form:

```
web(action: "navigate", url: "the-form-url.com")
web(action: "read_page")
```

If the form is on a landing page, note the surrounding context too — headline, copy, trust signals. These affect form conversion even though they're not part of the form itself.

### Step 3: Audit Field Count

Count every field the user must fill out. This is the single biggest factor in form conversion.

**Benchmarks:**
- 1-3 fields: High conversion, low data quality
- 4-5 fields: Balanced — good for most lead capture
- 6-8 fields: Conversion starts dropping noticeably
- 9+ fields: Only works for high-intent situations (job applications, insurance quotes)

**Rule of thumb:** Each field you remove can lift conversion by roughly 10%. If the form has 8 fields and could work with 5, that's potentially a 30% improvement.

Ask yourself for each field: *"Would we lose the deal if we didn't collect this right now?"* If not, remove it or collect it later.

### Step 4: Audit Labels and Placeholder Text

Check every label for clarity:
- **Labels should be above the field, not inside it.** Placeholder text that disappears when you start typing causes confusion — people forget what the field was asking for.
- **Labels should be plain language.** "Full Name" not "Name (First, MI, Last)." "Phone Number" not "Primary Contact Number."
- **Avoid redundant labels.** If you have separate First Name and Last Name fields, consider combining into one "Name" field.
- **Placeholder text should show format, not repeat the label.** For a phone field, the placeholder could be "(555) 123-4567" — not "Enter your phone number."

### Step 5: Audit the CTA Button

The submit button is where people decide to commit. Check:
- **Does it say "Submit"?** That's generic and creates anxiety. Better options: "Get My Free Quote," "Book My Demo," "Send Message," "Start Free Trial."
- **Does the button copy match the offer?** If the page promises a free consultation, the button should say "Get My Free Consultation" — not "Submit Form."
- **Is the button visually obvious?** It should be a contrasting color that stands out from the rest of the page.
- **Is there a single, clear CTA?** Multiple buttons or competing actions confuse people.

**Good CTA formula:** Start with a verb + state the benefit. "Get My Free Quote" beats "Submit" every time.

### Step 6: Check Progressive Disclosure and Smart Defaults

Look for opportunities to reduce perceived complexity:

- **Progressive disclosure:** Can the form start with just 2-3 fields and reveal more based on answers? Example: Show "Company Size" only after they select "Business" as account type.
- **Smart defaults:** Can you pre-select the most common option? If 80% of users choose "United States," default to that.
- **Conditional fields:** Are there fields that only apply to some users? Hide them until relevant.
- **Auto-detect where possible:** Use IP-based location to pre-fill country/state. Use email domain to pre-fill company name.

### Step 7: Check Inline Validation and Error Messages

Fill out the form incorrectly on purpose (in your analysis) and check:

- **Does validation happen inline (as you fill fields) or only after submission?** Inline is better — people can fix mistakes immediately.
- **Are error messages helpful?** "Invalid input" is useless. "Please enter a valid email address (example: you@company.com)" is helpful.
- **Does the form preserve entered data after an error?** If someone makes one mistake and the form clears everything, they'll leave.
- **Are required fields marked clearly?** Use an asterisk (*) or the word "Required" — not just a red border after they try to submit.

### Step 8: Assess Mobile Experience

More than half of web traffic is mobile. Check:

- **Do fields stack vertically?** Side-by-side fields on mobile are frustrating.
- **Are tap targets large enough?** Minimum 44px height for input fields and buttons on mobile.
- **Does the right keyboard appear?** Email fields should trigger the email keyboard. Phone fields should trigger the number pad.
- **Can the form be completed with one thumb?** If it requires pinching, zooming, or horizontal scrolling, it's broken.
- **Is the CTA button visible without scrolling?** On long forms, consider a sticky button at the bottom.

### Step 9: Suggest Multi-Step If Needed

If the form genuinely needs more than 5 fields (and you've already cut everything non-essential), recommend breaking it into steps:

- **Step 1:** The easiest fields (name, email) — get a micro-commitment
- **Step 2:** More specific fields (company, role, needs)
- **Step 3:** Optional or detailed fields (budget, timeline, comments)

**Show a progress indicator** ("Step 1 of 3") so people know how much is left.

**Why this works:** Once someone completes Step 1, they've invested effort and are more likely to finish. This is the "foot in the door" principle.

### Step 10: Compile Recommendations

Organize your findings into prioritized recommendations:
1. **Quick wins** — changes that take minutes and have high impact (reducing fields, rewriting the CTA button)
2. **Medium effort** — changes that need some development work (adding inline validation, making it multi-step)
3. **Bigger improvements** — changes that require design or strategy shifts (progressive disclosure, smart defaults, redesigning the mobile experience)

---

## Output Format

Present your audit as a structured report:

```markdown
# Form Conversion Audit
**Form URL:** [URL]
**Form Goal:** [What the form is meant to accomplish]
**Date:** [DATE]

## Current State
- **Number of fields:** [X]
- **CTA button text:** "[Current text]"
- **Mobile friendly:** [Yes/No/Partially]
- **Inline validation:** [Yes/No]

## Top Issues Found
1. [Biggest issue — the one change that would make the most difference]
2. [Second biggest issue]
3. [Third biggest issue]

## Field-by-Field Review
| Field | Keep/Remove/Modify | Reason |
|-------|-------------------|--------|
| [Field name] | [Keep/Remove/Modify] | [Why] |

## Recommendations

### Quick Wins (Do These First)
- [ ] [Recommendation 1]
- [ ] [Recommendation 2]

### Medium Effort
- [ ] [Recommendation 1]
- [ ] [Recommendation 2]

### Bigger Improvements
- [ ] [Recommendation 1]
- [ ] [Recommendation 2]

## Expected Impact
[Estimate of potential conversion lift based on changes recommended]
```

---

## Quality Checks

Before delivering the audit, verify:

- [ ] **You loaded the actual form** — not guessing from a description
- [ ] **Product marketing context was reviewed** — recommendations align with the target audience
- [ ] **Every field was evaluated** — each one has a keep/remove/modify recommendation with a reason
- [ ] **CTA button was assessed** — specific rewrite suggested if needed
- [ ] **Mobile experience was checked** — not just desktop
- [ ] **Recommendations are prioritized** — quick wins first, bigger changes last
- [ ] **Field reduction math is included** — tell them the potential lift from removing unnecessary fields
- [ ] **No jargon** — "call to action button" not "CTA," or define it on first use
- [ ] **Suggestions are specific** — "Change button text from 'Submit' to 'Get My Free Quote'" not "Improve your CTA"

---

## Examples

### Example: B2B Lead Capture Form

**User says:** "My landing page form isn't converting. Can you take a look?"

**You do:**
1. Check memory for product marketing context — find they're a B2B SaaS selling project management software to marketing teams
2. Ask for the form URL
3. User shares: "acmetools.com/demo"
4. Load the page in the browser

**What you find:**
- The form has 9 fields: First Name, Last Name, Email, Phone, Company, Job Title, Company Size, Industry, Message
- The CTA button says "Submit"
- No inline validation
- Fields don't stack properly on mobile
- No progress indicator or multi-step

**Your audit:**

*"I loaded your demo request form and found several opportunities. Here's what stands out:*

*Your form has 9 fields. For a demo request, that's too many. Here's my field-by-field breakdown:*

| Field | Recommendation | Reason |
|-------|---------------|--------|
| First Name + Last Name | Combine into one "Name" field | Saves a field, you can split the name later |
| Email | Keep | Essential for follow-up |
| Phone | Remove | You can ask during the demo scheduling email |
| Company | Keep | Helps your sales team prepare |
| Job Title | Remove | Your sales team can look this up on LinkedIn |
| Company Size | Move to Step 2 or remove | Nice to have, not essential to book the demo |
| Industry | Remove | Visible from the company name in most cases |
| Message | Make optional and move last | Most people leave this blank anyway |

*That takes you from 9 fields down to 3-4 required fields. Based on the rough guideline that each removed field lifts conversion by about 10%, you could see a 40-50% improvement.*

*For your button, change 'Submit' to 'Book My Free Demo' — it matches what you're offering and feels less like a commitment.*

*On mobile, your First Name and Last Name fields sit side by side, making them hard to tap. Stacking them vertically (or combining into one Name field) fixes that.*

*Quick wins to do today:*
- *Combine First + Last Name into one field*
- *Remove Phone, Job Title, and Industry*
- *Change the button to 'Book My Free Demo'*
- *Make the Message field optional with a note like 'Anything specific you'd like to see in the demo?'*

*These changes are straightforward and should give you a noticeable lift within a week of data."*

---

## Related Skills

- **page-cro** — audits the full landing page around the form, not just the form itself
- **signup-flow-cro** — optimizes the entire signup process, including post-form steps like onboarding
- **popup-cro** — improves popup forms (exit intent, scroll-triggered, timed)
- **lead-magnets** — creates compelling offers that give people a reason to fill out the form
- **product-marketing-context** — provides the customer and positioning info needed to evaluate form relevance
