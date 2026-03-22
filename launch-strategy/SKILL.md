---
name: launch-strategy
description: Plan and execute product launches, feature announcements, and campaign rollouts. Use when launching a new product, announcing a major feature, planning a partnership reveal, or celebrating a milestone — covers pre-launch hype, launch day coordination, post-launch follow-up, and success metrics.
---

# Launch Strategy

Plan and execute product launches, feature announcements, and campaigns that get noticed by the right people at the right time.

---

## When to Use

Trigger this skill when you hear:
- "I'm launching a new product"
- "We have a feature release coming up"
- "Help me plan a launch"
- "We need a go-to-market plan"
- "How should we announce this?"
- "We're launching a partnership"
- "We hit a milestone and want to make noise about it"
- "Plan a campaign for our new thing"

This skill handles everything from pre-launch hype to post-launch follow-up. It gives you a timeline, deliverables, and success metrics — not just ideas.

---

## Context Gathering

Ask these questions conversationally, not all at once. Adapt based on what the user tells you.

### What's Launching
1. **What are you launching?** (New product, major feature, minor update, partnership, milestone, rebrand?)
2. **What does it do for the customer?** (The benefit, not the feature list)
3. **When do you want to launch?** (Hard date or flexible?)

### Audience
4. **Who needs to know about this?** (Existing users, new prospects, press, partners, investors?)
5. **What should they do when they hear about it?** (Sign up, upgrade, share, buy, join a waitlist?)
6. **Is there an existing audience you can reach directly?** (Email list size, social following, community, partner audiences?)

### Scale & Resources
7. **How big is this launch?** (Major event or quiet rollout? Scale of effort you can put in?)
8. **What channels do you have access to?** (Email, social, blog, PR contacts, paid ads budget, partnerships?)
9. **Who's involved on your team?** (Just you? Marketing team? Design? Engineering for demos?)

### Competitive Timing
10. **Is there anything happening in your market around the same time?** (Competitor launches, industry events, seasonal trends?)

---

## Methodology

### Step 1: Check Product Marketing Context
Before planning any launch, pull in the marketing foundation:
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run **product-marketing-context** first. You need positioning, customer profile, and brand voice before you can plan a launch that lands.

### Step 2: Define the Launch Type

Classify the launch to set the right scope and expectations:

| Launch Type | Scope | Timeline | Example |
|---|---|---|---|
| **New Product** | Full campaign, all channels | 4-8 weeks | First public release |
| **Major Feature** | Multi-channel push | 2-4 weeks | New capability that changes the value prop |
| **Minor Update** | Targeted announcement | 1-2 weeks | Quality-of-life improvement |
| **Partnership** | Co-marketing campaign | 2-4 weeks | Integration or collaboration announcement |
| **Milestone** | Celebration + credibility play | 1-2 weeks | 10K users, funding round, anniversary |

Match the effort to the impact. Not everything needs a full launch. Ask the user: *"Based on what you've described, this feels like a [type] launch. Does that match your expectations?"*

### Step 3: Pre-Launch Planning

Build anticipation before launch day. Choose tactics based on launch type and available channels.

**Teaser Content (1-3 weeks before)**
- Social posts hinting at what's coming ("Something big is brewing...")
- Behind-the-scenes content (building in public, sneak peeks)
- Countdown or "coming soon" updates

**Waitlist / Early Access (2-4 weeks before)**
- Landing page to capture interest
- Email sequence for waitlist signups
- Exclusive early access for loyal customers or community members

**Press & Influencer Outreach (2-3 weeks before)**
- Identify relevant journalists, bloggers, or influencers in the space
- Craft a pitch — not a press release, a story angle they'd want to cover
- Offer exclusives or early access for coverage

**Internal Alignment (1-2 weeks before)**
- Brief the team — everyone should know the talking points
- Prepare support/sales for incoming questions
- Stage all content and assets for launch day

### Step 4: Launch Day Execution

Coordinate a synchronized announcement across channels. Everything goes live within the same window.

