# Epistemic Boundary Map

## Status

Orientation and reference document. Docs-only. This map defines no
doctrine, overrides no ADR or contract, and adds no mechanisms. Where this
map and any governing document appear to differ, the governing document is
right — the map is wrong and should be corrected to match.

## Purpose

Magpie's boundaries have been ratified or explored one membrane at a time.
This map exists for orientation: given a future implementation question,
which existing boundary owns it?

It answers "which existing boundary owns this question?" — never "how
should this boundary be implemented?"

The base invariant applies unchanged:

> Magpie runs alone; integrations add evidence, not authority.

## 1. Pipeline overview

The conceptual flow the boundaries guard:

```text
external material
        |
        v
interfaces / proposals / external sources
        |
        v
admission boundary
        |
        v
historical record
        |
        v
evidence interpretation boundary
        |
        v
standing resolver
        |
        v
epistemic status
```

Information may move downstream. Authority does not.

## 2. Ownership map

| Question | Owning boundary | Explicitly not owned |
| --- | --- | --- |
| May this enter history? | Admission boundary (`admission-record-boundary-contract`, `admission-boundary-minimal-semantics`) | truth, evidence, standing, policy decisions |
| Where is canonical history? | Signed append-only log; `LogWriter` sole append authority (ADR-0001) | projections, caches, side stores |
| What may an interface expose? | MCP boundary contract; librarian and query contracts | authority of any kind |
| Is tool availability meaningful? | Capability boundary contract | permission, endorsement, grants |
| What accompanies a derived answer? | Provenance response contract | trust, identity, authority |
| Is recorded material evidence? | Evidence admission boundary contract | admission itself |
| What standing follows? | Epistemic resolution boundary contract (standing resolver, explicit policy version) | evidence creation, policy selection, truth |
| Can agents decide? | AgentProposer boundary contract — agents propose only | settlement, evidence status, authority |
| Can provenance establish trust? | Provenance response contract — no | trust, identity |
| How do the stages compose? | Epistemic pipeline separation contract | any single-stage authority over another stage |
| What could violate all of this? | Admission threat model; readiness checklist | (explorations, not doctrine) |

## 3. Forbidden collapses index

Each collapse below is prohibited by existing doctrine. This index changes
nothing; it points.

| Collapse | Prohibited by |
| --- | --- |
| admission ≠ endorsement | admission record boundary contract |
| history ≠ evidence | evidence admission boundary contract |
| provenance ≠ trust | provenance response contract |
| identity ≠ authority | AgentProposer and capability boundary contracts |
| source ≠ evidential weight | evidence admission boundary contract |
| evidence ≠ standing | epistemic resolution boundary contract |
| standing ≠ truth | epistemic resolution boundary contract |
| projection ≠ history | pipeline separation contract; README |
| availability ≠ permission | capability boundary contract |

## 4. Implementation routing guidance

When implementing a feature, ask which single question it raises:

- Does this create history? → `LogWriter` and admission questions.
- Does this interpret history? → evidence and resolver questions.
- Does this expose information? → MCP, query, and provenance questions.
- Does this create agent output? → AgentProposer questions.
- Does this create derived state? → projection rules.

This is routing, not process: it assigns a question to its owner, and
imposes no workflow.

## 5. Open questions

Detailed unresolved questions remain owned by their source documents —
principally the admission exploration set
(`admission-boundary-questions`, `admission-boundary-threat-model`,
`admission-design-readiness-checklist`,
`admission-boundary-scenario-analysis`,
`admission-boundary-minimal-semantics`) and the open-questions sections of
the boundary contracts. This map duplicates none of them.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` design;
- admission implementation;
- evidence registration;
- resolver algorithms;
- APIs or schemas;
- identity or trust systems;
- scoring or ranking.
