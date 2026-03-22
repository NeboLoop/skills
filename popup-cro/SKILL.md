---
name: popup-cro
description: Design and optimize modals and overlays that convert without annoying visitors. Use when creating, improving, or deciding whether to add popups, exit-intent overlays, or welcome mats.
---

# Popup CRO

Design and optimize modals and overlays that convert without annoying visitors.

---

## When to Use

Trigger this skill when you hear:
- "I need a popup on my site"
- "Help me create an exit intent popup"
- "My popup isn't converting"
- "Should I add a popup?"
- "I want to capture more emails with a popup"
- "My popup is annoying visitors"
- "Design an overlay for my landing page"
- "How do I set up a welcome mat?"
- "Optimize my popup"

This skill helps you decide if a popup is the right move, what type to use, what to say in it, when to show it, and how often. The goal is always: give visitors something valuable at the right moment, not interrupt them.

---

## Context Gathering

Before designing any popup, you need to understand:

### From the User
1. **What page(s) will the popup appear on?** (Homepage, blog, pricing, product pages, all pages?)
2. **What's the goal of the popup?** (Collect emails, announce a sale, promote a lead magnet, reduce cart abandonment, get feedback?)
3. **What are you offering visitors?** (Discount, free resource, early access, newsletter — there has to be something in it for them)
4. **Do you already have popups running?** (If yes, what's working and what isn't?)
5. **What does your mobile traffic look like?** (Rough percentage — important for how we design this)
6. **Any platform constraints?** (What tool are you using — Klaviyo, OptinMonster, ConvertKit, custom code, etc.?)

### From Nebo Tools
Check for existing product marketing context:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use the brand voice, customer profile, and current goals to inform the popup design. If it doesn't exist, run the **product-marketing-context** skill first.

If the user has a live site, check what popups are already running:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

Look for existing popups, overlays, banners, and slide-ins. Note what they say, when they appear, and whether they feel intrusive.

---

## Methodology

### Step 1: Check Product Marketing Context

Pull context from memory to understand the brand, customer, and goals:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

You need to know:
- Who the visitor is (so you can speak their language)
- What the brand voice sounds like (so the popup matches the site)
- What the current marketing goals are (so the popup serves a real purpose)

If no context exists, pause and run the **product-marketing-context** skill first. A popup without context is a guess.

### Step 2: Determine the Popup Type

Choose the right type based on the goal and the visitor's behavior:

**Exit intent** — shows when the visitor moves their cursor toward the browser close button. Best for: last-chance offers, cart abandonment, lead magnets. Low annoyance because it only fires when they're already leaving.

**Timed delay** — shows after the visitor has been on the page for a set amount of time (usually 15-60 seconds). Best for: email signups on blog posts, general offers. Medium annoyance — make sure the delay is long enough that they've had time to engage.

**Scroll-triggered** — shows when the visitor scrolls past a certain percentage of the page (usually 50-70%). Best for: blog content upgrades, related offers. Low annoyance because it proves the visitor is engaged before interrupting.

**Welcome mat** — full-screen overlay that shows immediately on arrival. Best for: major announcements, limited-time sales, high-value lead magnets. High annoyance potential — use sparingly and only when the offer is genuinely worth it.

**Slide-in** — a small box that slides up from the bottom corner. Best for: subtle email signups, live chat prompts, low-pressure offers. Lowest annoyance — doesn't block content.

Pick one. Don't stack multiple popup types on the same page. That's a quick way to frustrate visitors.

### Step 3: Design the Offer

The popup must answer one question for the visitor: *"What's in it for me?"*

Good offers:
- A discount or free shipping (for ecommerce)
- A free resource like a checklist, template, or guide (for content/SaaS)
- Early access or a waitlist spot (for pre-launch)
- A meaningful content upgrade related to what they're reading (for blogs)

Bad offers:
- "Subscribe to our newsletter" (no one wakes up wanting another newsletter)
- "Stay updated" (too vague — updated about what?)
- "Don't miss out" (miss out on what exactly?)

The offer needs to be specific and immediately valuable. If you can't articulate what the visitor gets, the popup shouldn't exist.

### Step 4: Write the Copy

Every popup needs four elements:

**Headline** — One short sentence that states the benefit. Lead with what they get, not what you want.
- Good: "Get 15% off your first order"
- Good: "Free checklist: 10 things to fix on your landing page"
- Bad: "Sign up for our newsletter"
- Bad: "Don't leave yet!"

