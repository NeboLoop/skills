---
name: referral-program
description: Design customer referral and affiliate programs that drive word-of-mouth growth. Use when setting up a refer-a-friend program, choosing incentive structures, defining eligibility rules, planning referral tracking, or creating sharing mechanics and communication for a referral launch.
---

# Referral Program

Design customer referral and affiliate programs that drive word-of-mouth growth.

---

## When to Use

Trigger this skill when you hear:
- "I want a referral program"
- "How do I get customers to refer others?"
- "Set up an affiliate program"
- "Help me grow through word of mouth"
- "I want to reward customers for sharing"
- "How do I build a refer-a-friend program?"
- "I need more customers from referrals"

This skill helps you design the full referral program — the incentive, the mechanics, the messaging, and the tracking. It does not build the technical implementation, but gives you everything you need to hand off to a developer or set up in a referral tool.

---

## Context Gathering

Before designing a referral program, you need to understand the business. Ask these conversationally — don't dump them all at once.

### About Your Product
1. **What do you sell and how much does it cost?** (Needed to size the incentive correctly)
2. **How do customers currently find you?** (Understand existing acquisition channels)
3. **Do customers already recommend you informally?** (Natural word-of-mouth is a strong signal)

### About Your Customers
4. **Who are your happiest customers?** (These are your best referrers)
5. **When are customers most excited about your product?** (The ideal moment to ask for referrals)
6. **What would motivate your customers to share?** (Discounts? Cash? Credits? Status?)

### About Your Goals
7. **What's your budget for acquiring a new customer?** (Sets the ceiling for referral rewards)
8. **How many new customers per month are you aiming for from referrals?** (Helps set realistic expectations)
9. **Do you have any tools or platforms already in place?** (Email provider, CRM, referral software)

---

## Methodology

### Step 1: Check Product Marketing Context

Before designing anything, pull in what you already know about the business:

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If context exists, use it to inform the program design — especially pricing, customer profile, and brand voice.

If no context exists, suggest running the **product-marketing-context** skill first. You need to understand the product, customer, and pricing before designing a referral program that fits.

### Step 2: Choose the Program Type

There are three main types. Help the user pick the right one based on their business:

**One-sided (referrer gets rewarded)**
- Best for: Products where the referrer's recommendation is the main value
- How it works: Existing customer shares, gets a reward when the new person signs up or buys
- Example: "Give a friend our link, get $20 credit when they subscribe"
- Good when: Your product doesn't need a discount to convert new customers

**Two-sided (both people get rewarded)**
- Best for: Products where you need to motivate both sides — the sharer and the new customer
- How it works: Existing customer shares, both get a reward when the new person converts
- Example: "Give $10, get $10" or "You both get a free month"
- Good when: New customers need a nudge to try the product

**Affiliate (commission-based)**
- Best for: Products with higher price points or when you want partners (bloggers, influencers, other businesses) promoting you
- How it works: Partner gets a percentage of each sale they drive, usually ongoing
- Example: "Earn 20% commission on every customer you refer"
- Good when: You want to attract dedicated promoters, not just casual sharers

Ask the user: *"Based on your product and customers, which of these feels right? Or I can recommend one."*

If they want a recommendation, use their pricing and customer profile to suggest the best fit:
- Low price point, high volume → two-sided with small rewards
- Medium price point, loyal customers → one-sided or two-sided
- High price point, long sales cycle → affiliate with commissions

### Step 3: Design the Incentive

The reward needs to be big enough to motivate sharing but small enough to be profitable. Use these guidelines:

**For discounts or credits:**
- The reward should be 10-25% of the customer's first payment or monthly subscription
- Two-sided programs: split the budget between referrer and new customer
- Example: $50/month product → $10 credit for referrer, $10 off first month for new customer

**For free time (subscription products):**
- One free month for the referrer is a strong, simple incentive
- One free month (or extended trial) for the new customer lowers the barrier
- Example: "You both get a free month"

