# AGENTS.md

You are working on Magpie.

Magpie is a local-first research memory system for agentic R&D. Its core direction is:

- append-only event log as the source of truth
- replayable state
- explicit provenance
- claim, source, entity, and retrieval tracking
- derived projections for search, graph, and memory
- no hidden authority
- small, reviewable changes

## Worker rules

- Do exactly the requested ticket.
- Do not broaden scope.
- Do not invent major architecture.
- Do not merge.
- Do not touch secrets, credentials, `.env*`, or unrelated files.
- Do not add network dependencies unless the ticket explicitly allows it.
- Prefer boring, typed, testable code.
- Prefer small diffs.
- If the ticket is under-specified, stop and explain what is missing.
- If shell execution fails with `CreateProcessAsUserW failed: 5`, do not thrash or repeatedly retry.
- Continue only if the ticket can be completed safely through direct file edits.
- Clearly report which validation commands were skipped.
- Never report tests as passing unless they actually ran and passed.
- Prefer small patches and explicit reviewer checks.

## Rust preferences

- Prefer explicit types.
- Prefer `serde` for boundary types.
- Prefer small modules.
- Prefer focused unit tests.
- Avoid clever macros unless the repo already uses them.
- Follow existing crate conventions before inventing new ones.

## Final response format

End with:

- Files changed
- Behaviour added
- Tests run
- Known risks
- Suggested reviewer checks
