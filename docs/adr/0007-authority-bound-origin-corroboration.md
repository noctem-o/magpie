# ADR-0007: Authority-Bound Origin Corroboration

**Status:** Proposed

## Summary / Y-statement

In the context of origin-binding bundles whose canonical bytes, provenance
anchors, contribution subjects, authority labels, and opaque origin groups can
all be valid while the asserted grouping authority remains claimant-controlled,
facing the risk that fresh claimant labels manufacture corroboration separation
and thereby amplify standing, we decide that an origin group may create
corroboration separation only when a separate authority-binding object has been
completely verified under an explicitly selected authority profile and external
trust-root coordinate, binds one exact contribution-scoped assignment, and is
replayed from finite immutable inputs. We accept an additive authority-bound
policy path and preserve current v0 and standing-v3 outputs only under their
existing claimant-label compatibility semantics.

This is a proposal for owner review. Owner ratification and a later, separately
reviewed implementation are required before this decision can govern runtime
origin admission or standing.

## Context and problem statement

The origin-binding bundle deliberately carries an asserted governance path.
Its `authority.kind`, `authority.reference`, and `origin_group` fields are exact,
audit-visible data, but their issuer is the bundle claimant. The current origin
binding verifier establishes canonical form, the selected anchor occurrence,
the contribution subject, the provenance prerequisites, and the claimed
assignment. It has no independent authority credential, trust root, or
authority-verification profile.

The current origin-admission audit then compares the claimed authority fields
with this fixed pair:

```text
human-review
authority:origin-review-v0
```

That fixed pair prevents policy-label substitution. It does not authenticate
the claimant's right to make the assignment. Exact bytes, a matching anchor, a
valid origin-binding schema, and a compiled string comparison do not establish
authority.

The complete present failure is:

```text
canonical, correctly anchored origin-binding bytes
+ claimant-supplied authority.kind/reference matching compiled literals
+ claimant-supplied fresh origin_group values
-> groups admitted into the current fold
-> distinct groups counted by standing v3
-> Supported may be reached without an independently authenticated
   grouping-authority decision
```

This is the doctrine gap identified by audit finding A-024. The current code is
evidence of that gap, not doctrine this ADR preserves.

## Current failure path

The current authority transition is exact and short:

1. `origin_binding_verifier.rs` parses `authority.kind`,
   `authority.reference`, and `origin_group`, validates their protocol shape,
   and carries them into `ValidatedOriginBindingSubjectV0` and a matched trace.
   A match verifies the statement and its prerequisites, not its issuer's
   authority.
2. `origin_admission_audit.rs` compares the two authority strings with compiled
   literals. On equality, it places the claimant's `origin_group` into
   `TrustedFoldInputV0`; conflict folding can then emit `OriginAdmitted`.
3. `admitted_contribution_audit.rs` and
   `support_contribution_audit.rs` copy that group through their otherwise
   standing-inert projections.
4. `standing_v3.rs` groups support contributions by the copied string, counts
   distinct groups, applies the threshold of two, and may achieve
   `Supported`.

Neither the Magpie event `agent` or `source`, the origin-binding anchor, the
number of matching statements, nor the number of reviewers or signatures adds
the missing authority decision.

## Decision questions

1. Which origin-binding fields are assertions, and which facts do they prove?
2. What is one independently verifiable authority binding?
3. Which exact subject and substitution boundaries must it cover?
4. Which object owns the authority decision, and where is it verified?
5. Which replay coordinates make the decision deterministic?
6. What narrow grant follows from successful authority verification?
7. How do absence, failure, duplication, and conflict fail closed?
8. How are current v0 and standing-v3 outputs preserved without being
   reinterpreted as authenticated?
9. How do rotation, delegation, and revocation remain explicit without this
   ADR inventing an identity platform?
10. What relationship remains between authority and corroboration separation?

## Decision

### Claimant assertions remain claimant assertions

Bundle-carried `authority.kind`, `authority.reference`, and `origin_group`
remain claimant assertions. They remain exact and audit-visible. They do not:

- authenticate themselves or select a trust root;
- prove reviewer, signer, publisher, or source identity;
- establish the claimant's authority to assign a group;
- establish statistical, causal, or organisational independence; or
- create corroboration authority merely because the bundle is canonical,
  verified, or anchored.

A verifier may require those asserted values to match an authenticated
authority decision. Equality is a subject-consistency check, not the source of
authority.

### Authority binding

An **authority binding** is a separately verified, replayable authorization of
one exact origin-group assignment. The authority-bearing statement is owned by
the authenticated grouping authority, even if any party transports its bytes.
Its validity is decided at a distinct verifier-owned boundary using an
externally selected authority profile and trust-root coordinate.

An authority binding is not any of the following by itself:

- an exact authority-label match;
- a Magpie event `agent` or `source` value;
- an actor class;
- a `SegmentAnchored` occurrence;
- a bundle-carried key;
- a self-signed object whose root is selected from that object;
- signature count or signer count;
- URL or domain identity;
- a metadata flag; or
- the existence of a verified origin-binding statement.

### Authentication choice

The selected architecture is a **separate authority-binding object** that
references the immutable identity of the exact origin-binding statement and
authenticates the complete assignment subject defined below. It is carried as
foreign material whose exact selector is independently designated under `A`,
whose required `SegmentAnchored` occurrence is established in `H`, whose bytes
are resolved from `M`, and whose authority decision is verified under the
explicit authority coordinate `A`. `SegmentAnchored` establishes occurrence
and inclusion only; it neither designates the object nor authenticates or
authorizes its assignment.

```text
designation under A
+ required exact occurrence in H
-> candidate-universe membership

designation under A
+ no required occurrence in H
-> no candidate-universe membership

occurrence in H
+ no designation under A
-> no candidate-universe membership
```

| Alternative | Decision | Reason |
| --- | --- | --- |
| Authenticate the existing origin-binding canonical bytes under a separately selected authority profile | Rejected for this boundary | It combines a claimant statement and an authority decision, couples authority issuance to the existing bundle, and makes accidental reinterpretation of v0 material more likely. |
| Separate authority-attestation or authority-binding object referencing the exact origin-binding identity and assignment subject | Chosen | It preserves the current bundle as a claimant assertion, keeps authority verification verifier-owned, permits additive versioning, and does not require a canonical or L0 format mutation. |
| Rely on Magpie event provenance | Rejected | Signed `agent` and `source` labels are attribution, not authenticated grouping authority or delegation. |
| Rely on `SegmentAnchored` | Rejected | An anchor proves that selected foreign bytes occurred in the verified record; it does not authenticate the issuer or authorize the assignment. |
| Rely on the compiled authority-label pair | Rejected | Literal equality selects a policy vocabulary but cannot prove the claimant's right to use it. |

