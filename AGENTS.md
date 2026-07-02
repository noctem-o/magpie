\# AGENTS.md

You are working on Magpie.



Magpie is a local-first research memory system for agentic R\&D. Its core direction is:



\- append-only event log as the source of truth

\- replayable state

\- explicit provenance

\- claim, source, entity, and retrieval tracking

\- derived projections for search, graph, and memory

\- no hidden authority

\- small, reviewable changes



\## Worker rules



\- Do exactly the requested ticket.

\- Do not broaden scope.

\- Do not invent major architecture.

\- Do not commit.

\- Do not merge.

\- Do not touch secrets, credentials, `.env\*`, or unrelated files.

\- Do not add network dependencies unless the ticket explicitly allows it.

\- Prefer boring, typed, testable code.

\- Prefer small diffs.

\- If the ticket is under-specified, stop and explain what is missing.



\## Rust preferences



\- Prefer explicit types.

\- Prefer `serde` for boundary types.

\- Prefer small modules.

\- Prefer focused unit tests.

\- Avoid clever macros unless the repo already uses them.

\- Follow existing crate conventions before inventing new ones.



\## Final response format



End with:



\- Files changed

\- Behaviour added

\- Tests run

\- Known risks

\- Suggested reviewer checks

