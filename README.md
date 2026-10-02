# ai-workflows

Team-wide AI workflows for daily development.

Real Cursor skills in `skills/`, plus an always-on engineering rule in `rules/`.
Install with `aiw` or with the skills.sh CLI.

## Daily loop

```text
start-branch -> investigate -> implement -> review -> fix-ci -> commit -> open-pr -> release
```

Also available: `unit-test`, `annotate`.

## Install with aiw

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

### Other commands

```bash
aiw update
aiw remove
aiw list
aiw doctor
```

## Install with skills.sh

Skills only. Does not install the engineering rule.

```bash
npx skills add BreacherGames/ai-workflows
```

Then use `aiw install` if you also want the always-on rule.

## Layout

```text
skills/   one folder per skill, each with SKILL.md
rules/    always-on engineering standards
src/      aiw CLI
```

## License

See [LICENSE](LICENSE). Free to use and share. Do not sell this pack as a product.
