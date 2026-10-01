---
name: app-studio
description: "The method for building a stunning Nebo app or a rich game: one intake, a written brief, generated reference boards, real generated art, a build into the app's own folder, one signature effect, and a mechanical gate before it ships. Use when the owner asks for an app, a landing app, a showcase, an interactive story, or a game and wants it to look designed, not generated."
triggers:
  - make it stunning
  - beautiful app
  - landing app
  - showcase app
  - build a game
  - make a game
  - interactive story
  - animated app
  - scroll animation
  - app studio
metadata:
  version: "0.1.0"
---

# App Studio

An app is an employee with a page (the bundled `build-an-app` skill covers
creating one and the SDK contract). This skill is the method that makes the
page look designed instead of generated. The results come from process, not
from a better model: one intake, a brief with every choice written down,
pictures of each screen before any code, real generated art, one signature
effect, and a script that fails the build on the patterns models fall into
by habit.

Run every phase in order. Each one leaves an artifact the next one reads.
"Simple" requests are where generic output happens, so they get the full
method too. A small edit to an app that already went through it (a copy
tweak, one component, a color) does not restart the method: make the edit,
rebuild, rerun the gate.

## When to Use

- "Build me an app that shows off..." / "a landing app for..." / "make it beautiful"
- "Make me a game" / "a game where..." / "something my customers can play"
- "An interactive story" / "scroll through the product" / "animated"
- Any app the owner will show to other people

For a plain internal tool (a tracker, a form, a table the owner fills in),
`build-an-app` alone is enough. Use this when looks matter.

## The Flow

| # | Phase | Artifact | Gate |
|---|-------|----------|------|
| 0 | Intake | the owner's answers | one batched round, never a second |
| 1 | Brief | `brief.md` in the app folder | no generic line, front-matter complete |
| 2 | Boards | `boards/*.png` (served folder; deleted before publish) | every board looked at, template-looking ones redone |
| 3 | Assets | `ui/assets/*` | every planned asset exists and is used |
| 4 | Build | `src/` built into `ui/` | the page matches the boards |
| 5 | Motion | the signature effect, wired | responds to the person's input |
| 6 | Gate | `scripts/gate.js` passes | zero failures |
| 7 | Publish | App Developer mode, Publish | the owner says yes |

### Before anything: you are the app

If the owner asks you for the app, you become it: make yourself an app with
`update_employee` on your own name (see `references/nebo-app.md`), never a
second "<name> App" employee unless they ask for one. Then build into your
own folder, reload as you go, and publish yourself.

### 0. Intake (one round)

Ask everything in ONE message, then never ask again. Always ask:

1. **App or game?** (when the request does not already say)
2. **Animated or still?** Recommend animated: the person's scroll or touch
   drives a film, a 3D scene or the page itself. Still means a well-made
   page with light motion, never a dead flat one.
3. **Brand: their own or free rein?** If their own, ask for colors, fonts,
   logo, photos and links. If free rein, you are also the brand studio:
   you design the identity and generate everything they lack.

If the owner does not answer, go ahead with animated and free rein and say
so in one line. Then take the fullest reasonable reading of the request and
build the real thing, never a mockup.

### 1. Brief

Write `brief.md` in the app folder (beside `ui/`) before any board or code.
Template and rules: `references/brief.md`. It holds:

- **Front-matter with the six variety axes:** `palette`, `type`,
  `hero_layout`, `signature_effect`, `button_style`, `corner_shape`, plus
  `mode` (`generation` or `no-generation`).