The exact authority-object wire schema, bundle kind, canonicalization profile,
and authentication algorithm belong to a later implementation contract. This
ADR fixes the object's semantic ownership, complete subject, external trust
selection, verifier boundary, replay law, and failure behavior now.

### Narrow grant

A complete verified authority binding establishes only:

```text
under the explicitly selected policy and authority profile,
this exact grouping authority authorized this exact contribution-scoped
assignment to this exact opaque group
```

It does not establish claim truth, evidence truth, publisher identity,
authorship, source quality, support, standing, settlement, statistical
independence, causal independence, organisational independence, writer
authority, generic admission authority, or authority over another assignment.

## Authority-binding subject

The authenticated semantic subject is substitution-resistant. Define the
authority-binding subject as:

```text
AuthorityBindingSubject = (
  exact_origin_binding_identity,
  exact_ContributionIdentityV0,
  exact_OriginComparisonNamespaceV0,
  exact_opaque_origin_group,
  exact_origin_admission_policy_identity,
  exact_authority_verification_profile_identity,
  exact_externally_selected_authority_trust_coordinate,
  exact_authenticated_grouping_authority_identity
)
```

Conflict and duplicate folding use a broader governed key so that a claimant
cannot avoid a conflict by issuing a second origin-binding statement:

```text
GovernedAssignmentKey = (
  exact_ContributionIdentityV0,
  exact_OriginComparisonNamespaceV0,
  exact_origin_admission_policy_identity,
  exact_authority_verification_profile_identity,
  exact_externally_selected_authority_trust_coordinate
)

GovernedAssignment = (
  GovernedAssignmentKey,
  exact_opaque_origin_group
)
```

Every coordinate participates in exact equality. Redundancy is intentional:
the namespace repeats policy, claim, and scope coordinates already present
elsewhere so that a verifier must establish both exact objects, not reconstruct
or weaken either one. The authority profile, trust coordinate, and authenticated
authority identity are bound into the verified result by the verifier; an
object's repetition of those values cannot select them or grant trust.

The coordinates mean:

- `exact_origin_binding_identity` is the immutable canonical identity of the
  complete origin-binding statement. For the current bundle family, a plain
  `binding_id` is insufficient; the identity must commit to the exact canonical
  bytes and the complete selected origin-binding anchor selector, including
  bundle kind, witness root, witness algorithm, canonicalization profile, and
  run ID.
- `exact_ContributionIdentityV0` includes target claim ID, source evidence ID,
  justification edge ID, scope reference, artifact algorithm, and artifact digest.
- `exact_OriginComparisonNamespaceV0` includes origin-admission policy ID,
  target claim ID, and scope reference.
- `exact_origin_admission_policy_identity` is the explicit authority-bound
  origin-admission policy selected by the consumer. It is not inferred from the
  object or aliased to current v0.
- `exact_authority_verification_profile_identity` selects the complete
  verification and authorization rules.
- `exact_externally_selected_authority_trust_coordinate` immutably identifies
  the root material or equivalent explicit trust coordinate under which the
  profile authenticates the grouping authority. A mutable name such as
  `current-origin-review-key` is not an identity.
- `exact_authenticated_grouping_authority_identity` is the profile-scoped
  credential, key, or equivalent authority identity produced by successful
  verification under `A`. It is not a global person or organisation identity;
  when the root directly issues decisions, it may be the root identity itself.
- `exact_opaque_origin_group` is the one authorized assignment value. It has no
  intrinsic semantics outside its exact namespace and policy.

The verified result must also retain the exact authority-object identity and
the authenticated authority identity yielded by the selected profile for
audit. Neither is selected from `authority.kind`, `authority.reference`, an
organisational name, or a key embedded in the claimant's bundle.

This subject prevents reuse across another contribution, claim, scope,
evidence node, justification edge, artifact, namespace, group, policy,
authority profile, authority root, or authenticated grouping authority. A
decision may be replayed at a later snapshot only when that snapshot contains
the same immutable governed material and the consumer selects the same explicit
coordinates; that is deterministic replay, not cross-subject reuse.

Valid bindings with the same `GovernedAssignment` collapse without
amplification even when they arrive in different objects or carry different
signatures. Valid bindings conflict when their `GovernedAssignmentKey` values
are identical but their exact opaque groups differ, including when they refer
to different origin-binding statements. The group remains part of every
authenticated `AuthorityBindingSubject` and is the value used for conflict
detection.

## Verification and replay coordinates

### Authenticated candidate designation

An authority-looking anchor occurrence is not a completeness-blocking authority
candidate:

```text
authority-looking anchor occurrence
!= completeness-blocking authority candidate
```

Candidate-designation authority and origin-group assignment authority are
separate roles. Authentication identifies a credential under the selected
coordinates; it does not by itself authorize either role:

```text
authentication
!= authorization

authenticated grouping authority
!= candidate-designation authority

authenticated identity
+ grouping-assignment authorization
-> may issue only the exact grouping assignments permitted by that scope

authenticated identity
+ candidate-designation authorization
-> may issue only the exact candidate designations permitted by that scope

grouping-assignment authorization
!= candidate-designation authorization

grouping-assignment authorization
+ no candidate-designation authorization
-> cannot establish candidate completeness
```

A credential may hold both roles only when the selected profile under `A`
explicitly and independently authorizes both exact scopes. Shared identity or
key material does not collapse the scopes. An authenticated grouping authority
cannot create completeness-blocking candidates merely by:

- signing a designation set;
- referencing an authority object;
- using the same key that signs grouping assignments;
- embedding a designation claim inside an origin-binding or authority object;
- appearing in a trusted grouping-assignment profile; or
- being named by `authority.kind` or `authority.reference`.

Only a candidate-designation authorization path selected under `A` grants that
role. Neither the claimant nor an authority or designation object may designate
itself as completeness-blocking.

Define the semantic designation subject as:

```text
AuthorityCandidateDesignation = (
  exact_authority_object_selector_or_identity,
  exact_origin_binding_identity,
  exact_GovernedAssignmentKey
)
```

The designation binds enough information before authority-object bytes are
loaded to identify exactly which `GovernedAssignmentKey` may become incomplete.
It establishes only:

```text
this exact selector must be considered when proving the exact governed
assignment conflict domain complete under the selected coordinates
```

It does not authorize an origin group. The designated authority object must
still pass the complete authority-binding verification law before it can supply
an assignment:

```text
candidate designation
!= authority decision

candidate designation
!= group authorization

candidate designation
!= support

candidate designation
!= standing
```

