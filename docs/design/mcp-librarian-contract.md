# MCP Librarian Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server, librarian, or query service exists in this
repository.

## Purpose

The MCP boundary contract (`docs/design/mcp-boundary-contract.md`) ratified
the membrane between Magpie and any future MCP layer: MCP is an optional
interface layer, writes route only through a governed admission path, and
before `EpistemicGate` exists an MCP implementation exposes no ordinary
claim-bearing or evidence-bearing write tools.

This document narrows one slice of that boundary to a contract: the
**read-only librarian surface** — how an MCP implementation may expose
verified Magpie state and derived projections to agents and tools before any
write path exists. Everything here is a restriction of the boundary
contract, never an exception to it. Where the two appear to conflict, the
boundary contract governs.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Librarian role

The librarian is the read-only member of the future MCP surface family: the
README's "read-only librarian or research navigator" given its contract.

- It is an optional interface for agents and tools: LLM clients, agent
  harnesses, and editor or tool integrations.
- It exposes verified Magpie state and derived projections. Nothing else.
- It is never a source of truth. Every answer is derived from a verified
  record snapshot and remains reproducible from recorded history.
- It has no write path of any kind — no ordinary writes (none exist before
  `EpistemicGate`), and no back-door writes through caches, annotations, or
  server-side state.

## 2. Allowed read operations

The librarian may:

- answer queries against a verified record snapshot or its derived
  projections;
- retrieve claim, evidence, and edge records as recorded, including their
  closed `actor_class`, `evidence_kind`, and `edge_kind` attributions;
- retrieve provenance and origin-admission information as recorded;
- retrieve deterministic standing and contribution traces from a named
  standing resolver and policy version;
- expose audit explanations. Standing and contribution traces are
  deterministic, one-way audit output: they can be inspected, stored, and
  compared, but not fed back in as authority.

The librarian may not:

- mutate anything — it holds no `LogWriter`, no signing authority, and no
  write tools of any kind;
- present retrieved material as settled truth — records are exposed as
  recorded history and resolver outputs, with their provenance;
- adjudicate, merge, or editorialize across records.

## 3. Required provenance

The boundary contract's provenance requirement, carried into read practice:

- Responses derived from Magpie state must identify their underlying
  recorded provenance.
- Convenience summaries are allowed, but a summary cannot replace
  provenance: an agent must never receive an opaque conclusion detached from
  its source history.
- Claim, evidence, and edge answers carry their recorded attribution
  (`actor_class` and kind labels) as asserted data — vocabulary presence
  grants no authority, and the librarian presents attribution as recorded,
  never as verified identity.

## 4. Projection boundary

- Derived projections are rebuildable views over recorded history: search
  indexes, claim graphs, standing views, audit traces.
- Canonical authority remains the signed append-only log and its canonical
  encodings. Projection outputs and query semantics are deterministically
  reproducible from recorded history; storage representations (SQLite pages,
  FTS indexes, cache layouts) are not canonical.
- Caches and indexes the librarian maintains are themselves projections:
  droppable, rebuildable, never authoritative. Server-side state must be
  reconstructable from the log, or it must not influence answers.

## 5. Failure semantics

- Typed failures stay typed failures. A missing artifact, a malformed
  record, a binding mismatch, or an unknown kind is a failure report, never
  a negative fact. Failure is not falsity.
- Unknown kinds and errors are surfaced as such, not interpreted into the
  nearest known answer.
- Absence of a record is reported as absence; it is not evidence of
  falsehood, and the librarian must not present it as one.

## 6. Reproducibility

- Query answers are tied to explicit coordinates: the verified record
  snapshot or log prefix they were computed from, and — for standing
  answers — the named standing resolver and policy version.
- Policy selection is explicit. There is no ambient latest: the librarian
  never silently selects a resolver, policy version, or snapshot on a
  caller's behalf.
- The exact wire naming of snapshot, policy, and version identifiers is
  interface design, pinned at implementation time (the boundary contract's
  deferred "reproducible query naming" question). This contract states the
  requirement, not the format.

## 7. Non-goals

This document adds, and the surface it bounds may add, none of the
following:

- an MCP server, librarian implementation, SDK, or transport binding;
- writes of any kind, or an `EpistemicGate` implementation;
- embeddings, vector retrieval, or retrieval scoring;
- autonomous memory ingestion;
- new vocabularies, schemas, payload tags, or canonical-encoding changes;
- network dependencies in Magpie core.

## Relationship to the boundary contract

This contract is a strict narrowing of
`docs/design/mcp-boundary-contract.md`. It defines the read side only, for
the pre-`EpistemicGate` period in which read-only is the only MCP surface
that may exist. Nothing here licenses writes, weakens the four-way boundary
separation, or grants the librarian any authority the boundary contract
withholds.

## Reviewer checklist

- Confirm the librarian is read-only in every section: no write tool, no
  cache write-back, no annotation channel.
- Confirm every answer class is tied to a verified record snapshot or log
  prefix, and standing answers to a named standing resolver and policy
  version.
- Confirm provenance, projection, and failure wording matches the boundary
  contract, README, and the v0.1.0 contract.
- Confirm non-goals exclude implementation, transport, retrieval machinery,
  and gate work.