**For cash or gift cards:**
- Use when your product isn't something people buy repeatedly
- Pay out after the new customer completes a qualifying action (not just signup)
- Example: "$25 gift card after your friend makes their first purchase"

**For affiliate commissions:**
- 10-30% of the sale price is standard
- Recurring commissions (on subscriptions) attract serious affiliates
- Example: "20% of every monthly payment, for as long as they're a customer"

Ask the user: *"Does this reward structure feel right for your economics? What's the most you'd be comfortable giving away per referral?"*

### Step 4: Define Eligibility

Decide who can refer, who counts as a valid referral, and when the reward is earned.

**Who can refer:**
- All customers? Only paid customers? Only customers who've been active for 30+ days?
- Recommend: Paid customers who've been around long enough to genuinely recommend the product

**What counts as a valid referral:**
- New customer signs up (low bar, higher fraud risk)
- New customer makes a purchase or starts a paid plan (higher bar, better quality)
- New customer stays for 30 days (highest bar, best quality)
- Recommend: Tie the reward to a meaningful conversion — not just a signup

**Limits:**
- Is there a cap on how many people someone can refer? (Prevents abuse)
- Is there a time limit on the offer? (Creates urgency)
- Can existing customers be referred? (Usually no)
- Recommend: Cap at 10-20 referrals per person unless running an affiliate program

**Anti-fraud basics:**
- No self-referrals (same email, same payment method)
- New customer must be genuinely new (never had an account before)
- Rewards are revoked if the referred customer cancels within the qualifying period

### Step 5: Create Sharing Mechanics

Make it dead simple for customers to share. The easier it is, the more people do it.

**Unique referral link:**
- Every customer gets a personal link like `yourproduct.com/r/jane123`
- Easiest to set up, works everywhere (email, social, text)
- Must: Link should be short and clean, not a mess of tracking parameters

**Invite code:**
- Customer gets a code like `JANE20` that the new person enters at signup
- Good for: Products where the signup flow has a "promo code" field
- Less ideal for social sharing (links are easier to click)

**Email invite:**
- Customer enters friend's email, your system sends the invite on their behalf
- Good for: B2B or professional products where a personal intro matters
- Must: Let the customer preview and edit the message before it sends

**Social sharing:**
- One-click buttons to share on common platforms
- Pre-written message the customer can customize
- Good for: Consumer products with broad appeal

Recommend offering at least two methods — unique link (for flexibility) plus one other that fits the product.

### Step 6: Set Up Tracking and Attribution

You need to track the full journey from share to conversion to reward. Define these:

**What to track:**
- Who shared (the referrer)
- How they shared (link, code, email invite)
- Who clicked or signed up (the referred person)
- Whether the referred person converted (hit the qualifying action)
- When the reward was earned and delivered

**Attribution window:**
- How long after clicking a referral link does the new customer have to convert?
- Standard: 30-90 days
- Shorter windows reduce fraud, longer windows capture slower decision-makers

**Tools to consider:**
- Built-in referral features in your product (if your platform supports it)
- Dedicated referral platforms (ReferralCandy, Friendbuy, GrowSurf, Rewardful)
- Custom tracking with your existing analytics and CRM

Ask the user: *"Do you have a preference for how you'd track this, or should I recommend a simple approach?"*

### Step 7: Design the Communication

A referral program only works if customers know about it and feel good sharing. Design these messages:

**Program announcement:**
- Tell existing customers about the program
- Keep it simple: what they get, what their friend gets, how to share
- Send via email and add to the product dashboard

**Invitation copy (what the referrer sends):**
- Write 2-3 versions of the sharing message
- Make it personal and genuine, not salesy
- Include the value for the friend, not just "sign up for this thing"
- Example: "I've been using [Product] for [use case] and it's been great. Here's a link to get [reward] off your first month — [referral link]"

**Reward notification:**
- When someone earns a reward, celebrate it
- Tell them exactly what they earned and how to use it
- Example: "Your friend just signed up! You've earned a $10 credit — it'll apply to your next bill."

