# MCP Capability Boundary Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server exists in this repository, and nothing here
defines, names, or licenses any concrete capability.

## Purpose

This document defines the boundary around **future MCP capability
discovery**: what a capability description is, and what exposing one can
never mean. It sits below four governing documents:

- `docs/design/mcp-boundary-contract.md` — the membrane between Magpie and
  any MCP layer;
- `docs/design/mcp-librarian-contract.md` — the read-only librarian surface;
- `docs/design/verified-librarian-query-contract.md` — queries as
  reproducible views over verified history;
- `docs/design/provenance-response-contract.md` — provenance carriage for
  derived responses.

If anything here appears to conflict with any of them, those documents
govern. This contract narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Capability role

A capability description is:

- a description of an available interface operation.

It is never:

- a source of truth;
- evidence;
- standing;
- permission to modify Magpie.

Capability discovery describes what an interface can request. It does not
describe what Magpie believes.

## 2. Capability authority boundary

A capability name does not grant authority. A discovered operation is
never:

- a claim;
- evidence;
- a policy decision;
- a trust signal;
- a permission escalation;
- a substitute for governed admission.

The MCP layer cannot create authority merely by exposing a tool.
Vocabulary presence grants no authority — and a capability list is
vocabulary.

## 3. Read capability boundary

Before `EpistemicGate` exists:

- capability discovery may expose read-only librarian and query surfaces
  only;
- no ordinary claim-bearing or evidence-bearing write capabilities exist;
- no hidden write paths exist through annotations, memory, caches, or
  server-side state.

The Deadbolt seam rule is preserved: separately reviewed capability seams
retain exactly the authority granted by their own contracts, and wrapping
such a seam in MCP grants no additional authority.

## 4. Capability versus provenance

A capability that returns Magpie-derived information obeys the governing
contracts unchanged:

- results are computed from a verified record snapshot or log prefix;
- responses carry recorded provenance per the provenance response contract;
- derived projections remain subordinate to the signed append-only log;
- failure semantics apply — failure is not falsity.

Holding a capability does not remove a single provenance requirement.

## 5. Capability versus identity

This contract introduces none of the following, and a capability surface
built under it may not smuggle them in:

- agent identity;
- authentication;
- authorization systems;
- capability tokens;
- trust systems.

Verified agent identity remains future ADR territory, exactly as the
boundary contract's deferred question records.

## 6. No ambient capability authority

- The newest capability list is not authoritative by virtue of being
  newest; there is no ambient latest.
- A capability's semantics must not change silently; changes are reviewed
  contract amendments, not implementation drift.
- Capability names are not policy selection. Policy selection is explicit,
  and no discovered name may substitute for a named standing resolver and
  policy version.
- Tool availability is not epistemic endorsement: a client must not treat
  an operation's presence as Magpie affirming anything.

## 7. Non-goals

This document adds, and the surface it bounds may add, none of the
following:

- an MCP server implementation or SDK;
- transports, schemas, or a tool naming format;
- a permission model, authentication, or identity;
- write paths, or `EpistemicGate` work;
- retrieval systems, embeddings, or scoring.

## Relationship to the governing documents

This contract is a strict narrowing of the four governing documents named
in the Purpose. It defines the authority boundary of capability discovery
only. Nothing here licenses writes, creates authority, adds identity or
permission machinery, or exempts any capability from the provenance,
projection, and failure rules of the governing contracts.

## Reviewer checklist

- Could an engineer implement capability discovery against this contract
  without granting any capability authority?
- Does every capability returning Magpie-derived information remain bound
  to snapshot coordinates and provenance carriage?
- Is the pre-`EpistemicGate` surface still read-only, with separately
  reviewed seams governed by their own contracts?
- Is capability discovery still free of identity, authentication, and
  permission machinery?
- Does nothing here define a tool name, schema, transport, or API?
