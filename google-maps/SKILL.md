---
name: google-maps
description: Find places, compare them, read their reviews and get directions on Google Maps in the browser — the best burger nearby, a dentist open Saturday, how long the drive is. Use when someone asks about places, ratings, hours, or how to get somewhere.
---

# Google Maps

Answer place questions from what Google Maps actually shows, in the
person's own browser tab, and hand back links they can tap.

## When to use

- "Find the best burger near me" / "a dentist open Saturday in Provo"
- "What do people say about Bombay House?"
- "How long to drive from the office to the airport at 5?"
- "Compare these three plumbers"

## Open Maps with a URL, not by typing

Go straight to the result with `browser_open` instead of clicking the
search box:

- Search: `https://www.google.com/maps/search/?api=1&query=<words, url-encoded>`
  — put the place in the words: `best burger Provo UT`, `dentist near 84604`.
- Directions: `https://www.google.com/maps/dir/?api=1&origin=<from>&destination=<to>&travelmode=driving`
  (`walking`, `transit`, `bicycling`). Leave `origin` out to use the person's location.
- One place: open its result, then use the page's URL as the link you give back.

"Near me": use the location the person shared. If you don't have one,
ask for the city or zip — never guess one.

## Read the results

1. `browser_read` the results list (the left panel). Note each place's
   name, star rating, number of reviews, price level and category exactly
   as shown.
2. Open the top few (five at most unless asked for more) one at a time and
   read: hours (and whether it's open now), address, phone, website.
3. For reviews, open the Reviews tab. Quote short snippets word for word,
   and say how many reviews the rating rests on.
4. Stop when you can answer. Don't wander through every result.

## Report

- A short ranked list: name — rating (review count) — price — one line
  on why — open/closed now — the Maps link.
- Only what you saw on the page. If hours or a price aren't shown, say
  "not listed" — never fill a gap from memory.
- Pictures: if the person wants to see a place, take a screenshot of it
  and share it in the chat.
- Directions: distance, time with current traffic, and the route name,
  plus the directions link.

## Don't

- Sign in, write or edit reviews, or suggest edits to a place.
- Book, order or call anything unless the person asked for exactly that.
- Solve a "verify you're human" check. Tell the person and stop.
