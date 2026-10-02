---
name: pr-description
description: >-
  Generate a paste-ready GitHub PR description (Summary, Breaking changes, Test plan) from commits versus a base branch. Use when the user asks for a PR description or PR body only (no create).
disable-model-invocation: true
---

# Pull request description workflow

Produce a PR description from commits on the current branch vs a base branch. **Read-only** for git history: no commit, amend, rebase, or push.

## Goal

Paste-ready Markdown a reviewer can use immediately.

## Steps

1. **Base branch**
   - Resolve like engineering rules / `start-branch`: user -> CONTRIBUTING/pipeline -> release train -> `main`.
   - `git log <base>..HEAD --oneline` (+ paths if useful).

2. **Summarize**
   - Conventional Commit types/scopes and path groups.
   - **3-7** high-level bullets (behavior/value, not file lists).
   - Note the topic prefix if relevant (`feature` / `fix` / ...).

3. **Breaking changes**
   - From `!` / `BREAKING CHANGE` and obvious contract breaks.
   - If none: `- _None expected._`

4. **Test plan**
   - **4-8** realistic checklist items (`- [ ] ...`).
   - Prefer repo commands from CONTRIBUTING when known.

5. **Output**
   - Reply with **only** one fenced code block (plain triple backticks, no language tag):

```markdown
## Summary
- ...

## Breaking changes
- ...

## Test plan
- [ ] ...
```

## Done when

- Single paste-ready block; no prose outside the fence.
- Summary is accurate for `<base>..HEAD`.

## Next

Prefer `open-pr` when they want the PR created (`--base` = same base).
