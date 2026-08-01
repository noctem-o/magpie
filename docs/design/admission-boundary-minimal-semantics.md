# Admission Boundary Minimal Semantics

## Status

Design exploration. Docs-only. No admission mechanism exists, and none is
defined here. This document decides nothing. Any future adoption of an
admission semantics requires explicit governance — an ADR or contract
amendment — per the readiness checklist's gate. ADR-0002,
the ratified MCP contracts, and the admission exploration documents govern
wherever this note touches them.

## Purpose

The previous explorations answered "what could go wrong?" (threat model)
and "what must be true before implementation?" (readiness checklist), and
the scenario analysis walked one concrete crossing. This document narrows
the remaining question:

> What is the minimum semantic claim admission is permitted to make?

Questions over answers; constraints over mechanisms.

The base invariant constrains every observation:

> Magpie runs alone; integrations add evidence, not authority.

## 1. The central admission question

What does admission decide? Four candidate interpretations:

- **Controlled historical inclusion.** Admission decides only that external
  material may enter the signed append-only log — that it was submitted,
  from this recorded attribution, at this chain position.
- **Validation boundary.** Admission decides that material is structurally
  acceptable — well-formed, vocabulary-conformant, canonical.
- **Semantic acceptance.** Admission decides that material is
  content-acceptable — relevant, admissible in kind, worth recording.
- **Epistemic judgment.** Admission decides something about the material's
  truth, support, or status.

The analysis, without deciding:

- Validation-as-admission duplicates L0. Structural validation already
  belongs to the log layer; an admission boundary that re-does it adds a
  second, weaker copy of an existing check.
- Semantic acceptance is policy in disguise. Deciding what is "worth
  recording" by content is a policy judgment operating before the record
  exists — a shadow epistemic system (§2).
- Epistemic judgment is already owned. Truth, support, and standing belong
  to the standing resolvers over verified history; an admission boundary
  that judges them collapses two layers into one (§5).
- Only controlled historical inclusion leaves every existing owner with
  its existing domain: L0 keeps structure, resolvers keep status, and the
  log keeps authority.

Whether "controlled historical inclusion" is *the* admission semantics is
therefore the question a future ADR must answer explicitly. This document
only observes that the three stronger readings conflict with ratified
doctrine.

## 2. Authority separation

Admission does not become:

- truth authority;
- standing authority;
- policy authority;
- evidence weighting authority;
- trust authority.

Each of these already has an owner or a prohibition. Truth and standing
belong to named resolvers over a verified record snapshot. Policy selection
is explicit; there is no ambient latest. Evidence weighting is policy
material, evaluated per resolver, not at a gate. Trust authority is
excluded outright — Magpie has no trust systems.

An admission boundary carrying any of these would create a shadow
epistemic system: a second, unreviewed evaluation layer whose outputs look
like doctrine results but were produced outside the replay-verified path.
That is the duplicated-truth failure in a new costume — evaluation
happening somewhere other than the one place it is defined.

## 3. The historical transition

```text
external material
        |
        v
admission boundary
        |
        v
LogWriter
        |
        v
historical record
```

What changes at this transition: material that was outside the record
becomes part of the append-only historical record — signed, hash-chained,
and positioned.

What does not change: the material's epistemic situation. Admission alters
where the material lives, never what it means. Meaning is downstream
(§5) or nowhere.

What authority remains absent: any authority for the boundary itself
beyond permitting inclusion. The boundary does not sign on its own
account, does not endorse, and does not evaluate; `LogWriter` remains the
sole append authority, and the mechanism that invokes it is undefined
here.

## 4. Structural validation boundary

```text
structural validity
        |
        v
admission
        |
        v
historical record
```

- Structural validation belongs to L0: canonical form, closed-vocabulary
  conformance, chain integrity. This document does not redesign any of it.
- This exploration treats structural validity as a prerequisite boundary,
  not as an admission responsibility.
