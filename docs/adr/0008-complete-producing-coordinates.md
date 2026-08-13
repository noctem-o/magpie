# ADR-0008: Complete Producing Coordinates

**Status:** Proposed (explicit repository-owner ratification required)

## Summary / Y-statement

In the context of Magpie policies whose results can depend on verified history,
immutable external material, caller-selected subjects, inherited derivations,
and future authority inputs, facing the contradiction between ADR-0003's closed
three-coordinate definition and the closure-dependent standing-v3/v4 runtime,
we propose one profile-relative law:

```text
for one immutable derivation profile G:

D = Derive_G(I_G)

G   = the immutable identity of the complete normative derivation semantics
I_G = the closed, profile-specific, complete immutable semantic inputs
D   = a semantic outcome defined by G, including its typed semantic failures
```

No semantic input on which `D` can depend may be absent from the producing
context `(G, I_G)`. Inputs are present either as exact immutable values or
through immutable identities that commit to them transitively and
unambiguously. No ambient lookup, mutable alias, implementation default, or
result-content equivalence may supply a missing input.

This is a proposed constitutional amendment. It is not Accepted doctrine until
the repository owner explicitly ratifies it.

## Context and contradiction

ADR-0003 defines standing as the deterministic output of a named resolver over
a verified record snapshot and says that standing is reproducible from exactly
three coordinates:

```text
snapshot + policy version + resolver identity
```

That closed list does not describe the implemented standing-v3/v4 paths:

- v3 derives policy v2 and a support-contribution audit from one
  `OriginAdmissionReplayContextV0`, and the audit also consumes one immutable
  `ResolutionContentClosureV0`;
- `StandingResolutionV3` retains both verified-prefix and closure identities;
- v4 internally derives v3 from that same replay context and closure, validates
  the inherited prefix and closure identities, and retains them again; and
- ADR-0007 requires future authority-bound derivations to consume additional
  explicit authority inputs.

For v3, two evaluations can use the same verified prefix and standing policy
while different closure material changes availability, origin admission,
support contributions, and therefore standing. Calling all of that material a
"snapshot" would hide a semantically distinct external input rather than repair
the law.

The opposite reaction is also unsafe. Freezing a larger universal tuple such as
`H + M + A + P + R + Q` would make today's architecture constitutional syntax,
encourage meaningless optional fields, and still need amendment for the next
genuine input class.

## Scope

This ADR governs a **governed detached derived result**: a result whose claimed
derivation may be retained, compared, exposed, or consumed outside the private
computation that constructed it. This includes standing and can also govern a
future currentness, query, provenance, projection, or authority-bound result
when that result crosses such a boundary.

"Governed" here means that callers rely on the result's claimed derivation. It
does not mean that the result is truth, evidence, historical authority, or
admission authority.

Private intermediate values need not each duplicate every coordinate. Their
enclosing producing context must nevertheless close over every transitive
semantic input, and any value that crosses that context must obey the carriage
law below.

This ADR does not require every result class to use the same fields. It requires
every profile to close its own input type under one common rule.

## Terms

**Derivation profile (`G`).** The immutable identity of one complete normative
semantic procedure: its input interpretation, rules, ordering, typed semantic
failure law, and semantic output. A profile may be identified by one value or
by a closed tuple of existing semantic identifiers. It is one semantic
coordinate even when its representation is structured.

**Complete semantic inputs (`I_G`).** The closed typed input value required by
`G`. It includes every explicit historical, external-material, authority,
caller, query, or inherited-derivation input that the profile can use. Its
shape is profile-specific, not a universal optional-field bag.

**Producing context.** The pair `(G, I_G)`. It answers: "What exact semantic
computation was requested?"

**Producing coordinates.** Exact values or immutable typed identities through
which the producing context identifies `G` and every member of `I_G`.

**Semantic outcome (`D`).** A conclusion defined by `G` over `I_G`: either a
profile-defined successful result or a profile-defined typed semantic failure.
An operational execution failure such as an I/O error, resource exhaustion,
process interruption, unavailable storage, host crash, or an equivalent
failure to complete evaluation is not `D`. It produces no semantic conclusion
and does not force environmental state into `I_G`. If `G` deliberately gives
finite availability or absence semantic meaning, that explicit immutable value
remains a member of `I_G`. Equal outcome content does not imply equal producing
contexts.

