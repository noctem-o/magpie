# MCP Boundary Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server exists in this repository, and nothing here licenses
building one outside the boundary below.

## Purpose

This document defines the architectural contract that any future MCP (Model
Context Protocol) layer must obey when exposing Magpie to agents and tools.
The architecture closure audits identified an MCP-first implementation layer
as the next practical step; this contract pins the boundary that layer must
respect *before* any implementation exists, so a future developer cannot
accidentally give the interface layer authority that belongs only to Magpie's
governed core.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. MCP role

MCP is an optional interface layer for agents and tools: LLM clients, agent
harnesses, and editor or tool integrations.

It may:

- expose verified Magpie state (query results over a verified record
  snapshot or its derived projections);
- expose deterministic projections (search, claim graph, standing views,
  traces);
- submit draft proposal material to an approved governed admission path
  once such a write path exists (§3) — MCP does not own admission and does
  not decide whether something enters Magpie history.

It may not:

- become a source of truth — the signed append-only log is the sole
  historical authority;
- create authority — speaking for a model, aggregating many clients, or
  persisting server-side state grants nothing;
- replace governance — admission and policy evaluation live in Magpie's
  governed core, never in protocol handling;
- adjudicate standing (§4);
- represent model output as Magpie knowledge (§2, category 4).

An MCP server is optional tooling around a Magpie installation. Core tests,
examples, `tools/verify_chain.py`, replay, and projections must remain
runnable with no MCP server present. MCP is not part of Magpie core, and
nothing in this contract makes it one.

## 2. Four-way boundary separation

Four categories must stay distinct. Confusing these categories — treating a
suggestion as an event, a projection as authority, or a query as a write —
is an architectural violation, not an implementation detail.

1. **Events submitted to Magpie** are signed historical records. Only these
   become history. The log is append-only: there is no update or delete; a
   correction is a later signed event, and the prior record remains
   inspectable.
2. **Queries** are read-only requests against a verified replay or its
   derived projections. A query never mutates anything. Typed failures stay
   typed failures — a missing artifact, a malformed record, or an unknown
   kind is a failure report, never a negative fact. Failure is not falsity.
3. **Derived projections** are regenerable views: search indexes, claim
   graphs, standing views, audit traces. Their outputs and query semantics
   are deterministically reproducible from recorded history; their storage
   representations (SQLite pages, index layouts, server caches) are not
   canonical. Only the recorded history and its canonical encodings carry
   authority. Projections remain droppable and rebuildable; they explain
   history and never write authority back into it. Anything an MCP server
   caches, indexes, or summarizes is itself a projection — droppable,
   rebuildable, never authority.
4. **Agent-generated suggestions** are drafts. A suggestion an agent produces
   in MCP conversation is not an event, has no standing, and carries no
   epistemic authority until it enters the governed admission path and is
   recorded as an attributed event. Its authority then comes solely from that
   recorded event and policy evaluation — never from the suggesting model.

## 3. Write boundary

- MCP must not hold raw `LogWriter` capability. `LogWriter` remains the sole
  low-level append capability over the log.
- MCP must not possess signing authority.
- MCP writes, when they exist, route through governed paths: a separately
  reviewed `EpistemicGate` governs event construction and admission and
  invokes the raw append capability, keeping L0 structural validation
  separate from epistemic policy (ADR-0002 rollout;
  `historical-review-remediation-ledger` §5.3).
- Before `EpistemicGate` exists, ordinary claim-bearing or evidence-bearing
  MCP write tools remain unavailable; a pre-gate MCP server may expose state
  and projections only. Separately reviewed capability seams — such as
  Deadbolt anchoring — continue to obey their own contracts, and an MCP
  server that wraps such a seam gains no new authority by doing so.

The MCP layer may therefore: submit draft proposal material (through the
governed admission path, once it exists); query existing Magpie state; and
request explanations or projections. MCP submits drafts only. The governed
admission path constructs, validates, signs, and appends any historical
event; MCP clients never submit canonical records, signed events, or
append-ready log material, and `LogWriter` remains the sole append
capability. The chain of custody is: agent suggestion → draft proposal
material → governed admission → constructed canonical event → signed append
→ history. Standing and contribution traces are deterministic, one-way audit
output: they can be inspected, stored, and compared, but not fed back in as
authority.

