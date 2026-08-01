# Provenance Response Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP implementation exists in this repository, and no response
schema is introduced here.

## Purpose

This document defines the minimum provenance requirements when
Magpie-derived information leaves the governed core through future read-only
surfaces. It answers one question:

> What must accompany a Magpie-derived response so that an agent never
> receives an opaque conclusion detached from the history that produced it?

It sits below three governing documents:

- `docs/design/mcp-boundary-contract.md` — the membrane between Magpie and
  any MCP layer;
- `docs/design/mcp-librarian-contract.md` — the read-only librarian surface;
- `docs/design/verified-librarian-query-contract.md` — queries as
  reproducible views over verified history.

If anything here appears to conflict with any of them, those documents
govern. This contract defines provenance carriage only; it creates no
authority.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Provenance requirement

Any response derived from Magpie state must remain connected to recorded
provenance. Such a response must conceptually identify:

- the verified record snapshot or log prefix it was computed from;
- the derivation class of the answer: recorded history, derived projection,
  standing resolver output, or audit trace;
- the relevant standing resolver and policy version, where a resolver
  result is involved;
- the recorded attribution metadata, where the underlying records carry it.

This contract defines no wire formats, no JSON structures, and no APIs.
Concrete identifier naming remains implementation design under the governing
documents' deferred "reproducible query naming" question.

## 2. Authority boundary

A provenance package is not authority.

It is:

- a pointer to origin;
- a reproducibility coordinate;
- an audit aid.

It is not:

- evidence by itself;
- a new claim;
- a standing decision;
- a trust score;
- a confidence score.

Vocabulary presence grants no authority: a response that names its snapshot,
resolver, or attribution has described its origins, not earned standing.

## 3. Summary boundary

Convenience summaries may exist. However:

- summaries cannot replace provenance;
- summaries cannot detach conclusions from source history;
- summaries cannot become new Magpie records;
- summaries cannot upgrade standing.

The receiving agent may reason over returned material. The interface layer
must not convert summarisation into authority.

## 4. Attribution boundary

Attribution is recorded, not trusted identity. Responses exposing
`actor_class`, `evidence_kind`, or `edge_kind` values must present them as
recorded labels — asserted data carried by the record, never as verified
identity or authority.

This contract introduces no identity systems and no cryptographic agent
identity. Those remain future ADR territory, exactly as the boundary
contract's deferred "verified agent identity" question records.

## 5. Projection boundary

Existing projection doctrine applies unchanged:

- projections are rebuildable derived views over recorded history;
- projection storage is not canonical;
- caches are projections;
- derived answers remain subordinate to the signed append-only log.

A provenance reference that names a projection identifies the derivation
path; it must not elevate the projection above the log. The snapshot or log
prefix the projection was built from remains the authority coordinate.

## 6. Failure and incomplete provenance

- Missing provenance is a failure condition, reported as such.
- Incomplete provenance must not silently become a normal answer: a
  response that cannot identify its derivation coordinates is a typed
  failure, not a degraded-but-ordinary result.
- Unknown provenance is not permission to invent provenance.

Failure is not falsity.

## 7. Non-goals

This document adds, and the surface it bounds may add, none of the
following:

- an MCP implementation, SDK, or transport;
- response schemas, wire protocols, or query-language definitions;
- agent identity systems or signing changes;
- new vocabularies, payload tags, or canonical-encoding changes;
- embeddings, retrieval ranking, confidence scoring, or truth scoring;
- `EpistemicGate` work.

## 8. Reviewer checklist

- Could an engineer implement a response surface against this contract
  without creating authority?
- Is every derived answer traceable to recorded provenance?
- Are summaries subordinate to provenance in every section?
- Are projections still subordinate to the log?
- Is attribution still recorded rather than trusted?
- Does this document introduce any schema or implementation assumption?
