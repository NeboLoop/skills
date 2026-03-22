---
name: paid-ads
description: Plan and optimize paid advertising campaigns across Google Ads, Meta Ads, and LinkedIn Ads — from strategy to structure to ongoing improvement. Use when setting up, troubleshooting, or optimizing paid campaigns.
---

# Paid Ads

Plan and optimize paid advertising campaigns across Google Ads, Meta Ads, and LinkedIn Ads — from strategy to structure to ongoing improvement.

---

## When to Use

Trigger this skill when you hear:
- "I want to run ads"
- "Help me set up a Google Ads campaign"
- "Should I advertise on Facebook or Google?"
- "My ads aren't working"
- "How should I spend my ad budget?"
- "I need more leads from paid channels"
- "Set up Meta Ads for my product"
- "Help me with LinkedIn advertising"
- "What should I bid on?"
- "My cost per lead is too high"

This skill helps you go from "I want to run ads" to a structured campaign plan with targeting, budget, ad formats, and tracking — ready to build in the ad platform.

---

## Context Gathering

Before building any campaign plan, understand these things. Ask conversationally — don't dump a questionnaire.

### Business Basics
1. **What are you selling?** (Product, service, offer — and at what price point?)
2. **Who are you trying to reach?** (Be specific: job titles, interests, demographics, location)
3. **What do you want people to do after they see your ad?** (Buy something, sign up, book a call, download a guide?)

### Advertising Experience
4. **Have you run paid ads before?** (Which platforms? What happened?)
5. **What's your monthly budget for ads?** (Even a rough range helps)
6. **Do you have a landing page ready?** (Where will people go after they click?)