## 4. Standing boundary

Standing remains a deterministic resolver output over verified history — a
named policy's result over a completely verified record snapshot, nothing
else. MCP must not:

- modify standing;
- cache standing as authority (a cached result is a projection snapshot:
  recomputable, never self-authenticating);
- decide policy;
- silently select resolver versions — policy selection is explicit, and
  there is no ambient latest.

## 5. Attribution boundary

Current doctrine is preserved unchanged:

- Model-generated content maps to `AgentProposer` in the closed
  `actor_class` vocabulary (`HumanRoot`, `AgentProposer`,
  `AutomatedVerifier`, `DeadboltAnchorer`, `LensWitness`, `SourceImporter`).
  An ordinary agent may propose claims, evidence, and review debt; it cannot
  settle claims.
- Attribution is recorded, not trusted identity.
- Self-asserted labels grant no authority (v0.1.0 contract: vocabulary
  presence grants no authority).
- Multiple agents do not amplify evidence automatically. N clients
  submitting equivalent material is policy input, not N-fold authority.
  Repetition does not amplify without policy.

## 6. Compatibility boundaries

This contract deliberately leaves room for:

- **Local MCP servers** — local-first transport; Magpie core must never
  require an always-on server.
- **Future multi-agent workflows** — Claude, Codex, Kimi, local models, and
  others enter through the same governed path under the same closed
  vocabulary; no client family is privileged.
- **Future provenance projections** — responses derived from Magpie state
  must identify their underlying recorded provenance. Convenience summaries
  are allowed, but a summary cannot replace provenance: an agent must never
  receive an opaque conclusion detached from its source history. Richer
  provenance-graph projections remain open.

Each of these is a future layer requiring its own reviewed contract; none of
them enters by default through an interface layer. Future epistemic features
(`EpistemicGate`, contradiction debt, invalidation, supersession,
currentness, new payload tags) are format and governance decisions made
through ADRs and the freeze ritual — they amend this contract explicitly,
never silently.

## 7. Non-goals

This document adds, and the layer it bounds may add, none of the following:

- an MCP server implementation, SDK, transport code, or MCP dependencies;
- new schemas, payload tags, canonical-encoding, golden-vector, or verifier
  changes;
- changes to actor, evidence, or edge vocabularies or to the closed
  claim-domain vocabulary (all are closed; extending any of them is an
  amendment through its existing governance path);
- autonomous memory ingestion;
- embeddings, vector search, or retrieval scoring;
- scoring systems, numeric confidence values, or truth claims;
- core network dependencies;
- any change to existing kernel authority boundaries.

## Deferred questions (future ADRs, not decided here)

- **Verified agent identity.** How attribution becomes cryptographic rather
  than self-asserted — per-agent keys, custody, and their relation to the
  single `LogWriter` signing key — needs its own ADR before any multi-agent
  write surface is treated as real.
- **Proposal staging.** Whether submissions queue as reviewable artifacts
  before gate admission (a proposal inbox) or are admitted synchronously is
  gate-design territory, deferred with `EpistemicGate`.
- **Transport and deployment shape.** Stdio-local server versus a hub service
  is operational design; this contract constrains authority, not sockets.
- **Reproducible query naming.** How an MCP response names the policy
  version, snapshot, or log prefix it was computed from, so results are
  reproducible across clients, is interface design to be pinned at
  implementation time, consistent with explicit policy selection.

## Reviewer checklist

- The final review question: could a future developer implement an MCP server
  from this contract without accidentally giving the interface layer
  authority that belongs only to Magpie's governed core? If any section
  admits doubt, the contract is incomplete.
- Confirm §3 routes every write through the governed path and that pre-gate
  MCP exposes no ordinary claim-bearing or evidence-bearing write tools.
- Confirm the four categories of §2 stay distinct.
- Confirm terminology matches README, ADR-0002, `docs/FORMAT.md`, the v0.1.0
  contract, and the Deadbolt anchor contract: `LogWriter`, `EpistemicGate`,
  closed `actor_class`, standing resolver, derived projections, append-only
  log.
- Confirm §7 excludes the deferred future layers named above.