**Milestone celebrations:**
- When someone refers 3, 5, or 10 people, acknowledge it
- Consider bonus rewards at milestones (tiered program)
- Example: "You've referred 5 friends! Here's a bonus month on us."

**Reminder nudges:**
- Gentle reminders to customers who haven't shared yet
- Triggered at high-satisfaction moments (after a positive interaction, after achieving a result)
- Example: "Enjoying [Product]? Share the love — give a friend [reward] and get [reward] for yourself."

---

## Output Format

Deliver the referral program as a structured document:

```markdown
# Referral Program Plan
Created: [DATE]

## Program Overview
**Type:** [One-sided / Two-sided / Affiliate]
**Summary:** [One sentence describing the program]

## Incentive Structure
**Referrer reward:** [What the referrer gets]
**New customer reward:** [What the referred person gets, if two-sided]
**When reward is earned:** [The qualifying action]

## Eligibility Rules
**Who can refer:** [Criteria for referrers]
**Valid referral:** [What counts as a successful referral]
**Limits:** [Caps, time limits, restrictions]
**Anti-fraud:** [Rules to prevent abuse]

## Sharing Mechanics
**Primary method:** [Unique link / Invite code / Email invite]
**Secondary method:** [Additional sharing option]
**Sharing surfaces:** [Where in the product and communications the sharing option appears]

## Tracking & Attribution
**Attribution window:** [How long the referral link stays valid]
**Tracking method:** [Tool or approach for tracking]
**Key metrics:** [What to measure — shares, clicks, conversions, reward cost]

## Communication Plan
### Program Announcement
[Draft of the launch email or in-app message]

### Invitation Copy
[2-3 versions of the sharing message]

### Reward Notification
[Message sent when a reward is earned]

### Milestone Messages
[Messages for referral milestones]

### Reminder Nudges
[Messages to encourage sharing at key moments]

## Economics
**Estimated cost per referred customer:** [Reward cost]
**Current customer acquisition cost:** [For comparison]
**Break-even point:** [When the referred customer covers the reward cost]

## Launch Checklist
- [ ] Referral tracking set up and tested
- [ ] Sharing link/code generation working
- [ ] Reward fulfillment process ready
- [ ] Announcement email written and scheduled
- [ ] In-app referral prompt added
- [ ] Reward notification emails configured
- [ ] Anti-fraud rules in place
- [ ] Reporting dashboard set up
```

---

## Quality Checks

Before delivering the program, verify:

- [ ] **Incentive makes financial sense** — the reward cost is less than your normal customer acquisition cost
- [ ] **Reward is compelling** — it's big enough that customers actually care about earning it
- [ ] **Sharing is simple** — a customer can share in under 30 seconds with no confusion
- [ ] **Rules are clear** — no ambiguity about who qualifies, what counts, or when rewards are delivered
- [ ] **Copy sounds human** — the invitation message sounds like something a real person would send, not corporate marketing
- [ ] **Anti-fraud is covered** — basic protections against self-referrals and fake accounts
- [ ] **Tracking is feasible** — the user has a realistic way to track referrals with their current tools
- [ ] **Launch plan exists** — the user knows how to announce the program and where to place the sharing prompt
- [ ] **Economics are documented** — the user understands cost per referral vs. normal acquisition cost

---

## Examples

### Example: Two-Sided Referral Program for a SaaS

**User says:** "I want to set up a referral program. We have a project management tool for small teams, $29/month per team."

**You do:**
1. Search memory for product marketing context (found: project management SaaS, small teams of 5-15 people, $29/month)
2. Ask: "Do your customers already recommend you to other teams informally?"
3. User: "Yeah, we get some organic referrals. People mention us in Slack communities."
4. Ask: "That's a great sign. What do you think would motivate your customers to share more actively — a discount on their own subscription, or something for their friend too?"
5. User: "Probably both. If my friend gets something too, I feel better recommending it."
6. Recommend a two-sided program and design it:

