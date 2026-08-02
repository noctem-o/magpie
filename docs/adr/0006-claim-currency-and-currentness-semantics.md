# ADR-0006: Claim Currency and Currentness Semantics

**Status:** Proposed

## Summary / Y-statement

In the context of a record that accumulates attributed acts — assertions
and withdrawals — and recorded relationships — supersession and
invalidation edges — facing the risk that "current" hardens into stored
lifecycle state or into ambient-latest authority that no policy chose and
no snapshot can audit, we decide that currentness is a derived facet:
recorded acts and relationships supply the history, and a named resolver
at explicit snapshot, policy-version, and resolver-identity coordinates
computes which recorded material is applicable — never writing
currentness back into the record — to achieve currency answers that are
replayable, policy-auditable, and able to disagree legitimately,
accepting that "current" has no global answer, that multiple resolvers
and policies may coexist over the same record, and that any additional
representation must follow FORMAT's additive evolution rules.

## Context and problem statement

ADR-0004 drew the lifecycle facet boundary: a claim has no stored
lifecycle state; the record holds attributed acts; lifecycle is a derived
facet view over verified history. It named currentness as a future
currency facet and seeded its doctrine: a later event may mark an earlier
one superseded, but both remain in the record, and "current" is always a
resolver-relative, snapshot-relative answer.

ADR-0005 settled withdrawal: an attributed act targeting the actor's own
assertion act, asserting no falsity, determining neither standing nor
currentness by itself.

Before this ADR, the currency facet itself remained unsettled. FORMAT §3
already froze tags 6–8 and their exact field encodings under
`magpie-core-v1`, including the `supersedes` and `invalidates` edge
vocabulary. ADR-0002 settled one invalidation rule — invalidated evidence
contributes no support — and the contradiction rule: contradiction creates
debt and blocks settlement until the explicitly selected standing policy
determines that eligible invalidation, supersession, or ratification resolves
it. Historical implementation-plan statements about how those edges might be
interpreted were semantic sketches, not doctrine and never mutable
representations of the frozen tags.

Without a boundary, "current" is where hidden authority enters. A reader
who treats the latest recorded assertion as current has installed a
policy nobody wrote down: recency, applied universally, immune to
snapshot and inspection. A record that stored currency on the claim
would reintroduce the mutable lifecycle state ADR-0004 prohibited. Both
failure modes are quiet. This ADR makes the boundary explicit before
either becomes load-bearing.

## Decision questions

1. What does "current" mean in Magpie?
2. At what granularity does the currency facet answer — per claim, or per
   assertion/material?
3. What does `supersedes` assert, and what does it not assert?
4. What does `invalidates` assert, and over which targets?
5. May a currency policy derive currency from recorded time metadata?
6. What is the boundary of a currentness resolver?

## Decision

**1. "Current" means resolver-relative applicability.** A currency answer
is the output of a named resolver, running an explicitly selected policy
version, over a verified record snapshot — reproducible from exactly
those coordinates: snapshot, policy version, resolver identity.
Applicability is always applicability under an explicitly selected policy
to recorded material within the scope that policy evaluates; it is never
a property of the record itself. Rejected readings:

- *latest recorded assertion* — latest is a property of log ordering, not
  of applicability; adopting it makes recency a policy nobody chose;
- *latest event wins* / *newest means correct* — the same error with a
  truth claim attached;
- *ambient-latest semantics* — a currency answer without coordinates is
  not a Magpie currency answer;
- *latest surviving assertion* — "surviving" hides policy inside an
  undefined word and collapses currency into standing, smuggling the
  fold's support accounting into a facet that must stay policy-explicit.

**2. The currency facet is per-assertion/material.** Here, "material"
means any recorded item a policy takes as input. Each relevant assertion
or item receives a currency interpretation under the resolver. The facet
does not elect one current item per claim: a claim-level single answer
would become a canonical truth carrier, which Magpie's multiplicity,
provenance, and actor-separation doctrine forbids. A policy may derive a
claim-level summary from per-assertion facet output as a convenience
view; that summary is identified by the full coordinates of the facet it
derives from, is never stored, and is never canonical.

**3. `supersedes` records lineage — nothing more as recorded history.**
The edge records that an attributed actor registered one item as the
successor of another. Its standing and currency consequences belong to
separately governed resolver policies, never to the edge itself.

**4. `invalidates` remains limited to evidence targets.** The settled
ADR-0002 consequence stands — an eligible invalidation deactivates the
target evidence's contribution, support and contradiction alike, within
the exact affected scope on replay — and this ADR does not extend
invalidation to claims, assertion acts, or relationship edges.