**Result identity.** A possible future commitment to both a producing context
and its semantic outcome. It answers a different question: "What exact outcome
did that computation produce?" This ADR fixes the distinction but no runtime
type, encoding, or digest.

## Decision

Upon explicit owner ratification, the following three invariants govern the
scope above.

### 1. Complete profile-specific inputs

For a selected `G`, `I_G` is closed and complete. If changing an input while
holding the other producing inputs fixed can change `D`, including a
profile-defined typed semantic failure or applicability outcome, that input
must be represented in `I_G` directly or through an immutable identity that
commits to it.

The same rule applies when an input defines the exact history, subject, scope,
or trust interpretation to which an otherwise equal conclusion refers. Equal
output values do not erase different producing contexts.

An identity may commit to material without embedding all of its bytes. Replay
still requires the exact material to be supplied explicitly and checked against
that identity. An identity is not permission for an ambient fetch.

An operational failure to obtain material or complete evaluation yields no
`D`. Recording host, storage, resource, interruption, or other environmental
state in `I_G` does not convert that failure into a semantic outcome. This does
not alter a profile that deliberately defines a finite supplied-material or
absence value as an immutable semantic input.

### 2. No ambient or laundered semantics

Every semantic input is selected explicitly and is immutable for the
evaluation. A current, latest, default, compatible, mutable-by-name, or
environment-selected value is not a producing coordinate. Neither are hidden
filesystem, content-addressed-store, network, callback, plugin, registry, or
process-configuration lookups.

A nested derivation cannot launder omitted inputs. When an inherited result is
re-derived internally from its complete producing context, the enclosing
profile must either:

1. retain the inherited profile and complete producing coordinates; or
2. bind an immutable inherited producing-context identity that transitively
   commits to them.

If both flattened coordinates and a context identity are retained, they must
exact-match. For this internally re-derived case, the complete context binding
is sufficient because the inherited `D` is recomputed under that context.

If an inherited `D` is instead detached or caller-supplied, a producing-context
identity alone identifies only the requested computation; it does not verify
the supplied outcome. The consumer must additionally verify that `D` is the
exact semantic outcome bound to that context, either by re-deriving and
exact-matching it or by checking a future result identity or equivalent
immutable binding that commits to both context and `D`. `(context_id,
arbitrary_D)` is not a valid inherited semantic input. A result-content digest
and a separately asserted context identity do not establish that binding.

### 3. Profile-relative derived result and boundary carriage

`D` states only the semantic outcome that `G` concluded from `I_G`. Every
completed conforming evaluation of the same `(G, I_G)` must produce the same
`D`. An operational failure to complete evaluation produces no `D` and is not
a counterexample to semantic determinism. A change to normative semantics
requires a different `G`; behavioral drift under the same `G` is
non-conformance.

Any governed derived result that leaves its constructing context must carry, or
be unambiguously bound to, its complete producing context. An incomplete
detached value may remain inspectable compatibility or audit output, but it
must not claim complete reproducibility and must not be accepted as a
coordinate-complete input merely because its semantic fields look equal.

Serialization does not make a result historical authority. Standing remains
derived, policy/profile-relative, and authoritative about nothing beyond its
producing context.

The compact governing law is therefore:

> A governed Magpie-derived result may never silently depend on a semantic
> input that its producing context does not identify.

## Policy identity and resolver identity

ADR-0003 treats policy version and resolver identity as independent standing
coordinates. The safer boundary is to separate **normative semantics** from
**executable implementation identity**.

`G` identifies the complete normative derivation semantics. Existing policy
and resolver semantic identifiers may jointly form `G`, or one existing policy
identifier may identify the whole closed profile when its contract actually
does so. This ADR does not require a new scalar runtime identifier or rename any
current policy.

An implementation, package, build, language, host library, or process is not a
semantic coordinate merely because it executes `G`. Conformant Rust, Go, and
Python implementations of the same `G` over the same `I_G` must produce the
same `D` whenever evaluation completes, without creating three epistemically
different results. Implementation identity may remain assurance provenance.

