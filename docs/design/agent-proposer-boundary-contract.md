# AgentProposer Boundary Contract

## Status

Proposed boundary contract. It becomes ratified doctrine only by explicit
repository-owner acceptance of the PR that introduces it. This document is
docs-only: no MCP server, admission mechanism, or proposer implementation
exists in this repository.

## Purpose

`AgentProposer` is an existing member of the closed `actor_class`
vocabulary (ADR-0002, `docs/FORMAT.md`). This document defines the boundary
around future AgentProposer behaviour — the maximum semantic weight an
agent-generated proposal may carry before governed admission. It sits below
five governing documents:

- `docs/design/mcp-boundary-contract.md` — the membrane between Magpie and
  any MCP layer;
- `docs/design/mcp-librarian-contract.md` — the read-only librarian surface;
- `docs/design/verified-librarian-query-contract.md` — queries as
  reproducible views over verified history;
- `docs/design/provenance-response-contract.md` — provenance carriage for
  derived responses;
- `docs/design/mcp-capability-boundary-contract.md` — capability discovery
  as metadata only.

If anything here appears to conflict with any of them, or with ADR-0002,
those documents govern. This contract narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. AgentProposer role

AgentProposer denotes an agent, model, or interface layer that produces
proposals or suggestions.

AgentProposer is not:

- a source of truth;
- an evidence authority;
- a standing authority;
- a policy authority;
- a replacement for governed admission.

Agents may propose. They cannot settle claims.

## 2. Proposal boundary

A proposal is not automatically:

- a claim;
- evidence;
- a justification edge;
- a standing contribution;
- a policy decision.

The existence of generated content grants no authority. Model output, agent
output, summaries, annotations, and suggestions do not enter Magpie
epistemic structures merely because they exist. Per the boundary contract's
four-way separation, such content is an agent-generated suggestion until it
is submitted as draft proposal material and admitted through the governed
path.

## 3. Vocabulary protection

ADR-0002 vocabulary closure is preserved. Agent-generated content must not
create:

- new `actor_class` values;
- new `evidence_kind` values;
- new `edge_kind` values;
- new claim-domain vocabulary.

Vocabulary presence grants no authority. Model-generated content continues
to map to `AgentProposer` attribution where applicable — recorded as
asserted data, never as verified identity.

## 4. Admission boundary

The future path is:

```text
AgentProposer
    |
    v
proposal / suggestion (draft proposal material)
    |
    v
future governed admission mechanism
    |
    v
LogWriter
```

This contract defines none of that mechanism. It does not define
`EpistemicGate` internals, staging, or review flow. It establishes only
that proposal generation and historical admission are separate concerns:
generation happens outside Magpie's authority, admission happens through
the governed path, and `LogWriter` remains the sole append authority.

## 5. Provenance requirements

Any future admitted agent-originated material must preserve provenance per
the provenance response contract and the boundary contract: recorded
attribution through the closed vocabularies, with snapshot and record
coordinates carried as those contracts define.

This contract introduces no:

- trust scores;
- confidence scores;
- identity verification;
- autonomous authority;
- ranking systems.

Recorded attribution is not verified identity. Verified agent identity
remains future ADR territory, exactly as the boundary contract's deferred
question records.

## 6. MCP relationship

MCP doctrine is preserved. Exposing AgentProposer-related interfaces
through MCP grants:

- no write authority;
- no admission authority;
- no evidence authority;
- no standing authority.

MCP remains an interface layer only. Before `EpistemicGate` exists, no
ordinary claim-bearing or evidence-bearing write surface exists at all, and
this contract creates none.

## 7. Deadbolt relationship

Deadbolt boundaries are preserved. AgentProposer exposure cannot extend
Deadbolt authority, and wrapping an independently governed seam — such as
Deadbolt anchoring — grants no additional authority. Separately reviewed
capability seams retain exactly the authority granted by their own
contracts.

## Non-goals

This document adds, and the surface it bounds may add, none of the
following:

- an `EpistemicGate` implementation or admission mechanism design;
- agent identity systems, authentication, permissions, or capability
  tokens;
- MCP write tools;
- ingestion pipelines or memory mutation;
- retrieval systems, embeddings, or scoring/ranking;
- model evaluation systems;
- new vocabularies, schemas, payload tags, or canonical-encoding changes.

## Relationship to the governing documents

This contract is a strict narrowing of the five governing documents named
in the Purpose and of ADR-0002's actor-class doctrine. Nothing here
licenses writes, creates authority, extends a closed vocabulary, defines an
admission mechanism, or exempts agent-originated material from provenance,
projection, and failure rules.

## Reviewer checklist

- Could an engineer build proposal tooling against this contract without
  giving generated content authority?
- Do proposals remain draft material until governed admission, with
  `LogWriter` as the sole append capability?
- Are all four closed vocabularies protected from agent-generated
  extension?
- Is attribution still recorded rather than verified identity?
- Do MCP and Deadbolt boundaries survive AgentProposer exposure unchanged?
- Does this document introduce any mechanism, schema, or implementation
  assumption?