Exactly two modes may make the complete candidate-designation set authoritative.

#### Mode 1: direct external commitment

```text
external selection of A
directly fixes the exact immutable candidate-designation-set identity
-> that exact set is authoritative for candidate completeness
```

Authority in this mode comes from external selection of the complete `A`
coordinate. The set identity must commit to one complete finite set. No object
inside the set selects or authenticates itself, and no mutable alias, current
set, or object-carried label can select or replace that identity.

#### Mode 2: authenticated designation issuer

```text
A selects exact profile, root, and verification-material coordinates
+ the selected profile authenticates an issuer
+ the verified authorization path grants that issuer exact
  candidate-designation authority
+ the verified set identity matches the set identity selected by A
-> the set is authoritative for candidate completeness
```

Successful issuer authentication is insufficient by itself. The selected
profile must verify authorization for the exact candidate-designation role and
scope. That scope must cover issuance of each exact
`AuthorityCandidateDesignation` entry and cannot authorize an origin group,
issue a grouping assignment, alter `P` or `A`, add an unrelated governed key,
designate an object outside its verified scope, or grant generic admission,
writer, standing, or truth authority.

When this mode is used, the verified designation result must retain for audit:

- the exact authenticated candidate-designation-authority identity;
- the exact verified authorization-scope identity;
- the exact authorization-path or equivalent immutable verification identity;
- the exact candidate-designation-set identity; and
- the complete producing `H + P + M + A` coordinates.

These deterministically derived identities do not add a fifth top-level replay
coordinate when the material from which they are derived is fully committed by
the selected `A`. A correctly signed set whose issuer lacks the exact
candidate-designation authorization is not authoritative:

```text
authenticated issuer
+ no exact candidate-designation authorization
-> designation-universe verification failure
-> no authority-bound decisions
```

Because `A` selected this set as the candidate-universe authority, that failure
is not non-designated incidental material; it prevents establishment of the
candidate universe.

The exact future representation may use a manifest, authenticated index,
foreign bundle, or equivalent finite structure. This ADR does not choose that
wire mechanism. Whatever representation is selected, candidate-designation
authority cannot originate from:

- a `SegmentAnchored` occurrence alone;
- bundle-kind or profile resemblance;
- authority-object or origin-binding fields;
- claimant references;
- caller-supplied candidate lists, caller-selected subsets, or available-only
  enumeration;
- successful parsing, signature count, or reviewer count;
- event order, timestamps, lexical order, or arrival order;
- mutable registries, ambient keyrings, directory or filesystem discovery, or
  network lookup; or
- a current, latest, default, or compatible designation state.

The complete candidate universe is:

```text
DesignatedAuthorityCandidateUniverse(H, P, A)
=
every exact authority-object selector that:

1. is present in the complete immutable designation set selected by A;
2. is designated for an exact origin-binding identity and
   GovernedAssignmentKey under P and A; and
3. has the required exact occurrence in H.
```

Membership is fixed before availability or object verification is evaluated.
It is independent of candidate-byte availability, a candidate's claimed group,
signer or reviewer identity, signature count, parse success, event order,
caller preference, and whether another candidate already produced a favourable
decision. Every designated candidate occurring in `H` enters the exact per-key
universe. No caller-supplied subset is permitted.

A designation is not evidence that its authority object occurred in Magpie
history:

```text
designation
!= occurrence

designation under A
+ required exact occurrence in H
-> candidate membership

designation under A
+ no required occurrence in H
-> designation remains audit-visible
-> not a candidate at snapshot H
-> no assignment incompleteness
-> no authority-bound assignment

occurrence in H
+ no designation under A
-> no candidate membership
```

Designation without the required occurrence must not be treated as missing
candidate bytes, make its governed key incomplete, block an unrelated valid
decision, create conflict, become negative evidence, lower inherited standing,
or trigger an ambient object lookup. A later verified prefix containing the
required occurrence has a different `H` and may therefore produce a different
candidate universe under the same other coordinates.

```text
available candidates
!= complete candidate universe

authority-looking anchors
!= designated candidate universe

caller-selected subset
!= complete candidate universe
```

Candidate-byte incompleteness is assignment-scoped because the designation
exposes the exact governed key without loading those bytes:

```text
designated candidate for key K
+ required occurrence in H
+ candidate bytes unavailable
-> authority evaluation for K is incomplete
-> no partial positive authority-bound result for K

missing designated candidate for K1
!= incompleteness authority over unrelated K2
```

An unrelated key may resolve only when its own complete designated candidate
universe is available, verified, and conflict-free.

The designation universe itself is a resolver-level prerequisite. If the
selected designation set is unavailable, its immutable identity does not match
`A`, its required authentication or exact candidate-designation role and scope
authorization under the selected profile and root fails, it is malformed or
internally ambiguous, or one authority selector is designated for incompatible
origin-binding identities or governed keys, the resolver cannot establish the
candidate universe:

```text
designation universe unavailable or invalid
-> authority-bound audit cannot establish completeness
-> no authority-bound decisions are emitted
```

No future serialized variant name is selected here. The set's exact bytes may
be carried by `M`, by the immutable verification-material closure committed in
`A`, by an anchored foreign object, or by another finite immutable
representation selected by the implementation contract. Its exact identity is
always part of `A`; it is never selected from an evaluated object; and replay
performs no ambient lookup. The same `H + P + M + A` therefore observes the
same complete designation set.

A non-designated authority-looking occurrence remains audit-visible but cannot
block completeness:

```text
authority-looking anchor in H
+ no exact designation under selected A
-> visible non-designated material
-> not completeness-blocking
-> no authority-bound assignment
-> no veto over a separately valid designated candidate
```

Non-designated material cannot create conflict, amplify standing, lower
inherited standing, or become negative evidence. Byte-identical or semantically
identical designations collapse structurally and create neither another
candidate nor authority weight. The same exact authority-object selector
designated for incompatible origin-binding identities or
`GovernedAssignmentKey` values makes the designation universe invalid. No
sequence, timestamp, signature count, reviewer count, lexical order, or arrival
order selects a winner. A designation conflict is a failure to establish an
unambiguous candidate universe, not an origin-group conflict.

The authority-bearing resolver has four explicit deterministic inputs:

```text
H = exact completely verified Magpie log prefix
P = explicit authority-bound origin-admission policy identity
M = immutable resolution-content closure identity
A = exact authority verification coordinate
```

`M` must commit to the finite bytes of every origin-binding and
authority-binding object, plus any other foreign content the selected policy
can inspect. A later contract may extend the resolution closure additively or
place an equivalent immutable authority-material closure inside `A`; it may not
read authority objects from an ambient source.

`A` consists of:

