---
name: marketing-ideas
description: Generate marketing ideas tailored to your product, stage, and goals. Use when you need fresh tactics to grow, are stuck on what to try next, or want a prioritized list of marketing experiments matched to your specific funnel bottleneck, budget, and team size.
---

# Marketing Ideas

Generate marketing ideas tailored to your product, stage, and goals — so you always know what to try next.

---

## When to Use

Trigger this skill when you hear:
- "Give me marketing ideas"
- "What should I try next for marketing?"
- "I'm stuck on how to grow"
- "How do I get more customers?"
- "What marketing tactics should I use?"
- "I need ideas for [awareness / signups / retention / revenue]"
- "What's working for companies like mine?"

This skill is for when you know what you're marketing but need ideas for how to market it. If you don't have product-marketing-context saved yet, that skill runs first.

---

## Context Gathering

Before generating ideas, you need to understand three things:

### 1. Current Situation
Pull from memory first, then confirm:
- **What's your product and who's it for?** (Should already exist in product-marketing-context)
- **What stage are you at?** (Pre-launch, just launched, growing, established)
- **What's your budget like?** (Bootstrapped/tight, moderate, well-funded)
- **How big is your team?** (Just you, small team, dedicated marketing person)

### 2. Current Bottleneck
Ask: **"Where are you getting stuck right now?"** and listen for clues about which part of the funnel needs attention.

### 3. What You've Already Tried
Ask: **"What have you already tried, and what happened?"** — so you don't suggest things they've done or things that flopped.

---

## Methodology

### Step 1: Check Product Marketing Context

Pull existing context from memory so you understand what they're working with:

```
agent(resource: "memory", action: "search", query: "product marketing context")
```

If no context exists, run the **product-marketing-context** skill first. You need to understand the product, customer, positioning, and goals before generating useful ideas.

If context exists, quickly confirm: *"I see you're [product description] targeting [customer]. Your goals are [goals]. Is that still accurate?"*

### Step 2: Identify the Funnel Bottleneck

Every business has a funnel — the journey from stranger to loyal customer. Your job is to figure out where things are breaking down. Walk through each stage with the user:

1. **Awareness** — Do people know you exist? Are you getting in front of the right audience? If nobody's heard of you, this is the bottleneck.

2. **Acquisition** — Are people visiting your site or landing page? If people know about you but aren't showing up to learn more, this is the bottleneck.

3. **Activation** — Are visitors taking the first meaningful action (signing up, starting a trial, booking a call)? If people visit but don't engage, this is the bottleneck.

4. **Revenue** — Are activated users paying you? If people try your product but don't buy, this is the bottleneck.

5. **Retention** — Are paying customers sticking around? If people buy once but leave, this is the bottleneck.

6. **Referral** — Are happy customers telling others about you? If customers love you but nobody else hears about it, this is the bottleneck.

Ask the user: *"Let me walk through your customer journey quickly. Where do you think things are breaking down — getting noticed, getting people to your site, getting them to try it, getting them to pay, keeping them around, or getting them to spread the word?"*

Most users will have an intuitive answer. If they're unsure, ask about their numbers at each stage and look for the biggest drop-off.

### Step 3: Research What Competitors Are Doing

Use Nebo's browser to look at what competitors or similar companies are doing for marketing:

```
web(action: "navigate", url: "competitor-website.com")
web(action: "read_page")
```

Check their homepage messaging, blog strategy, social presence, and any visible campaigns. This gives you raw material for ideas that are proven in the market.

### Step 4: Generate 10 Ideas for the Bottleneck

Generate 10 marketing ideas specifically targeting the identified bottleneck. For each idea, categorize it by:

**Effort level:**
- **Easy** — You can do this today or this week with no special tools or budget
- **Medium** — Takes a few weeks, might need a small budget or a specific tool
- **Hard** — A bigger project that takes a month or more, needs meaningful investment

**Expected impact:**
- **Low** — Incremental improvement, worth doing but won't change the game
- **Medium** — Noticeable improvement, moves the needle
- **High** — Could significantly change your trajectory

For each idea, provide:
1. **One-sentence description** — What the idea is, plainly stated
2. **Why it works for your situation** — Connect it to their specific product, customer, and stage
3. **First step to execute** — The single next action to get started

### Step 5: Recommend Top 3

From the 10 ideas, recommend the top 3 to start with. Choose based on:
- Best ratio of impact to effort
- Fits their current resources (team size, budget, skills)
- Builds on what's already working
- Creates compounding results (each one makes the next one easier)

Explain why you picked these three and suggest an order to tackle them.

---

## Output Format

Present ideas in this structure:

