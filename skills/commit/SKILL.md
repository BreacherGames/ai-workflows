---
name: commit
description: >-
  Create Conventional Commit messages from staged changes, with per-package bump classification, Changesets or version files, and optional README updates. Use when the user asks to commit, draft a commit message, or run the commit workflow.
disable-model-invocation: true
---

# Commit workflow

Create a Conventional Commits message from **staged** changes, then stage any generated files and commit with `git commit -m "..."`.

Do **not** use `--trailer`. Do **not** amend or push unless the user asks.

## Goal

One clean commit that matches the staged intent, with correct versioning metadata when the repo uses it.

## Steps

1. **Inspect staged work**
   - `git diff --cached --name-only` and `git diff --cached`
   - If nothing is staged, stop and say so.
   - On Windows PowerShell, chain with `;` not `&&`.

2. **Map repo layout**
   - Monorepo vs single package (workspaces, `pnpm-workspace.yaml`, `nx.json`, `turbo.json`, multiple manifests, etc.).
   - Which package roots the staged paths belong to (real `"name"` from manifests when present).

3. **Classify each modified package**

   | Impact | Examples | Bump |
   | ------ | -------- | ---- |
   | Contract-breaking | Removed/renamed public export, API route/payload/auth change, incompatible DB migration | `major` for **that** package |
   | Normal | `feat` / `fix` / ... with no consumer break | `feat` -> `minor`; most else -> `patch` |
   | Ops / config only | Env rename with unchanged HTTP contract, CI secrets, docs | `patch` (or type default); `Migration:` - **not** `BREAKING CHANGE:` |
   | Not versioned | Paths outside packages | no bump |

   Internal table: `package -> bump`, `contract-breaking: yes|no`.

4. **Draft the message (English)**
   - Subject: `type(scope): description` (present tense, ~50-72 chars, no trailing period).
   - Body: short **what/why** bullets, not a file dump.
   - Contract-breaking: `type(scope)!: ...` and/or `BREAKING CHANGE:` + migration notes.
   - Ops-only: no `!` / no `BREAKING CHANGE:`; add `Migration:` when deployers must change env/secrets/config.

5. **Versioning (mutually exclusive)**
   - If `.changeset/` exists: **one changeset per modified package**; do **not** edit version files.
   - Else: bump existing version sources per package (`package.json` version, `VERSION`, or one obvious root version file). Preserve formatting. If none exist, create root `VERSION` (e.g. `0.1.0`) then bump when appropriate.
   - Skip bumping pure `docs` / `chore` / `ci` / `build` unless a public contract changed; still create a missing root `VERSION` if the repo has no version source at all.

6. **README**
   - Update an existing package `README.md` only when user-facing docs clearly need it. Never invent a README.

7. **Stage and commit**
   - Stage generated/updated files, then `git commit -m "..."`.
   - Show the final message to the user.

## Types

`feat` `fix` `refactor` `style` `docs` `test` `chore` `perf` `ci` `build` `revert`

## Done when

- Commit exists on the current branch with the intended staged set (including version metadata you generated).
- Message is Conventional Commit quality; user can see it.
- No push unless asked.

## Next

`open-pr` (or `pr-description` if they only need text)