```text
A = (
  authority_verification_profile_identity,
  authority_trust_root_identity,
  authority_verification_material_closure_identity,
  authority_candidate_designation_set_identity
)
```

The verification-material component commits to any finite root, intermediate,
authorization, or equivalent verification material required by the selected
profile and not already committed by `M`. The candidate-designation-set
identity is externally selected as part of `A`, commits to one finite immutable
complete designation set, and is not a mutable alias. The root identity must
commit to immutable material, not resolve a mutable alias. Repeating a profile,
root, or designation-set label inside an authority object is only a consistency
assertion and cannot select `A`.

In authenticated-issuer mode, the verification-material closure or equivalent
immutable material committed under `A` must contain everything needed to verify
the designation issuer identity, its immutable authorization path, its exact
candidate-designation authorization scope, and the designation-set
authentication. No issuer, set, or ambient resolver state may supply an
uncommitted authorization input.

The replay law is:

```text
same H + same P + same M + same A
-> byte-identical authority-bound origin-admission result
```

### Coordinate-bearing authority outputs

Every authority-bound audit or result that may cross its constructing context
must retain the exact identities of all four producing coordinates. This law
applies to detachable authority-bound audits, admitted-origin results,
contribution results, aggregation inputs, and standing inputs. Each must carry:

- `H`: the verified Magpie log-prefix identity;
- `P`: the selected authority-bound origin-admission policy identity;
- `M`: the immutable resolution-content closure identity; and
- `A`: the complete authority-verification coordinate identity, including the
  authority verification profile identity, immutable trust-root identity,
  authority verification-material closure identity, and exact immutable
  authority-candidate-designation-set identity.

A complete typed coordinate object may represent `H`, `P`, `M`, and `A`
together. No component may be omitted. This ADR does not choose that object's
wire schema or Rust type.

In authenticated-issuer mode, the audit trace must additionally retain the
derived designation-authority, authorization-scope, authorization-path, and set
identities required above. Two results using equal designation-set bytes but
different verified designation-authority scopes or authorization material are
different authority-bound results; those differences must be committed by a
different `A`. Identical `A` coordinates commit identical material and derive
the same trace under the replay law.

A result's assignment content is not its replay identity:

```text
equal assignment content
!= equal authority-bound result
```

Two outputs containing the same contribution, namespace, and origin group are
distinct authority-bound results when produced under different verified
prefixes, authority-bound policies, content closures, authority profiles,
trust roots, verification-material closures, or candidate-designation-set
identities. The group assignment alone is never sufficient provenance for an
authority-bearing result.

Before composition, every downstream authority-bearing consumer must exact-match
the producer's `H + P + M + A` identities against the expected producing
coordinates explicitly selected by that consumer. This applies to
admitted-contribution and support-contribution policies, aggregation-lane
construction, standing policies, provenance or explanation output, and any
future librarian or query response that exposes an authority-bearing
conclusion. A downstream policy's own identity remains separately explicit; it
does not replace or reconstruct the producer's `P`.

```text
authority-bound result coordinates
!= selected consumer coordinates
-> composition rejected
-> zero positive amplification
```

This rejection is a coordinate mismatch. It is not refutation, invalidation,
contradiction, negative evidence, or a lowering of inherited standing.

No downstream consumer may:

- reconstruct `H`, `P`, `M`, or `A` from assignment fields;
- infer a coordinate from authority labels or an origin-group string;
- substitute caller assertions for producing coordinates;
- use ambient consumer context without comparing it with the producer's exact
  coordinates;
- substitute a current, latest, default, compatible, or mutable-alias
  coordinate;
- omit `A` because authentication occurred elsewhere; or
- treat equal serialized assignment content as proof that coordinates match.

A result that omits any producing coordinate is not an authority-bearing
detachable Magpie result. Current v0 and standing-v3 outputs do not carry the
new complete `H + P + M + A` authority-bound coordinate set. They therefore
cannot be coerced, wrapped by convention, or treated as authority-bound merely
because their assignment fields resemble a future output. This consequence
reinforces, and does not replace, the additive policy and type quarantine.

No ambient keyring, mutable registry, current organisational membership,
filesystem discovery, environment variable, network lookup, callback, plugin
lookup, or "latest authority" selection may affect replay.

The minimum verification sequence is:

1. establish the exact complete candidate-designation set selected by `A`
   through a supported mode, including exact issuer-role and scope verification
   in authenticated-issuer mode;
2. derive the complete per-key candidate universe from that set and exact
   occurrences in `H`;
3. resolve designated candidate bytes only from `M` and immutable material
   committed by `A`;
4. verify origin-binding statements as claimant assertions;
5. verify authority objects under externally selected `A`;
6. require exact equality for every `AuthorityBindingSubject` coordinate;
7. fold all complete valid decisions for each `GovernedAssignmentKey`; and
8. emit no positive result for an incomplete or conflicting key.

Transport signatures, Magpie log signatures, foreign witness roots, and the
authority authentication may all be useful at different layers. Their counts
must not be combined into authority or origin multiplicity.

### Rotation, delegation, and revocation boundary

`A` pins authority interpretation to one snapshot. A mutable "current authority
key" must not alter a prior result. Root or profile rotation creates a new,
explicitly versioned authority coordinate; it is a different evaluation, not an
update to the old one. Changing the candidate-designation-set identity likewise
creates a new `A` and a different evaluation.

Later revocation, supersession, or changed organisational membership cannot
rewrite `same H + same P + same M + same A`. A future policy may evaluate a
later snapshot against new explicit revocation or supersession material and a
new `A`, but the earlier result remains reproducible under its earlier
coordinates.

Delegation is valid only if a future selected profile explicitly verifies an
immutable authorization path from its externally selected root to the exact
authority decision. Delegation cannot be inferred from claimant labels,
organisational names, event actors, bundle content, or an embedded key.

The representation and operational rules for rotation, delegation, and
revocation are outside this ADR. No implementation may enable those mechanisms
until a separately owner-approved contract pins their immutable inputs,
authorization scope, replay behavior, failure law, and hostile tests. A direct
root-authorized first implementation need not invent a global identity or
delegation system. Production key custody and organisational identity remain
separate future work.

## Fail-closed behavior

The governing law is:

```text
no complete verified authority binding
-> no authority-bound admitted origin
-> zero corroboration separation
-> no positive standing amplification
```

The following outcomes are fixed semantically. Names of future serialized
variants are deferred; their meaning is not.