**Core Announcement**
- Blog post or landing page with the full story (what, why, who it's for)
- Email to full list (or segmented based on relevance)
- Social posts across all active platforms

**Supporting Content**
- Demo video or walkthrough (keep it short — under 2 minutes)
- Customer quote or early-access testimonial if available
- FAQ or "what this means for you" breakdown for existing users

**Amplification**
- Ask team members, advisors, and partners to share
- Post in relevant communities (but add value, don't spam)
- Respond to every comment and question quickly — launch day engagement matters

### Step 5: Post-Launch Follow-Up

The launch isn't over on launch day. The week after is where you compound the momentum.

**Days 1-3: Ride the Wave**
- Share engagement metrics and social proof ("500 signups in 24 hours")
- Reshare user reactions and positive comments
- Publish a "behind the launch" or lessons-learned post

**Days 4-7: Deepen the Story**
- Publish user stories or case studies from early adopters
- Create how-to content showing the new thing in action
- Run a follow-up email for people who opened but didn't act

**Week 2+: Measure and Learn**
- Review all success metrics (see Step 7)
- Identify what worked and what fell flat
- Document learnings for the next launch

### Step 6: Build the Timeline

Create a concrete timeline with deliverables and owners. Adjust based on launch type.

**Example: Major Feature Launch (3-week timeline)**

```
Week 1 (Pre-Launch)
├── Mon: Finalize messaging and positioning
├── Tue: Draft blog post and email copy
├── Wed: Create social content calendar
├── Thu: Design visual assets (graphics, screenshots, demo)
├── Fri: Begin teaser posts on social

Week 2 (Build-Up)
├── Mon: Send early-access invites to top customers
├── Tue: Pitch press/influencers with story angle
├── Wed: Schedule all launch-day posts and emails
├── Thu: Internal briefing — align team on talking points
├── Fri: Final review of all assets and go/no-go check

Week 3 (Launch + Follow-Up)
├── Mon: LAUNCH DAY — coordinated announcements go live
├── Tue: Engage with responses, share early traction
├── Wed: Publish user reactions and social proof
├── Thu: Follow-up email to non-openers
├── Fri: Metrics review and post-launch retro
```

Present the timeline and ask: *"Does this timeline work with your team and resources? What needs to shift?"*

### Step 7: Define Success Metrics

Every launch needs numbers to measure against. Define these before launch day.

**Awareness Metrics**
- Impressions and reach across channels
- Press mentions or influencer coverage
- Social shares and engagement rate

**Acquisition Metrics**
- New signups, waitlist adds, or trial starts
- Landing page conversion rate
- Email click-through rate

**Activation Metrics**
- Feature adoption rate (for feature launches)
- First-use or onboarding completion
- Upgrade or purchase rate

**Engagement Metrics**
- Social comments and DMs
- Support tickets (positive signal if people are trying it)
- Community discussions

Ask the user: *"What does a successful launch look like to you? Give me a number — signups, revenue, press hits, whatever matters most."*

---

## Output Format

The launch strategy document follows this structure:

```markdown
# Launch Strategy: [Name of Launch]
Date: [Launch Date]
Type: [New Product / Major Feature / Minor Update / Partnership / Milestone]

## What We're Launching
[One-paragraph summary of what's launching and why it matters to the customer]

## Target Audience
**Primary:** [Who must know about this]
**Secondary:** [Who would benefit from knowing]
**Call to action:** [What we want them to do]

## Key Messages
1. [Main headline / value statement]
2. [Supporting point — the problem this solves]
3. [Proof point — credibility or social proof]

## Pre-Launch (Weeks before)
| Date | Deliverable | Channel | Owner |
|------|-------------|---------|-------|
| [Date] | [Teaser post] | [Social] | [Name] |
| [Date] | [Waitlist page live] | [Website] | [Name] |
| [Date] | [Press pitch sent] | [Email] | [Name] |

## Launch Day
| Time | Deliverable | Channel | Owner |
|------|-------------|---------|-------|
| [AM] | [Blog post live] | [Blog] | [Name] |
| [AM] | [Email blast sent] | [Email] | [Name] |
| [AM] | [Social posts go live] | [All platforms] | [Name] |

## Post-Launch (Week after)
| Date | Deliverable | Channel | Owner |
|------|-------------|---------|-------|
| [Day 2] | [Share traction metrics] | [Social] | [Name] |
| [Day 3] | [User story published] | [Blog] | [Name] |
| [Day 5] | [Follow-up email] | [Email] | [Name] |
| [Day 7] | [Metrics review] | [Internal] | [Name] |

## Success Metrics
| Metric | Target | How We'll Measure |
|--------|--------|-------------------|
| [Signups] | [500] | [Analytics dashboard] |
| [Email CTR] | [15%] | [Email platform] |
| [Social reach] | [50K] | [Social analytics] |

## Assets Needed
- [ ] Blog post / announcement copy
- [ ] Email copy (announcement + follow-up)
- [ ] Social posts (platform-specific)
- [ ] Visual assets (graphics, screenshots, video)
- [ ] Landing page or updated product page
- [ ] Press pitch / media kit (if applicable)
- [ ] Internal talking points
```

---

## Quality Checks

Before delivering the launch strategy, verify:

- [ ] **Launch type matches the effort** — don't plan a 6-week campaign for a minor update
- [ ] **Messaging ties back to positioning** — the announcement reinforces the product's unique angle, not generic hype
- [ ] **Timeline is realistic** — check against the user's team size and resources
- [ ] **Every deliverable has an owner** — if it's just one person, the plan should reflect that
- [ ] **Call to action is clear** — every piece of content points to one specific action
- [ ] **Channels match the audience** — don't plan a LinkedIn campaign for a B2C consumer product
- [ ] **Success metrics are defined before launch** — not decided after the fact
- [ ] **Post-launch plan exists** — launch day isn't the end, it's the middle
- [ ] **No jargon unless the user used it first** — speak their language

If any check fails, revise the strategy before presenting it.

---

## Examples

### Example 1: New Product Launch

**User says:** "We're launching our product publicly next month. Help me plan the launch."

**You do:**
1. Check memory for product marketing context (found — B2B SaaS, email automation for sales teams)
2. Ask: "Tell me about the launch — is this the first time the product is available to everyone?"
3. User: "Yes, we've been in private beta with 200 users. Now we're opening up."
4. Ask: "What's your launch date? And what do you want people to do — sign up for free, start a trial?"
5. User: "March 15th. We want free trial signups."
6. Ask: "What channels can you use? Email list, social, any press contacts?"
7. User: "We have a 2,000-person email list, active on Twitter and LinkedIn. No press contacts."
8. Classify as **New Product** launch, plan 4-week timeline
9. Build pre-launch plan: teaser content on social, early-access email to beta users for testimonials, waitlist landing page
10. Build launch day plan: coordinated blog + email + social announcement
11. Build post-launch plan: share signup numbers, publish beta user stories, follow-up email
12. Define metrics: 500 trial signups in first week, 10% email CTR, 20K social impressions
13. Present full strategy with timeline and deliverables
14. User confirms, adjusts one date
15. Finalize and deliver

### Example 2: Minor Feature Update

**User says:** "We just added dark mode. Should we announce it?"

**You do:**
1. Check memory for context (found)
2. Say: "Dark mode is a nice quality-of-life feature. This is a minor update — I'd recommend a lightweight announcement, not a full campaign. Here's what I'd do."
3. Plan a 1-week effort: social post, in-app notification, brief mention in next newsletter
4. No press outreach, no waitlist, no multi-week buildup
5. Suggest a fun angle: "Your eyes called. They said thank you." — lean into the personality
6. Deliver a simple plan with 3-4 deliverables

### Example 3: Partnership Announcement

**User says:** "We just integrated with Stripe. How do we announce this?"

**You do:**
1. Check memory for context (found)
2. Ask: "Is Stripe doing any co-marketing with you, or is this your announcement?"
3. User: "They'll share it on their integrations page but we're driving the announcement."
4. Classify as **Partnership** launch
5. Plan co-marketing angle: leverage Stripe's brand credibility, target users who already use Stripe
6. Build 2-week plan: teaser, joint blog post, social campaign highlighting the integration benefit
7. Suggest reaching out to Stripe's community channels
8. Define metrics: integration activations, new signups from Stripe referral traffic

---

## Related Skills

- **product-marketing-context** — must exist before planning any launch (positioning, customer, voice)
- **copywriting** — writes the announcement copy, blog posts, and landing page text
- **social-content** — creates platform-specific posts for pre-launch, launch day, and follow-up
- **email-sequence** — designs the launch email series (teaser, announcement, follow-up)
- **paid-ads** — amplifies launch reach with targeted ad spend
