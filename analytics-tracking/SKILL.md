---
name: analytics-tracking
description: Set up conversion tracking and event measurement for your website and marketing campaigns. Use when configuring GA4, defining conversion events, creating UTM parameters, building tracking plans, or verifying that analytics are firing correctly.
---

# Analytics Tracking

Set up conversion tracking and event measurement so you know what's working across your website and marketing campaigns.

---

## When to Use

Trigger this skill when you hear:
- "Set up tracking"
- "I need analytics"
- "How do I track conversions?"
- "Set up GA4"
- "I don't know what's converting"
- "Add UTM parameters"
- "Track signups / purchases / form submissions"
- "My tracking is broken"
- "I need to measure my campaigns"

This skill covers the full tracking setup — from deciding what to measure, to naming your events, to generating the code or tag configuration, to verifying everything fires correctly.

---

## Context Gathering

Ask these questions conversationally before diving into setup:

### Current State
1. **Do you have any analytics installed right now?** (Google Analytics, Mixpanel, Plausible, nothing at all — just need to know the starting point)
2. **Do you use Google Tag Manager?** (Tag Manager is a container that holds your tracking code — it makes adding and updating tracking much easier without touching your website code)
3. **What platform is your site built on?** (WordPress, Shopify, custom code, Webflow, etc. — this affects how we install things)

### Goals
4. **What are the most important actions someone can take on your site?** (Sign up, buy, fill out a form, book a demo, download something — these become your conversion events)
5. **What marketing channels are you using?** (Email, social, paid ads, SEO, partnerships — helps us set up UTM tracking for each)
6. **What questions do you want analytics to answer?** (Where do my customers come from? What pages lead to signups? Which ads are working? — helps us focus on what matters)

### Technical
7. **Do you or your developer have access to the site code?** (Determines whether we use code snippets or a no-code approach like Tag Manager)
8. **Any privacy requirements?** (GDPR consent banners, cookie policies, data processing agreements — affects how we configure tracking)

---

## Methodology

### Step 1: Check Product Marketing Context

Pull your existing marketing context to understand what you're trying to achieve:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

The marketing goals and ideal customer profile tell us what events matter most. A SaaS focused on trial signups needs different tracking than an e-commerce store optimizing for purchases.

If no marketing context exists, recommend running the **product-marketing-context** skill first — or at minimum, ask questions 4-6 from Context Gathering above.

### Step 2: Identify Key Conversion Events

Based on the product goals, define the events that actually matter. These fall into a few categories:

**Conversion events** — the actions that directly mean revenue or progress:
- Completed a purchase
- Signed up for a trial or free account
- Submitted a lead form
- Booked a demo or call
- Started a subscription

**Engagement events** — actions that signal interest and predict conversions:
- Viewed a pricing page
- Watched a product video
- Downloaded a resource (ebook, guide, template)
- Added an item to cart
- Clicked a CTA button
- Scrolled past a key section

**Campaign events** — actions tied to specific marketing efforts:
- Clicked a link in an email
- Arrived from a specific ad
- Used a referral link
- Redeemed a promo code

Don't track everything. Pick 3-5 conversion events and 5-10 engagement events that map directly to business goals. More than that creates noise.

### Step 3: Define Event Taxonomy

Create a naming convention so events stay organized as the list grows. Consistency matters more than cleverness.

**Naming rules:**
- Use `snake_case` (all lowercase, underscores between words)
- Start with the object, then the action: `form_submit`, `button_click`, `page_view`
- Keep names short but descriptive
- Add parameters for details instead of creating separate events

**Event parameters** add detail without creating a new event for every variation:
- `form_submit` with parameter `form_name: "contact"` or `form_name: "demo_request"`
- `button_click` with parameter `button_text: "Start Free Trial"` and `page: "/pricing"`
- `purchase_complete` with parameter `value: 49.00` and `plan: "pro"`

Present the full event list to the user for confirmation before generating any code.

### Step 4: Generate Tracking Code or Tag Manager Config

Based on the user's setup (code access vs. Tag Manager vs. platform), generate the appropriate implementation.

