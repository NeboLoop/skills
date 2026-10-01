# The Nebo App (Phase 4 and publishing)

The bundled `build-an-app` skill is the source of truth for creating an app
employee and for the SDK contract. This file is what a designed, built app
adds on top.

## The app folder

Make the app employee first, one of two ways (a one-file starter page is
enough either way):

- **Yourself**, when the owner puts the app on you ("you are the app"):
  `update_employee(name: "<your own name>", app: {...}, ui: {"index.html": ...})`.
  You stay yourself; the owner can talk to you while the page reloads, and
  you can publish yourself at the end.
- **A new employee**, when the app is a separate thing or another
  employee's job: `create_employee` (see the bundled `build-an-app` skill). The result gives its id; `app_status` and the line every
teammate reads ("<name> is an app; its files are served from `<path>`")
give the folder the page is served from. Everything lives in that one app
folder:

```
<app folder>/
├── AGENT.md         # persona; frontmatter must say artifact_type: app
├── manifest.json    # "type": "app", window, permissions
├── agent.json       # config ({} is fine)
├── brief.md         # Phase 1 (never ships)
├── refs/            # boards and rejected candidates (never ship)
│   (boards from generate_media land in ui/boards/; delete before publish)
├── src/             # source you write
├── package.json     # build deps (never ship)
└── ui/              # what Nebo serves and what publishes
    ├── index.html
    ├── main-<hash>.js
    ├── app.css
    └── assets/      # images, film, models, sound, fonts
```

Never build in a copy somewhere else and never keep two copies. A page
edited in one folder while the app is served from another never changes on
screen.

Small hand-written pages can keep going through `update_employee(ui: ...)`
as `build-an-app` says. A built app writes its build output straight into
`ui/`, because `ui` in that tool carries text, not film or models.

## AGENT.md

```markdown
---
name: harbor-light
description: "A scroll-driven story of the harbor at dawn: the boats, the catch, and the people who bring it in."
artifact_type: app
metadata:
  version: "0.1.0"
---
# Harbor Light

You work beside the Harbor Light app. ...
```

- `artifact_type: app` in the frontmatter. Without it the marketplace makes
  a plain employee with no page.
- **Quote any value that contains `: `** (colon then space). An unquoted
  `description: Track deals: fast` is broken YAML, and the marketplace then
  silently treats the app as a plain employee. Quoting every description is
  the simple habit.

## manifest.json

```json
{
  "id": "harbor-light",
  "name": "Harbor Light",
  "version": "0.1.0",
  "type": "app",
  "description": "A scroll-driven story of the harbor at dawn.",
  "permissions": ["storage:readwrite"],
  "window": { "title": "Harbor Light", "width": 1024, "height": 768, "resizable": true }
}
```

- The key is `"type"`, never `"artifact_type"`, in manifest.json.
- `window` accepts `title`, `width`, `height`, `resizable`, `fullscreen`,
  `orientation` (`portrait` default, `landscape`, `any`). A game or a film
  that wants the whole phone screen sets `fullscreen: true` and pads itself
  with `env(safe-area-inset-*)` (and `viewport-fit=cover` in the viewport
  meta). The close button sits in the top-left corner; keep controls clear
  of it.
- `permissions` use `prefix:scope`: `storage:readwrite`, `subagent:<id>`,
  `network:<host>` or `network:*`, `device:motion` (tilt). Ask for the
  least the app needs.

## The SDK

Load `<script src="/sdk/nebo.global.js"></script>` before your code. It
puts ONE global on the page, `NeboAppSDK`; the singleton is
`NeboAppSDK.nebo`. There is no bare `nebo` global.

```js
const { nebo } = window.NeboAppSDK;
await nebo.storage.setItem('progress', { level: 3, best: 1240 }); // JSON in, JSON out
const saved = await nebo.storage.getItem('progress');
const me = await nebo.identity.get();          // the app's employee: id, name, ...
const { text } = await nebo.agents.invoke('Write a caption for this photo', { data: { id } });
const line = await nebo.janus.complete({ messages: [{ role: 'user', content: '...' }] });
```

The full table (chat, surfaces, the employee socket, the proxy fetch,
`storage.onChange`) is in `build-an-app`. Keep every bit of state the owner cares about (progress,
settings, saves) in `storage`; the window is closed and reopened, and the
page is reloaded on every rebuild.

In a bundled build, read it at runtime (`window.NeboAppSDK`); do not import
a package for it.

## The app's data is yours too

The page's `storage` and your `app_data` tool are one store: the same keys,
the same values. You have `app_data` because you are an app; it reaches only
your own app's data, and a coworker who needs it asks you.