If an implementation changes behavior while retaining `G`, it has a
conformance defect. It has not created legitimate new policy semantics. If a
different resolver algorithm or typed semantic-failure law is intended, it
requires a different `G`.

A human-readable resolver name is sufficient only when it immutably and
unambiguously identifies the normative semantics. A mutable alias that resolves
through a registry, build default, or deployment configuration is not `G`.

Policy selection remains explicit: selecting the derivation profile is a
caller act, and there is no ambient latest profile.

## Exact current standing matrix

The current implementation supplies the following evidence. "Detached
complete" asks whether the public result type alone identifies the complete
producing context under this proposed law; it does not judge whether the
in-process path is deterministic.

| Policy | Verified-history call substrate | External immutable material | Internally inherited derivations | Profile / implementation treatment | Caller semantic input | Currentness interaction | Detached complete |
| --- | --- | --- | --- | --- | --- | --- | --- |
| v0 | `StandingView`, normally folded from verified replay; no prefix identity is retained | none | none | fixed `magpie-claims-standing-v0` semantics; executable identity is implicit | exact `claim_id` | emits `Unknown` | No. It carries claim and policy, but no producing history or separately complete profile commitment. |
| v1 | `StandingReplaySnapshot`: standing plus anchor index co-derived by one verified replay, with event count | none | v0 is derived internally from the same snapshot | fixed `magpie-claims-standing-v1` semantics; no independent implementation coordinate | exact `claim_id` | copies v0 `Unknown` | No. It carries no verified-prefix or equivalent producing-history identity. |
| v2 | the same `StandingReplaySnapshot`; deterministic verifier context is derived from its claim, evidence, edge, and witness material | none | v0 and v1 are derived internally and their candidate traces are exact-matched | fixed `magpie-claims-standing-v2` semantics | exact `claim_id` | copies inherited currentness | No. It retains policy and nested trace, but no producing-history identity. |
| v3 | `OriginAdmissionReplayContextV0`: one replay snapshot plus `VerifiedLogPrefixIdentityV0` | one exact `ResolutionContentClosureV0` (`M`) | v2 and the origin/admitted/support audit chain are derived internally from the same context; the audit consumes the same closure | fixed v3 semantics plus exact checks of inherited v2, support, admitted, and origin policy IDs | exact `claim_id`; target scope is read from the same history | copies inherited v2 currentness | No. It carries claim, policy, prefix, closure, inherited v2, and support audit, but not a complete general producing-context/profile identity; the prefix type also does not by itself name every verification/trust interpretation contemplated by future `H`. |
| v4 | the same `OriginAdmissionReplayContextV0`; negative candidates are enumerated from that snapshot | the same closure is consumed transitively through internally derived v3 and retained by v4 | the exact v3 result is derived internally, then claim, policy, prefix, and closure are exact-matched before extension | fixed `magpie-claims-standing-v4` semantics; inherited v3 policy is checked | exact `claim_id` | copies inherited v3 currentness | No. It retains prefix and closure both directly and in inherited v3, but there is no complete producing-context identity and the same detached limitations remain. |

For v4, the direct-refutation candidate lane does not itself read closure bytes.
The v4 result still depends transitively on closure because v3 is derived first,
v4 validates its closure identity, and inherited v3 standing controls v4
precedence. Omitting `M` would therefore be coordinate laundering.

The matrix does not create a new defect classification or upgrade any current
surface. In particular, it does not call current v3/v4 compatibility output
coordinate-complete or reproducible. Current one-way audit output remains
available under its existing contracts and non-authorizing boundaries.

## Coordinate classification

The profile owns the exact typed input shape. These classification rules avoid
collapsing distinct concerns:

| Candidate value | Treatment under this ADR |
| --- | --- |
| Exact verified historical material | A semantic input when `G` derives from it. Its coordinate identifies the exact historical input used. |
| Historical verification profile or externally selected historical trust root | Included in `I_G`, inside the historical coordinate or separately, whenever it affects the verified interpretation or whether the derivation is valid. ADR-0007 requires future `H` to commit both; this ADR does not force that packaging universally. |
| Immutable external closure or supplied material | A separate semantic input when consumed. It must not be renamed "snapshot." Finite absence or availability is committed by an explicit immutable input only where `G` deliberately gives it semantic meaning; operational inability to obtain material produces no `D`. |
| Future authority profile, trust root, verification material, or designation universe | Explicit profile-specific inputs. They may form an `A` value; authority may not be inferred from history or labels. |
| Caller-selected claim, subject, scope, query, or other request parameter | A semantic input whenever changing it can change what is evaluated or returned. Explicit caller selection does not make it non-semantic. |
| Normative policy and resolver rules | Identified by `G`, not hidden in an implementation. |
| Executable build, package, language, or host library | Assurance provenance, not semantic input, for a conforming implementation of `G`. Behavioral divergence is non-conformance or requires a new `G`. |
| Projection database layout, cache layout, or derived-state implementation version | Representation or operational metadata unless `G` deliberately makes it part of semantic interpretation. Hidden semantic dependence on it is forbidden. |
| JSON field order, prose wording, or explanatory formatting | Representation, not standing semantics. A profile may separately govern exact audit bytes without making formatting an input to semantic standing. |
| Audit provenance | Evidence about execution and carriage. It does not become truth, authority, or an omitted semantic input merely by being serialized. |

This table distinguishes producing coordinates from assurance and
representation metadata. It does not prohibit retaining extra audit
provenance; it prohibits using that provenance as a substitute for a required
semantic input.

## Historical coordinate boundary

The historical member of `I_G` identifies the exact verified historical input
used by that profile. It does not, merely by existing, prove:

- latest or current history;
- global completeness;
- freshness;
- rollback protection;
- checkpoint agreement;
- non-equivocation; or
- remembered-head continuity.

`VerifiedLogPrefixIdentityV0` is today's narrow count-and-tip identity. This ADR
does not widen its meaning. A future checkpoint-matched historical input may be
a different typed `H` under the same complete-input law. Choosing checkpoint,
rollback, witness, or freshness semantics is the next constitutional tranche,
not this one.

## Nested derivations

The universal requirement is transitive completeness, not mandatory field
duplication.

An outer derivation that internally re-derives an inherited semantic result from
its complete inherited producing context must bind that exact context. It may:

- flatten the inherited profile and inputs into its own producing context;
- retain one immutable inherited producing-context identity that commits to
  them; or
- retain both for audit clarity, with exact equality checks.

For this internally re-derived case, complete context binding is sufficient:
the inherited `D` is recomputed rather than accepted by assertion. The second
form is the preferred compact model once such an identity is designed. It lets
v4-like successors avoid an ever-growing repeated tuple while preserving the
exact inherited computation. Until a complete context identity exists,
retaining and matching the actual transitive coordinates is the safe form.

An outer derivation that accepts a detached or caller-supplied inherited `D`
must also verify that exact outcome against the bound inherited context. It may
re-derive and exact-match `D`, or a future result identity or equivalent
immutable commitment may bind the inherited producing context and semantic
outcome together. A context identity paired with arbitrary result content is
invalid, as is a result-content digest that does not unambiguously bind the
complete context. Concrete `ResultIdentity` runtime and API design remains
deferred to C3.

Current v4 effectively uses the third form for the coordinates current types
expose: it retains prefix and closure directly, embeds v3, and exact-matches
them. That is useful evidence, not a claim that the current types already
implement the complete future identity model.

## Generalization boundaries

### Currentness

Currentness remains a distinct derived facet under ADR-0006. A standing
profile does not silently consume a separately derived currentness result, and
identical standing may coexist with a different currentness result produced by
another profile and context.

ADR-0006's three-coordinate statement remains a profile-specific closed input
description for the currency semantics it defines: verified history plus
explicitly selected policy and normative resolver semantics. The
policy/resolver pair jointly identifies `G`; "resolver identity" there is not
an executable build identity. This ADR does not authorize an additional
currentness input. If a future currentness profile genuinely consumes one,
that profile must identify it explicitly and any required ADR-0006 amendment
must be reviewed separately.