| Condition | Audit result | Authority-bound result |
| --- | --- | --- |
| No authority object is designated for a claimant origin binding | Visible absence; the claimant assertion may remain in compatibility audit output | No authority-bound admitted origin |
| Authority-looking anchor in `H` with no exact designation under selected `A` | Visible non-designated material; no incompleteness and no conflict | None; it cannot veto a separately valid designated candidate |
| Exact selector is designated under selected `A`, but its required occurrence is absent from `H` | Designation visible; selector is not a candidate at this snapshot; no incompleteness | None |
| Designated authority selector for key `K` occurs in `H`, but its bytes are unavailable | The exact `GovernedAssignmentKey` `K` is incomplete; unrelated complete keys may still resolve | None for `K`; no partial positive result or group winner for `K` |
| Selected candidate-designation set is unavailable | The complete authority candidate universe cannot be established; resolver-level audit incompleteness | No authority-bound decisions |
| Candidate-designation-set identity differs from selected `A` | Visible wrong-coordinate material; the selected universe is not established | No authority-bound decisions under selected `A` |
| Required designation-set verification under the selected mode fails, or the set is malformed or internally ambiguous | Resolver-level designation-universe failure | No authority-bound decisions |
| Candidate-designation set is authenticated to an identity that lacks exact designation authorization | Designation-universe verification failure | No authority-bound decisions |
| Duplicate byte-identical or semantically identical designations | All occurrences may remain visible; the designation collapses structurally | One candidate designation, no additional authority weight |
| One authority selector is designated for incompatible origin-binding identities or governed keys | Designation universe is invalid; no sequence, timestamp, count, or lexical winner | No authority-bound decisions |
| A required verification-material object committed by selected `A` is unavailable | Resolver-level authority verification is incomplete; no network or ambient fallback is attempted | No authority-bound decisions |
| Malformed authority object | Definitively rejected and retained in audit | That object supplies no decision |
| Unsupported authority profile | Resolver/profile failure under selected `A` | No authority-bound decisions |
| Unknown or non-immutable trust root | Resolver/root failure under selected `A` | No authority-bound decisions |
| Failed signature or equivalent authentication | Definitively rejected and retained in audit | That object supplies no decision |
| Origin-binding subject mismatch | Definitively rejected for the governed assignment | None for that assignment |
| Contribution-subject mismatch | Definitively rejected for the governed assignment | None for that assignment |
| Policy mismatch | Visible but ineligible under selected `P` | None under selected `P` |
| Namespace mismatch | Definitively rejected for the governed assignment | None for that assignment |
| Group mismatch | Definitively rejected for the governed assignment | None for that assignment |
| Duplicate byte-identical or semantically identical valid decisions | All occurrences remain visible; the decision collapses structurally | One assignment, never amplification |
| Multiple valid conflicting groups for one `GovernedAssignmentKey` | Terminal conflict, independent of event order, timestamp, or lexical order | No assignment for that key |
| Valid decision under another root or profile | Visible but ineligible under selected `A`; it cannot select a new `A` | None under selected `A` |
| Claimant-only v0 admitted origin | Visible as compatibility output only | Not authority-bound |

Definitively invalid, mismatched, wrong-coordinate, or non-designated objects
cannot veto a separate complete valid decision for its exact subject. Missing
bytes for a designated candidate make only its exact `GovernedAssignmentKey`
incomplete because the resolver cannot prove that key's candidate universe
conflict-free. No available subset is folded for that key. A missing candidate
for one key has no incompleteness authority over an unrelated key whose own
complete designated universe is available, verified, and conflict-free.

Failure to establish the selected designation universe is different: an
unavailable, wrong-identity, malformed, or ambiguous designation set, or failed
required authentication or authorization, prevents the resolver from
identifying any complete candidate universe, so the authority-bound audit emits
no authority-bound decisions.
Authentication of a designation-set issuer does not cure missing exact role or
scope authorization. Grouping-assignment authority does not satisfy
candidate-designation authorization, even when both would use the same
authenticated identity or key.
Neither assignment-scoped nor resolver-level incompleteness is refutation,
negative evidence, invalidation, or a lowering of inherited standing. It
selects no write-order winner and triggers no ambient fallback.

Multiple identical authority bindings do not amplify. Multiple valid
conflicting assignments have no sequence, timestamp, reviewer-count,
signature-count, or lexical winner.

## Corroboration-separation semantics

An authority-bound origin admission licenses only policy-recognised
**corroboration separation** for the exact assignment. It does not prove that
two sources are statistically independent.

```text
authenticated grouping authority
!= statistical-independence proof

distinct authority-bound groups
!= truth

authority-bound origin admission
!= support contribution

support contribution
!= standing

standing
!= settlement
```

One trusted grouping authority may classify different exact contributions into
different groups when the selected policy authorizes that authority to make
each exact assignment. If those groups later survive contribution eligibility,
support projection, lane selection, and a named standing threshold, they may
provide corroboration separation. The result is a policy-recognised separation,
not a scientific proof of independence, source quality, or truth.

Authority bindings themselves are standing-inert. They authorize group
assignments; they do not create support contributions. The existing staged
boundaries remain mandatory:

```text
authority-bound origin admission
-> admitted contribution under an explicit policy
-> support contribution under an explicit policy
-> aggregation under an explicit standing policy
```

## Compatibility and migration law

This ADR chooses additive versioning: preserve current outputs as historical or
compatibility-only and introduce a new explicitly versioned authority-bound
policy path.

If this ADR is accepted:

- `OriginAdmissionAuditV0`, its canonical bytes, its `OriginAdmitted` results,
  admitted-contribution v0, support-contribution v0, and standing-v3 results
  remain replayable under their exact existing `H + P + M` coordinates.
- Those outputs are not reinterpreted as independently authority-bound. A v3
  distinct-group count remains evidence of what current v3 counted, not proof
  of authenticated corroboration separation.
- Current v0 derives candidates through shape matching over anchor kind,
  algorithm, and profile. It has no externally authenticated candidate
  designation, no candidate-designation-set identity in `A`, and no
  assignment-scoped designation law. That candidate-completeness behavior
  remains compatibility-only and cannot satisfy, be coerced into, or be wrapped
  by convention as the authority-bound successor contract.
- A new authority-bearing origin-admission policy must have a distinct explicit
  policy identity and a result type that cannot accept, coerce, deserialize, or
  alias a claimant-only v0 admitted origin.
- Every authority-bound successor audit or result that may cross its
  constructing context must carry its complete producing `H + P + M + A`
  identities and require downstream exact matching.
- Any authority-bearing standing consumer must select its own explicit policy
  identity and accept only authority-bound contribution inputs. It must not
  accept current v0 grouping merely because field shapes match.
- Existing policy identifiers, schemas, fixtures, and canonical bytes remain
  unchanged. No existing serialized output gains a new meaning.