**5. Recorded time creates no currency semantics.** The log is ordered,
not clocked. A currency policy governed by this ADR must not derive
applicability from timestamps, age, elapsed duration, recency,
expiration, decay, or wall-clock cutoffs. Any future time-aware currency
policy requires an explicit amendment to ADR-0004 and ADR-0006 before
implementation.

**6. The currentness resolver is a pure function of coordinates.**
`resolver(record snapshot, policy version, resolver identity)
-> currentness facet`, with the boundary stated below.

**7. Withdrawal, supersession, invalidation, and contradiction are
distinct recorded acts and relationships.** Withdrawal changes neither
currentness nor standing automatically; both remain resolver policy
decisions, exactly as ADR-0005 settled.

## Currency facet semantics

**Acts versus views.** The record holds attributed acts and recorded
relationships: assertions, withdrawals, and edges such as `supersedes`
and `invalidates`. Views are computed. Currentness is a view. No event
writes a currency outcome; no reader trusts a stored one.

**Coordinates.** A currency answer has exactly three coordinates: the
verified record snapshot, the explicitly selected policy version, and the
named resolver identity. The facet is computed over the verified record
prefix ending at the snapshot, under the selected policy, by the named
resolver. Later events can never change an earlier snapshot's answer.

**No stored current state.** `claim.current = true` and
`claim.status = active` are forbidden shapes. The invariant:

> Acts and relationships bearing on currency are recorded history;
> currentness is never recorded. Currentness exists only as resolver
> output at explicit snapshot, policy-version, and resolver-identity
> coordinates.

**Determinism.** The same coordinates produce identical typed facet
output. Once a canonical encoding is ratified, the serialized output must
also be byte-identical — the replay discipline the standing view already
pins.

**The facet is not evidence.** Facet output never re-enters the record as
an act. A projection is a reading, not a contribution; it carries no
provenance authority of its own.

**Withdrawal interaction.** Withdrawal is an act; currentness is a view.
A withdrawal act does not move any currency answer by itself — a policy
decides what a recorded withdrawal means for applicability, just as it
decides what a recorded supersession means. No act reaches around policy
to set its own interpretation. Any currency consequence derived from a
withdrawal is confined to the exact assertion act that withdrawal
targets (ADR-0005): it may not alter the applicability of another
actor's assertion, another assertion act by the same actor, or the
shared claim identity merely because they express the same proposition.
Under ADR-0005, any valid withdrawal must identify exactly one prior assertion
act through an immutable, replayable target coordinate. A currency resolver
may not disambiguate a claim-level reference by choosing "latest", consulting
timestamps or ambient state, or affecting all matching assertion acts.
A broader delegated or organisational effect requires future identity
and governance doctrine and an explicit amendment.

## Supersession semantics

`A supersedes B` records that an attributed actor registered A as the
successor of B. It is a recorded lineage relationship, not a
self-executing currency judgment.

It does **not** mean:

- the earlier item was false;
- the earlier item is deleted;
- the earlier item is invalid;
- the newer item is automatically authoritative;
- the resolver must ignore the earlier item.

Both items remain in the record, replayable and auditable. The currency
consequence of supersession belongs to resolver policy: one policy may
treat superseded material as no longer applicable within a scope; another
may keep it applicable. The edge compels nothing — a self-executing
supersession would be authority smuggled into vocabulary.

A `supersedes` edge records attributed lineage and executes no
consequence by itself. A named standing policy may separately determine
whether an exact-scope, authority-eligible supersession discharges
contradiction debt for the affected material, preserving ADR-0002's
standing rule; such a policy pins its target and lineage binding, exact
scope requirements, and actor-authority interpretation through its own
policy version. Until such a standing policy is ratified, a raw
supersession edge discharges no debt. A named currency policy may
independently determine the relationship's applicability consequence.
Each consequence is pinned through the coordinates of the resolver that
produces it; neither projection consumes the other.

ADR-0006 supersedes only ADR-0002's former `StandingView` wording that
superseded claims "should not remain current unless explicitly ratified under
the new scope." That wording conflated standing and currentness. ADR-0002 now
assigns standing consequences to standing policy; ADR-0006 assigns
applicability and currentness consequences to currency policy. Ratification
has only the consequence granted by the policy consuming it and is not
ambient authority. No broader part of ADR-0002 is superseded.

**Target scope.** The existing tag 8 `source_id` and `target_id` fields and
their canonical encoding are frozen. Which semantic target kinds those IDs
may denote remains deferred. If a future target representation cannot be
expressed without reinterpreting existing fields, it must be additive under
FORMAT's evolution rules. This ADR defines doctrine, not a new payload.

**Scope equality.** Under the v1 exact-match scope model, a supersession
relationship may affect currency only when the supersession edge,
predecessor, and successor satisfy the required exact scope equality. A
mismatch remains recorded lineage but has no currency effect. A
mismatched relationship is not malformed history: it remains attributed,
recorded, auditable, and non-self-executing. Cross-scope supersession
requires an explicit amendment to ADR-0002 and ADR-0006 before
activation.

