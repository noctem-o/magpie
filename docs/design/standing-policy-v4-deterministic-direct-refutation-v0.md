# Standing Policy V4 Deterministic Direct Refutation v0

## Status

Contract ratified by Ticket 0056. Documentation only.

```text
standing policy v4:
contract ratified by Ticket 0056
runtime: not implemented

first rule:
Sha256BytesEqualsDirectRefutationV0 (single closed rule)

selected lane:
DeterministicVerification × ExactMachineCheckable × contradicts
× sha256_bytes_equals_v0

achieved standing:
governed Refuted only, never Settled, never Supported from this rule
```

This note ratifies policy. It adds no Rust, tests, fixtures, Cargo metadata,
dependency, CI, L0 surface, or runtime capability.

## Purpose

Define Magpie's first explicit direct-refutation rule: the smallest exact
case in which replay-derived evidence may produce governed `Refuted`.

The rule answers one question exactly: when may a machine check *falsify* a
claim? The answer is deliberately narrow — one already-closed predicate, one
already-closed evidence cell, one exact negative relationship, and one
freshly recomputed negative result, all derived inside one verified replay:

```text
one exact typed claim (DeterministicVerification × ExactMachineCheckable)
+ one exact contradicts justification edge
+ exact claim/evidence/edge/scope/predicate/statement/content-hash bindings
+ one exact same-snapshot negative evaluation proving
  sha256_bytes_equals_v0 false for the exact bound witness
→ one governed direct-refutation contribution → governed Refuted
```

This contract is a hypothesis tested against the repository by two
independent fresh read-only audits and adjudicated by the coordinator; the
audit outcomes and the adjudication are recorded in Ticket 0056.

## Non-goals

- No Rust changes.
- No tests or fixtures.
- No L0 payload variants.
- No canonical encoding or `docs/FORMAT.md` changes.
- No golden vector changes.
- No `tools/verify_chain.py` changes.
- No Cargo manifest or lockfile changes.
- No dependency changes.
- No direct-refutation runtime.
- No contradiction-debt implementation or design.
- No invalidation.
- No supersession/currentness.
- No `EpistemicGate`.
- No ordinary writer surface.
- No loader, CAS, filesystem ingestion, network ingestion, or model
  integration.
- No librarian/navigator functionality.
- No Deadbolt code change.
- No negative rule for any `Occurrence/Inclusion` cell.
- No new predicate family beyond `sha256_bytes_equals_v0`.
- No refutation aggregation, threshold, count, vote, quorum, score, weight,
  probability, source reputation, or fuzzy matching.
- No inference of falsity from silence, absence, timeout, unavailable bytes,
  or missing anchors.
- No generic policy registry or implicit latest-policy selector.
- No caller-selected negative authority.
- No production trust, certification, attestation, or live-readiness claim.

## Preserved laws

The contract sharpens, and everywhere preserves, these distinctions:

```text
refutation ceiling
!= automatic refutation

contradicts edge
!= proof of falsity

verifier failure
!= refutation

malformed evidence
!= refutation

missing evidence
!= refutation

unavailable witness
!= refutation

missing anchor
!= proof of non-occurrence

non-matching unrelated material
!= refutation

caller-provided receipt
!= authority

serialized audit output
!= replay authority

direct refutation
!= contradiction debt

direct refutation
!= invalidation

direct refutation
!= supersession/currentness

direct refutation
!= negative aggregation

legacy raw Refuted
!= governed replay-derived Refuted
```

And the standing Magpie invariants:

```text
one signed append-only log is the source of truth
standing remains derived and regenerable
authority remains private-construction and same-replay
no implicit latest-policy selection
no caller-provided policy identity
no caller-provided audit or receipt
no ambient lookup
no probabilistic confidence
no voting or majority rule
no inference from absence
```

## Current substrate

What exists today, verified against the repository at the exact base:

- The closed predicate `sha256_bytes_equals_v0` with strict claim-side
  machine-predicate parsing, exact canonical statement derivation
  (`sha256_bytes_equals_v0:` + lowercase expected digest), claim
  content-hash rebinding, strict witness parsing, bounded decoding, and a
  final digest comparison
  (`crates/magpie-claims/src/deterministic_verifier_context.rs`,
  `docs/design/replayable-deterministic-verifier-admission-v0.md`).