The existing `StandingCurrentness` field in standing v0-v4 output is a
compatibility-carried field: production v0 emits `Unknown`, and v1-v4 copy the
inherited value. No standing policy currently implements the ADR-0006
currentness resolver, and this ADR does not reinterpret that field.

### Queries, provenance responses, and projections

The Proposed provenance-response and verified-librarian contracts already state
that a detached answer remains connected to its producing history and
derivation. That convergent repository boundary supports the scoped general law
here rather than a standing-only exception; this ADR does not ratify either
contract wholesale.

A future query coordinate `Q` is simply a member of `I_G` for a query profile
that consumes it. This does not authorize a query runtime, librarian, wire
schema, ranking policy, or natural-language result identity. Existing episodic
rows and bare search sequence numbers remain coordinate-poor compatibility
surfaces under their existing audit disposition.

Internal projections remain rebuildable derived state. They need not attach a
full envelope to every private row while the constructing context remains
intact. A detached public result that claims governed reproducibility must bind
the relevant profile, historical input, query, and any other semantic input.

### Future authority input

For an authority-bound profile, `A` is an explicit member of `I_G`; it is never
ambient and never inferred from claimant labels or equal result content.
ADR-0007's `H + P + M + A` is one profile-specific closed input description,
not a new universal tuple. Its `P` identifies the producer's normative policy
and therefore contributes to that producer's `G`; `H`, `M`, and `A` are the
profile-specific semantic inputs.

Upon owner ratification, this ADR would select ADR-0007 reconciliation Outcome
B at the doctrinal level. It would not implement ADR-0007, provide its future
types, satisfy its runtime hostile tests, or authorize current v3 output as an
authority-bound input.

## ADR-0003 reconciliation

Upon explicit owner ratification, this ADR amends only ADR-0003's closed
producing-input assertions:

- the quoted definition's claim that snapshot, policy version, and resolver
  are always the complete coordinates;
- the statement that standing has exactly three coordinates; and
- the statement that those coordinates are sufficient "and nothing else."

They are replaced by the profile-relative `(G, I_G)` law and complete-input
invariants above. Existing references to snapshot, policy, and resolver remain
valid for profiles whose complete inputs have that shape; they cease to be a
universal ceiling.

ADR-0003 remains governing in every other respect. Standing remains:

- a conclusion, not truth;
- distinct from confidence, probability, evidence, and authority;
- derived and never written into historical authority;
- explicitly profile-relative;
- free of ambient latest selection;
- separate from currentness; and
- unavailable by human or agent decree.

ADR-0003's statement that standing is computed by replay remains a historical
source boundary, not a claim that verified history is the only possible
semantic input. Closure-dependent profiles derive standing from replayed
history **and** their other explicit immutable inputs.

Until this ADR is explicitly owner-ratified, ADR-0003 remains Accepted without
semantic amendment. A merge, green check, or presence of this file does not by
itself supply ratification.

## Hostile scenarios

| Scenario | Required outcome |
| --- | --- |
| C1 — same `H` and profile, different closure `M` | Different `I_G` and producing contexts; equal context may not be claimed. |
| C2 — detached v3/v4 result omits `M` | It is not coordinate-complete and may not claim complete reproducibility or enter a coordinate-complete consumer. |
| C3 — two conformant implementations | Every completed evaluation of the same `G` and `I_G` produces the same `D`; implementation identity does not split epistemic identity. |
| C4 — implementation drifts under unchanged `G` | Non-conformance. A behavior change is not legitimate semantics without a new `G`. |
| C5 — future policy consumes authority `A` | `A` is an explicit member of that profile's `I_G`; no constitutional rewrite is needed. |
| C6 — future query depends on `Q` | `Q` is an explicit member of the query profile's `I_G`; this ADR does not authorize the query implementation. |
| C7 — resolver silently loads "current closure" | Forbidden ambient semantics. The result is not a conforming Magpie derivation. |
| C8 — coordinate names `current-origin-review-key` | Rejected unless the value itself is an immutable identity with one fixed interpretation; mutable alias resolution is forbidden. |
| C9 — v4 consumes v3 | Internally re-derived v3 needs complete context binding. A detached/caller-supplied v3 additionally requires re-derivation and exact-match or one verified binding of context plus `D`; `(context_id, arbitrary_D)` is rejected. |
| C10 — conformers format explanatory traces differently | Semantic standing identity is unchanged when formatting is not part of `G`. A separately governed exact audit representation may have its own conformance requirement. |
| C11 — checkpoint-matched history replaces today's prefix type | The law still holds with a new profile-specific historical input; the coordinate does not imply freshness or latest history unless separately decided. |
| C12 — same standing, different currentness derivation | Standing remains identical under its own context while currentness differs under a separate profile and context. The facets do not collapse. |
| C13 — material is unavailable | Profile-defined finite supplied-material/absence input is explicit in `I_G` and may produce a typed `D`. I/O, storage, resource, process, or host failure to complete evaluation produces no `D` and cannot be silently reclassified. |
| C14 — equal result content under different contexts | The semantic value may be equal, but the producing contexts—and any future context-bound result identities—remain distinct. |

