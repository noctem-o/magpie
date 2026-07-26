# Standing Policy V4 Deterministic Direct Refutation v0 — Blocked Candidate Record

## Status

```text
candidate:
withdrawn and blocked — not ratified

policy identity:
none active; magpie-claims-standing-v4 is not ratified

rule identity:
none active

runtime:
not implemented
```

This document records why Magpie's first direct-refutation contract
candidate was examined and then blocked. It ratifies no policy, no rule,
no resolver, no receipt, and no conflict handling. Its only normative
content is the blocker itself and the prerequisite a future contract must
meet.

## What the candidate proposed

The candidate (Ticket 0056's initial draft) proposed one closed negative
rule:

```text
one exact typed ExactMachineCheckable claim
+ one exact typed DeterministicVerification evidence node
+ one exact contradicts justification edge
+ exact claim/evidence/edge/scope/predicate/statement/content-hash
  routing bindings
+ one exact same-snapshot negative evaluation observing that the decoded
  witness bytes do not have the expected SHA-256 digest
→ one governed direct-refutation contribution → governed Refuted
```

It also proposed a private-construction negative receipt, a closed
conflict blocker for competing exact verifications, and a new policy
identity. Every one of those proposals is withdrawn.

## The counterexample that blocks it

External review (Codex P1) demonstrated the following attack, verified
independently against the repository:

```text
claim C:
  statement = sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
  (the SHA-256 of the exact bytes "abc")
  domain ExactMachineCheckable, correct statement content hash, scope S

supports evidence E+:
  witness_hex = 616263 ("abc"), subject_claim_id = C,
  predicate_id = sha256_bytes_equals_v0, scope S, valid supports edge
  → Matched (v2 direct support)

contradicts evidence E-:
  witness_hex = 756e72656c61746564206d6174657269616c
  ("unrelated material"), same subject_claim_id = C, same predicate ID,
  same scope, valid contradicts edge
  → freshly computed digest
    d42273bca408c671996c4f6eea5efefd010450655a44d742b0a2ba055b0542dc
  ≠ H → the candidate's "PredicateFalse"
```

Every frozen prerequisite of the candidate passes for `E-`: kind, domain,
edge kind, scope, predicate schema and ID, statement binding, claim
content-hash binding, witness schema, subject claim binding, scope
binding, and size/shape bounds. Yet nothing establishes that
`"unrelated material"` is the same byte subject as `"abc"`. With `E+`
absent, the candidate's own precedence law produced governed `Refuted`
for a true claim from a lone arbitrary witness. With `E+` present, its
conflict blocker suppressed the application — but it suppressed it by
calling two evaluations of *different byte subjects* a "verified
conflict", which misclassifies unrelated bytes as the same proposition.

## Why the candidate is unsound

1. **The ratified positive predicate is evidence-relative.** The v0
   verifier contract defines the checked proposition as "the raw byte
   witness carried by this exact evidence node, under this exact claim,
   edge, and scope binding, has SHA-256 digest H"
   (`docs/design/replayable-deterministic-verifier-admission-v0.md`). The
   subject is selected by each evidence candidate; the claim names only
   the expected digest. Between two candidates the proposition silently
   changes subject.
2. **No record identifies the subject bytes.** The claim schema carries
   `claim_domain` and `machine_predicate.{schema, predicate_id,
   expected_sha256}`. The witness schema carries `schema, predicate_id,
   subject_claim_id, scope_ref, witness_hex`. `subject_claim_id` and
   scope are routing identity, not content identity.
   `ClaimAssertedV2.content_hash` is the SHA-256 of the canonical claim
   statement, not of subject bytes. `EvidenceRegistered.content_hash` is
   never read by the deterministic checker
   (`crates/magpie-claims/src/deterministic_verifier_context.rs`).
3. **The positive direction is guarded; the negative direction is not.**
   A matching witness proves knowledge of an actual preimage of the
   expected digest — adversarial manufacture is constrained by preimage
   resistance. A mismatching witness requires no knowledge of anything:
   almost any byte string mismatches.
4. **The conflict blocker cannot repair proposition identity.** It fires
   only when a `Matched` supports verification coexists, so the lone-
   witness attack is unaffected; and where it does fire it mislabels
   subject substitution as a genuine verified conflict.
5. **The candidate contradicted its own preserved law.** Its law list
   kept `non-matching unrelated material != refutation`, while its frozen
   algorithm turned any routed digest inequality into `PredicateFalse`.

## What remains true and unchanged

- Standing policies v0, v1, v2, and v3, their identifiers, behavior,
  tests, fixtures, and bytes are unchanged.
- The v2 positive rule remains valid for its narrow evidence-local
  meaning: one exact `Matched` verification contributes `Supported`.
- The existing `DeterministicVerifierContextTraceV0::DigestMismatch`
  trace remains a positive-lane failure token and never negative
  authority.
- The three refutation-ceiling cells
  (`DeterministicVerification × Occurrence/Inclusion`,
  `DeterministicVerification × ExactMachineCheckable`,
  `DeadboltAnchor × Occurrence/Inclusion`) remain ceilings only — maximum
  candidate contribution, never achieved refutation.
- Legacy raw standing stays quarantined. A bare `contradicts` edge never
  refutes. Contradiction debt, invalidation, supersession/currentness,
  `EpistemicGate`, ordinary writers, loaders, CAS, ingestion, and
  librarian work remain future.

## Withdrawn identities — none active, none implementable from this text

The following candidate identifiers are **not ratified** and must not be
implemented from this record:

```text
magpie-claims-standing-v4
StandingPolicyRuleV4::Sha256BytesEqualsDirectRefutationV0
PredicateFalse / PredicateSatisfied (negative trace vocabulary)
ConflictingDeterministicVerification
UnsupportedDirectRefutationLane (and the rest of the candidate's
  structural failure vocabulary)
StandingResolutionV4 and the proposed negative receipt shape
OriginAdmissionReplayContextV0::resolved_standing_v4
OriginAdmissionReplayContextV0::resolved_standing_with_trace_v4
```

Reusing any of these names in a future contract requires a complete new
subject-bound design, not a relabel of this candidate.

## Required property of any future direct-refutation contract

```text
claim-invariant byte-subject identity
```

Before predicate inequality may carry negative authority, the claim must
independently commit to one immutable byte subject — exact inline subject
bytes or an exact immutable subject selector — bound into its canonical
representation, and every positive or negative candidate must establish
that its checked bytes are exactly that same claim-fixed subject.
Routing, scope strings, evidence or edge IDs, provenance, actor classes,
signatures, and append-only retention never suffice.

For a claim about an external artifact, verified acquisition/provenance
must additionally bind the claim to one exact object and its immutable
bytes; acquisition alone is insufficient if the claim never names the
object.

A future rule also requires a **new versioned predicate identity**:
`sha256_bytes_equals_v0` keeps its current evidence-local positive
meaning and must not be silently reinterpreted.

Possible shapes, deliberately not chosen here: claim-owned inline subject
bytes committed in the canonical claim representation; a claim-owned
immutable subject object resolved through verified acquisition; a closed
predicate over an already replay-owned canonical byte sequence. Choosing
among them needs its own architecture review and must not be folded into
a remediation PR.

## Next slice

The honest sequence is now:

```text
subject-binding contract (claim-invariant byte-subject identity)
→ direct-refutation contract (new versioned predicate)
→ direct-refutation runtime
→ contradiction debt, invalidation, supersession/currentness, ...
```

This record ratifies none of those steps. The chronological candidate
history, review rounds, and adjudication are preserved in
[`tickets/0056-standing-policy-v4-deterministic-direct-refutation-contract.md`](../../tickets/0056-standing-policy-v4-deterministic-direct-refutation-contract.md).
