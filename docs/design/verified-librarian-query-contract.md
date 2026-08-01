# Verified Librarian Query Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server, librarian, or query service exists in this
repository.

## Purpose

This document defines the **query contract** for the future read-only
librarian surface: what a query is, what a query result is, and what neither
may ever become. It sits below two governing documents:

- `docs/design/mcp-boundary-contract.md` — the membrane between Magpie and
  any MCP layer;
- `docs/design/mcp-librarian-contract.md` — the read-only librarian surface.

If anything here appears to conflict with either, those documents govern.
This contract narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Purpose and scope

A query inspects verified Magpie state. Within that frame:

- queries do not create knowledge — a query result adds nothing to the
  record;
- queries do not change history — the log is append-only and queries are
  not appends;
- query results are reproducible views over recorded history, computed from
  a verified record snapshot or its derived projections.

This contract is conceptual. It defines no schemas, APIs, transports, or
query language. Those are implementation design, pinned separately and
later, under the governing documents.

## 2. Query classes

Three conceptual query categories are permitted. They name kinds of reads,
not data formats.

- **Record queries** retrieve what the log records: recorded claims,
  evidence, and justification edges, including their recorded attribution
  metadata (closed `actor_class`, `evidence_kind`, and `edge_kind` labels as
  asserted data).
- **Projection queries** retrieve derived views: search-projection results,
  claim graph views, standing views, and audit traces.
- **Provenance queries** retrieve recorded lineage: origin-admission
  information, event ancestry within the chain, and recorded attribution.

No other query category is licensed by this contract. A read that cannot be
framed as one of these three is out of scope for the librarian surface.

## 3. Query authority boundary

A query result is:

- a view of recorded history;

and is never:

- a new assertion;
- evidence;
- a standing decision.

The librarian cannot:

- merge conflicting records;
- decide truth;
- resolve contradictions;
- upgrade standing (standing is a named standing resolver's output over a
  verified record snapshot, never a query byproduct);
- infer missing facts.

The caller may reason over returned material. The librarian itself does not
reason into authority.

## 4. Result provenance contract

Every result derived from Magpie state must carry enough provenance,
conceptually, to identify:

- the verified record snapshot or log prefix it was computed from;
- the projection or standing resolver that produced it;
- the policy version, where a resolver result is involved.

This contract defines no wire format for those identifiers; their concrete
naming is implementation design under the governing documents' deferred
"reproducible query naming" question.

Policy selection is explicit. There is no ambient latest.

## 5. Failure contract

Queries return typed failure conditions, never epistemic conclusions:

- a missing artifact is not falsehood;
- a malformed record is a typed failure, not a rejection of what the record
  was about;
- an unknown kind remains unknown — surfaced as such, not interpreted into
  the nearest known answer;
- absence is absence: the lack of a record is reported, never presented as
  evidence against.

Failure is not falsity.

## 6. Projection contract

Projection outputs may be exposed through query results. The subordination
rules of the governing documents apply unchanged:

- projections are rebuildable derived views over recorded history;
- projections are not history;
- projection storage is not canonical — SQLite files, page layouts, and
  index structures are implementation representations, never authority;
- cache state has no authority: server-side state must be reconstructable
  from the log, or it must not influence answers.

## 7. Determinism and reproducibility

An implementation of this contract must make every query derivation
reproducible from:

- recorded history;
- explicit snapshot identity (the verified record snapshot or log prefix);
- explicit policy and standing-resolver identity, where applicable.

Reproducibility attaches to the derivation, not to phrasing: nothing here
requires identical natural-language responses, only that the underlying
answer is recomputable from the same coordinates.

## 8. Non-goals

This document adds, and the surface it bounds may add, none of the
following:

- autonomous agents or autonomous memory ingestion;
- retrieval ranking, confidence scoring, or truth scoring;
- hidden summarization layers (convenience summaries remain subordinate to
  recorded provenance, per the librarian contract);
- write paths of any kind, or policy decisions;
- an MCP server, SDK, transport, or query-language definition;
- new schemas, payload tags, canonical-encoding changes, or verifier
  changes;
- embeddings or vector search;
- `EpistemicGate` work.

## Relationship to the governing documents

This contract is a strict narrowing of
`docs/design/mcp-librarian-contract.md`, which is itself a strict narrowing
of `docs/design/mcp-boundary-contract.md`. Nothing here licenses writes,
creates authority, weakens the four-way boundary separation, or exempts any
result from the provenance, projection, and failure rules above.

## 9. Reviewer checklist

- Could an engineer implement against this contract without giving the
  librarian authority?
- Are queries still read-only in every section?
- Is provenance preserved on every derived result?
- Are projections still subordinate to the log?
- Are failures prevented from becoming negative facts?
- Are no schemas or APIs accidentally introduced?
