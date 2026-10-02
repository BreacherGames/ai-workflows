# ai-workflows

Team-wide AI workflows for daily development.

Real Cursor skills in `skills/`, plus an always-on engineering rule in `rules/`.

## Daily loop

```text
start-branch -> investigate -> implement -> review -> fix-ci -> commit -> open-pr -> release
```

Also available: `unit-test`, `annotate`.

## Install the Breacher pack

```bash
cargo install --path .
aiw install
```

Restart Cursor after install.

### Project

Shares skills and rules with the team. Commit `.ai-workflows/` and `.cursor/`.

```bash
aiw install --scope project --project /path/to/app
```

## Open ecosystem

`aiw` wraps skills.sh. Needs Node.js for `npx`.

```bash
aiw add owner/repo
aiw add owner/repo --skill name
aiw find tdd
```

Examples:

```bash
aiw add BreacherGames/ai-workflows
aiw add vercel-labs/agent-skills
aiw add mattpocock/skills --skill tdd
```

## Other commands

```bash
aiw update
aiw remove
aiw list
aiw doctor
```

## Layout

```text
skills/   one folder per skill, each with SKILL.md
rules/    always-on engineering standards
src/      aiw CLI
```

## License

See [LICENSE](LICENSE). Free to use and share. Do not sell this pack as a product.