- What remains outside both: all epistemic evaluation (§5), and all
  questions of meaning, relevance, and trust.

Open at this layer: whether admission checks anything at all beyond
delegating structural checks — and if it does, what that residue is.

## 5. Epistemic evaluation boundary

```text
historical record
        |
        v
standing resolver
        |
        v
epistemic status
```

Standing is a deterministic resolver output over verified history, under
an explicitly selected policy version. Admission must not collapse into
this layer, because the layers answer different questions at different
times with different authority: admission is a one-way transition into
history; resolution is a repeatable, replayable evaluation *of* history.
Merging them would make inclusion itself an evaluation — unauditable by
replay, invisible to resolvers, and irreversible by construction.

## 6. Minimum semantic statement

Compare:

- "This material became part of Magpie history."

against the stronger statements:

- "This material is true."
- "This material is supported."
- "This material is trustworthy."
- "This material should affect standing."

The first is the strongest semantic statement admission may make. The
others exceed its
role, each for a doctrinal reason: truth is not Magpie's to assert at a
boundary (humans settle nothing epistemically, and a gate settles less);
support belongs to a named resolver under an explicit policy version;
trustworthiness names a trust system, which Magpie does not have; and
"should affect standing" smuggles policy selection into inclusion.

The weaker statement is not a weak guarantee — it is the exact guarantee
the append-only log exists to make: inclusion, order, signature, and
attribution, inspectable forever.

## 7. AgentProposer implications

```text
AgentProposer
    |
    v
proposal material
    |
    ?
    |
    v
historical record
```

Preserved from the AgentProposer boundary contract: generated content is
not evidence by existing; repetition does not amplify without policy;
model confidence is not epistemic status; recorded attribution is asserted
data, not verified identity; and

```text
proposal ≠ evidence
proposal ≠ settlement
proposal ≠ authority
```

What this layer adds to the analysis: for agent-originated material the
minimum semantic statement of §6 is the *whole* story. Admission of a
proposal records that the proposal was made — full stop. The "?" in the
diagram remains undefined here; whatever fills it cannot strengthen the
statement without crossing into §6's forbidden list.

## 8. Deadbolt and specialised seams

The Deadbolt anchor seam is independently governed by its own ratified
contract, and remains so. Two cautions follow:

- a specialised seam's semantics do not automatically define generic
  admission semantics — the anchor seam's reviewed shape is a precedent
  for itself, not a template the boundary inherits by default;
- wrapping a seam grants no additional authority, and a generic admission
  boundary must not read a specialised seam's existence as permission to
  generalize its semantics.

No Deadbolt redesign is proposed or implied.

## 9. Failure semantics

Rejected admission, failed admission, and retries all reduce to one rule:
failure is not falsity. A rejection is a typed failure about the attempt;
it says nothing about the material's proposition. Whether rejections are
recorded, and how retry-amplification is prevented, are open questions
carried from the questions exploration — this document does not invent
operational behaviour to fill them.

## 10. Remaining questions

Unresolved, each requiring a future ADR or contract:

- Is "controlled historical inclusion" the adopted admission semantics?
  (§1 — the analysis says the stronger readings conflict with doctrine;
  adoption is a governance act.)
- Does admission check anything beyond delegating structural validation,
  and if so, what? (§4)
- What may a record contain when referencing external material, without
  importing unverified content? (scenario analysis, §7 there)
- Are rejected attempts recorded, and how is retry-amplification
  prevented? (§9)
- Is the generic boundary the same kind of thing as the ratified Deadbolt
  anchor seam, or a different kind? (§8)

None of these is answered here, and none should be answered implicitly by
an implementation.

## Non-goals

This document adds none of the following:

- an `EpistemicGate` implementation or design;
- APIs or schemas;
- MCP write paths;
- authentication, identity, permissions, or capability tokens;
- trust scoring or confidence scoring;
- ranking or retrieval systems;
- agent autonomy mechanisms.
