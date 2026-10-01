# NeboLoop Skills

Official skills for the NeboLoop AI agent platform. Each skill follows the [Agent Skills specification](https://agentskills.io/specification).

## Structure

Each skill is a directory containing:

```
skill-name/
├── SKILL.md          # Required: YAML frontmatter + instructions
├── scripts/          # Optional: compiled binaries
├── references/       # Optional: detailed docs
└── assets/           # Optional: templates, resources
```

## Featured: App Studio

`app-studio/` is the method a Nebo employee follows to build an app or game
that looks designed, not generated: one intake, a written brief, generated
design boards and assets, a real build into the app's own folder, one
signature effect, and a gate (`scripts/gate.js`, runs with bun or node) that
must pass before publishing. Its references cover the film-scrub template
proven on iPhone, games, and the Nebo app specifics (manifest window, the SDK
global, hashed builds, size limits, publishing).

## License

Apache-2.0