## Alternatives considered

### A. Keep ADR-0003 and broaden "snapshot"

Rejected. Historical input, external supplied material, and authority input
have different ownership and trust semantics. Hiding `M` or future `A` inside
"snapshot" would make availability or authority look historical and would
obscure which bytes must be supplied.

### B. Freeze one universal `H + M + A + P + R` tuple

Rejected. Optional members become an extensible metadata bag, current names
become permanent architecture, and a future genuine input still forces a
constitutional edit. The tuple is useful as a profile-specific audit notation,
not as the universal law.

### C. Let each policy define coordinates with no universal invariant

Rejected. Per-profile types are necessary, but without one completeness and
no-ambient rule a future policy could omit a dependency or resolve it through a
mutable default while still calling the result deterministic.

### D. Typed profile-specific complete inputs under `D = Derive_G(I_G)`

Selected. It gives each profile a closed exact input type while preserving one
small universal safety rule. Future `A`, `Q`, or another genuine input fits by
changing the profile and its typed input, not by widening a global bag.

### E. Retain only one opaque digest of all inputs

Rejected as the sole doctrine. A well-defined typed context identity may use a
digest, but an opaque digest does not prove that the producer included every
required input, disclose coordinate roles, or prevent a mutable interpretation
of what was hashed.

### F. Keep policy and executable resolver identity as independent semantics

Rejected. It would make conforming implementations of one normative procedure
epistemically different and make build churn part of standing. Normative policy
and resolver rules belong in `G`; executable identity is assurance provenance.
Existing semantic policy/resolver identifiers may remain structured components
of `G` where their contracts require both.

### G. Restrict the law to standing only

Rejected as too narrow. Existing provenance-response and query contracts
already require detached derived answers to identify their derivation. The
scope here remains conservative by governing only results whose claimed
derivation crosses its private constructing context, not every internal value
or cache row.

## Consequences

Positive consequences:

- ADR-0003 no longer hides closure-dependent standing inputs;
- future authority and query inputs fit without a universal optional tuple;
- independent conformers can share semantic identity;
- implementation drift remains a defect rather than silent policy evolution;
- internally re-derived nested results can be compact without allowing a
  detached result to substitute arbitrary content beside a context identity;
- currentness and standing remain separate; and
- exact history remains distinguishable from freshness or checkpoint status.

Costs and constraints:

- each future profile must define a closed complete input type or equivalent
  semantic contract;
- detached result surfaces need an explicit carriage design before they can
  claim complete producing context;
- profile and context identities need immutable, reviewable interpretation;
  and
- current coordinate-poor compatibility outputs remain quarantined rather than
  being upgraded by prose.

## Result-identity deferral

This ADR establishes only the conceptual distinction:

```text
producing-context identity
    commits to G + complete I_G

result identity
    commits to producing context + semantic D
```

A producing-context identity therefore cannot authenticate a separately
supplied `D`. Such a value must be re-derived and exact-matched or verified
through one binding of context and outcome. This requirement does not choose
the future binding representation.

The later C3 tranche may decide whether these identities need runtime types,
how they are encoded, whether a context is flattened or nested, which semantic
result fields are committed, and how explanatory representation is versioned.
No `DerivationContextId`, `ResultIdentity`, envelope, hash profile, constructor,
or public API is authorized here.

