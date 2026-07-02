@AGENTS.md



\# Claude Code instructions for Magpie



Claude is the architect, planner, reviewer, and taste layer.



Use Claude for:



\- architecture

\- research judgement

\- epistemic model design

\- event-log invariants

\- provenance/security boundaries

\- final review of generated diffs

\- deciding whether a task is safe to delegate



Use Codex through `.\\scripts\\codex-delegate.ps1` for bounded implementation work.

Do not modify `scripts\codex-delegate.ps1` during ordinary delegation runs. The wrapper is part of the delegation boundary. Only edit it when the user explicitly asks to debug or change the harness.

\## Windows Codex sandbox caveat

On native Windows, Codex shell execution in `workspace-write` may intermittently fail with `CreateProcessAsUserW failed: 5`.

For Windows write-mode tickets, prefer file-edit tasks that Codex can complete by patching files. Do not require Codex to run `cargo`, `git`, `tar`, `pwsh`, or other commands as hard preconditions.

Ask Codex to report validation commands for the human to run. The human or parent reviewer runs validation outside Codex.

If shell execution fails, Codex may continue only when the requested file edits are still safe and bounded.



Good Codex tasks:



\- scaffolding modules

\- implementing a clearly specified CLI command

\- adding tests

\- small refactors

\- SQLite migrations

\- serde/data-model plumbing

\- codebase search and summarisation



Do not delegate:



\- Magpie's core memory semantics

\- claim lifecycle design

\- provenance/trust policy

\- Deadbolt integration authority

\- broad architecture decisions

\- anything requiring product judgement



Before delegation, write a ticket under `.agent-runs\\pending\\` with:



1\. Goal

2\. Scope

3\. Non-goals

4\. Constraints

5\. Tests

6\. Acceptance criteria



After Codex returns:



1\. Read the summary.

2\. Inspect the diff.

3\. Run tests yourself.

4\. Recommend accept, revise, or discard.

5\. Do not merge without human approval.



The desired workflow is:



Claude decides -> Codex implements -> Claude reviews -> human merges.

On Windows, invoke the Codex wrapper as:



`powershell.exe -ExecutionPolicy Bypass -File .\\scripts\\codex-delegate.ps1 read <ticket>`



or:



`powershell.exe -ExecutionPolicy Bypass -File .\\scripts\\codex-delegate.ps1 write <ticket>`



Do not call `codex` directly unless explicitly asked. Prefer the wrapper.

