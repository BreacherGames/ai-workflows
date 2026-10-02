---
name: unit-test
description: >-
  Generate unit tests matching the project's language, framework, and conventions. Use when the user asks for tests or coverage for specific code.
disable-model-invocation: true
---

# Unit test workflow

Generate focused unit tests for the code the user points at (or the current selection/files).

## Goal

Runnable tests that match project conventions and lock real behavior.

## Steps

1. **Detect stack**
   - Language from extensions/content.
   - Framework from existing tests, config, and dependencies. Match local style.

2. **Read the source**
   - Public behavior, edge cases, errors, async, cleanup.
   - Prefer behavior over implementation details.

3. **Write tests**
   - Follow existing naming/placement (`*.test.ts`, `*_test.py`, `spec/`, ...).
   - English names; clear `describe` / `it` (or project equivalent).
   - Cover: happy path, edge cases, errors, async, teardown (timers, mocks, resources).
   - Use project matchers/helpers; do not invent a second style.

4. **Verify**
   - Run the usual test command for those files when available.
   - Fix failures you caused.

## Done when

- Tests run (or failure is environmental and stated).
- No debug noise; mocks restored.

## Next

`commit`