**For Google Analytics 4 (GA4) with gtag.js:**
Provide the base installation snippet and custom event calls that go on specific pages or buttons.

**For Google Tag Manager (GTM):**
Walk through creating tags, triggers, and variables. Explain each piece:
- **Tag** = what fires (the tracking code that sends data to GA4)
- **Trigger** = when it fires (page load, button click, form submit)
- **Variable** = dynamic info included with the event (page URL, button text, form name)

**For platforms like Shopify or WordPress:**
Reference built-in integrations and plugins that handle the base install, then layer custom events on top.

Always include the GA4 Measurement ID placeholder and remind users where to find theirs (GA4 > Admin > Data Streams > select stream > Measurement ID).

### Step 5: Set Up Conversion Funnels

Map out the steps users take before converting, then make sure each step is tracked:

1. Identify the funnel stages (e.g., Landing Page > Pricing Page > Signup Form > Account Created)
2. Confirm each stage has a corresponding event or pageview
3. Note where drop-off is most likely (this is where you focus optimization later)
4. In GA4, mark your most important events as "conversions" (GA4 > Admin > Conversions > New Conversion Event)

Explain that a funnel is just a sequence of steps — like a path through your site. Tracking each step lets you see where people leave and where to focus improvements.

### Step 6: Create UTM Parameter Templates

UTM parameters are tags added to the end of URLs so analytics can tell you exactly which campaign, channel, or ad sent someone to your site.

Create templates for each active marketing channel:

**The five UTM parameters:**
- `utm_source` — where the traffic comes from (google, facebook, newsletter)
- `utm_medium` — the channel type (cpc, email, social, referral)
- `utm_campaign` — the specific campaign name (spring_sale, product_launch)
- `utm_term` — the keyword or audience (optional, mainly for paid search)
- `utm_content` — which specific link or ad variation (optional, for A/B testing)

**Naming rules for UTMs:**
- All lowercase
- Use underscores instead of spaces
- Be consistent — `facebook` not sometimes `fb`
- Be specific — `spring_2026_sale` not just `sale`

Provide a filled-in template for each channel the user is actively using.

### Step 7: Verify Tracking Is Firing

Use the browser to check that tracking is actually working:
```
web(action: "navigate", url: "their-site.com")
web(action: "read_page")
```

**Verification checklist:**
- GA4 tag is present in the page source
- Events fire on the expected actions (check via GA4 Realtime report or Tag Assistant)
- UTM parameters pass through correctly
- Conversion events register in GA4
- No duplicate tags (a common problem that inflates your numbers)
- Consent/cookie banners work correctly if required

Walk the user through checking GA4's Realtime report (GA4 > Reports > Realtime) to see events come in live. This is the fastest way to confirm things are working.

---

## Output Format

Deliver the tracking plan as a structured document:

```markdown
# Analytics Tracking Plan
Last updated: [DATE]

## Tracking Platform
**Analytics:** [GA4 / Mixpanel / Plausible / etc.]
**Implementation:** [gtag.js / Google Tag Manager / Platform plugin]
**Measurement ID:** [G-XXXXXXXXXX]

## Conversion Events
| Event Name | Description | Parameters | Marked as Conversion |
|---|---|---|---|
| [event_name] | [What this tracks] | [key: value] | [Yes/No] |

## Engagement Events
| Event Name | Description | Parameters |
|---|---|---|
| [event_name] | [What this tracks] | [key: value] |

## Conversion Funnel
1. [Stage 1] — tracked by: [event/pageview]
2. [Stage 2] — tracked by: [event/pageview]
3. [Stage 3] — tracked by: [event/pageview]
4. [Conversion] — tracked by: [event]

## UTM Templates
### [Channel Name]
`https://yoursite.com/page?utm_source=[source]&utm_medium=[medium]&utm_campaign=[campaign]`

## Installation Code
[Code snippets or Tag Manager instructions]

