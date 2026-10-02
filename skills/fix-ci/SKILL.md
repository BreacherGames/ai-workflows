---
name: fix-ci
description: >-
  Diagnose and fix failing CI or local checks that mirror CI. Use when the user reports a red pipeline, failed GitHub Action, or broken checks.
disable-model-invocation: true
---

# Fix CI workflow

Diagnose and fix failing CI (or local checks that mirror CI) with minimal, targeted changes.

## Goal

Green checks for the right reason, not silenced failures.

## Steps

1. **Collect signal**
   - Job logs, `gh run view` / `gh run list`, or pasted output.
   - Identify the **first real error**, not only the last cascade line.

2. **Reproduce**
   - Run the closest local command when possible.
   - Note env gaps (runtime version, OS, secrets, caches).

3. **Root cause**
   - Classify: real regression, flake, config/env, dependency, permissions.
   - Prefer fixing the cause over skipping or inflating timeouts, unless the user accepts a quarantine for a proven flake.

4. **Fix**
   - Smallest diff. Do not refactor green areas.
   - Multiple unrelated failures -> fix the highest-confidence one first, then re-run.

5. **Confirm**
   - Re-run the failing check locally when feasible.
   - Summarize cause -> fix -> residual risk.

## Done when

- The failing check is explained and addressed (or blocked on missing access/secrets, stated clearly).
- Diff stays scoped to the failure.

## Next

`commit` -> re-run CI / `open-pr`