```
app_data(action: "set",    key: "contacts", value: [{ "name": "John Smith", "phone": { "mobile": "+1 555 0100" } }])
app_data(action: "get",    key: "contacts")
app_data(action: "query",  where: { "name": "john smith" })          // field contains, any case; inside lists item by item
app_data(action: "query",  where: { "phone.mobile": "0100" })        // dotted paths
app_data(action: "query",  text: "smith", prefix: "contact:", limit: 5)
app_data(action: "list",   prefix: "contact:")
app_data(action: "delete", key: "draft")
```

- Load it with `find_tools` if it is not in your list yet.
- `limit` defaults to 20, at most 100; `total` in the answer counts every
  match, so you know when you saw only part.
- `text` matches the record's whole JSON text (field names included), as
  one phrase.
- After a `set` or `delete` every open view of the app hears it. Write the
  page so it redraws:

```js
async function load() { render((await nebo.storage.getItem('contacts')) ?? []); }
load();
nebo.storage.onChange((c) => { if (c.keys.includes('contacts')) load(); });
// c = { appId, keys, action: 'set' | 'delete', source: 'employee' | 'page' }
```

- Pick keys you and the page both find: one key holding a list
  (`contacts`), or one key per record under a prefix (`contact:<id>`). Write
  the shape into your own instructions so later turns use the same keys.
- `getItem` returns exactly what `setItem` stored, except that a string which
  is itself JSON (`"42"`, `"true"`) comes back parsed. Store such values
  inside an object.

Judgments over that data: your `decide` tool (load it with `find_tools`)
takes the same request as the page's `nebo.decide`. Read records with
`app_data`, then ask typed questions about them in one fast call:

```
decide(state: <records from app_data>, questions: {
  "tier":  { "type": "choice", "instructions": "How warm is this lead, judging by `status`?",
             "criteria": { "hot": "ready to buy", "warm": "interested", "cold": "not now", "other": "can't tell" } },
  "reply": { "type": "noul", "instructions": "`status` asks us for a reply." } })
```

- `choice`: 2 to 255 named options, with an escape option when the list is
  not complete; `score`: 2 to 10 ordered levels, lowest first, answered as a
  fraction; `noul`: one statement, answered as the probability it holds.
- The whole question lives in `instructions`; name the state's fields in
  backticks and keep the state to what the questions need (very long state
  is shortened in the middle).
- Counting, dates and thresholds stay in your own steps. Every decision is
  billed to the owner like any model call, so do not loop it over every
  record on every turn.
- If it fails, say so plainly and stop asking: "You've used all the work
  included in your account..." means nothing more will be answered until the
  owner adds a plan or credits; "Decisions need NeboAI connected" means the
  bot is signed out; "Too many decisions at once" can be tried again in a
  moment. The page's `nebo.decide` throws the same words (HTTP 429, 503,
  429), a malformed question is 400 with what is wrong, and a service
  failure is 502.

Cards: your `a2ui` tool can show a card on the page (it arrives over
`surfaces.connect()`; the page bundles an `@a2ui/web_core` renderer and calls
`nebo.a2ui.init`). A click comes back to you. A page opened later does not
get earlier cards, so keep what must always show in storage. The SDK's other
`surfaces` events (`state_snapshot`, `state_delta`, `text_content`, ...) are
not sent to app pages yet: do not build on them.

## Building

Use bun when the bot has it (`bun` on the path, or the bundled one at
`/tmp/nebo-runtimes/bun`, `%TEMP%\nebo-runtimes\bun.exe` on Windows). Cloud
bots have node and npx but no bun (checked 2026-10-01); there, the same build
is `npm init -y && npm i gsap lenis split-type` and
`npx esbuild src/main.js --bundle --minify --format=esm --outdir=ui --entry-names=[name]-[hash] --asset-names=assets/[name]-[hash]`,
and Tailwind is `npx @tailwindcss/cli -i src/app.css -o ui/app.css --minify`.
Check with `command -v bun` first and use whichever exists.

```bash
cd "<app folder>"
bun init -y                 # once; creates package.json
bun add gsap lenis split-type
bun build ./src/main.js --outdir ./ui --entry-naming "[name]-[hash].[ext]" \
  --asset-naming "assets/[name]-[hash].[ext]" --minify --target browser
bunx @tailwindcss/cli -i src/app.css -o ui/app.css --minify   # when using Tailwind
```

Then point `ui/index.html` at the new `main-<hash>.js` (write index.html as
part of the build, or read the name bun prints). Delete the previous
hashed bundles from `ui/` so the folder holds one build.