```markdown
# Marketing Ideas for [Product Name]
**Bottleneck:** [Stage] — [Brief description of the problem]
**Generated:** [DATE]

---

## All 10 Ideas

### 1. [Idea Name]
**Effort:** [Easy/Medium/Hard] | **Impact:** [Low/Medium/High]

[One-sentence description of the idea.]

**Why this works for you:** [Connection to their specific situation — product, customer, stage, goals.]

**First step:** [The single next action to get started.]

---

### 2. [Idea Name]
...

---

## Top 3 Recommendations

### Start with these, in this order:

**1. [Idea Name]** — [Why this one first. What makes it the best starting point given their resources and situation.]

**2. [Idea Name]** — [Why this one second. How it builds on the first.]

**3. [Idea Name]** — [Why this one third. How it compounds with the first two.]

### Timeline
- **This week:** [What to do for idea 1]
- **Next 2 weeks:** [What to do for idea 2]
- **This month:** [What to do for idea 3]
```

---

## Quality Checks

Before presenting ideas, verify:

- [ ] **Ideas target the identified bottleneck** — not scattered across the whole funnel
- [ ] **Ideas match their stage** — don't suggest enterprise tactics to a solo founder, don't suggest "launch on Product Hunt" to an established company
- [ ] **Ideas match their budget** — don't suggest paid ads to someone who's bootstrapped with no budget
- [ ] **Ideas match their team** — don't suggest "hire a content team" to a one-person shop
- [ ] **Each idea has a clear first step** — not vague advice like "do content marketing"
- [ ] **Ideas are specific to their product and customer** — not generic marketing advice you'd find in any blog post
- [ ] **Effort and impact ratings are honest** — don't rate everything as "easy, high impact"
- [ ] **Top 3 recommendations have clear reasoning** — not just "these are the best"
- [ ] **No jargon** — if you must use a marketing term, explain it in plain language
- [ ] **Competitor research was used** — ideas should reflect what's actually happening in their market

---

## Examples

### Example: Early-Stage B2B SaaS — Proposal Software

**Context from memory:**
- Product: ProposalFlow — helps freelancers and small agencies create professional proposals in minutes
- Customer: Freelance designers and small design agencies (2-10 people)
- Stage: Just launched, 50 free users, 5 paying customers
- Pricing: Free tier with 3 proposals/month, $29/month for unlimited
- Budget: Bootstrapped, under $500/month for marketing
- Goals: Get to 100 paying customers in 6 months
- What's working: A few Reddit posts drove signups
- What hasn't worked: Cold outreach on LinkedIn got zero responses

**Bottleneck identified:** Awareness — most of their target audience has never heard of ProposalFlow. The 50 free users came from a couple of Reddit posts that happened to take off. There's no consistent way new people are discovering the product.

**Generated ideas:**

---

**1. Create a Free Proposal Template Gallery**
**Effort:** Medium | **Impact:** High

Build a public page with 10-15 beautiful, ready-to-use proposal templates that freelancers can browse and customize in ProposalFlow.

**Why this works for you:** Freelance designers are constantly searching for "freelance design proposal template" and similar terms. A template gallery brings them to your site through search and gives them an immediate reason to sign up.

**First step:** List the 10 most common project types your users create proposals for (website redesign, brand identity, etc.) and create one polished template for each.

---

**2. Write "How I Won a $10K Client With a Better Proposal" for Freelance Communities**
**Effort:** Easy | **Impact:** Medium