**Cycles.** A supersession cycle is degenerate lineage. The record keeps
it without comment; a resolver policy must treat cycles
deterministically; cyclic or self-referential edge sets do not
self-promote into currency.

## Invalidation semantics

**Recorded history.** An attributed `invalidates` edge is recorded
history: it exists in the append-only record, inspectable forever. Its
presence is a recorded fact, not an authority.

**Eligibility.** The presence of an `invalidates` edge is not sufficient
to deactivate contribution. ADR-0002 states the consequence of
invalidation; it does not establish what makes an invalidation eligible.
That consequence applies only when a separately governed
invalidation-eligibility rule determines that the recorded edge is
eligible, including all required target, scope, and actor-authority
conditions. This ADR defines no such actor-authority or identity policy.
Until an invalidation-eligibility rule is ratified, no write path, fold,
or resolver may activate invalidation effects from a recorded
`invalidates` edge.

**Scope binding.** Under the v1 exact-match scope model, an invalidation
may affect only a contribution whose invalidation edge, target evidence,
and relevant contribution relationship are all exact-match equal in
scope — the same exact-match discipline the standing fold already applies
to support evaluation. A scope mismatch is ineligible and deactivates
nothing. Scope inheritance, containment, wildcard matching, and
cross-scope invalidation are not defined. A mismatch is not falsity, not
contradiction, not negative evidence, not authority to suppress support,
and not permission to guess scope equivalence.

**Consequence.** Only an eligible invalidation has consequence: under
ADR-0002, an eligible invalidation makes the target evidence ineligible
to contribute within the exact affected scope. This deactivates both
support contribution and contradiction contribution sourced from that
evidence. It does not delete the evidence or its recorded relationships,
erase audit history, assert falsity, or mutate the record. The resolver
consequence changes; history does not.

**Non-effects.** An ineligible or scope-mismatched invalidation
deactivates no contribution, asserts no falsity, creates no
contradiction, deletes neither evidence nor relationships, rewrites no
history, erases no audit record, and remains inspectable history.

**Coordinate closure.** Any resolver policy that applies invalidation's
consequence must pin the complete invalidation-eligibility rule through
its own selected policy version — the rule and its version, the exact
target-binding rules, the exact scope-binding rules, the actor-authority
interpretation rules, and any immutable configuration necessary to
interpret them. All variable eligibility inputs, including recorded
attribution, delegation, or authority material if later admitted, must
come from that resolver's verified snapshot. The resolver may consult no
mutable ambient identity store, permission database, unversioned trust
configuration, or external current-authority lookup. If a future
eligibility design cannot be reproduced from the existing snapshot,
policy-version, and resolver-identity coordinates, the coordinate model
must be amended before that design is activated.

This ADR does not extend invalidation beyond evidence. The self-directed
case for assertions is already covered by withdrawal (ADR-0005). Removing
the contribution of a claim, assertion, or edge is a stronger power whose
authority rules are undeveloped, and conservative doctrine admits no
scope that existing acts cannot yet justify. Any extension requires a
future ADR and, where representation is needed, additive evolution under
FORMAT.

Invalidation remains distinct from:

- **falsity** — invalidation asserts no falsity; any epistemic conclusion
  remains the output of a separately governed resolver;
- **contradiction** — contradiction signals conflict and creates debt
  while deactivating nothing; invalidation deactivates contribution and
  creates no debt;
- **withdrawal** — withdrawal records an actor's cessation of endorsement
  of its own assertion; invalidation deactivates an evidence item's
  contribution;
- **deletion** — an append-only record admits no deletion.

## Consequence ownership across facets

A recorded act or relationship may be relevant to several projections.
Recorded history, standing interpretation, and currency interpretation
are three different things: the same recorded relationship may be input
to standing and currency resolvers, but its standing consequence is
governed only by standing coordinates and its currency consequence only
by currency coordinates.

**Consumer-policy ownership.** Any resolver policy that applies a
consequence from withdrawal, supersession, invalidation, or
contradiction must pin every eligibility rule used for that consequence
through that resolver's selected policy version. All variable historical
inputs used by eligibility must come from that resolver's verified
snapshot. No eligibility rule may consult mutable ambient identity,
permission, or trust state.

- **Standing policy ownership.** A standing policy that deactivates
  evidence contribution or discharges contradiction debt must pin the
  complete relevant eligibility semantics through the standing policy
  version: invalidation eligibility, supersession eligibility, target
  binding, exact scope binding, actor-authority interpretation, and any
  immutable rule configuration.
- **Currency policy ownership.** A currency policy that derives
  applicability consequences from withdrawal, supersession, or
  invalidation must independently pin its eligibility and interpretation
  semantics through the currency policy version.