## Non-goals

This documentation-only proposal does not:

- change Rust runtime, tests, fixtures, schemas, policy IDs, or behavior;
- change `docs/FORMAT.md`, canonical event bytes, Cargo metadata, CI, or release
  material;
- implement coordinate-bearing result envelopes or repair A-003, A-019,
  A-023, or RQ-011;
- define checkpoint matching, remembered heads, rollback protection,
  witnesses, freshness, or global history completeness;
- implement or redesign ADR-0007, authority objects, candidate designation,
  delegation, or authority-bound standing;
- reinterpret, upgrade, or authorize current standing-v3 compatibility
  behavior;
- implement currentness, queries, a librarian, a portable verifier, or
  `EpistemicGate`;
- alter audit dispositions, readiness classifications, or NEW-ADR-01; or
- announce a release.

The exact recommended next constitutional tranche remains:

```text
VerifiedPrefix versus checkpoint-matched-history semantics
```

## Ratification and implementation boundary

This ADR is Proposed. Explicit repository-owner ratification is required before
it becomes Accepted or satisfies ADR-0007's Outcome B doctrine gate.

Owner ratification would settle the constitutional law only. It would not prove
that current result types carry complete producing coordinates. Runtime
envelopes, type quarantine, exact encodings, mismatch failures, hostile tests,
compatibility review, and living audit reconciliation remain later work.

The public pre-alpha convergence baseline remains a living administrative
record, not doctrine. This PR does not edit it or close its decision record.

## Reviewer checklist

- Confirm `I_G` is closed and profile-specific, not an optional universal bag.
- Confirm `M` is not hidden inside historical "snapshot" terminology.
- Confirm every semantic dependency is explicit or transitively committed.
- Confirm mutable aliases and ambient lookup cannot qualify as coordinates.
- Confirm implementation identity is assurance provenance for conformers.
- Confirm drift under one `G` is non-conformance.
- Confirm internally re-derived inheritance binds complete context, while a
  detached inherited result also binds and verifies its exact `D`.
- Confirm `(context_id, arbitrary_D)` cannot qualify as inherited input.
- Confirm operational inability to complete evaluation produces no `D` and
  does not force environmental state into `I_G`.
- Confirm result content and producing context remain distinct.
- Confirm currentness remains separate from standing.
- Confirm history identity makes no freshness, latest, checkpoint, rollback, or
  non-equivocation claim.
- Confirm future `A` and `Q` fit without changing the universal law.
- Confirm current v3/v4 output is not upgraded or called coordinate-complete.
- Confirm ADR-0007 remains unimplemented and A-024 remains unchanged.
- Confirm no runtime, FORMAT, fixture, CI, audit-disposition, or release path
  changes.
- Confirm Proposed status does not imply owner ratification.

## References

- ADR-0003, canonical definition of standing.
- ADR-0006, claim currency and currentness semantics.
- ADR-0007, authority-bound origin corroboration and its reconciliation gate.
- `docs/design/epistemic-resolution-boundary-contract.md`.
- `docs/design/provenance-response-contract.md`.
- `docs/design/verified-librarian-query-contract.md`.
- `docs/design/resolution-content-closure-v0.md`.
- `docs/design/standing-policy-v3-external-report-corroboration-v0.md`.
- `docs/design/standing-policy-v4-claim-inline-direct-refutation-v0.md`.
- `docs/design/portable-verifier-input-language-contract.md`.
- `docs/design/a021-ed25519-verification-semantics-contract.md`.
- `docs/design/verified-replay-projection-boundary-contract.md`.
- `docs/audits/public-pre-alpha-convergence-baseline-2026-08.md`.
- `docs/audits/audit-disposition-2026-08.md`.
- `crates/magpie-claims/src/standing.rs` and `standing_v1.rs` through
  `standing_v4.rs`.
- `crates/magpie-claims/src/replay_snapshot.rs`.
- `crates/magpie-claims/src/origin_admission_replay.rs`.
- `crates/magpie-claims/src/resolution_content_closure.rs`.