## Verification Status
- [ ] Base analytics tag installed
- [ ] Conversion events firing
- [ ] Engagement events firing
- [ ] UTM parameters passing through
- [ ] Funnels configured in GA4
- [ ] No duplicate tags
```

---

## Quality Checks

Before delivering the tracking plan, verify:

- [ ] **Conversion events map to real business goals** — not vanity metrics, but actions that drive revenue or growth
- [ ] **Event names follow the naming convention** — consistent snake_case, object-first pattern
- [ ] **No more than 15-20 custom events** — GA4 has limits, and too many events create noise
- [ ] **UTM templates exist for every active channel** — so campaign tracking is consistent from day one
- [ ] **Funnel stages are trackable** — every step has a corresponding event or pageview
- [ ] **Privacy requirements are addressed** — consent banners, cookie policies, data retention settings
- [ ] **Code snippets are correct for the platform** — tested or verified against current GA4 documentation
- [ ] **No duplicate tracking** — only one GA4 tag per page, not multiple installations competing
- [ ] **User understands what each event means** — no jargon without explanation

---

## Examples

### Example: Setting Up GA4 Tracking for a B2B SaaS

**User says:** "I need to set up tracking for my SaaS. I have no idea what's converting."

**You do:**

1. Search memory for product marketing context. Found: Project management tool for freelancers, $19/mo, goal is to increase trial signups.

2. Ask: "Do you have any analytics on your site right now?"
   User: "I think someone added Google Analytics years ago but I never look at it."

3. Use browser to check their site. Find an old Universal Analytics tag (UA-XXXXXXX) — this stopped collecting data in July 2023. No GA4 installed.

4. Ask: "What are the most important things someone does on your site?"
   User: "Sign up for a free trial, upgrade to paid, and submit a contact form."

5. Define the event plan:

   **Conversion events:**
   - `trial_signup` — user creates a free account (parameters: `method: "email"` or `method: "google"`)
   - `plan_upgrade` — user upgrades from free to paid (parameters: `plan: "monthly"`, `value: 19.00`)
   - `form_submit` — user submits the contact form (parameters: `form_name: "contact"`)

   **Engagement events:**
   - `pricing_page_view` — user visits /pricing
   - `feature_page_view` — user visits any /features/* page (parameters: `feature: "time-tracking"`)
   - `cta_click` — user clicks a call-to-action button (parameters: `button_text`, `page`)
   - `video_play` — user plays the demo video (parameters: `video_title`)
   - `scroll_depth` — user scrolls 50% or more of a key page (parameters: `page`, `percent: 50`)

6. Present event plan to user. User confirms.

7. Generate GA4 setup:

   **Base installation (goes on every page, in the `<head>`):**
   ```html
   <script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
   <script>
     window.dataLayer = window.dataLayer || [];
     function gtag(){dataLayer.push(arguments);}
     gtag('js', new Date());
     gtag('config', 'G-XXXXXXXXXX');
   </script>
   ```

   **Trial signup event (fires when signup completes):**
   ```javascript
   gtag('event', 'trial_signup', {
     method: 'email'
   });
   ```

   **Plan upgrade event (fires on successful payment):**
   ```javascript
   gtag('event', 'plan_upgrade', {
     plan: 'monthly',
     value: 19.00,
     currency: 'USD'
   });
   ```

8. Map the conversion funnel:
   - Homepage > Pricing Page > Signup Form > Trial Created > Plan Upgrade
   - Each step has a tracked event or pageview

9. Create UTM templates:
   - Newsletter: `?utm_source=newsletter&utm_medium=email&utm_campaign=[campaign_name]`
   - Twitter: `?utm_source=twitter&utm_medium=social&utm_campaign=[campaign_name]`
   - Google Ads: `?utm_source=google&utm_medium=cpc&utm_campaign=[campaign_name]&utm_term=[keyword]`

10. Tell user to install the code, then verify together using GA4 Realtime.

11. Walk through marking `trial_signup`, `plan_upgrade`, and `form_submit` as conversions in GA4 Admin.

12. User: "This is great. I can finally see what's actually happening."

---

## Related Skills

- **ab-test-setup** — uses tracked events as success metrics for experiments
- **paid-ads** — relies on conversion tracking to measure ad performance and optimize spend
- **page-cro** — uses funnel data and engagement events to identify where visitors drop off
- **product-marketing-context** — provides the goals and customer profile that determine what to track