- **Shared profiles.** A future reusable eligibility profile may be
  referenced by multiple policies, but it must have a stable identity
  and version, each consuming policy must pin the exact version, use by
  one policy does not bind another, no ambient "current eligibility
  profile" is permitted, and no resolver may consume another facet's
  output as eligibility authority.

Doctrine permits; nothing in this table is active by permission alone.
Every consequence requires a separately ratified policy with
policy-pinned eligibility, and none follows from vocabulary presence.

| Recorded input | Possible standing interpretation | Possible currency interpretation |
| --- | --- | --- |
| Withdrawal | no automatic effect; any future effect requires a ratified standing policy with policy-pinned eligibility | target-local applicability effect under a separately governed currency policy |
| Supersession | eligible contradiction-debt resolution under a standing policy; policy-pinned eligibility required | exact-scope applicability interpretation under a currency policy |
| Invalidation | eligible support- and contradiction-contribution deactivation under a standing policy; policy-pinned eligibility required | applicability interpretation only if separately governed by a currency policy |
| Contradiction | typed blocker and debt under standing policy | no automatic currency effect |

> A recorded act or relationship may be interpreted by multiple
> resolvers, but each resolver owns only its own projection, pins every
> eligibility rule through its own policy coordinates, and never
> consumes another facet as authority. Information may move downstream;
> authority does not.

## Currentness resolver boundary

- **Deterministic replay.** The same coordinates yield identical typed
  facet output; byte-identical serialization follows only once a
  canonical encoding is ratified.
- **Snapshot relativity.** The resolver reads only the verified record
  prefix ending at the snapshot. Later events never change an earlier
  snapshot's output.
- **Policy relativity.** Two policy versions, or two resolvers, over one
  snapshot may return different facets. Both are coordinate-bound policy
  outputs, each identified by its full coordinates; neither is the
  ambient answer.
- **Coexistence of projections.** Any number of resolvers and policies
  may operate over one record. No currentness projection becomes
  universal authority — an authoritative projection is the ambient-latest
  trap in resolver clothing.
- **No act creation.** The resolver interprets history; it does not
  create acts. Interpretation never produces withdrawal, supersession, or
  invalidation acts — ADR-0005's resolver boundary carried into currency.

## Compatibility with the legacy standing surface

`StandingCurrentness` in the existing standing-policy v0–v4 resolution
surfaces is a legacy compatibility field, not an ADR-0006 currency
facet. Under the current implementations the field is never computed
from recorded history: the base resolver constructs it as `Unknown`,
and successor policies propagate the inherited value (failure paths
quarantine foreign inherited surfaces to `Unknown`), so no log-derived
resolution carries `Current`. Future currency work must not
populate or reinterpret it as canonical currentness. ADR-0006 currency
requires a separate per-material, coordinate-complete output surface.
Removing, repurposing, or changing the canonical serialization of the
legacy field requires a separate compatibility, format, and vector
review.

## Permanent non-definition

Currentness is not:

- truth;
- latest;
- freshness;
- a score;
- wall-clock validity;
- deletion;
- stored state.

Supersession is not:

- falsity;
- deletion.

Invalidation is not:

- contradiction;
- withdrawal.

## Deferred decisions

- Existing tags 6–8 and their exact field encodings are frozen under
  `magpie-core-v1`; existing fields and bytes are not mutable sketches and
  must not be reinterpreted.
- Any future representation required for unique withdrawal targets,
  supersession or invalidation targets, or currency output must be additive
  and follow FORMAT's evolution rules.
- The invalidation-eligibility rule — the target, scope, and
  actor-authority conditions under which a recorded `invalidates` edge's
  consequence activates — together with the identity and authority
  doctrine it depends on.
- Currency facet field naming and output encoding.
- Resolver algorithm details.
- Future policy specifics.
- Any time-aware currency policy — admissible only after explicit
  amendment of ADR-0004 and this ADR.
- Any claim-level summary derivation convention, if one is ever wanted.

## References

- ADR-0002 — fold rules, contradiction debt, invalidated evidence
  contributes no support, corrections are new events;
- ADR-0003 — the canonical definition of standing and its three resolver
  coordinates;
- ADR-0004 — lifecycle facets; the currency seed sentence; the log is
  ordered, not clocked;
- ADR-0005 — the withdrawal boundary; resolvers interpret history and do
  not create acts;
- `docs/FORMAT.md` §3 — the frozen payload vocabulary and its evolution
  rule;
- `docs/design/historical-review-remediation-ledger.md` — the historical
  remediation ledger;
- `docs/design/adr-0002-tags-6-8-implementation-plan.md` — historical
  semantic exploration for `supersedes` and `invalidates`; it does not
  reopen the frozen tag encodings.