**Body** — One to two sentences that add context. Explain what they'll get and why it matters.
- Good: "Join 5,000 marketers who get weekly conversion tips. No fluff, unsubscribe anytime."
- Bad: "Enter your email below to subscribe to our mailing list for updates and news."

**Call to action (CTA)** — The button text. Make it specific to the action.
- Good: "Get my 15% off"
- Good: "Send me the checklist"
- Bad: "Submit"
- Bad: "Subscribe"

**Dismiss option** — Always give visitors a clear way to close the popup. Never make it hard to find.
- Good: A visible X button plus a text link like "No thanks, I'll pay full price"
- Bad: A tiny X in the corner that's hard to tap on mobile
- Bad: No dismiss option at all (never do this)

Keep the total word count under 50. Popups are not the place for long copy.

### Step 5: Set Triggering Rules

Define exactly when and where the popup appears:

**Timing:**
- Exit intent: fires on cursor movement toward browser chrome (desktop only)
- Timed delay: 15-30 seconds for high-intent pages (pricing, product), 30-60 seconds for content pages (blog)
- Scroll trigger: 50% for short pages, 60-70% for long content

**Page targeting:**
- Show on specific pages, not everywhere. A popup about a blog content upgrade shouldn't appear on the pricing page.
- Exclude pages where popups hurt: checkout, login, account settings, support/help pages

**Visitor targeting:**
- New visitors vs. returning visitors (different offers for each)
- Visitors who haven't already converted (don't show an email popup to someone already on your list)
- Traffic source (you might show a different popup to visitors from a specific campaign)

### Step 6: Set Frequency Capping

How often the same visitor sees the popup:

- **Once per session** — good for exit intent offers and announcements
- **Once per 7 days** — good for email signup popups
- **Once per 30 days** — good for general promotions
- **Once ever (until cookies clear)** — good for welcome mats and major announcements

The rule: if a visitor dismissed your popup, respect that. Showing it again immediately says you don't care about their choice.

After someone converts (fills out the form), never show them that popup again. Use cookies or your email tool's targeting to suppress it.

### Step 7: Mobile Considerations

Mobile popups need special treatment:

- **Never use full-screen popups on mobile.** Google penalizes intrusive interstitials on mobile, and they're genuinely terrible to use on a small screen.
- **Use a bottom banner or slide-in instead.** It should take up no more than 30-40% of the screen.
- **Make the close button easy to tap.** At least 44x44 pixels — the minimum comfortable tap target.
- **Make sure form fields are usable.** One field (email) is ideal for mobile. Two fields maximum.
- **Test on actual phones.** What looks fine in a desktop preview can be unusable on a real device.
- **Consider disabling popups on mobile entirely** if your mobile conversion rate is low anyway. Focus mobile visitors on the core page experience.

---

## Output Format

Present popup recommendations in this structure:

```markdown
# Popup Recommendation: [Page or Campaign Name]

## Overview
**Type:** [Exit intent / Timed delay / Scroll-triggered / Welcome mat / Slide-in]
**Goal:** [What this popup is trying to achieve]
**Target audience:** [Who sees this popup]
**Offer:** [What the visitor gets]

## Copy
**Headline:** [The headline]
**Body:** [1-2 sentence body copy]
**CTA button:** [Button text]
**Dismiss text:** [Close/dismiss text]

## Triggering Rules
**Trigger:** [When it fires — exit intent, 30-second delay, 60% scroll, etc.]
**Pages:** [Which pages it appears on]
**Excluded pages:** [Pages where it should NOT appear]
**Visitor targeting:** [New vs. returning, non-subscribers only, etc.]

## Frequency Capping
**Show:** [Once per session / once per 7 days / once per 30 days / once ever]
**After conversion:** [Suppress permanently]

## Mobile
**Mobile version:** [How it adapts — bottom banner, slide-in, disabled, etc.]
**Close button:** [44x44px minimum tap target]
**Fields:** [Number of form fields on mobile]

## Measurement
**Track:** [What to measure — conversion rate, impact on bounce rate, revenue per visitor]
**Goal conversion rate:** [Target percentage, typically 3-8% for well-designed popups]
```

---

## Quality Checks

Before delivering a popup recommendation, verify:

- [ ] **The offer is specific and valuable** — not "subscribe to our newsletter" but a concrete benefit
- [ ] **The headline leads with the visitor's benefit** — not what the business wants
- [ ] **Copy is under 50 words total** — popups are not landing pages
- [ ] **CTA button text is specific** — not "Submit" or "Subscribe"
- [ ] **There's a clear dismiss option** — visible X and/or text link
- [ ] **Triggering rules match the page context** — the popup makes sense for where it appears
- [ ] **Frequency capping is set** — the same visitor won't see it repeatedly
- [ ] **Mobile is addressed** — no full-screen popups on mobile, close button is tappable
- [ ] **Excluded pages are defined** — checkout, login, support pages are excluded
- [ ] **Brand voice matches** — the popup sounds like the rest of the site
- [ ] **Only one popup per page** — no stacking multiple overlays
- [ ] **Measurement is defined** — you know what success looks like

---

## Examples

### Example 1: Exit Intent Popup for an Ecommerce Store

**User says:** "I want to add an exit intent popup to my product pages. We sell handmade candles."

**You do:**
1. Check memory for product marketing context (found: handmade candle company, brand voice is warm and casual, goal is to grow email list and increase first-time purchases)
2. Check their site with the browser — no existing popups, product pages have strong imagery
3. Recommend an exit intent popup with a first-purchase discount

**Recommendation:**

```markdown
# Popup Recommendation: Product Page Exit Intent

## Overview
**Type:** Exit intent
**Goal:** Capture email and convert first-time visitors into buyers
**Target audience:** New visitors on product pages who haven't purchased before
**Offer:** 10% off their first order

## Copy
**Headline:** "Before you go — 10% off your first candle"
**Body:** "We hand-pour every candle in small batches. Try one with 10% off, delivered to your inbox."
**CTA button:** "Send my 10% off"
**Dismiss text:** "No thanks, I'll pay full price"

## Triggering Rules
**Trigger:** Exit intent (cursor moves toward browser chrome)
**Pages:** All product pages
**Excluded pages:** Cart, checkout, account, FAQ, shipping info
**Visitor targeting:** New visitors only, exclude existing email subscribers

## Frequency Capping
**Show:** Once per 7 days
**After conversion:** Suppress permanently

## Mobile
**Mobile version:** Bottom banner (no exit intent on mobile — use 45-second timed delay instead)
**Close button:** 44x44px tap target, top-right of banner
**Fields:** Email only (one field)

## Measurement
**Track:** Email capture rate, discount redemption rate, revenue from popup subscribers
**Goal conversion rate:** 5-8% of visitors who see it
```

### Example 2: Scroll-Triggered Popup for a Blog

**User says:** "I write a blog about personal finance. I want to grow my email list but I don't want to be annoying."

**You do:**
1. Check memory for product marketing context (found: personal finance blog, audience is millennials managing money for the first time, voice is friendly and relatable)
2. Check their site — no popups, long-form blog posts averaging 1,500 words
3. Recommend a scroll-triggered slide-in with a content upgrade

**Recommendation:**

```markdown
# Popup Recommendation: Blog Scroll-Triggered Slide-In

## Overview
**Type:** Slide-in (bottom right corner)
**Goal:** Grow email list from engaged blog readers
**Target audience:** Visitors who read at least half of a blog post
**Offer:** Free monthly budget spreadsheet template

## Copy
**Headline:** "Want the budget template I actually use?"
**Body:** "It's the same spreadsheet I use to manage my own money. Free, no strings."
**CTA button:** "Send me the template"
**Dismiss text:** "I'm good, thanks"

## Triggering Rules
**Trigger:** 60% scroll depth
**Pages:** All blog posts
**Excluded pages:** Homepage, about, contact, any non-blog page
**Visitor targeting:** Non-subscribers only

## Frequency Capping
**Show:** Once per 7 days
**After conversion:** Suppress permanently

## Mobile
**Mobile version:** Same slide-in, bottom of screen, 35% screen height max
**Close button:** 44x44px tap target
**Fields:** Email only

## Measurement
**Track:** Slide-in conversion rate, unsubscribe rate of popup subscribers (to gauge quality)
**Goal conversion rate:** 3-5% of visitors who trigger it
```

---

## Related Skills

- **page-cro** — optimize the full page experience around and behind the popup
- **form-cro** — improve the form inside the popup for higher completion rates
- **lead-magnets** — create the free resource or offer that makes the popup worth clicking
- **copywriting** — write popup copy that matches brand voice and converts
- **product-marketing-context** — the foundation for knowing what to say and who to say it to
