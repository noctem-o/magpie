# Evidence Admission Boundary Contract

## Status

Design exploration / proposed boundary contract. Docs-only. No evidence
admission mechanism exists, no evidence registration mechanism exists, and
no resolver implementation is defined here. This document defines no
schemas, APIs, or payload formats. It becomes doctrine only through
explicit repository-owner acceptance, and any adoption of its semantics
requires explicit governance — an ADR or contract amendment.

## Purpose

The admission record boundary established that historical inclusion is not
evidence assignment. This document explores the next boundary:

```text
historical record
        |
        v
evidence interpretation / registration
        |
        v
standing resolver inputs
```

It answers one question:

> What prevents recorded material from silently becoming evidence?

It does not answer how evidence is registered, how evidence is weighted,
or how standing is calculated. Those remain separate governance surfaces.

Governing documents: ADR-0002, the six ratified boundary contracts, and
the admission exploration set including
`admission-boundary-minimal-semantics` and
`admission-record-boundary-contract`. Where this document touches those
areas, they govern. This contract narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Historical record versus evidence boundary

Historical record ≠ evidence.

Magpie history may contain observations, proposals, occurrences,
references, imported records, and execution anchors. Presence in history
alone grants no evidence status. The distinction to preserve:

```text
recording something happened ≠ asserting that it supports a claim
```

## 2. Evidence interpretation boundary

```text
historical record
        |
        v
evidence interpretation
        |
        v
resolver input
```

- Evidence semantics must not be created by mere inclusion.
- Evidence interpretation belongs to governed policy and resolver paths.
- Admission and evidence interpretation answer different questions:
  admission asks "may this become part of Magpie history?"; evidence
  interpretation asks "what, if anything, does recorded history support?"
  — and even the second is answered only by a resolver, not by an
  interpretation layer acting on its own.

Preserved: admission ≠ evidence assignment.

## 3. Evidence vocabulary protection

ADR-0002 closure is preserved:

- `evidence_kind` remains closed;
- `actor_class` remains closed;
- `edge_kind` remains closed;
- claim-domain vocabulary remains governed.

Vocabulary presence grants no authority. No external system, agent, MCP
layer, or imported record may create new epistemic vocabulary by appearing
as evidence.

## 4. Evidence and provenance

Preserved per the provenance response contract:

- recorded attribution;
- provenance coordinates;
- source linkage;
- historical placement.

The distinctions stay intact:

```text
attribution ≠ verified identity
identity ≠ trust
source ≠ authority
origin ≠ evidential weight
```

A provenance coordinate allows inspection and replay; it does not itself
establish reliability.

## 5. AgentProposer boundary

```text
AgentProposer
        |
        v
proposal material
        |
        ?
        |
        v
evidence-related status
```

The unresolved boundary must not be filled by model confidence,
repetition, popularity, identity, capability exposure, or autonomous
judgement. Stated without qualification:

```text
proposal ≠ evidence
proposal ≠ settlement
proposal ≠ authority
```

Agent output remains attributed asserted data unless a future governed
process determines otherwise.

## 6. Deadbolt relationship

Existing seam doctrine is preserved:

- Deadbolt anchor semantics remain independently governed by their own
  ratified contract;
- a specialised seam does not define generic evidence semantics;
- wrapping a seam grants no additional authority.

No Deadbolt redesign is proposed or implied.

## 7. Resolver separation

```text
evidence interpretation
        |
        v
standing resolver
        |
        v
epistemic status
```

- Evidence-related material may become resolver input.
- Resolver output remains deterministic over verified history.
- Policy selection remains explicit; there is no ambient latest.

Evidence participation must not become standing assignment.

## 8. Failure semantics

Failure is not falsity. A failure in an evidence-related path is a failure
of the attempt, not evidence against the proposition. This document
defines no evidence rejection workflows, no scoring, no confidence, no
retry handling, and no operational processes.

## 9. Remaining questions

Unresolved, each requiring a future ADR or contract amendment:

- what exact event represents evidence interpretation;
- whether evidence assignment is itself historical, derived, or both;
- how evidence kinds are selected under governance;
- how conflicting evidence interpretations are represented;
- what belongs in the evidence layer versus the standing resolver.

None is answered here, and none should be answered implicitly by an
implementation.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` implementation;
- evidence APIs or schemas;
- MCP write surfaces;
- trust systems;
- confidence scores;
- ranking or retrieval systems;
- agent autonomy.

## Reviewer checklist

- Does this preserve historical record ≠ evidence?
- Does it protect the closed vocabularies?
- Does it prevent provenance becoming trust?
- Does it prevent AgentProposer output becoming evidence automatically?
- Does it preserve the Deadbolt seam boundaries?
- Does it keep standing resolution separate?
- Does it avoid mechanisms and schemas?
