# Admission Record Boundary Contract

## Status

Design exploration / proposed boundary contract. Docs-only. No admission
mechanism exists, and none is defined here. This document defines no
schemas, APIs, or implementation. It becomes doctrine only through explicit
repository-owner acceptance of the PR that introduces it, and any adoption
of representation semantics requires explicit governance — an ADR or
contract amendment.

## Purpose

The admission exploration established the unresolved transition:

```text
external material
        |
        v
future governed admission boundary
        |
        v
LogWriter
        |
        v
historical record
```

The minimal-semantics exploration narrowed what admission may *decide*.
This document narrows the adjacent question:

> What exactly is the historical object that crosses this boundary?

Not what admission decides — what may become part of the historical
record. It explores and constrains that question without answering it
through implementation.

Governing documents: ADR-0002, the six ratified boundary contracts, and
the admission exploration set (`admission-boundary-questions`,
`admission-boundary-threat-model`, `admission-design-readiness-checklist`,
`admission-boundary-scenario-analysis`,
`admission-boundary-minimal-semantics`). Where this document touches those
areas, they govern. This contract narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Historical object boundary

```text
external material
        |
        v
admission representation
        |
        v
historical record
```

Three constraints frame whatever the "admission representation" turns out
to be:

- admission does not automatically import authority — crossing the
  boundary changes where material lives, never what it is worth;
- recording a reference is not endorsing the referenced material;
- recording occurrence is not asserting every interpretation of
  occurrence.

## 2. Reference versus content boundary

Distinct things a historical object could, in principle, carry — none
decided here:

- a reference to external material;
- copied external content;
- derived summaries;
- provenance coordinates;
- occurrence descriptions.

The danger this boundary exists to state: an external representation is
not external authority. Whatever form the object takes, its presence in
the record is inclusion, not endorsement — and a copied form would carry
no more authority than a referenced one. Which of these forms is
admissible, and in what combination, remains an open question (§8).

## 3. Provenance requirements

Whatever crosses must preserve, per the provenance response contract:

- recorded attribution;
- provenance coordinates;
- source linkage;
- historical placement.

The distinctions stay intact:

```text
attribution ≠ verified identity
identity ≠ trust
trust ≠ standing
```

## 4. Evidence boundary

Historical inclusion is not evidence assignment. Admission must not
silently create:

- evidence kinds;
- justification edges;
- standing contributions;
- resolver outcomes.

Evidence interpretation remains governed by the existing closed
vocabularies and resolver policy. A record that crosses the boundary is
material a resolver *may* evaluate under an explicit policy version —
never material that has already been evaluated by crossing.

## 5. AgentProposer relationship

```text
AgentProposer
    |
    v
proposal material
    |
    v
historical record
```

Preserved without relaxation:

- proposal does not become evidence by crossing;
- proposal does not become authority by crossing;
- proposal does not become settlement by crossing.

Generated material remains attributed asserted data unless a future
governed process determines otherwise.

## 6. Deadbolt relationship

Existing doctrine is preserved:

- the Deadbolt anchor seam remains independently governed by its own
  ratified contract;
- a specialised seam does not define generic admission semantics;
- wrapping a seam grants no additional authority.

## 7. Failure boundary

Failure is not falsity. An admission failure — rejected, malformed, or
incomplete — cannot become a claim about the material. This document
defines no retry handling, no rejection storage, and no operational
workflows; those are operational design, deferred.

## 8. Remaining questions

Unresolved, each requiring a future ADR or contract amendment:

- what exact representation may be admitted;
- when a reference becomes a sufficient historical record;
- what content, if any, is copied versus referenced;
- how external provenance coordinates are represented;
- how specialised seams relate to generic admission.

None is answered here, and none should be answered implicitly by an
implementation.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` implementation or admission mechanism design;
- APIs, schemas, or payload formats;
- MCP write paths;
- authentication or identity systems;
- permissions or capability models;
- trust systems;
- scoring or ranking systems.

## Reviewer checklist

- Does this document define what crosses the boundary without defining an
  implementation?
- Does it preserve `LogWriter` as the sole append authority?
- Does it prevent admission from becoming evidence assignment?
- Does it preserve provenance without creating identity or trust?
- Does it protect the AgentProposer and Deadbolt boundaries?
- Does it avoid schemas, APIs, and mechanisms?
