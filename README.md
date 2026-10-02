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

## App Studio

App Studio is built into Nebo 0.16.7 and later as the bundled `app-studio`
skill (nebo `crates/tools/src/skills/bundled/app-studio/`). `app-studio/` here
is a stub so the marketplace listing carries no second set of instructions.

## License

Apache-2.0