- **Concept spine:** one sentence the whole app pays off ("the app is a
  flight instrument", "the app is a field notebook").
- **Delivery tier:** `cinema` (default), `spectacle` or `editorial`.
- **Locked palette** as hex, one accent, with a one-line defense.
- **Locked type pair.**
- **Animation mode**, the literal value from the intake.
- **Screen plan** (or section plan), **asset plan**, **CTA inventory**.

A generic line ("modern and clean", "Inter", "blue accent") means the brief
is not done. The brief is a contract: later phases may not quietly
contradict it. Change the brief first and say why.

**Anti-convergence.** Before locking the brief, read the `brief.md` of every
other app on this bot (the other app folders beside this one). The new app
must differ from each on at least 4 of the 6 axes. The gate checks it.

### 2. Boards (the boards ARE the design)

One generated image per screen or section, palette locked across all of
them. Generate them with `generate_media(kind: "image", ..., into:
"boards/<screen>.png")`. Boards are working files: the gate refuses to
publish while `boards/` is still in the served folder, so delete it once the
build matches them. Then look at each one (read the
file; the vision helper describes it) and regenerate any that reads as a
template: centered dark hero, glowing blob, three identical cards, beige
serif "luxury". Budget two redos per board. How to pick the combination and
write the prompts: `references/boards-and-assets.md`.

When your coding habit and the board disagree later, the board wins.

**No-generation mode.** If `generate_media` is not in your tools, write a
board per screen in words inside the brief (layout, type scale, color
placement, the one focal visual) and record `mode: no-generation`. Art is
then hand-made SVG, canvas or shader work. A generative canvas hero counts;
a gradient behind text still does not. When generation appears, the same
brief switches to `mode: generation` with no other change.

### 3. Assets

Submit every asset at once (hero, section plates, icon set, logo or
monogram, sprites, textures, video, sound), and build while they render.
Every prompt carries the locked hexes, the spine's material words, and "no
text, no logos, no watermark". No stock photos, no placeholder services, no
CSS-only heroes. The owner's own assets always win; generation fills gaps.
Kit list and prompt rules: `references/boards-and-assets.md`.

### 4. Build

Source lives in `src/` beside `ui/` in the same app folder; the build writes
into `ui/`, which is what Nebo serves. Never build in a separate copy
somewhere else. Build with bun, with content-hashed file names. Folder
layout, the build command, the SDK, the manifest and the App Developer tools:
`references/nebo-app.md`.

Build each screen to its board: reread the board, extract type ratios,
spacing logic, color placement and component shapes, then code it.
Craft floor (type, color, hero, layout, copy, states):
`references/design-recipe.md`. Build it static but complete first; motion
is the next phase.

### 5. Motion pass

Pick ONE Tier-1 signature effect from `references/wow-catalog.md`, chosen
because it pays off the concept spine, and execute it fully. It must
respond to the person's input (scroll, drag, tilt, pointer). A half-wired
version is worse than a clean still page. Everything else is motivated
reveals and feedback. Every animation has a `prefers-reduced-motion`
fallback.

The film scrub (a generated video scrubbed by scroll) has a ready template
proven on iPhone: `references/film-scrub.md`. Use it as written.

### 6. Gate

Run the gate against the app folder and fix every failure, then run it
again:

```
bun <this skill>/scripts/gate.js <app folder>
```

(`node` runs it too.) It checks the brief, the banned palette and words,
em-dashes, placeholders, unused or oversize assets, reduced motion, touch
handling, the film rules, the app's manifest and AGENT.md, and the 4-of-6
difference from the bot's other apps. Details: `references/gate.md`. A build
with a failing gate is not done.

Then look at it running: `app_reload` refreshes every open view,
`app_status` shows what is served, `app_console` shows errors from open
views, and `app_screenshot` lets you see a screen (phone size 390x844 and
desktop 1280x800). Fix what the console shows.

### 7. Publish

Only when the owner asks. With App Developer mode on, the app's chat has
**Publish**; follow the bundled `publish-an-app` skill (screenshots, the
listing, the owner's yes). Never submit on your own.

## Banned Defaults (the model's own habits)

- **Palettes:** near-black plus orange or amber; near-black plus neon cyan,
  blue or green; purple or violet glow; beige or cream plus brass, clay or
  oxblood. Allowed only when the owner's brand names those colors.
- **Type:** Inter as the display face. Serif by default for "premium".
- **Layout:** three equal cards in a row; a fake product UI built from divs
  in the hero (fake dashboard, fake terminal, fake task list).
- **Copy:** em-dashes and en-dash separators anywhere visible; Elevate,
  Seamless, Unleash, Next-Gen, Revolutionize; invented performance stats;
  "Jane Doe" testimonials.
- **Motion as the answer:** fade-ins, marquees, tilt cards and autoplay
  loops are seasoning, never the signature effect.

## The Kit

GSAP + ScrollTrigger (scroll and timelines), Lenis bridged to the GSAP
ticker (smooth scroll), motion (UI springs and layout), split-type
(headline builds), three + react-three-fiber + drei + postprocessing (3D),
ogl (light shaders), Tailwind v4. Only free, permissively licensed
libraries. How to load each, with and without a build:
`references/kit.md`.

## Games

A game runs the same flow with a game brief: an experience formula, a core
loop, a style formula every asset prompt repeats word for word, and an asset
manifest. Read `references/games.md` before the brief. It covers 2D sprites
from video, procedural animation, 3D models, game audio, the first-tap
unlock for sound and tilt, saves through `storage`, fullscreen and
orientation, and multiplayer (the page opens a WebSocket straight to the
game server).

## Turn Economy

- Write each file once, complete. No write-then-patch loops.
- Submit every independent generation together; wait on a job only when its
  output is the next input.
- Never look at a generated image twice. Boards get one look each; the asset
  kit gets one batched look for coherence.

## Talking to the Owner

Speak in product terms: "Designing the screens", "Making the art",
"Your app is ready, open it from your workforce". Never narrate build
plumbing (bundlers, hashes, folders) unless the owner asks. Say employee,
app and owner. At the end, list what the owner now owns (logo, icon set,
art, film) and anything honestly skipped.

## References

| File | Read when |
|------|-----------|
| `references/brief.md` | Phase 1, always |
| `references/design-recipe.md` | Phase 1 and 4, always |
| `references/boards-and-assets.md` | Phases 2 to 4 |
| `references/wow-catalog.md` | Phase 1 (pick) and 5 (build) |
| `references/film-scrub.md` | when the signature effect is a film scrub |
| `references/kit.md` | Phase 4 and 5 |
| `references/nebo-app.md` | Phase 4, always |
| `references/games.md` | every game, before the brief |
| `references/gate.md` | Phase 6 |

Parts of this method are adapted from an MIT-licensed work; the notice is in
`LICENSE-THIRD-PARTY.txt`.