**Program delivered:**

```markdown
# Referral Program Plan
Created: 2026-03-21

## Program Overview
**Type:** Two-sided
**Summary:** Existing customers give friends a free month, and earn a free month themselves when the friend subscribes.

## Incentive Structure
**Referrer reward:** One free month ($29 credit applied to next bill)
**New customer reward:** First month free (no charge until month 2)
**When reward is earned:** When the referred team starts a paid subscription (after free month ends and they stay)

## Eligibility Rules
**Who can refer:** Any customer on a paid plan for at least 14 days
**Valid referral:** New team that has never had an account, starts a paid plan after their free month
**Limits:** Up to 12 referrals per year (one free month earned per referral — up to a full free year)
**Anti-fraud:** No self-referrals, referred team must have a unique payment method, reward revoked if referred team cancels within first paid month

## Sharing Mechanics
**Primary method:** Unique referral link (e.g., teamtool.com/r/acme-co)
**Secondary method:** Email invite — enter a colleague's email, preview and send a personal note
**Sharing surfaces:** Dashboard sidebar, settings page, and a prompt after positive moments (completing a project, hitting a milestone)

## Tracking & Attribution
**Attribution window:** 60 days from link click
**Tracking method:** GrowSurf integration (connects to existing Stripe billing)
**Key metrics:** Referral links shared, link clicks, signups from referrals, conversion to paid, total rewards paid out

## Communication Plan

### Program Announcement
Subject: Give a free month, get a free month

Hey [Name],

Know another team that could use [Product]? Share your personal link and when they sign up, they'll get their first month free. When they stick around, you'll get a free month too.

Your link: [referral link]

Simple as that.

### Invitation Copy
**Version 1 (casual):**
"My team's been using [Product] to manage our projects and it's made a real difference. Here's a link to try it free for a month — [referral link]"

**Version 2 (specific):**
"We switched to [Product] a few months ago and it cut our meeting time in half because everyone can see where things stand. Free month if you want to try it — [referral link]"

**Version 3 (short, for Slack/text):**
"We use [Product] for project management and love it. Free month here: [referral link]"

### Reward Notification
Subject: You earned a free month!

[Friend's team name] just started their paid plan. Your $29 credit has been applied to your next bill.

You've referred [X] teams so far. Keep sharing — every referral earns another free month, up to 12 per year.

### Milestone Messages
**3 referrals:** "Three teams joined because of you! You've got 3 months of free service banked. Here's a small thank-you from our team — [bonus, like a branded sticker pack or extended feature access]."

**5 referrals:** "Five referrals! You're one of our top advocates. We'd love to feature your team in a case study — interested?"

### Reminder Nudges
**After completing a project:** "Nice work finishing [Project Name]! Know another team that could use this? Share your link and you'll both get a free month — [referral link]"

**After 60 days of no shares:** "Your referral link is still ready whenever you are. Every friend who joins gets a free month, and so do you — [referral link]"

## Economics
**Estimated cost per referred customer:** $29 (one free month for referrer) + $29 (one free month for new customer) = $58
**Current customer acquisition cost:** $85 (from paid ads)
**Break-even point:** The referred customer covers the reward cost in month 2 of their paid subscription
**Note:** Referred customers historically have 40% lower churn than ad-acquired customers, making the real lifetime value significantly higher
```

7. Ask: "Does this structure work for your team? Anything you'd adjust on the rewards or the rules?"
8. User confirms
9. Say: "Great. Your next steps are to pick a referral tracking tool — I'd suggest GrowSurf for your setup — and get the sharing link and email invite built into your dashboard. Want me to help draft the full announcement email next?"

---

## Related Skills

- **email-sequence** — build the email flows for program announcements, reward notifications, and reminder nudges
- **pricing-strategy** — make sure the referral incentive fits your pricing model and margins
- **product-marketing-context** — foundation for understanding your product, customer, and positioning before designing the program
- **marketing-ideas** — generate additional growth tactics that complement your referral program