Write a genuine story (yours or a user's) about how a well-crafted proposal helped win a significant project. Share it on Reddit (r/freelance, r/design), Indie Hackers, and freelance Slack groups.

**Why this works for you:** Reddit already works for you. This doubles down on that channel with a story format that freelancers love — real numbers, real results. It doesn't feel like an ad.

**First step:** Reach out to your 5 paying customers and ask if any of them won a project they can attribute to a better proposal. Get their story.

---

**3. Partner with Freelance Newsletter Authors**
**Effort:** Medium | **Impact:** High

Find 5-10 newsletters that freelance designers subscribe to (like The Futur, Freelance Friday, etc.) and offer them a free sponsorship or guest article swap.

**Why this works for you:** Your audience already reads these newsletters. A recommendation from a trusted newsletter author carries more weight than any ad. At your price point ($29/month), even a small newsletter can drive meaningful signups.

**First step:** Ask your current users what newsletters or blogs they read about freelancing. Make a list of 10.

---

**4. Build a "Proposal Pricing Calculator" Free Tool**
**Effort:** Hard | **Impact:** High

Create a simple, free web tool where freelancers enter their project type, hours, and rate — and it generates a recommended price range based on market data. Embed a "Create this proposal in ProposalFlow" button.

**Why this works for you:** Pricing is the #1 anxiety for freelancers writing proposals. A free tool that solves that pain gets shared widely and puts your product in front of people at the exact moment they need it.

**First step:** Survey your existing users about how they decide what to charge. Use their answers to define the calculator's logic.

---

**5. Answer Every "How Do I Write a Proposal?" Question Online**
**Effort:** Easy | **Impact:** Medium

Set up alerts for proposal-related questions on Reddit, Quora, and freelance forums. When someone asks about writing proposals, give a genuinely helpful answer and mention ProposalFlow naturally.

**Why this works for you:** You already proved Reddit works. This systematizes it instead of relying on occasional posts. Each answer is a tiny asset that keeps driving traffic for months.

**First step:** Search Reddit for "freelance proposal" and "how to write a proposal" — bookmark the top 10 threads and write helpful responses today.

---

**6. Create a 5-Day "Win Better Clients" Email Course**
**Effort:** Medium | **Impact:** Medium

Build a free email course that teaches freelancers how to write proposals that win higher-value projects. Each day covers one element (pricing, case studies, scope, timeline, the close). Day 5 introduces ProposalFlow.

**Why this works for you:** Email courses convert well for tools because they educate first. Freelancers who take the course are pre-sold on the value of good proposals before they ever see your product.

**First step:** Outline the 5 days. Each one should teach something valuable even if they never use your tool.

---

**7. Launch on Product Hunt with a "Proposals That Won" Angle**
**Effort:** Medium | **Impact:** Medium

Launch on Product Hunt, but instead of leading with features, lead with real examples of proposals that won projects — and how ProposalFlow makes it easy to create them.

**Why this works for you:** Product Hunt drives a spike of early-adopter signups. The "proposals that won" angle makes it interesting even to people who don't need proposal software right now.

**First step:** Set a launch date 3 weeks out. Start collecting real proposal examples from your users (anonymized) to feature in the launch.

---

**8. Guest Post on Design Blogs About the Business Side of Freelancing**
**Effort:** Medium | **Impact:** Low

Write articles for popular design blogs (Smashing Magazine, Creative Boom, Dribbble stories) about the business side of freelancing — proposals, pricing, client management.

**Why this works for you:** Designers read these sites for inspiration but rarely see business advice there. Your articles stand out because they fill a gap, and they position ProposalFlow as a tool built by someone who understands freelance designers.

**First step:** Pitch one article to Smashing Magazine about "5 Proposal Mistakes That Cost Freelance Designers Money."

---

**9. Create Short Video Tutorials Showing Real Proposal Builds**
**Effort:** Easy | **Impact:** Low

Record 2-3 minute screen recordings of yourself building a real proposal in ProposalFlow. Post them on YouTube, TikTok, and LinkedIn.

**Why this works for you:** Freelancers are visual learners (especially designers). Seeing a proposal come together in minutes is more convincing than any landing page copy.

**First step:** Pick your best-looking template, hit record, and narrate as you customize it for a fictional client project. Keep it under 3 minutes.

---

**10. Offer Existing Users a Referral Incentive**
**Effort:** Easy | **Impact:** Medium

Give your 50 existing users a reason to invite other freelancers. Offer an extra month free (or extra proposals on the free tier) for every referral who signs up.

**Why this works for you:** Your users are freelancers who know other freelancers. A simple referral program turns your small user base into a sales team. At 50 users, even a 20% referral rate adds 10 new signups.

**First step:** Email your users: "Know a freelancer who hates writing proposals? Send them to ProposalFlow and you both get a free month."

---

**Top 3 Recommendations:**

**1. Answer Every Proposal Question Online (Idea 5)** — Start here because it costs nothing, takes 30 minutes a day, and you already know Reddit works for you. This turns a lucky break into a repeatable system. Every answer you write keeps working for months.

**2. Create a Free Proposal Template Gallery (Idea 1)** — Do this next because it creates a permanent traffic source through search. People searching for "freelance proposal template" are exactly your customers, and a template gallery gives them a reason to sign up immediately. This takes more effort but compounds over time.

**3. Offer Existing Users a Referral Incentive (Idea 10)** — Layer this on third because it's easy to set up and activates your existing users while the first two ideas bring in new ones. Referrals from fellow freelancers convert better than any other source.

**Timeline:**
- **This week:** Set up Reddit and Quora alerts, write 10 helpful answers about proposals
- **Next 2 weeks:** Build the template gallery with 10 templates, publish it as a public page
- **This month:** Launch referral program to existing users, email everyone about it

---

## Related Skills

- **launch-strategy** — plans a structured launch to maximize first impressions
- **free-tool-strategy** — designs free tools that attract your ideal customers
- **referral-program** — builds a referral system that turns users into advocates
- **social-content** — creates social posts that match your brand voice
- **product-marketing-context** — captures and maintains everything about your product, customer, and positioning (this skill always checks context first)