- Policy selection remains explicit and exact. No wildcard, default, ambient
  latest version, or compatibility fallback may cross the boundary.

The exact future identifiers are not chosen merely because a next number is
available. The implementation contract must choose identifiers that make the
semantic break explicit and must pin their accepted input and output types.

Changing current v0/v3 semantics in place is rejected because it would silently
alter replay interpretation. Keeping current APIs and adding only a wrapper or
consumer convention is also rejected: it leaves authority-looking v0 values
structurally available to new consumers. Internal code may be reused, but the
public policy and type boundary must be additive and unambiguous.

## Hostile scenario table

| Scenario | Audit visibility | Authority-bound admission | Corroboration separation | Standing effect | Reason |
| --- | --- | --- | --- | --- | --- |
| 1. Two claimant-authored valid bundles use the fixed labels and fresh groups, with no authority binding | Both claimant statements and groups remain visible | None | Zero | No positive amplification | Canonical bytes, anchors, and literal equality do not authenticate either assignment |
| 2. One valid authority binding is replayed onto another contribution | Subject mismatch is visible | None for the second contribution | Zero for the second contribution | No amplification from the replay | Exact claim, evidence, edge, scope, and artifact coordinates differ |
| 3. One valid binding is replayed into another namespace or group | Namespace or group mismatch is visible | None for the substituted assignment | Zero | No amplification from the replay | Namespace and group are authenticated assignment coordinates |
| 4. An authority object embeds and trusts its own key | Embedded key remains visible; verification under external `A` fails or ignores it | None | Zero | No amplification | An object cannot select its own trust root |
| 5. Two identical authority decisions are present | Both occurrences are visible and collapse | One assignment | At most one group membership for that assignment | No duplicate amplification | Occurrence, signature, and reviewer counts are not origins |
| 6. Two valid authority decisions assign conflicting groups to the same key | Terminal conflict is visible with both groups | None for that key | Zero for that key | No amplification and no winner | Sequence, timestamp, lexical order, and counts are non-authoritative |
| 7. A decision is valid under the wrong authority root or profile | Visible as wrong-coordinate material | None under selected `A` | Zero under selected `A` | No amplification | Objects cannot change the externally selected profile or root |
| 8. A required authority-verification-material object committed by selected `A` is unavailable | The exact missing material and inability to evaluate selected `A` completely are visible; resolver-level verification is incomplete | No authority-bound decisions | Zero | No positive amplification; inherited standing is unchanged | No network, ambient keyring, mutable registry, or fallback source is used; incompleteness is not refutation or negative evidence |
| 9. A current v0 admitted group is offered to a future authority-bearing policy | Compatibility input and type/policy rejection are visible | None | Zero | No amplification | Additive policy and type gates forbid claimant-only input |
| 10. Two signatures or reviewers are treated as two origins | Signatures/reviewers remain visible; identical decisions collapse | One exact assignment if otherwise valid | At most one group membership | No count amplification | Signer multiplicity is not origin multiplicity |
| 11. One authenticated authority assigns two exact contributions to two groups | Two exact verified decisions are visible | Both assignments admitted | Two authority-bound groups within the same exact comparison namespace | May satisfy a later explicit threshold; never settles by itself | The policy authorizes each exact assignment; this is corroboration separation, not proof of independence |
| 12. Authority rotation occurs after an earlier snapshot | Earlier result retains its pinned `A`; later material is outside that snapshot | Same result for the earlier coordinates | Same as the earlier replay | Byte-identical earlier standing input/result | A mutable current key cannot rewrite an earlier snapshot; a new root/profile creates new coordinates |
| 13. An authority-bound admitted-origin result produced under one `H + P + M + A` set is presented to a support or standing consumer selecting a different prefix, policy, closure, authority profile, trust root, verification-material closure, or candidate-designation-set identity | Producer and consumer coordinates remain visible, including the exact mismatch | Producer result remains scoped to its own coordinates; downstream composition is rejected | Zero through the mismatched input | No positive amplification; inherited standing is not lowered | Equal assignment content is not coordinate equality, and the mismatch is not refutation, invalidation, contradiction, or negative evidence |
| 14. A claimant records an authority-looking `SegmentAnchored` selector that is absent from the designation set selected by `A`, and its bytes are unavailable | The anchor remains visible as non-designated material | None; no incompleteness and no veto over a separate valid designated decision | Zero | No standing effect | Occurrence without external designation has no candidate or completeness authority |
| 15. The designation set in `A` designates a selector for key `K`, the selector occurs in `H`, but its bytes are unavailable | The exact designation, occurrence, missing bytes, and incomplete key are visible | None for `K`; no partial positive result or group winner | Zero for `K`; unrelated complete keys may resolve | No amplification for `K`; inherited standing is unchanged | Membership precedes availability, and incompleteness is scoped by the designation's exact governed key |
| 16. A caller supplies only available designated candidates and omits an unavailable designated selector that could conflict | The caller subset and complete designation-controlled universe remain distinguishable | Caller subset is rejected; the affected key remains incomplete | Zero for the affected key | No partial positive amplification | The complete set selected by `A`, not availability or caller preference, controls enumeration |
| 17. A result produced under one candidate-designation-set identity is composed under another `A` with a different set identity | The exact producer and consumer designation-set identities and mismatch remain visible | Composition rejected | Zero through the mismatched input | No positive amplification; inherited standing is unchanged | Candidate-designation-set identity is a producing coordinate, and mismatch is not refutation or negative evidence |
| 18. The same authority-object selector is designated for incompatible governed keys or origin-binding identities | The incompatible designations remain visible; the designation universe is invalid | None; no order-based winner | Zero | No positive amplification | Candidate designation must be unambiguous before authority-object verification; sequence, timestamp, reviewer, signature, and lexical order are non-authoritative |
| 19. An authority authenticated and authorized for exact origin-group assignments signs or emits a candidate-designation set without separately verified candidate-designation authorization | The identity, grouping scope, and attempted set remain visible; designation-set authority verification fails | No candidate universe is established and no authority-bound decisions are emitted | Zero | No positive amplification | Grouping-assignment authorization is not transposed into candidate-designation authority |
| 20. External selection of `A` directly commits to one exact immutable complete candidate-designation-set identity without relying on an issuer carried by the set | The exact directly selected set identity and complete producing coordinates remain visible | The set is authoritative for candidate completeness; designation alone supplies no group assignment | Zero from designation alone | No standing effect from designation alone | Accepted intentionally: objects inside the set cannot alter its identity or select another set, and replay remains deterministic under `H + P + M + A` |
| 21. `A` designates an authority-object selector for key `K`, but snapshot `H` contains no required exact `SegmentAnchored` occurrence for it | The designation and absent occurrence remain visible | The selector is not a candidate at `H`; `K` is not incomplete merely from designation, and no assignment is emitted | Zero | No standing effect; unrelated decisions are unaffected | Designation is not occurrence; a later prefix containing the occurrence has a different `H` |