**Why hashed names.** Nebo caches by one rule: a file whose name carries a
content hash (`main-0a8ksftt.js`) is kept for a year; every other file
(`index.html`, `app.css`, `assets/hero.webp`) is checked on every open and
costs one round trip when unchanged. So a rebuild that renames changed
files shows at once, and big unchanged assets never travel twice. Never
rename files by hand to get past a cache. With App Developer mode on,
nothing is cached at all.

**If bun is not there**, skip the build: write plain ES modules in `ui/`
and load libraries from the CDN lines in `kit.md`.

**React or three with React** builds the same way with an entry
`src/main.jsx`; bun handles JSX.

## Developer tools

An app the owner made (on this bot, not installed from the marketplace)
always has these tools for ITSELF, with no setting: leave `app` out when
you are the app. App Developer mode (Bot settings, Developer) opens them to
teammates on any of the owner's apps, adds the floating console to the page,
and serves app files with nothing cached. An app installed from the
marketplace never gets them, mode or not.

- `app_reload(app)`: every open view of the app (phone, desktop window,
  browser) reloads. Use after every build.
- `app_status(app)`: the served folder, files with sizes and times, the
  entry file and what it loads, and whether an open view has errors.
  Answers "why isn't my change showing" in one call.
- `app_console(app)`: console output, uncaught errors and failed requests
  from open views, newest last. Read it after every reload.
- `app_screenshot(...)`: a screenshot of the app as served (390x844 for a
  phone, 1280x800 for a desktop), returned as a description plus a saved
  file. Use it to check screens against the boards.
- `app_listing` / `app_submit`: publishing with the owner (below).

If you are working on another employee's app and the tools refuse, ask the
owner to turn App Developer mode on, or to open the app and tell you what
they see.

## What a page can carry

- Images (`png jpg webp avif gif svg`), fonts (`woff2 woff ttf otf`), video
  (`mp4 webm mov`), sound (`mp3 wav ogg m4a`), 3D (`glb gltf`), `wasm`,
  `js mjs css html json`. A file with another extension is served as
  `application/octet-stream` and the browser will not use it. Each is served with its real content type, and
  video and sound answer range requests, so seeking works.
- **At most 10 MB per file and 50 MB for the whole package**, or it cannot
  be published: the publish refuses a file past the limit, and the
  marketplace skips any page file over 10 MB. The gate checks both.
- Anything else in `ui/` of another type, dot files and `node_modules/` are
  dropped at publish.
- Video plays inline on the phone: `<video muted playsinline>` (plus
  `autoplay loop` for a loop). Setting `currentTime` from scroll scrubs it;
  follow `film-scrub.md`.

## Phone rules

- Vertical panning only for a scrolling page:
  `html, body { touch-action: pan-y; overscroll-behavior-x: none; }`.
  A game's play area takes every gesture with `touch-action: none`.
- Nothing wider than the screen; long strings get `overflow-wrap: anywhere`.
- `dvh` units, never `100vh`.
- Sound needs a first tap before it plays, and on iPhone tilt needs
  `DeviceMotionEvent.requestPermission()` from a tap: ask for both on the
  same first tap (see `games.md`).

## Talking to a game server (multiplayer)

The page opens a WebSocket straight to the game server:
`new WebSocket('wss://<game server>/rooms/<id>')`. Game traffic touches
neither Nebo nor the marketplace. App pages carry no content security
policy that blocks it, on desktop or phone. Who the player is comes from
`nebo.identity`; a signed player token for the server is planned. Design
the protocol so the page survives a reconnect (resend its state on open).

## Publishing

Only when the owner asks ("publish yourself", or the **Publish** button: in
the app's chat, on the phone's app screen, or **Publish This App…** in the
desktop menu). Follow the bundled `publish-an-app` skill: `app_listing`
drafts the listing, `app_screenshot` (for_listing: true) takes 3 to 5
screenshots, then `app_submit`. The bundle is AGENT.md, agent.json,
manifest.json, `ui/` and your own skills under `skills/<name>/`, never
`src/`, `refs/` or `brief.md`. The owner sends it by answering the card in
the chat (voice can answer it). Never submit on your own. Delete
`ui/boards/` first; the gate refuses while it exists.

After it is published: for a private or loop app, every new bundle is put in
place on the bots that installed it right away, keeping their settings and
data, with no version change. A raised version is offered to each owner in
Settings, Updates (applied at once only where they turned on automatic
updates). If NeboAI ever withdraws the app, installed bots turn it off and
keep everything it saved.
