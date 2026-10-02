---
name: start-branch
description: >-
  Create a topic branch (feature/, fix/, hotfix/, refactor/, experiment/) from the correct base (main or release/* train). Use when the user asks to start a branch, create a feature branch, or begin a new piece of work.
disable-model-invocation: true
---

# Start branch workflow

Create (or switch to) a correctly named topic branch.

## Goal

A topic branch with the right prefix, off the right base, ready for `implement`.

## Steps

1. **Resolve base (integration line)**
   - User override wins.
   - Else read `CONTRIBUTING.md` / pipeline docs when present.
   - If an active `release/*` train is documented, use that as base for day-to-day topic work unless the user wants `main` / stable.
   - Otherwise: `main` (or `origin/HEAD`).
   - Never invent a `dev` branch.

2. **Pick prefix** from the request:

   | Intent | Prefix |
   | ------ | ------ |
   | New capability | `feature/` |
   | Bug | `fix/` |
   | Urgent prod fix | `hotfix/` |
   | Restructure, same behavior | `refactor/` |
   | Spike (if repo allows) | `experiment/` |

3. **Slug**
   - Short kebab-case: `feature/portal-menu`, `fix/auth-timeout`.
   - Optional ticket id if the user gives one: `fix/1234-auth-timeout` (do not invent ticket numbers).

4. **Create**
   - Fetch if needed: `git fetch origin`
   - Update and check out `<base>`, then `git switch -c <prefix><slug>` (or equivalent).
   - If the branch already exists locally or remotely, switch to it instead of recreating.

5. **Confirm**
   - Show: base <- new branch name.
   - Do not push unless asked.

## Done when

- Current branch matches the prefix conventions above (or project docs).
- It was created from the correct integration line.
- Working tree ready for implementation.

## Next

`implement` -> ... -> `commit` -> `open-pr` (PR into the **same base** used here)