## Relationship to existing ADRs and contracts

This Proposed ADR does not silently amend an Accepted ADR or a ratified design
contract merely by existing.

- ADR-0001's anchored hierarchy remains intact. Existing `SegmentAnchored`
  events may record a future authority object without a new L0 payload tag, but
  the anchor remains occurrence and inclusion evidence, not authentication.
- ADR-0002's no-hidden-authority boundary and separation of standing from truth
  are reinforced.
- ADR-0003's coordinate-bound standing definition remains intact. An
  authority-bound contribution would be an explicit input to a later named
  standing policy, not ambient authority.
- ADR-0004 and ADR-0005 remain orthogonal; lifecycle and attributed withdrawal
  acts do not authenticate grouping authority.
- ADR-0006's explicit policy-selection and compatibility laws are followed, but
  its currentness and invalidation decision does not ratify origin authority.
- The current origin-binding, origin-admission, admitted-contribution,
  support-contribution, and standing-v3 contracts remain accurate records of
  current v0/v3 behavior until separately reconciled after acceptance.

## Required reconciliation if accepted

Acceptance would require a separate documentation change that explicitly
narrows or supersedes the following living-contract statements. This proposal
does not edit them:

1. `docs/design/origin-admission-audit-v0.md` §5, especially the statement that
   compiled selection of exact `human-review` plus
   `authority:origin-review-v0` supplies trust, must be narrowed to
   compatibility label selection. Sections 11, 15, and 19 must state that
   current v0 `OriginAdmitted` is not independently authority-bound. Sections
   6–8's shape-matched candidate universe and global byte-completeness gate must
   remain compatibility behavior and must not define the authority-bound
   successor's candidate designation or assignment-scoped incompleteness.
2. Any living language that calls equality with that pair independently
   trusted authority must distinguish fixed vocabulary selection from
   authentication.
3. `docs/design/origin-binding-bundle-v0.md` §13.2 must preserve the fields as
   claimed authority data while removing any implication that a consumer can
   obtain corroboration authority by trusting the claim itself. Its receipt and
   duplicate/conflict sections must point to the separate authority boundary.
4. `docs/design/artifact-provenance-origin-admission.md` §§4 and 10–14 must
   place the separate authority object and coordinate `A` at the missing
   authority seam while retaining matched origin bindings as claimant
   assertions.
5. `docs/design/standing-aggregation-independence-groups.md`'s authority and
   self-declared-group laws must distinguish current claimant-only v0 admission
   from authority-bound admission and reserve corroboration separation for the
   latter.
6. `docs/design/standing-policy-v3-external-report-corroboration-v0.md`'s
   authority path and distinct-group contract must classify existing v3 as
   claimant-label compatibility behavior and identify the additive
   authority-bound successor boundary without changing v3 bytes.
7. `docs/design/admitted-contribution-audit-v0.md` and
   `docs/design/support-contribution-audit-v0.md` must clarify that carrying a
   v0 group preserves upstream compatibility data and does not authenticate it.
8. Tickets 0046 and 0048–0053 remain historical implementation records. A new
   implementation contract must supersede their authority-bearing
   interpretation rather than rewriting those records.

No reconciliation may be applied as if this ADR were already accepted. The
separate reconciliation review must preserve historical replay and identify
every authority-bearing consumer.

## Consequences

Positive consequences:

- authority comes from an externally selected, replayable verifier boundary,
  not claimant vocabulary;
- the exact subject blocks cross-contribution, cross-namespace, cross-group,
  cross-policy, cross-profile, and cross-root substitution;
- detachable authority-bound outputs retain complete producing coordinates and
  cannot be substituted merely because assignment content is equal;
- claimants cannot manufacture completeness-blocking candidates merely by
  anchoring authority-looking selectors;
- an unavailable designated candidate suppresses only its exact governed
  assignment key while unrelated complete keys may resolve;
- current serialized outputs remain reproducible and byte-stable;
- invalid or duplicate authority material cannot amplify standing; and
- the architecture remains consistent with anchored foreign bundles without a
  new L0 payload tag or FORMAT change.

Costs and constraints:

- a separate authority object, verifier profile, trust coordinate, immutable
  verification-material closure, finite immutable complete candidate-
  designation-set representation, and additive policy/result boundary must be
  contracted and implemented;
- unavailable designated authority material can make its exact governed key
  incomplete, while failure to establish the designation universe prevents all
  authority-bound decisions;
- operators must select and retain immutable authority coordinates, including
  the candidate-designation-set identity, for replay;
  and
- existing v0/v3 APIs remain available for historical replay, so consumers need
  an explicit quarantine boundary rather than relying on names or convention.

## Rejected alternatives

- **Treat the compiled pair as a credential.** It authenticates no issuer.
- **Treat canonical origin-binding bytes as authority.** Canonicalization fixes
  representation, not authorization.
- **Treat an anchor as authority.** Inclusion does not prove the right to make
  the assignment.
- **Treat every shape-matching authority-looking anchor as a candidate.** That
  lets unauthenticated material manufacture completeness-blocking authority.
- **Use caller-selected, referenced, or available-only candidates.** A
  favourable subset can hide a missing conflicting decision.
- **Authenticate only the current origin-binding object.** This erases the
  claimant/authority ownership boundary and invites silent v0 reinterpretation.
- **Trust a key carried by the authority object.** That lets the claimant choose
  the root and is self-authentication.
- **Use event actor/source or organisational names.** Attribution and names do
  not establish identity, delegation, or an authorization chain.
- **Count signatures, signers, or reviewers as origins.** These are occurrences
  or actors, not contribution origin groups.
- **Infer publisher identity from URL or domain.** Locator resemblance is not
  authenticated source identity or grouping authority.
- **Use a trusted boolean, reputation score, or generic registry.** These hide
  policy and mutable authority outside replay coordinates.
- **Change v0/v3 in place.** That would silently reinterpret serialized
  historical output.
- **Wrap current APIs without a new typed policy path.** A convention-only gate
  cannot prevent accidental claimant-only input reuse.

## Deferred implementation decisions

The following mechanism details remain for an owner-approved implementation
contract after this ADR is accepted:

- exact authority-object schema, canonicalization profile, bundle kind, size
  bound, witness or signature envelope, and algorithm set;
- exact identifiers for the authority verification profile, trust-coordinate
  representation, authority-bound origin-admission policy, downstream
  contribution policies, and standing policy;
