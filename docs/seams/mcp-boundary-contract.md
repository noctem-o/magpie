# MCP Boundary Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server exists in this repository, and nothing here licenses
building one outside the boundary below.

## Purpose

This document defines the contract that any future MCP (Model Context
Protocol) server must obey when exposing Magpie to agents and tools. The
architecture closure audits identified an MCP-first implementation layer as
the next practical step; this contract pins the boundary that layer must
respect *before* any implementation exists, so a future developer cannot
accidentally violate Magpie's invariants while building it.

It is a protocol boundary, not a runtime dependency — the same genre as
`docs/seams/deadbolt-anchor-contract.md`. The base invariant applies
unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## MCP role

- MCP is an **interface layer for agents and tools**: LLM clients, agent
  harnesses, and editor or tool integrations.
- It exposes **controlled interaction** with Magpie: reading state,
  submitting proposals, and requesting derived explanations.
- It is **not a source of truth**. The signed append-only log is the sole
  historical authority. An MCP server adds no authority by existing, by
  speaking for a model, or by aggregating many clients.
- An MCP server is optional tooling around a Magpie installation. Core tests,
  examples, `tools/verify_chain.py`, replay, and projections must remain
  runnable with no MCP server present, exactly as the portable base requires
  of Deadbolt today.
- MCP is one future surface among several. Existing doctrine speaks of
  "ordinary CLI, MCP, or application writer" surfaces and of read-only
  librarian/navigator tooling; this contract gives the MCP member of that set
  its boundary. No MCP capability is privileged over the equivalent CLI or
  application path.

## Write boundary

Four categories must stay distinct. Collapsing any pair of them is the
failure this contract exists to prevent.

1. **Events submitted to Magpie.** A proposal that enters the log through
   the governed write path and becomes a signed, hash-chained event. Only
   these are history.
2. **Queries.** Read-only requests against a verified replay or its derived
   projections. A query never appends, and its answer is derived state.
3. **Derived projections.** Search results, claim graphs, standing views, and
   traces served by the server. They are regenerable from recorded history,
   may be dropped and rebuilt, and explain history without writing authority
   back into it.
4. **Agent-generated suggestions.** Content an agent drafts in MCP
   conversation that has not been submitted and admitted. A suggestion is not
   an event, has no standing, and must never be represented as Magpie state.
   It acquires a record only by being submitted through the governed path as
   an attributed proposal; its authority then comes solely from that recorded
   event and policy evaluation — never from the suggesting model.

The MCP layer may:

- submit proposed records/events;
- query existing Magpie state;
- request explanations or projections (standing and contribution traces are
  deterministic, one-way audit output; they can be inspected, stored, and
  compared, but not fed back in as authority).

The MCP layer must not:

- mutate historical records (the log has no update or delete; a correction is
  a later signed event, and the prior record remains inspectable);
- rewrite existing events (canonical bytes are frozen; any "edit" is a new
  event under the format evolution rule, not a modification);
- directly alter standing (governed standing is a named resolver's result
  over replayed history; no MCP tool may write, patch, or cache standing as
  authority);
- bypass provenance (every submitted event carries recorded attribution and
  enters through the admission path; there is no anonymous or
  server-adjudicated write);
- insert hidden authority decisions (the server performs no adjudication of
  its own, makes no truth claims, and does not select policies silently —
  policy selection is explicit, and there is no ambient latest).

### The governed-path clause

Existing doctrine (ADR-0002 rollout; `historical-review-remediation-ledger`
§5.3) requires that before any ordinary CLI, MCP, or application
claim-bearing writer is exposed, a separately reviewed `EpistemicGate` must
govern event construction and admission, invoking the existing raw append
capability while keeping L0 structural validation separate from epistemic
policy. Consequences for MCP:

- An MCP server never holds `LogWriter` or a signing key. `LogWriter`
  remains the sole low-level append capability.
- MCP write tools may exist only over the governed write path. Before the
  gate exists, an MCP implementation is read-only.
- MCP does not replace or weaken the gate: admission policy is decided in the
  kernel-side governed path, never in protocol handling.

## Event ownership

- The append-only log remains authoritative. MCP responses describe the log
  and its projections; they never compete with them.
- MCP clients are **actors producing signed/attributed events**. Events caused
  by an MCP client are signed at the governed boundary and carry attribution
  through the closed `actor_class` vocabulary (`HumanRoot`, `AgentProposer`,
  `AutomatedVerifier`, `DeadboltAnchorer`, `LensWitness`, `SourceImporter`).
  An ordinary model client maps to `AgentProposer`: it may propose claims,
  evidence, and review debt, but it cannot settle claims.
- Attribution is recorded, not verified identity. Per the v0.1.0 contract,
  self-asserted actor classes and labels establish no authority. Per-agent
  signing keys and custody are future work (production key custody and
  rotation are already on the future list) and are deferred below.
- Derived state is regenerated from recorded history. Anything an MCP server
  caches, indexes, or summarizes is itself a projection: droppable,
  rebuildable, and never authority. Server-side state must be reconstructable
  from the log, or it must not influence answers.
- Query answers come from a verified record snapshot or projections of one.
  Typed failures stay typed failures — a missing artifact, a malformed
  record, or an unknown kind surfaced over MCP is a failure report, never a
  negative fact. Failure is not falsity.

## Future compatibility

This contract deliberately leaves room for:

- **A local MCP server.** Local-first transport for a single workstation or
  hub. Magpie core must never require an always-on server, and MCP must not
  become a base dependency.
- **Multiple agents** (Claude, Codex, Kimi, local models, and others). Every
  client enters through the same governed path under the same closed
  vocabulary. No client family is privileged, and multiplicity does not
  amplify: N agents submitting equivalent material is policy input, not
  N-fold authority. Repetition does not amplify without policy.
- **Future provenance graphs.** Provenance and origin admission are staged
  projections today. MCP responses should expose recorded provenance rather
  than summarize it away, and nothing here forecloses richer provenance-graph
  projections later.
- **Future epistemic features.** `EpistemicGate`, contradiction debt,
  invalidation, supersession, currentness, broader policies, and new payload
  tags are format and governance decisions made through ADRs and the freeze
  ritual — not MCP decisions. This contract is amended by them explicitly,
  never silently.

## Non-goals

This document adds, and the layer it bounds may add, none of the following:

- an MCP server implementation, SDK, or transport binding;
- code, payload tag, canonical-encoding, golden-vector, or verifier changes;
- new actor classes, evidence kinds, or edge kinds (those vocabularies are
  closed; extending them is an amendment there first);
- embeddings, vector databases, or retrieval scoring;
- autonomous memory ingestion (crawlers, watchers, or self-writing agents);
- scoring systems, numeric confidence values, or truth claims;
- formal verification requirements;
- network dependencies in Magpie core.

These are possible future layers, each earning its place through its own
reviewed contract. None of them enters through an interface layer by default.

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

- Confirm the contract adds no writer authority: all writes route through the
  governed path, and a pre-gate MCP implementation is read-only.
- Confirm the four write-boundary categories stay distinct and that the
  must-not list covers each authority escape route current doctrine names.
- Confirm terminology matches README, ADR-0002, `docs/FORMAT.md`, and the
  v0.1.0 contract: `LogWriter`, `EpistemicGate`, closed `actor_class`, named
  standing resolvers, and derived projections.
- Confirm portability: nothing here makes MCP a base requirement or a core
  dependency.
- Confirm the non-goals exclude the deferred future layers named above.
