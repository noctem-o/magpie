# Epistemic Resolution Boundary Contract

## Status

Design exploration / proposed boundary contract. Docs-only. No resolver
implementation is defined here, and no evidence-registration or admission
mechanism exists or is designed. This document defines no schemas, APIs, or
event formats. It becomes doctrine only through explicit repository-owner
acceptance, and any adoption of its semantics requires explicit governance
— an ADR or contract amendment.

## Purpose

The pipeline separation contract consolidated the stages; this document
defines the membrane between the last two:

```text
evidence-related interpretation
        |
        v
standing resolver
        |
        v
epistemic status
```

The central question: what transformation is permitted between
evidence-related interpretation and standing resolution? This contract
defines the membrane, not the mechanism.

Governing documents: ADR-0001, ADR-0002, the ratified MCP and boundary
contracts, the admission exploration set, and the pipeline separation
contract. Where this document touches those, they govern. This contract
narrows; it never widens.

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Standing boundary

Evidence interpretation ≠ standing resolution. Standing ≠ truth.

Three things stay distinct: the standing resolver (a named mechanism that
computes), the standing result (its deterministic derived output over
verified history), and the epistemic status reported from that output. A
resolver output is a derived view, never a truth assertion.

- Evidence-related material may become resolver input.
- Standing resolution does not mutate history.
- Standing resolution does not create evidence.
- Standing resolution does not create policy.
- Standing resolution does not become truth authority.

## 2. Resolver ownership

The resolver owns:

- evaluating verified history under an explicitly selected policy version;
- producing a reproducible standing result as derived output.

The resolver does not own:

- historical inclusion;
- evidence creation;
- source authority;
- identity verification;
- trust assignment;
- policy selection itself.

## 3. Policy separation

Policy selection is explicit. There is no ambient latest.

The resolver applies a chosen policy version; it does not decide which
policy should exist. Prevented:

- the resolver becoming hidden governance;
- resolver output becoming authority over history;
- implementation defaults becoming implicit policy.

## 4. Evidence boundary

Evidence ≠ standing. Evidence-related material is input; standing is an
evaluated output. The automatic collapse

```text
recorded material → evidence → standing
```

is forbidden without explicit governed stages — each stage boundary is the
one its own contract defines, and none is skipped.

## 5. Projection boundary

Standing results are derived projections, tied to the verified record
snapshot and selected policy version they were computed from — never
"current truth" or an ambient latest state. They are not:

- history;
- new evidence;
- new authority;
- replacement records.

The signed append-only log remains the source of replay; every standing
result is recomputable from its stated coordinates and subordinate to
them.

## 6. AgentProposer implications

```text
proposal ≠ evidence
proposal ≠ standing
proposal ≠ settlement
```

Agent-generated material can influence standing only through the same
governed path as everything else. No special agent privilege exists.

## 7. Deadbolt implications

Preserved:

- specialised seams remain independently governed by their own ratified
  contracts;
- Deadbolt anchors do not automatically define resolver semantics;
- wrapping a seam grants no additional authority.

## 8. Failure semantics

Failure is not falsity. A resolver failure is a failure of computation,
availability, or evaluation — never evidence against the underlying
proposition. This document defines no failure workflows.

## 9. Open questions

Unresolved, each requiring a future ADR or contract amendment:

- the exact standing resolver interface;
- policy version representation;
- conflict handling;
- resolver determinism guarantees;
- standing invalidation and supersession semantics.

None is answered here, and none should be answered implicitly by an
implementation.

## Non-goals

This document adds none of the following:

- resolver algorithms or implementations;
- evidence registration or admission mechanisms;
- APIs, schemas, or event formats;
- identity, trust, scoring, ranking, or confidence systems;
- MCP write surfaces;
- `EpistemicGate` work.

## Reviewer checklist

- Does this preserve `LogWriter` as sole append authority?
- Does it preserve the signed append-only log as sole historical
  authority?
- Does it prevent evidence becoming standing automatically?
- Does it prevent resolver output becoming truth authority?
- Does it preserve explicit policy selection?
- Does it avoid introducing scoring, ranking, confidence, trust, or
  identity systems?
- Does it define boundaries rather than mechanisms?
