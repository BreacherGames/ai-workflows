---
name: annotate
description: >-
  Add or improve documentation comments on public APIs (JSDoc, docstrings, etc.). Use when the user asks to document, annotate, or add API comments.
disable-model-invocation: true
---

# Documentation annotation workflow

Add or improve documentation comments for public/exported APIs in the files the user targets.

## Goal

IDE-friendly docs on the public surface: accurate, concise, consistent with the language.

## Steps

1. **Detect language** and its usual doc format (JSDoc, LuaDoc, docstrings, Go doc, etc.).

2. **Scope**
   - Exported/public functions, classes, types, constants, modules.
   - Improve incomplete docs; do not rewrite unrelated code.
   - Leave private internals alone unless asked.

3. **Content**
   - Purpose, parameters, returns, important edge cases, short example when helpful.
   - Match project tone and existing comment style.

4. **Language notes**
   - **TypeScript/JSDoc**: descriptions only; **do not** restate types in `@param` / `@returns`.
   - Document interface **properties** with inline comments so IDEs surface them.

## Done when

- Public surface for the targeted files is documented or explicitly skipped with reason.
- No type-duplicating JSDoc on TypeScript code.

## Next

`commit`