- exact immutable closure representation for root, intermediate, delegation,
  revocation, or equivalent verification material;
- exact finite candidate-designation-set representation, canonicalization,
  identity verification, and, when applicable, authentication mechanism;
- whether the first implementation supports direct external commitment,
  authenticated designation issuance, or both, and for issuer mode the exact
  designation-authority, authorization-path, and authorization-scope identity
  representations;
- exact representation of the selector-to-origin-binding and
  `GovernedAssignmentKey` designation tuple;
- exact candidate-universe enumeration, assignment-scoped incompleteness
  representation, designation-universe failure precedence, verifier API,
  private-construction boundary, and serialized audit variants;
- exact representation of the complete coordinate-bearing result and the
  cross-coordinate checks at every downstream authority-bearing type boundary;
- exact storage and transport mechanism, provided it uses finite immutable
  inputs and no network or ambient lookup during replay;
- hostile fixtures and compile-time/runtime type-quarantine tests;
- rotation, delegation, revocation, and supersession representation;
- production key custody and operational identity integration; and
- staged migration, release, and compatibility-review sequence.

Those decisions may choose among cryptographic mechanisms that satisfy this
ADR. They may not weaken the exact subject, external trust selection, replay
law, fail-closed behavior, additive compatibility boundary, or narrow grant.

## Non-goals

This ADR and its documentation-only proposal do not provide or authorize:

- runtime implementation;
- new Rust types;
- tests or fixtures;
- a new L0 payload tag;
- changes to `docs/FORMAT.md`;
- canonical encoding changes;
- a global identity system;
- organisational membership resolution;
- production key custody;
- automatic source clustering;
- URL/domain publisher inference;
- probabilistic independence scoring;
- source reputation;
- truth scoring;
- generic admission or `EpistemicGate`;
- currentness or withdrawal implementation;
- MCP or librarian authority; or
- a release decision.

## Acceptance and implementation gates

This ADR remains Proposed until an owner explicitly ratifies it. This prompt,
green tests, existing code, merged audit documents, or the presence of this file
do not constitute ratification.

After owner ratification, a separately reviewed implementation contract must at
minimum pin:

1. the exact authority object and immutable identity;
2. the exact bound subject and equality checks;
3. the exact distinction between grouping-assignment authorization and
   candidate-designation authorization, including the law that authentication
   alone grants neither role;
4. whether the first implementation supports direct external commitment,
   authenticated designation issuance, or both;
5. for authenticated-issuer mode, the exact designation-authority identity,
   immutable authorization-path identity, exact authorization-scope identity,
   and failure behavior when issuer authentication succeeds but exact role or
   scope authorization fails;
6. the complete external authority coordinate `A`, including one exact finite
   immutable candidate-designation-set identity;
7. the exact `AuthorityCandidateDesignation` tuple binding an authority-object
   selector or identity, origin-binding identity, and `GovernedAssignmentKey`;
8. the exact rule deriving all and only designated candidates with required
   occurrences in `H` under `P` and `A`;
9. proof that candidate membership is fixed before and independently of
   availability, parsing, authentication outcome, and caller selection;
10. snapshot-relative designated-but-no-occurrence behavior that creates no
    candidate, incompleteness, assignment, conflict, or ambient lookup;
11. assignment-key-scoped incompleteness for unavailable designated candidates;
12. whole-audit failure when the selected designation universe is unavailable,
    wrong-coordinate, malformed, ambiguous, or fails required authentication
    or authorization;
13. closed fail-closed audit outcomes, authority-decision conflict precedence,
    and distinct designation-universe failure precedence;
14. additive policy and type quarantine from current v0/v3 output;
15. a coordinate-bearing authority-bound output type that retains the exact
    identities of `H`, `P`, `M`, and the complete four-component `A`, including
    its candidate-designation-set identity, with authenticated-issuer outputs
    also retaining the required derived designation-authority audit identities;
16. hostile cross-coordinate tests that independently reject a wrong `H`, wrong
    `P`, wrong `M`, wrong authority profile, wrong trust root, wrong
    verification-material closure, and wrong candidate-designation-set
    identity, plus identical assignment content produced under different
    coordinates;
17. hostile authorization-role tests proving that grouping authority alone
    cannot designate candidates and that a signed designation set from an
    authenticated but unauthorized issuer establishes no candidate universe;
18. hostile candidate-designation tests covering:
    - an undesignated missing authority-looking anchor;
    - a designated selector without its required occurrence in `H`;
    - a designated missing candidate;
    - an available-only or caller-selected subset;
    - a wrong candidate-designation-set identity;
    - duplicate and conflicting designations; and
    - a missing candidate for one key while an unrelated key remains complete;
19. hostile tests distinguishing resolver-level missing authority-verification
    material from assignment-scoped missing candidate bytes;
20. hostile tests for every scenario in this ADR; and
21. compatibility and documentation reconciliation.

Only a later runtime change, hostile-test suite, compatibility review, and
accepted contract can implement this decision. The audit disposition can then
be reassessed on evidence.

ADR-0007 addresses the doctrine gap identified by A-024. A Proposed ADR does
not remediate the current runtime finding. A-024 remains **Confirmed** until
accepted doctrine, runtime changes, hostile tests, and compatibility review are
all complete. This documentation change must not move the audit disposition
ledger to Partially remediated, Scheduled, or Closed.

## References

- `docs/audits/audit-disposition-2026-08.md`, A-024 and §§9–11.
- `docs/audits/magpie-architecture-audit-2026-08-01.md`, A-024 details,
  hostile two-bundle scenario, authority-flow analysis, compatibility
  inventory, and authority-pass continuations.
- `docs/design/artifact-provenance-origin-admission.md`.
- `docs/design/origin-binding-bundle-v0.md`, especially §13.2.
- `docs/design/origin-admission-audit-v0.md`, especially §§3, 5, 11, 15,
  19, and 22.
- `docs/design/standing-aggregation-independence-groups.md`.
- `docs/design/standing-policy-v3-external-report-corroboration-v0.md`.
- `docs/design/admitted-contribution-audit-v0.md`.
- `docs/design/support-contribution-audit-v0.md`.
- ADR-0001 through ADR-0006.
- Tickets 0037, 0042, 0046, 0048, 0049, 0050, 0051, 0052, and 0053.
- `crates/magpie-claims/src/origin_binding_verifier.rs`.
- `crates/magpie-claims/src/origin_admission_audit.rs`.
- `crates/magpie-claims/src/admitted_contribution_audit.rs`.
- `crates/magpie-claims/src/support_contribution_audit.rs`.
- `crates/magpie-claims/src/standing_v3.rs` and their targeted tests.
