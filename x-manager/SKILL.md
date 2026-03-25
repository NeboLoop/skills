---
name: x-manager
description: Manage X/Twitter accounts — post tweets, read timelines, search, monitor mentions, manage followers, and schedule content. Use when working with X/Twitter for social media management, brand monitoring, or content distribution.
---

# X Manager

Manage one or more X/Twitter accounts from Nebo. Post, read, search, monitor, and analyze — all via the X API.

---

## When to Use

- "Post this to X"
- "What's trending on Twitter?"
- "Check my mentions"
- "Schedule a tweet for tomorrow"
- "Search X for [topic]"
- "Who followed me today?"
- "Monitor mentions of [brand]"
- "Post a thread"
- "Reply to this tweet"
- "Like and retweet this"
- Any time the Social Media Manager role needs X integration

---

## How It Works

The X Manager is a compiled binary at `scripts/x-manager`. It handles all X API communication so the LLM doesn't need to manage OAuth tokens or API calls directly.

```
scripts/x-manager <command> [options]
```

### Authentication

Before using, the user must configure their X API credentials:

```
scripts/x-manager auth --api-key <KEY> --api-secret <SECRET> --access-token <TOKEN> --access-secret <TOKEN_SECRET>
```

Credentials are stored encrypted in `~/.config/nebo/x-manager/credentials.json`. Supports multiple accounts:

```
scripts/x-manager auth --account work --api-key <KEY> ...
scripts/x-manager auth --account personal --api-key <KEY> ...
```

### Commands

#### Posts

```bash
# Post a tweet
scripts/x-manager post --text "Hello world"

# Post with media
scripts/x-manager post --text "Check this out" --media /path/to/image.png

# Post a thread (pipe newline-separated tweets)
scripts/x-manager thread --text "1/ First tweet\n---\n2/ Second tweet\n---\n3/ Third tweet"

# Reply to a tweet
scripts/x-manager reply --to <tweet-id> --text "Great point!"

# Delete a tweet
scripts/x-manager delete --id <tweet-id>

# Schedule a post (stores locally, posts at specified time)
scripts/x-manager schedule --text "Morning everyone!" --at "2026-03-26T09:00:00"

# List scheduled posts
scripts/x-manager schedule --list

# Post from a specific account
scripts/x-manager post --account work --text "Company update"
```

#### Reading & Search

```bash
# Get your timeline
scripts/x-manager timeline --limit 20

# Get a user's recent posts
scripts/x-manager user-posts --username elonmusk --limit 10

# Search recent tweets (last 7 days)
scripts/x-manager search --query "nebo AI agent" --limit 20

# Get a specific tweet
scripts/x-manager get --id <tweet-id>

# Get trending topics
scripts/x-manager trends --location 1  # 1 = Worldwide WOEID
```

#### Mentions & Monitoring

```bash
# Get your mentions
scripts/x-manager mentions --limit 20

# Monitor mentions in real-time (polls every N seconds)
scripts/x-manager monitor --query "@neboloop OR neboloop" --interval 60

# Monitor and output as JSON (for piping to other tools)
scripts/x-manager monitor --query "neboloop" --interval 60 --json
```

#### Engagement

```bash
# Like a tweet
scripts/x-manager like --id <tweet-id>

# Retweet
scripts/x-manager retweet --id <tweet-id>

# Quote tweet
scripts/x-manager quote --id <tweet-id> --text "This is interesting because..."

# Bookmark a tweet
scripts/x-manager bookmark --id <tweet-id>

# Get your bookmarks
scripts/x-manager bookmarks --limit 20
```

#### Followers & Following

```bash
# Get your followers
scripts/x-manager followers --limit 50

# Get who you follow
scripts/x-manager following --limit 50

# Follow a user
scripts/x-manager follow --username <username>

# Unfollow a user
scripts/x-manager unfollow --username <username>

# Get new followers since last check
scripts/x-manager followers --new
```

#### Analytics

```bash
# Get engagement stats for your recent posts
scripts/x-manager analytics --days 7

# Get stats for a specific post
scripts/x-manager analytics --id <tweet-id>
```

### Output Format

All commands output JSON by default:

```json
{
  "success": true,
  "data": {
    "id": "1234567890",
    "text": "Hello world",
    "created_at": "2026-03-25T10:00:00Z",
    "metrics": {
      "likes": 5,
      "retweets": 2,
      "replies": 1,
      "impressions": 450
    }
  }
}
```

Use `--format text` for human-readable output.

---

## Quality Checks

- [ ] Credentials stored encrypted, never logged or output
- [ ] Rate limits respected (backs off on 429 responses)
- [ ] Media uploads validate file type and size before sending
- [ ] Thread posts maintain correct reply chain
- [ ] Scheduled posts persist across restarts
- [ ] Multi-account support isolates credentials per account
- [ ] All API errors return structured JSON with actionable messages

---

## Related Skills

- **social-content** — generates the content, x-manager posts it
- **web-scraper** — scrapes pages that get shared as tweets
- **deep-research** — researches topics that become threads
- **copywriting** — writes the copy, x-manager distributes it
- **product-marketing-context** — ensures posts align with brand voice