- A positive-only resolver,
  `StandingReplaySnapshot::resolve_deterministic_verifier_context_v0`,
  which selects an exact accepted v0 *supports* candidate and revalidates
  `edge.edge_kind == "supports"`. Every non-`supports` edge yields no
  context attempt. Its `Matched(receipt)` is audit material, and every
  failure variant — including the unit `DigestMismatch` trace — is a
  failure token carrying no receipt, no identities, and no authority.
- The refutation ceiling surface `magpie_claims::policy::refutation_ceiling`
  (Ticket 0021), which permits `Some(Refuted)` for exactly three cells:
  `DeterministicVerification × Occurrence/Inclusion`,
  `DeterministicVerification × ExactMachineCheckable`,
  `DeadboltAnchor × Occurrence/Inclusion`. A ceiling is a maximum, not an
  achieved contribution.
- The v0 baseline explanation, which rejects every non-`supports` edge kind
  with the closed `UnsupportedEdgeKindForV0` structural reason.
- Explicit standing policies v1 (exact Deadbolt occurrence settlement), v2
  (one deterministic direct-support rule capped at `Supported`), and v3
  (one exact `ExternalReport` corroboration rule capped at `Supported`),
  each selected only through explicitly versioned resolvers.
- `OriginAdmissionReplayContextV0`, the context-owned authority carrier for
  policy v3, holding the private same-replay snapshot and the exact
  verified-prefix identity; the immutable resolution-content closure is
  supplied to its resolvers as exact content input.