### Competitive Landscape
7. **Do you know if competitors are running ads?** (We can check this together)
8. **What makes your offer better or different?** (The reason someone should click your ad instead of a competitor's)

### Tracking
9. **Do you have tracking set up?** (Google Analytics, Meta Pixel, LinkedIn Insight Tag — it's okay if you don't know)
10. **What does a "win" look like?** (A purchase? A form fill? A phone call? An app install?)

---

## Methodology

### Step 1: Check Product Marketing Context

Before doing anything, pull existing knowledge about the business:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to pre-fill what you already know — product details, ideal customer, pricing, differentiation. Skip questions you already have answers for.

If no context exists, ask the user to set it up first, or gather the basics yourself (see Context Gathering above).

### Step 2: Define the Campaign Objective

Every ad campaign needs a clear goal. Help the user pick one:

| Objective | What it means | When to use it |
|-----------|--------------|----------------|
| **Awareness** | Get your name in front of people | New brand, new market, big audience |
| **Traffic** | Drive visitors to your website | You have strong content or a compelling landing page |
| **Leads** | Collect contact information | You have a sales team or email nurture in place |
| **Sales** | Get direct purchases or signups | You sell online and can track purchases |

Ask: *"What's the one thing you want this campaign to accomplish?"*

Don't let them say "all of the above." One campaign, one objective.

### Step 3: Match Audience to Platform

Each ad platform works best for different situations. Guide the user:

**Google Ads** — Best when people are actively searching for what you offer.
- Someone types "project management software" — they already want what you sell.
- Works well for: products with clear search demand, local services, comparison shoppers.
- Use Nebo's browser to check search volume and competitor ads:
```
web(action: "navigate", url: "https://ads.google.com/home/tools/keyword-planner/")
```

**Meta Ads (Facebook/Instagram)** — Best when you need to reach people based on interests, behaviors, or demographics, even if they aren't searching right now.
- Someone is scrolling Instagram and sees your ad for a meal delivery service — they didn't search for it, but it fits their lifestyle.
- Works well for: visual products, impulse purchases, community building, retargeting website visitors.

**LinkedIn Ads** — Best for reaching professionals by job title, company size, industry, or seniority.
- You want to reach "VP of Engineering at companies with 200-1000 employees" — LinkedIn is the only place you can target that precisely.
- Works well for: B2B products, high-ticket services, recruiting, thought leadership.
- Important: LinkedIn is expensive. Cost per click can be 5-10x higher than other platforms. Only use it when precise professional targeting matters enough to justify the cost.

**How to decide:** Ask the user — *"Are your customers actively searching for your type of product, or do they need to discover it?"*
- Searching = Google Ads
- Discovering = Meta Ads
- Professional/B2B decision makers = LinkedIn Ads

You can recommend multiple platforms, but start with one. Master it before expanding.

### Step 4: Budget Allocation and Bidding

Help the user set realistic expectations based on their budget.

**Minimum viable budgets (rough guidelines):**
- Google Ads: $500-1000/month to get meaningful data
- Meta Ads: $300-500/month minimum
- LinkedIn Ads: $1000-2000/month (higher cost per click)

**Bidding strategies in plain language:**
- **Maximize clicks** — the platform gets you as many clicks as possible. Good for starting out and learning.
- **Target cost per action** — you tell the platform how much you want to pay per lead or sale, and it optimizes toward that. Good when you have conversion data.
- **Manual bidding** — you set the exact amount you'll pay per click. Good when you know your numbers well.

Start with maximize clicks or an automated strategy. Switch to target cost per action after you have 30-50 conversions worth of data.

**Budget split guidance:**
- Put 70-80% of budget on your best-performing campaign
- Use 20-30% to test new audiences, ad copy, or keywords
- Never split budget evenly across everything — focus wins

### Step 5: Build the Campaign Structure

Explain how campaigns are organized — it's like folders on a computer:

```
Campaign (the big goal)
  └── Ad Group / Ad Set (a specific audience or theme)
        └── Ads (the actual messages people see)
        └── Keywords or Targeting (who sees the ads)
```

**For Google Ads campaigns:**
- One campaign per objective (e.g., "Lead Generation — Search")
- Separate ad groups by theme or keyword cluster (e.g., "project management tool" vs "team collaboration software")
- 3-5 keywords per ad group, closely related to each other
- 2-3 ad variations per ad group to test what works

**For Meta Ads campaigns:**
- One campaign per objective
- Separate ad sets by audience (e.g., "Interest: entrepreneurs" vs "Lookalike: existing customers")
- 2-4 ad variations per ad set
- Keep audience sizes between 500,000 and 5,000,000 for best results

**For LinkedIn Ads campaigns:**
- One campaign per objective
- Target by job title + industry + company size combinations
- Keep audience size above 50,000 for delivery
- Start with Sponsored Content (single image or video in the feed)

### Step 6: Select Ad Formats

Recommend formats based on the objective and platform:

**Google Ads formats:**
- **Search ads** — Text ads that appear in search results. Best for capturing people who are looking for what you sell.
- **Display ads** — Banner images across websites. Best for awareness and retargeting.
- **Performance Max** — Google uses automation to show your ads everywhere. Good when you have conversion data and want to scale.

**Meta Ads formats:**
- **Single image** — Simple and effective. Start here.
- **Video** — Higher engagement, great for storytelling or product demos. Even 15 seconds works.
- **Carousel** — Multiple images people swipe through. Good for showing features or products.

**LinkedIn Ads formats:**
- **Sponsored Content** — Posts that appear in the feed. Most versatile format.
- **Message Ads** — Direct messages to prospects. High open rates but use sparingly.
- **Document Ads** — Share PDFs or guides directly in the feed. Great for thought leadership.

### Step 7: Set Up Tracking

Without tracking, you're spending money blindly. Help the user set up:

**UTM parameters** — Tags added to your ad URLs so Google Analytics knows which ads brought visitors. Every ad link should have them:
```
yoursite.com/landing-page?utm_source=google&utm_medium=cpc&utm_campaign=lead-gen-search&utm_content=ad-variation-1
```

Explain it simply: *"UTMs are like labels on your ad links. They tell your analytics tool exactly which ad someone clicked to get to your site."*

**Conversion tracking** — Tell the ad platform when someone takes the action you want:
- Google Ads: Install a small piece of code on your "thank you" or confirmation page
- Meta Ads: Install the Meta Pixel on your website and set up events (like "Lead" or "Purchase")
- LinkedIn Ads: Install the LinkedIn Insight Tag and define conversions

**The rule:** Don't launch ads until tracking is working. Spend the first day verifying that conversions are recording correctly.

### Step 8: Optimization Recommendations

After the campaign launches, give the user a plan for improving results:

**Week 1: Let it run**
- Don't change anything for the first 3-5 days. The ad platform needs time to learn.
- Check daily but don't panic if early numbers look bad.

**Week 2: First adjustments**
- Pause keywords or audiences with lots of clicks but no conversions (they're wasting budget).
- Increase budget on ad groups or ad sets that are converting well.
- Review search terms report (Google Ads) — add irrelevant searches as negative keywords so you stop paying for them.

**Ongoing optimization:**
- Test new ad copy every 2-3 weeks. Change one thing at a time (headline, image, or call to action — not all three).
- Check your cost per conversion against your target. If it's too high, narrow your targeting or improve your landing page.
- Review and update negative keywords weekly (Google Ads).
- Refresh ad creative monthly to avoid "ad fatigue" — when people see the same ad too many times and stop clicking.

Use Nebo's browser to review the landing page:
```
web(action: "navigate", url: "user-landing-page-url")
web(action: "read_page")
```

Check that the landing page message matches the ad copy. If the ad promises "free trial" but the landing page says "book a demo," that mismatch will hurt conversions.

---

## Output Format

Deliver the campaign plan in this structure:

```markdown
# Paid Ads Campaign Plan
Created: [DATE]

## Objective
[One clear goal — awareness, traffic, leads, or sales]

## Platform
[Google Ads / Meta Ads / LinkedIn Ads — and why this platform fits]

## Target Audience
**Who:** [Specific description of the people you're trying to reach]
**Targeting approach:** [Keywords, interests, job titles, lookalikes, etc.]
**Location:** [Geographic targeting]

## Budget & Bidding
**Monthly budget:** [$X]
**Bidding strategy:** [Maximize clicks / Target CPA / Manual]
**Expected cost per click:** [$X range based on industry]

## Campaign Structure
**Campaign name:** [Descriptive name]

### Ad Group/Set 1: [Theme]
- **Targeting:** [Keywords or audience details]
- **Ad format:** [Search / Image / Video / Carousel]
- **Ad copy direction:** [Brief description of the message]

### Ad Group/Set 2: [Theme]
- **Targeting:** [Keywords or audience details]
- **Ad format:** [Search / Image / Video / Carousel]
- **Ad copy direction:** [Brief description of the message]

## Tracking Setup
- [ ] UTM parameters on all ad URLs
- [ ] Conversion tracking installed and verified
- [ ] Google Analytics connected (if applicable)
- [ ] Platform pixel/tag installed (if applicable)

## Optimization Schedule
**Week 1:** [What to watch]
**Week 2:** [First changes to make]
**Ongoing:** [Regular maintenance tasks]

## Key Metrics to Watch
- **Cost per click (CPC):** What you pay each time someone clicks
- **Click-through rate (CTR):** What percentage of people who see your ad actually click it
- **Cost per conversion:** What you pay for each lead, sale, or signup
- **Conversion rate:** What percentage of clickers take the action you want
- **Return on ad spend (ROAS):** How much revenue you get back for every dollar spent
```

---

## Quality Checks

Before delivering the campaign plan, verify:

- [ ] **One clear objective** — not trying to do everything at once
- [ ] **Platform choice is justified** — matches the audience behavior (searching vs discovering vs professional targeting)
- [ ] **Budget is realistic** — enough to get meaningful data on the chosen platform
- [ ] **Targeting is specific** — not "everyone aged 18-65"
- [ ] **Campaign structure is organized** — ad groups/sets have clear themes, not everything lumped together
- [ ] **Ad format matches the objective** — search ads for intent, video/image for awareness
- [ ] **Tracking plan exists** — UTMs and conversion tracking are accounted for
- [ ] **Landing page aligns with ad message** — what the ad promises is what the page delivers
- [ ] **Optimization plan is included** — the user knows what to do after launch
- [ ] **No unexplained jargon** — every technical term is explained in plain language
- [ ] **Product marketing context was referenced** — positioning and differentiation inform the ad strategy

---

## Examples

### Example: Google Ads Campaign for a SaaS Product

**User says:** "I want to run Google Ads for my project management tool. We're a startup competing with Asana and Monday.com."

**You do:**

1. Check memory for product marketing context:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

2. Context found — the product is "TaskFlow," a simple project management tool for freelancers and small teams. $19/month. Differentiation: no learning curve, set up in 2 minutes vs hours for enterprise tools. Goal: more trial signups.

3. Ask: *"Your goal is more trial signups — so this is a leads campaign. How much are you thinking of spending per month on Google Ads?"*

4. User: "Maybe $1500/month to start?"

5. Ask: *"Good budget to start. Do you have a landing page specifically for the free trial, or would people go to your homepage?"*

6. User: "We have a landing page at taskflow.com/free-trial."

7. Review the landing page with Nebo's browser:
```
web(action: "navigate", url: "https://taskflow.com/free-trial")
web(action: "read_page")
```

8. Research competitor ads:
```
web(action: "navigate", url: "https://google.com/search?q=simple+project+management+tool")
web(action: "read_page")
```

9. Deliver the campaign plan:

```markdown
# Paid Ads Campaign Plan — TaskFlow
Created: 2026-03-21

## Objective
Generate free trial signups (leads campaign)

## Platform
Google Ads — Search campaigns. Your ideal customers are actively searching
for project management tools. Google lets you show up right when they're
looking.

## Target Audience
**Who:** Freelancers and small team leads (2-10 people) looking for a
simple way to manage projects
**Targeting approach:** Search keywords — people typing things like
"simple project management tool" or "easy project tracker for small teams"
**Location:** United States, Canada, United Kingdom, Australia

## Budget & Bidding
**Monthly budget:** $1,500
**Bidding strategy:** Maximize clicks for the first 2-3 weeks, then switch
to Target CPA once you have 30+ trial signups tracked
**Expected cost per click:** $2-5 range for project management keywords

## Campaign Structure
**Campaign name:** TaskFlow — Trial Signups — Search

### Ad Group 1: Simple Project Management
- **Keywords:** "simple project management tool," "easy project management
  software," "project management for small teams," "lightweight project
  management"
- **Ad format:** Responsive search ads
- **Ad copy direction:** Lead with simplicity — "Set Up in 2 Minutes.
  No Training Needed." Contrast with complex enterprise tools.
  Call to action: "Start Free Trial"

### Ad Group 2: Freelancer Project Management
- **Keywords:** "project management for freelancers," "freelancer task
  tracker," "solo project management tool," "freelance project organizer"
- **Ad format:** Responsive search ads
- **Ad copy direction:** Speak directly to freelancers — "Built for
  Freelancers, Not Enterprises." Emphasize price ($19/mo) and speed.
  Call to action: "Try Free for 14 Days"

### Ad Group 3: Competitor Alternatives
- **Keywords:** "Asana alternative for small teams," "simpler than
  Monday.com," "Monday.com alternative," "Asana alternative simple"
- **Ad format:** Responsive search ads
- **Ad copy direction:** Position against enterprise complexity —
  "Tired of Tools Built for 500-Person Companies?" Focus on the
  2-minute setup vs hours of configuration.
  Call to action: "Start Free Trial"

## Tracking Setup
- [ ] UTMs on all ad URLs:
      taskflow.com/free-trial?utm_source=google&utm_medium=cpc&utm_campaign=trial-signups-search&utm_content=[ad-group-name]
- [ ] Google Ads conversion tracking on the post-signup confirmation page
- [ ] Google Analytics 4 connected to Google Ads
- [ ] Trial signup event created as primary conversion

## Optimization Schedule
**Week 1:** Let it run. Check that conversions are tracking correctly.
Watch for any keywords with very high cost per click ($10+) and pause them.
**Week 2:** Review the search terms report. Add irrelevant terms as
negative keywords (e.g., "free project management" if your trial requires
a credit card). Pause any ad group spending over $100 with zero signups.
**Ongoing:** Test new ad copy every 2 weeks. Add new negative keywords
weekly. Once you hit 30 trial signups, switch to Target CPA bidding
with a target of $30-40 per signup.

## Key Metrics to Watch
- **Cost per trial signup:** Target under $40 (given $19/mo product,
  aim for payback within 2-3 months)
- **Click-through rate:** Aim for 3-5% on search ads
- **Conversion rate on landing page:** Should be 10-20% for a free
  trial page. If below 10%, improve the landing page before spending
  more on ads.
```

10. Follow up: *"Want me to help write the actual ad copy for these ad groups? The ad-creative skill can build that out based on your positioning."*

---

## Related Skills

- **ad-creative** — writes the actual ad headlines, descriptions, and visual concepts for campaigns planned here
- **analytics-tracking** — sets up conversion tracking, UTMs, and reporting dashboards to measure ad performance
- **product-marketing-context** — provides the positioning, audience, and differentiation that inform ad strategy
- **copywriting** — crafts landing page copy that matches your ad messaging
- **page-cro** — optimizes the landing pages your ads send people to, improving conversion rates and lowering cost per lead
