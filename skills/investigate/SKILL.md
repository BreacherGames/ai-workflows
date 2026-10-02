---
name: investigate
description: >-
  Find root cause of a bug, incident, or surprising behavior before patching. Use when the user asks to investigate, debug, diagnose, or figure out why something fails.
disable-model-invocation: true
---

# Investigate workflow

Find the root cause of a bug, incident, or surprising behavior before changing code.

## Goal

A clear causal explanation and a minimal recommended fix path, not a shotgun patch.

## Steps

1. **Capture the symptom**
   - Expected vs actual, repro steps, environment, since when.
   - Gather logs, stack traces, failing tests, screenshots, or PR links the user has.

2. **Reproduce**
   - Prefer a local or scripted repro. Note if you cannot reproduce and why.

3. **Narrow**
   - Bisect by layer: input -> API/UI -> domain -> data -> infra.
   - Use git history / recent changes when "it worked yesterday".
   - Form 1-3 hypotheses; kill them with evidence (read code, run commands).

4. **Root cause**
   - State the cause in one paragraph: what breaks, where, why.
   - Separate **trigger** (what the user did) from **defect** (what the code/config got wrong).

5. **Fix plan**
   - Smallest safe fix, tests to add, and risks (data, auth, rollout).
   - CI-only -> `fix-ci`. Code change -> `implement`.

## Done when

- Root cause is evidenced (file/symbol/log line), not vibes.
- You proposed a concrete next workflow and fix shape.
- You did not spray unrelated changes while investigating (unless the user asked to fix immediately).

## Next

- Code fix -> `implement`
- Pipeline only -> `fix-ci`
- Second pair of eyes on a diff -> `review`
