# ADR-0004: Claim Lifecycle Facets

**Status:** Proposed

## Summary / Y-statement

In the context of a governed claim memory whose record is append-only and
whose every projection is derived, facing the question of how a claim's
lifecycle should be understood as later documents and policies reference
it, we decide that a claim has no stored lifecycle state — the record
holds acts, and lifecycle is a derived facet view over verified history —
to achieve a lifecycle vocabulary that cannot silently become authority,
accepting that withdrawal and currency semantics remain separate future
decisions.

This ADR invents nothing. ADR-0002 already decided that standing is not
stored as an authoritative mutable field and that corrections are new
events, never edits or deletes. This ADR extends that rule into the
lifecycle frame and names the facet families.

## Context and problem statement

Claims evolve: they are asserted, offered evidence, related, contradicted,
superseded. Without a canonical frame, "lifecycle" invites a stored state
machine — a `state` field that silently becomes authority. Doctrine
already forbids that shape for standing; the same shape must be forbidden
for lifecycle as a whole.

## Decision

> **A claim has no lifecycle state; the record holds acts, and lifecycle
> is a derived view over them.**

Three facet families organize that view:

- **Historical facets** — what was recorded.
- **Epistemic facets** — what a resolver derives under an explicit policy
  and verified snapshot.
- **Currency facets** — what later acts mark.

Each family is derived. None is stored. No facet transition exists;
facets are recomputed views, not states. New acts may alter future facet
projections without modifying prior records.

## The three facet families

### Historical facets — what was recorded

- Assertion, evidence offering, and relationship edges exist today as
  payload vocabulary (tags 6–8).
- Admission as a recorded act remains unresolved; the admission
  exploration set holds those questions.
- Withdrawal does not exist in the vocabulary. If a future decision,
  through the vocabulary freeze ritual, admits a withdrawal act, that act
  may record a retraction by an attributed actor; it may never assert the
  falsity of what it retracts.

### Epistemic facets — what a resolver derives under an explicit policy
and verified snapshot

- Standing remains governed by ADR-0003: it is the deterministic output
  of a named resolver over a verified record snapshot under an explicitly
  selected policy version.
- Contradiction evaluation follows ADR-0002's fold rules and the
  implemented policies v0–v4. Contradiction debt is a typed blocker, not
  a quantity: it can block a result, never rank one.

### Currency facets — what later acts mark

- `supersedes` and `invalidates` exist as edge vocabulary; their semantics
  are deferred doctrine.
- Currentness, when defined by the future lifecycle ADR, is a derived
  view: a later event may mark an earlier one superseded, but both remain
  in the record, and "current" is always a resolver-relative,
  snapshot-relative answer.
- There are no time semantics: the log is ordered, not clocked, and no
  wall-clock staleness exists.

## Permanent non-definition

Lifecycle is not:

- **stored truth** — no lifecycle field exists in history;
- **a state machine** — facets are recomputed views, not transitions;
- **time** — no clock, no decay;
- **a score** — no numeric freshness, debt, or currency;
- **authority** — a facet view is a derived projection and carries none;
- **a settlement path** — no facet settles a claim.

### Facet non-authority invariant

A lifecycle facet is a projection over recorded acts and evidence; it
cannot create, replace, or strengthen the evidence from which it is
derived. Facets are views: they do not become evidence, and they do not
settle claims by existing.

## Rejected alternatives

- **Stored lifecycle fields**, or lifecycle events interpreted as direct
  truth mutation — ADR-0002 rejected stored standing; this ADR rejects
  the same shape for lifecycle.
- **Time-based staleness** — the record is ordered, not clocked.
- **Numeric currency, freshness, or debt quantities** — the no-scoring
  doctrine forbids them.

## Relationship to existing documents

- ADR-0001, ADR-0002, and ADR-0003 are unamended; this ADR depends on
  them.
- The boundary contracts and the admission exploration set govern their
  domains unchanged; this ADR narrows nothing and widens nothing.
- Three decisions remain separate and future, and are not this ADR's
  content: withdrawal vocabulary (through the freeze ritual), currency
  semantics (the contradiction-and-currency lifecycle ADR), and admission
  or rejection of the terms "neutralization" and "staleness".

## Confirmation

This ADR changes no code and no tests. Existing fold rules, policies
v0–v4, golden vectors, and the governed-standing tour are untouched; the
facet model describes them exactly as they are.

## References

- ADR-0002 — no stored standing; corrections are new events; contradiction
  debt blocks `Settled`;
- ADR-0003 — the canonical definition of standing;
- `docs/design/admission-boundary-questions.md` and the admission
  exploration set — the unresolved admission boundary;
- README future boundaries — deferred invalidation, supersession, and
  currentness semantics.
