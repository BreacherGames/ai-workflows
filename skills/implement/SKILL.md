---
name: implement
description: >-
  Implement a feature or bug fix: clarify, explore, plan, build, verify. Use when the user asks to implement, build, or fix something in the codebase.
disable-model-invocation: true
---

# Implement workflow

Deliver a feature or bug fix with minimal thrash: understand -> plan -> build -> verify.

## Goal

Working change that matches the request, fits the codebase, and is ready to review or commit.

## Steps

1. **Clarify**
   - Restate the goal in one or two sentences.
   - Note constraints (APIs, packages, flags). Ask only if blocked.
   - If still on `main` / a release train with no topic branch, offer `start-branch` first (do not create a branch silently unless asked).

2. **Explore**
   - Find entry points, existing patterns, and tests.
   - Prefer extending current abstractions over new frameworks.

3. **Plan** (short)
   - Files/areas to touch + approach in a few bullets.
   - Call out risks (migrations, public API, auth, data).

4. **Implement**
   - Smallest change that meets the goal.
   - Match project style. No unrelated refactors.
   - Update/add tests when behavior changes or risk warrants it.

5. **Verify**
   - Run relevant project checks (tests, typecheck, lint) when available.
   - Fix failures you introduced.

6. **Hand off**
   - Summarize what changed and how to verify manually.
   - Suggest `review` / `commit` / `open-pr` as next steps; run them only if asked.

## Done when

- Request is met (or blockers are explicit).
- Checks you can run are green for your change.
- Diff is focused; no drive-by edits.

## Next

`review` -> `commit` -> `open-pr` (PR into the same base the topic branch used)