- The pinned raw/governed distinction (PR #56): legacy raw `Refuted` is
  quarantined audit material, never governed authority; an inherited
  governed `Refuted` remains `Refuted` under current policy.

What does not exist: any negative verifier authority, any governed path
that derives `Refuted`, any contradiction-debt fold, any invalidation or
supersession semantics, any `EpistemicGate`.

## Why the existing `DigestMismatch` trace is never negative authority

The existing `DeterministicVerifierContextTraceV0::DigestMismatch` value is
permanently disqualified from negative authority:

1. It is a unit failure variant. It carries no claim, evidence, edge,
   scope, predicate, expected/computed digest, witness identity, or replay
   identity. It serializes as `{"outcome":"digest_mismatch"}` and nothing
   more.
2. It means a *positive `supports` attempt* failed. Current pinned policy
   law allows one `supports` path to remain `Supported` beside another
   `supports` path whose trace is `DigestMismatch`; reclassifying the trace
   as refutation would silently reverse that pinned v2 law.
3. The negative rule below never reuses, reinterprets, wraps, or re-exports
   that value. The type `DeterministicVerifierContextTraceV0` and every
   pinned byte of it remain unchanged.

The *comparison* the positive checker performs last — recomputing the
witness digest after every exact prerequisite has passed — is the only
reusable element. The negative path performs its own fresh comparison for
its own exactly selected `contradicts` candidate and constructs a new
receipt only from the freshly computed inequality.

## Exact policy identity

```text
policy id:
magpie-claims-standing-v4

inheritance:
the complete explicit policy-v3 result, internally derived

runtime policy parameter:
none — there is no caller-visible policy knob
```

Policy v4 is selected only through explicitly versioned resolver methods on
`OriginAdmissionReplayContextV0`. It does not replace, alias, negotiate
with, or become an ambient default for v0, v1, v2, or v3. There is no
"latest" policy.

Inheritance is by internal derivation, exactly as v3 inherits v2: the v4
resolver first derives the complete v3 resolution for the requested claim
from the same context and closure, validates its claim identity, policy
identity, verified-prefix identity, closure identity, and failure-freedom,
and only then evaluates the negative lane. V4 never forks from v2: a post-v3
policy that dropped corroboration would silently redefine the inherited
ladder.

## Exact rule identity

```text
rule:
StandingPolicyRuleV4::Sha256BytesEqualsDirectRefutationV0
(serialized: sha256_bytes_equals_direct_refutation_v0)

closed rule vocabulary:
exactly one rule; no second rule, lane, threshold, or parameter
```

The rule contributes only `Refuted`, only for the exact requested claim,
only when every law below holds. It never contributes `Supported`, never
`Settled`, and never settles or refutes any other claim, evidence, or edge.

## Exact selected lane

```text
evidence kind:
DeterministicVerification

claim domain:
ExactMachineCheckable

edge kind:
contradicts (exact closed-vocabulary string)

predicate:
sha256_bytes_equals_v0

ceiling law:
refutation_ceiling(DeterministicVerification, ExactMachineCheckable)
== Some(Refuted)
```

No other cell participates. The two other `Some(Refuted)` ceiling cells —
`DeterministicVerification × Occurrence/Inclusion` and `DeadboltAnchor ×
Occurrence/Inclusion` — have no sound achieved-negative substrate today:
the deterministic checker is bound to the machine-predicate lane, and
absence of an anchor in an open append-only prefix is known inclusion, not
universal exclusion. This contract deliberately does not cover them. The
closed `UnsupportedDirectRefutationLane` structural failure enforces that
boundary: a known kind/domain pair outside the sole lane is rejected before
any ceiling check or negative evaluation, even when its compiled ceiling
cell is `Some(Refuted)`.

## Candidate-universe law

A candidate is eligible for negative evaluation only when all of the
following hold, revalidated by re-fetch from the same snapshot:

1. the requested claim exists in the legacy claim table and the typed claim
   table;
2. the selected edge exists, has `edge_kind == "contradicts"`, targets the
   requested claim, and names an existing typed evidence node as source;
3. the evidence kind parses into the closed `EvidenceKind` vocabulary (an
   unrecognised kind fails `UnknownEvidenceKind`);
4. the claim's `claim_domain` metadata parses, in the existing strict v0
   manner, into the closed `ClaimDomain` vocabulary (an unrecognised domain
   fails `UnknownClaimDomain`);
5. exact three-way scope equality:
   `evidence.scope_ref == edge.scope_ref == typed_claim.scope_ref`;
6. the (evidence kind, claim domain) pair is exactly the sole v4 lane —
   every other known pair, including the two other `Some(Refuted)` ceiling
   cells, is rejected before any ceiling check or negative evaluation;
7. the compiled refutation ceiling cell equals `Some(Refuted)`.

Structural first-failure order mirrors the existing v0 support-candidate
order, with the edge-kind check taking the closed `contradicts` string
instead of `supports`:

```text
MissingTargetClaim
MissingTypedTargetClaim
UnsupportedEdgeKind (any edge_kind != "contradicts")
MalformedMetadata
MissingClaimDomain
WrongTypeClaimDomain
DuplicateMetadataKey
UnknownClaimDomain
MissingSourceEvidence
ScopeMismatch
UnknownEvidenceKind
UnsupportedDirectRefutationLane
NoRefutationCeiling
```

`UnsupportedDirectRefutationLane` fires when kind and domain both parse
successfully but the pair is not exactly `DeterministicVerification ×
ExactMachineCheckable` — including `DeterministicVerification ×
Occurrence/Inclusion` and `DeadboltAnchor × Occurrence/Inclusion`, whose
`Some(Refuted)` ceilings never admit them to this lane.
`NoRefutationCeiling` then guards only the exact lane's own compiled cell
against policy-matrix drift. Any structural failure yields no
negative-context attempt. A `supports`
edge is never a negative candidate, and a `contradicts` edge is never a
positive candidate: each direction revalidates its own exact edge kind.

## Exact negative-verifier semantics

For each eligible candidate, the negative evaluation performs, in the
frozen order below, the same strict checks the positive checker performs,
followed by its own terminal comparison:

1. strict machine-predicate parsing of the typed claim metadata:
   `MissingMachinePredicate`, `MalformedMachinePredicate`,
   `DuplicatePredicateKey`, `UnknownPredicateKey`, `UnknownPredicateSchema`,
   `UnknownPredicateId`, `InvalidExpectedDigest`;
2. statement binding: legacy claim statement, typed claim statement, and
   the canonical predicate statement must be byte-identical, else
   `StatementPredicateMismatch`;
3. claim content-hash binding: the claim content hash must be non-empty,
   else `MissingClaimContentHash`, and must equal the recomputed lowercase
   SHA-256 of the exact UTF-8 statement bytes, else
   `ClaimContentHashMismatch`;
4. strict verification-witness parsing of the evidence metadata:
   `MissingVerificationWitness`, `MalformedVerificationWitness`,
   `DuplicateWitnessKey`, `UnknownWitnessKey`;
5. witness schema exact, else `UnknownWitnessSchema`;
6. witness predicate ID equals the claim predicate ID, else
   `PredicateBindingMismatch`;
7. witness subject claim ID equals the requested claim ID, else
   `ClaimBindingMismatch`;
8. witness scope equals the matched claim scope, else
   `ScopeBindingMismatch`;
9. witness encoded bound, strict even-length lowercase hex, decoded bound
   (4096 bytes), else `WitnessTooLarge` or `InvalidWitnessHex`;
10. fresh digest comparison over the decoded witness bytes:

```text
computed_sha256 == expected_sha256
→ PredicateSatisfied (the contradiction is not verified)

computed_sha256 != expected_sha256
→ PredicateFalse(receipt)
```

`PredicateSatisfied` is terminal audit material: the evidence offered as a
contradiction actually satisfies the claim's predicate, so the alleged
contradiction fails. It produces no refutation, no support, and no debt.

`PredicateFalse(receipt)` is the only authoritative negative outcome. It
proves exactly that the one inline witness, exactly bound to the exact
claim, predicate, and scope, does not satisfy `sha256_bytes_equals_v0`.
It proves nothing about external artifacts, URLs, file contents, program
execution, occurrence, non-occurrence, interpretation, or general truth.

Every failure variant is audit-only: none of them is falsity, none of them
is a negative receipt, and none of them may be strengthened into one.

The negative trace vocabulary is closed: `PredicateFalse(receipt)`,
`PredicateSatisfied`, and the prerequisite failures above. It contains no
`DigestMismatch` variant (a mismatch is this path's success case), no
`Matched` variant (a match is `PredicateSatisfied`), and no variant shared
with the positive trace type.

## Exact authority path

```text
one successful LogReader verification
→ one private composite replay
→ one OriginAdmissionReplayContextV0 (exact verified-prefix identity)
+ one immutable ResolutionContentClosureV0 (exact content input)
→ internally derived complete policy-v3 resolution
→ internally derived negative evaluation (crate-private)
→ one crate-private pure v4 composer
→ StandingResolutionV4
```

No caller supplies an audit, receipt, contribution, predicate outcome,
threshold, policy, lane, precedence table, snapshot, callback, serialized
output, or alternate authority table. The negative receipt is constructed
only inside the resolver, from the same verified replay, in the same call.
A serialized or cloned receipt is audit material and can never re-enter any
resolver as authority.

The immutable closure is exact content input, supplied to the v4 resolvers
exactly as the v3 resolvers receive it, and consumed only by the inherited
v3 derivation; the digest comparison consumes no closure content. It is
untrusted input, not authority: supplying it selects no outcome. This is
the honest cost of a non-forking post-v3 policy boundary, and the contract
states it rather than hiding it.

## Exact precedence and conflict law

Preconditions: the inherited v3 resolution is identity-valid and
failure-free. Under that precondition, v4 standing composition is exactly:

```text
1. inherited governed Refuted
   → Refuted (preserved; repeated refutation is non-amplifying)

2. inherited governed Settled
   → Settled (preserved; v4 creates no demotion of Settled)

3. exact conflict:
   the same claim carries both a Matched supports verification
   (visible in the inherited v2 trace) and a PredicateFalse
   contradicts verification
   → closed blocker ConflictingDeterministicVerification
   → no v4 application
   → inherited standing preserved

4. one or more uncontested valid direct refutations
   → governed Refuted

5. otherwise
   → inherited standing unchanged
```

Reachability notes, stated so the law is never misread:

- Under current policies, an `ExactMachineCheckable` claim can inherit
  `Supported` only from a `Matched` supports verification. That situation
  is exactly the conflict case of rule 3, so a valid refutation never
  silently overrides an existing positive verification.
- Under current policies, an `ExactMachineCheckable` claim cannot inherit
  `Settled` at all: policy v1 settles only `Occurrence/Inclusion` claims,
  and claim domains are single-valued. Rule 2 is stated for completeness
  and for future domain changes, which must re-review this precedence.
- Rule 4's reachable effect today is promotion from `Open`, `Conjectured`,
  or no inherited standing to `Refuted`.
- A valid direct refutation outranks inherited `Supported`, `Conjectured`,
  and `Open` for the exact same claim by construction of the conflict rule:
  any inherited positive standing that could contest it is either the
  conflict case (rule 3) or structurally absent.

Rule 3 is the boundary between direct refutation and contradiction debt.
Two machine-verified, mutually exclusive evaluations for one claim are a
genuine verified conflict. V4 does not resolve verified conflicts by fiat;
it marks them, preserves inherited standing, and leaves resolution to the
future contradiction-debt slice. Silently collapsing the conflict in either
direction would hide exactly the state debt exists to explain.

## Multiplicity and non-amplification

One valid direct refutation is sufficient for governed `Refuted`. This is a
direct rule, not aggregation: the refutation ceiling is a direct negative
contribution surface, mirroring policy v2's single exact direct support,
and nothing like policy v3's two-group corroboration threshold applies.

One or one hundred valid negative paths yield the same single `Refuted`.
Every application is retained deterministically for audit, but there is no
count, vote, score, quorum, threshold, or stronger status. Duplicate
occurrences, cloned receipts, repeated edges from the same evidence, and
repeated evaluation never amplify. Failed negative candidates never
aggregate into a refutation. Multiple `contradicts` edges sourced from one
evidence node are independent candidates: each must independently satisfy
the exact edge, identity, and scope bindings, and invalid siblings neither
manufacture nor destroy authority.

## Legacy raw quarantine

Legacy raw standing remains quarantined compatibility and audit material.
A legacy raw `Refuted` does not establish governed `Refuted`, does not
veto, and does not participate in v4 composition. A legacy raw `Refuted`
coexisting with a valid direct refutation adds nothing. An inherited
governed `Refuted` is preserved and remains authoritative; repeated valid
refutations beneath it are redundant audit, never recovery or revival.

## Outer-composition failure law

Mirrors the v3 outer law, identically ordered:

1. inherited claim identity: the inherited v3 resolution must resolve the
   exact requested claim;
2. inherited policy identity: the inherited policy ID must be exactly
   `magpie-claims-standing-v3`;
3. verified-prefix identity: the negative evaluation must derive from the
   context's exact verified-prefix identity;
4. closure identity: the inherited v3 result must derive from the exact
   supplied immutable closure identity;
5. inherited failure-freedom: an inherited v3 composition failure is a
   closed blocker, not an outer failure, and produces no v4 application and
   no promotion.

Identity-invalid inheritance (cases 1-2) clears top-level governed
standing, legacy raw standing, and sets currentness to `Unknown`, with
nested evidence retained verbatim and no evaluation. Identity-valid
inheritance with a later outer failure (cases 3-4) preserves the valid
inherited values without promotion. Outer failures return one exact first
failure in frozen order, bind the expected current identities, retain the
inherited resolution verbatim, and expose no negative evaluation material.
No outer failure ever manufactures a refutation.

## Failure and blocker behavior

- Structural candidate failure: no negative-context attempt; the candidate
  remains in the explanation with its closed structural reasons.
- Negative-context prerequisite failure: one application carrying the
  exact failure context and `achieved_standing = None`; audit-only.
- `PredicateSatisfied`: one application carrying that terminal outcome and
  `achieved_standing = None`; audit-only; the alleged contradiction failed.
- `PredicateFalse` uncontested: one application with
  `achieved_standing = Refuted`.
- Exact conflict (rule 3): no v4 application; the closed
  `ConflictingDeterministicVerification` blocker; inherited standing
  preserved; designated future contradiction-debt input.
- Inherited v3 composition failure: closed blocker; no application; no
  promotion.
- Outer-composition failure: the outer law above.

No malformed, missing, unavailable, or unrelated material ever creates a
synthetic refutation.

## Deterministic ordering

```text
candidates: lexicographic edge-ID order (the inherited v0 trace order)
applications: candidate order
structural failures: frozen first-failure order
negative-context failures: frozen check order above
outer failures: frozen first-failure order
blockers: declaration order
conflict detection: claim-level set law, order-independent
```

Caller order, vector order, key order, and edge insertion order never
select a result. The same verified log prefix, the same immutable closure,
the same compiled policies, and the same requested claim produce
byte-identical v4 output.

## Future public resolver surface

Exactly two methods, on the existing context carrier:

```rust
OriginAdmissionReplayContextV0::resolved_standing_v4(
    &self, claim_id: &str, closure: &ResolutionContentClosureV0,
) -> Option<Status>

OriginAdmissionReplayContextV0::resolved_standing_with_trace_v4(
    &self, claim_id: &str, closure: &ResolutionContentClosureV0,
) -> StandingResolutionV4
```

The closure parameter mirrors the v3 resolvers exactly: it is exact content
input, not a caller-supplied authority value. No free function, no
`StandingReplaySnapshot` or `StandingView` v4 method, no public sibling
negative-context resolver, no caller-visible negative evaluation API. The
negative evaluation is crate-private inside the future v4 module. These
signatures freeze nothing else: no field, variant, or return-type member
beyond this section is public API.

## Future public types and serialization

Mirrors the v3 boundary:

- `StandingResolutionV4`: private fields, read-only getters, `Serialize`
  only — no `Deserialize`, no `Default`, no public constructor, no public
  mutation. It exposes direct typed JSON `canonical_bytes()` as
  policy-defined derived audit bytes for deterministic comparison; these
  are not L0 canonical encoding, a new event format, or a persistent
  protocol.
- The negative receipt type: private fields, read-only getters,
  `Serialize` only, no `Deserialize`, no `Default`, no public constructor.
  Its fields mirror the positive receipt: predicate schema and ID, claim,
  evidence, and edge IDs, exact scope, canonical statement, claim content
  hash, expected SHA-256, computed SHA-256 (unequal to expected by
  construction), and `u64` witness length.
- Closed rule, context, trace, failure, and blocker vocabularies serialize
  with the existing snake_case tagging conventions.
- Cloning or serializing any value yields audit material only.
- Deterministic field and collection ordering everywhere; no set iteration
  leaks into output.

## Compile-fail and hostile-construction reservation

The future runtime must carry compile-fail documentation proving that:

- authority-bearing public structs — `StandingResolutionV4`, the negative
  receipt, and any application structs — admit no struct-literal
  construction, public constructor, `Deserialize`, `Default`, or public
  mutation;
- every new type, including the closed enum vocabularies, rejects
  `Deserialize` and `Default`;
- the closed enum vocabularies (rule, context, trace, failure, and blocker
  kinds) may be constructible inert tokens exactly as in v3, but no
  resolver or composer ever accepts them, or any serialized form of them,
  as authority;
- no resolver accepts a caller-provided receipt, audit, context, snapshot,
  policy ID, predicate outcome, threshold, lane, or precedence (the
  immutable closure is exact content input, not an authority value, and is
  the one permitted caller-supplied content argument, exactly as in v3);
- no standalone `StandingView` or `StandingReplaySnapshot` v4 method
  exists;
- test-only staging constructors cannot exist in production builds.

## Hostile cases

Every case below is a contract-level reservation; the future runtime pins
each with tests. Outcome vocabulary: candidate rejection, negative-context
failure, policy non-application, outer-composition rejection, inherited
standing preserved, or valid governed Refuted.

| Case | Expected result |
| --- | --- |
| `supports` edge presented as negative authority | candidate rejection (no negative-context attempt) |
| `contradicts` edge with no exact negative verifier result | negative-context failure or `PredicateSatisfied`; policy non-application |
| digest inequality claimed before claim binding succeeds | impossible by frozen order; the earliest exact failure wins; no refutation |
| digest inequality claimed before scope binding succeeds | impossible by frozen order; `ScopeBindingMismatch` wins; no refutation |
| malformed predicate | negative-context failure (`MalformedMachinePredicate`) |
| malformed witness | negative-context failure (`MalformedVerificationWitness`) |
| unknown predicate schema or predicate ID | negative-context failure (`UnknownPredicateSchema` / `UnknownPredicateId`) |
| claim content-hash mismatch | negative-context failure (`ClaimContentHashMismatch`) |
| missing witness | negative-context failure (`MissingVerificationWitness`) |
| witness too large | negative-context failure (`WitnessTooLarge`) |
| unavailable verifier material | negative-context failure or no snapshot; never falsity |
| wrong claim ID in witness | negative-context failure (`ClaimBindingMismatch`) |
| wrong evidence ID / edge ID | candidate rejection (structural revalidation) |
| wrong scope | candidate rejection or negative-context failure (`ScopeMismatch` / `ScopeBindingMismatch`) |
| unparseable evidence kind | candidate rejection (`UnknownEvidenceKind`) |
| unparseable claim domain | candidate rejection (`UnknownClaimDomain`) |
| known but non-lane kind/domain pair | candidate rejection (`UnsupportedDirectRefutationLane`) |
| `DeterministicVerification × Occurrence/Inclusion` with otherwise exact bindings | candidate rejection (`UnsupportedDirectRefutationLane`) |
| `DeadboltAnchor × Occurrence/Inclusion` with otherwise exact bindings | candidate rejection (`UnsupportedDirectRefutationLane`) |
| wrong policy ID | outer-composition rejection |
| caller-created negative receipt | unconstructible; any staged value fails outer composition |
| deserialized or cloned audit substitution | compile-time impossible; audit values never re-enter |
| receipt from another replay snapshot | outer-composition rejection (verified-prefix identity) |
| repeated identical negative evidence | one governed Refuted at most; repetition audit-only |
| multiple edges to the same evidence | independent exact candidates; no amplification; invalid siblings inert |
| legacy raw `Refuted` without governed negative proof | policy non-application; inherited preserved; raw stays quarantined |
| inherited governed `Settled` plus valid direct refutation | inherited `Settled` preserved (structurally unreachable today; law stated) |
| inherited governed `Refuted` plus repeated direct refutation | inherited `Refuted` preserved; redundant audit; no amplification or recovery |
| inherited `Supported` from `Matched` supports plus valid `PredicateFalse` contradicts | `ConflictingDeterministicVerification` blocker; inherited preserved; future debt input |
| missing Deadbolt anchor misrepresented as non-occurrence | never refutation; anchor absence is not negative authority |
| contradiction evidence that should create only future debt | policy non-application; inherited preserved; debt remains future |
| unrelated proposition with similar statement text | candidate rejection / statement-binding failure; no prose equivalence |
| reordered or duplicated serialized collections | no authority from serialized input; canonical output order fixed |
| outer-composition identity drift (claim/policy/prefix/closure) | outer-composition rejection; exact first failure |
| attempt to infer currentness or supersession | policy non-application; inherited currentness copied unchanged |

## Falsification list

This contract is falsified if any future contract or runtime:

1. lets the existing `DigestMismatch` unit trace, or any `supports`-lane
   failure, yield `Refuted`;
2. lets any failure other than a freshly recomputed predicate inequality
   after every exact prerequisite yield `Refuted`;
3. admits any evidence kind, claim domain, edge kind, or predicate beyond
   the exact selected lane;
4. treats a missing anchor, missing event, missing evidence, unavailable
   bytes, or Deadbolt refusal as non-occurrence or falsity;
5. collapses conflicting exact positive and negative verifications in
   either direction instead of the closed conflict blocker;
6. lets a caller-created, cloned, deserialized, staged, or foreign-prefix
   receipt produce standing;
7. forks v4 from v2, drops the exact internally derived v3 result, accepts
   caller-provided v3 authority, or adds an ambient/latest selector;
8. amplifies refutation by repetition, count, vote, threshold, origin
   group, or weak failures;
9. treats legacy raw `Refuted`, actor strings, labels, metadata booleans,
   or prose as authority;
10. claims external artifact falsity, universal non-occurrence,
    interpretation falsity, trust-root correctness, or general truth from
    an inline witness inequality;
11. implements contradiction debt, invalidation, supersession/currentness,
    `EpistemicGate`, writers, loaders, or librarian behavior under this
    policy;
12. changes v0-v3 types, bytes, behavior, fixtures, the policy matrices,
    L0, `docs/FORMAT.md`, golden vectors, or the Python verifier;
13. produces order-dependent or non-byte-identical output from the same
    `(verified prefix, closure, compiled policies, requested claim)` input.

## Exact implementation allowlist (future runtime ticket)

The future runtime ticket may change only:

```text
crates/magpie-claims/src/standing_v4.rs                    (new)
crates/magpie-claims/src/deterministic_refutation_context.rs (new, optional)
crates/magpie-claims/src/deterministic_verifier_context.rs
    (only to extract a shared crate-private strict parser/checker core;
     the positive resolver's public types, failure vocabulary, check
     order, and bytes must remain byte-identical and pinned by its
     existing tests)
crates/magpie-claims/src/origin_admission_replay.rs        (v4 resolvers)
crates/magpie-claims/src/lib.rs                            (exports)
crates/magpie-claims/tests/standing_v4.rs                  (new)
fixtures/standing-policy-v4-direct-refutation-v0/*         (new)
README.md
docs/design/standing-policy-v4-deterministic-direct-refutation-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
tickets/<future>-standing-policy-v4-implementation.md      (new)
```

The shared-core law mirrors the established one-shared-engine doctrine:
the positive and negative paths must use one crate-private strict
parser/checker implementation, because independent copies could drift. The
extraction must not change any existing public surface, trace vocabulary,
ordering, or byte.

Everything else is prohibited: v0/v1/v2/v3 production types or behavior,
`OriginAdmissionReplayContextV0` construction, support-audit and origin
surfaces, policy matrices, L0, payload tags, canonical encoding, golden
vectors, fixtures beyond the new family, `docs/FORMAT.md`, the Python
verifier, Cargo manifests, the lockfile, CI, release metadata, the tour,
and every historical ticket or ratified contract.

## Validation commands (for this documentation change)

```powershell
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
cargo package -p magpie-log --locked
git status --short
git diff --name-only
git diff --stat
```

Because this change is documentation-only, all cargo outputs must remain
identical to the exact base; the tour must remain 1917 bytes with SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`. The
release-metadata checker may refuse the intentionally dirty worktree; the
unchanged script is then run in a temporary VCS-free mirror. No
`--allow-dirty`. No golden or fixture regeneration.

## Reviewer checklist

- Confirm exactly the authorised documentation paths changed.
- Confirm no runtime claim appears: no Rust, test, fixture, Cargo, L0,
  format, golden, verifier, or CI change.
- Confirm the only selected lane is `DeterministicVerification ×
  ExactMachineCheckable × contradicts × sha256_bytes_equals_v0`.
- Confirm the existing `DigestMismatch` trace is permanently disqualified
  from negative authority and unchanged.
- Confirm `PredicateFalse` is the only authoritative negative outcome and
  requires every exact prerequisite first.
- Confirm the conflict law: verified positive plus verified negative is the
  closed blocker, never a silent override; contradiction debt stays future.
- Confirm inherited `Settled` and inherited `Refuted` are preserved;
  repetition is non-amplifying; legacy raw stays quarantined.
- Confirm v4 inherits the complete internally derived v3 result and adds no
  ambient or caller-selected policy path.
- Confirm absence never becomes falsity, and no `Occurrence/Inclusion`
  negative rule appears.
- Confirm serialization boundaries: `Serialize` only, no `Deserialize`, no
  `Default`, no public constructors, private fields, deterministic bytes.
- Confirm the ticket and this note agree exactly.

## Next-slice handoff

The next separately reviewed slice is the direct-refutation runtime: the
implementation of this contract under the exact implementation allowlist
above, with its own ticket, fixtures, hostile tests, compile-fail coverage,
and validation ladder.

After that, the sequence remains: contradiction debt and precedence,
invalidation, supersession/currentness, `EpistemicGate`, ordinary
claim-bearing writers, governed acquisition/loading, and librarian
integration. This contract ratifies none of them.
