# Admission Boundary Scenario Analysis

## Status

Design exploration. Docs-only. This document makes no decisions and
proposes no mechanism; it stress-tests the intentionally unresolved
admission boundary through one worked example. ADR-0002, the ratified MCP
contracts, and the admission exploration documents
(`admission-boundary-questions`, `admission-boundary-threat-model`,
`admission-design-readiness-checklist`) govern wherever this note touches
them.

## Purpose

The questions exploration named what is unresolved, the threat model named
what could break, and the readiness checklist named what "ready" means.
This document walks a single concrete scenario through the unresolved
boundary to expose hidden assumptions that abstract wording can hide.

The base invariant constrains every observation:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Scenario description

A Deadbolt-governed execution occurs outside Magpie. Some occurrence
information exists externally. A future governed admission path may
eventually allow a historical record to reference that occurrence.

The scenario is deliberately abstract: no Deadbolt implementation details,
no API assumptions, no schemas. (The ratified Deadbolt anchor seam is the
one already-reviewed instance of such a crossing and remains governed by
its own contract; this scenario explores the generic, still-unresolved
case.)

## 2. Pre-admission state

What exists:

- an external occurrence, outside Magpie;
- whatever external provenance the governing process recorded on its own
  side;
- at most, recorded attribution from the external process — asserted data,
  not verified identity.

What does not exist:

- any Magpie historical record of the occurrence;
- any Magpie-side provenance carriage;
- any standing, candidacy, or epistemic status within Magpie.

What authority does the external occurrence have within Magpie? None.
Before any admission, the occurrence is not part of the record, and nothing
about it is claimable from Magpie state.

## 3. Boundary analysis

The unresolved transition:

```text
external material
        |
        ?
        |
        v
historical record
```

What question is admission answering? At most: "may this become part of
Magpie history?" What is it explicitly not answering?

```text
admission ≠ truth creation
          ≠ standing assignment
          ≠ policy decision
```

What structural properties might matter at the boundary is an open
question (the readiness checklist holds it). What epistemic questions must
remain elsewhere is settled: anything about what the occurrence *means*
belongs to the standing resolvers over verified history, not to the
boundary.

What must not be inferred from a successful admission: that the occurrence
is true, endorsed, settled, supported, or policy-relevant. Admission, if it
happens, produces inclusion — nothing else.

What authority must remain absent at the boundary: any authority derived
from identity, capability, interface exposure, repetition, or model
confidence.

## 4. Post-admission state

If a record is admitted, what has changed:

- the occurrence's reference is now part of Magpie history, at a specific
  chain position, signed and hash-chained like every event;
- its recorded attribution is preserved as asserted data;
- it may become available to replay, projections, and explicitly selected
  resolvers according to their governed inputs.

What has not changed — what the record is not implied to be:

- not automatically a claim, evidence, or a justification edge;
- not a standing contribution;
- not a policy decision.

What the admitted material remains, until a resolver says otherwise under
an explicit policy version: a recorded proposition, resolver input, and
externally attributed material — and any view of it remains a projection,
subordinate to the log.

Preserved without qualification:

```text
recording occurrence ≠ endorsing all claims about occurrence
```

## 5. Standing and policy separation

The stages remain separate:

```text
historical recording
        |
        v
standing resolver (explicitly selected policy version)
        |
        v
epistemic status
```

Admission feeds the first stage only. No status exists until a named
standing resolver produces one over a verified record snapshot, and the
boundary plays no role in that evaluation.

## 6. Threat model cross-check

Against the threat model's categories, the scenario as walked:

- **authority escalation** — avoided only if admission grants inclusion
  and nothing more (§3);
- **shadow history** — avoided only if the record enters through
  `LogWriter`, the sole append authority, with no side channel;
- **provenance loss** — avoided only if recorded attribution and
  coordinates survive the crossing (the provenance response contract);
- **identity leakage** — avoided only if attribution stays asserted data,
  never verified identity;
- **projection confusion** — avoided only if any later view of the record
  is treated as a rebuildable projection;
- **failure-as-falsity** — a failed or rejected crossing is a typed failure
  about the attempt, never a fact about the occurrence. Failure is not
  falsity.

Each "avoided only if" is a property the future mechanism must be designed
to guarantee; none is guaranteed by this analysis.

## 7. Remaining open questions

The walkthrough sharpens, but does not answer:

- what the boundary's own check is, once L0's structural validation and
  the resolvers' epistemic policy are subtracted;
- how a record "references" an external occurrence without importing
  unverified content — and what that reference may contain;
- whether the generic boundary differs from the already-ratified Deadbolt
  anchor seam in kind or only in review status;
- what, if anything, is recorded about a rejected crossing.

Each requires an ADR or contract amendment before implementation, per the
readiness checklist's gate.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design or admission mechanism;
- schemas, APIs, or implementation;
- a security, identity, or trust model;
- scoring of any kind.
