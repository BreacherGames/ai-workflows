---
name: review
description: >-
  Review a diff, branch, or PR for correctness, security, and contract risk. Use when the user asks for a code review, PR review, or second pass on changes.
disable-model-invocation: true
---

# Code review workflow

Review a diff, branch, or PR like a careful senior engineer. Catch real risk; skip linter cosplay.

## Goal

A verdict a human can act on in minutes: merge, fix, or discuss.

## Steps

1. **Scope**
   - Prefer the user's target (PR URL, branch, files). Otherwise: `git diff` / `git diff <base>...HEAD`.
   - Resolve `<base>` like engineering rules: user -> CONTRIBUTING/pipeline -> `release/*` train -> `main`.
   - Expect topic branches `feature/*` Â· `fix/*` Â· `hotfix/*` Â· `refactor/*` (warn if the name is off-convention).
   - Skim commits for intent.

2. **Prioritize**
   - Correctness / regressions
   - Security (auth, injection, secrets, unsafe defaults)
   - Data loss / migrations / backwards compatibility
   - Contract breaks for external consumers
   - Missing tests on risky paths
   - Performance only when clearly relevant

3. **Output**

```markdown
## Verdict
Approve | Approve with nits | Request changes

## Findings
### Blockers
- ...

### Should fix
- ...

### Nits (optional)
- ...

## Questions
- ...

## Summary
1-3 sentences: what changed + overall risk.
```

4. **Rules**
   - Every finding: **what**, **where** (path/symbol), **why**, **fix** when obvious.
   - No blockers -> say so. Do not invent issues.
   - Skip pure style wars unless they hurt clarity vs nearby code.

## Done when

- Verdict is clear; blockers are actionable.
- You would defend this review to the author.

## Next

- Fixes needed -> `implement` then re-review
- Clean -> `commit` / `open-pr` if not already opened
