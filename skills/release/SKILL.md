---
name: release
description: >-
  Prepare a release: version bumps, notes, checks, and publish commands. Use when the user asks to release, cut a version, or draft release notes.
disable-model-invocation: true
---

# Release workflow

Prepare a clean release without surprise breaks.

## Goal

Version(s), notes, and exact publish commands, executed only when the user asks.

## Steps

1. **Preflight**
   - Branch, working tree state, last tag/release.
   - `git log <last-tag>..HEAD --oneline` (or since last release).
   - Respect repo trains when documented (e.g. `release/*` -> pre-stable channel, `main` -> stable). See CONTRIBUTING / pipeline docs.

2. **Versioning**
   - Follow the repo's system only: Changesets, semver files, or existing release tooling.
   - Do not invent a second scheme.
   - Monorepos: per-package bumps; major only for contract breaks.

3. **Notes**
   - User-facing Summary, Breaking changes, Migrations / ops steps (env, jobs, flags).

4. **Checks**
   - CI green or explicit accepted exceptions.
   - Changelog/changesets consistent with the bump.

5. **Publish** (propose, do not force)
   - List the repo's real commands (changeset publish, `gh release create`, deploy workflow, etc.).
   - Run tag/publish/push **only** when the user explicitly asks.

## Done when

- Versions + notes are ready; commands are copy-pastable or already run per request.
- Residual risks/blockers are listed.

## Next

Monitor CI/deploy; `fix-ci` if release pipelines fail.
