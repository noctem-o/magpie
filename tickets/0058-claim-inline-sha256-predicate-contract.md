# Ticket 0058: Claim-inline SHA-256 predicate contract

## 1. Exact base

```text
67a87e01575a9e9930ba3dc1ec52b95f963b6136
```

That commit is the merge of PR #72
(`docs: ratify claim-inline subject binding v0`) into `main`.

## 2. Branch, commits and PR title

Suggested branch:

```text
agent/claim-inline-sha256-predicate-contract
```

Suggested commit title and draft PR title:

```text
docs: ratify claim-inline SHA-256 predicate v0
```

`AGENTS.md` is binding. K3 does not commit, push, open or edit a PR, mark
readiness, merge, resolve review threads, or enable auto-merge. The user
retains sole publication and merge authority.

## 3. Documentation-only classification

```text
correctness:
ratifies one neutral claim-inline SHA-256 predicate contract over the
Ticket 0057 subject-binding substrate; no runtime

authority:
documentation only; no authority path changes

standing:
unchanged

policy:
none ratified (no policy identity introduced)

predicate:
one new ratified predicate identity,
sha256_claim_inline_bytes_equals_v0, contract only

runtime:
unchanged

tests:
unchanged

fixtures:
unchanged

canonical bytes:
unchanged

L0:
unchanged

dependencies:
unchanged

positive effect:
the first two-sided-decidable, polarity-neutral evaluation substrate
over claim-fixed subject bytes is frozen before any support or
refutation policy exists; the Ticket 0056 substitution class stays
closed by construction
```

This ticket implements no runtime capability. It changes no Rust, tests,
fixtures, formats, vectors, public APIs, policy matrices, standing
behavior, writer behavior, Deadbolt behavior, origin admission, or
aggregation.

## 4. Exact changed-path allowlist

Exactly these five paths may change:

```text
README.md
docs/design/claim-inline-subject-binding-v0.md  (narrow handoff/status only)
docs/design/claim-inline-sha256-predicate-v0.md  (new)
docs/design/standing-view-evidence-ceilings.md  (narrow living-status handoff)
tickets/0058-claim-inline-sha256-predicate-contract.md  (new)
```

No other path is permitted. Specifically prohibited: Rust source, tests,
fixtures, Cargo manifests, the lockfile, CI, release metadata, changelog,
`docs/FORMAT.md`, `AGENTS.md`, golden vectors, the Python verifier,
historical tickets including Tickets 0056 and 0057,
`docs/design/standing-aggregation-independence-groups.md` (see section
15), generated files, and GitHub metadata.

The subject-binding design note may receive only a narrow handoff/status
clarification; its ratified law is not altered.

## 5. The predicate law

Ratified in full in
[`docs/design/claim-inline-sha256-predicate-v0.md`](../docs/design/claim-inline-sha256-predicate-v0.md);
this ticket and that note agree exactly:

```text
exact verified replay
+ one exact claim selected by claim ID
+ strict Ticket 0057 outer-envelope parsing
+ strict inline descriptor parsing
+ compiled predicate-identity equality
+ descriptor-derived canonical statement
+ exact three-way statement equality (direct byte comparison)
+ separate claim content-hash recomputation
+ subject bytes decoded only from the claim, bounded
+ one compiled predicate identity
→ one deterministic standing-inert predicate evaluation
```

The exact proposition is:

```text
SHA-256(hex_decode_strict_lowercase(
    subject_hex from the strictly resolved replayed claim))
==
hex_decode_strict_lowercase(
    expected_sha256 from that same claim's descriptor)
```

The proposition ranges over the decoded subject bytes only — never the
encoded hex characters, never the statement bytes. `expected_sha256` is
proposition data: the claim author fixes it at assertion time, and the
evaluator reads it from the replayed claim. It is never evaluation
input. `ClaimAssertedV2.content_hash` commits to the statement, is
recomputed separately, and never enters the terminal comparison.

The proposition is two-sided decidable after successful resolution:
given one valid decoded byte string and one valid 32-byte expected
digest, exactly one of equality or inequality holds. The outcome proves
only the digest relation. SHA-256 is not injective; digest equality
never becomes unique object identity, and digest inequality never
becomes falsity of anything beyond the named relation.

## 6. Exact predicate identity

```text
sha256_claim_inline_bytes_equals_v0
```

New identity, as Ticket 0056's blocker and Ticket 0057's versioning
boundary require: `sha256_bytes_equals_v0` keeps its evidence-local
positive meaning and is not reinterpreted, aliased, or extended. The
identity is 35 ASCII bytes of closed `[a-z0-9_]+`, within the compiled
`MAX_PREDICATE_ID_BYTES_V0 = 64` bound. It is compiled into the future
evaluator as a constant; a descriptor `predicate_id` that parses cleanly
but is not byte-equal to the constant fails `UnknownPredicateId` before
canonical statement construction, mirroring the existing v0 checker's
compiled-identity comparison.

## 7. Exact neutral outcome vocabulary

```text
Sha256ClaimInlineBytesPredicateOutcomeV0::DigestEqual { receipt }
Sha256ClaimInlineBytesPredicateOutcomeV0::DigestUnequal { receipt }
Sha256ClaimInlineBytesPredicateOutcomeV0::ResolutionFailed { reason }
```

The candidate labels `Satisfied` / `Unsatisfied` were audited and
rejected: they sit one morpheme from the withdrawn Ticket 0056
vocabulary (`PredicateSatisfied` / `PredicateFalse`) and invite the
failure-to-falsity collapse this contract forbids. The existing v0
`Matched` / `DigestMismatch` tokens keep their evidence-local meaning
and are not reused or reinterpreted;
`DeterministicVerifierContextTraceV0::DigestMismatch` remains unchanged,
evidence-local, and non-authoritative.

Required laws:

```text
DigestEqual != Supported
DigestEqual != Settled

DigestUnequal != Refuted

ResolutionFailed != DigestUnequal

malformed != DigestUnequal
unavailable != DigestUnequal
oversized != DigestUnequal
unknown schema != DigestUnequal
unknown predicate != DigestUnequal

edge kind != predicate truth
evidence polarity != predicate truth

predicate outcome != standing
predicate outcome != admission
predicate outcome != contribution
```

One shared resolution and digest-computation path produces both
terminal outcomes; they differ only at the terminal exact digest
comparison. `DigestUnequal` is constructible only after every
resolution prerequisite succeeds and the 32 computed digest bytes
differ from the 32 strictly decoded expected digest bytes. Every
prerequisite failure is `ResolutionFailed` — never a negative result.
The empty subject is ordinary: empty bytes against the empty-byte
digest yield `DigestEqual`; against any other valid digest,
`DigestUnequal`.

## 8. Exact receipt shape

Exactly eight fields, each independently derived from the same replay
and each needed to audit the terminal comparison:

```text
predicate_schema, predicate_id, claim_id, scope_ref,
canonical_statement, claim_content_hash, expected_sha256,
computed_sha256
```

`scope_ref` is routing and later policy-binding context only, never
truth. `claim_content_hash` records the rebound statement commitment,
never the subject digest.

Excluded candidates: verified-prefix identity (the snapshot owns no
such field; including it would fabricate provenance), subject length
(exactly recomputable from the retained canonical statement), raw
subject bytes (already committed by the canonical statement), and any
`Status`, policy identity, evidence or edge ID, contribution,
admission result, or polarity (a predicate receipt is not standing
material).

## 9. Exact same-replay authority path

```text
one successful LogReader verification
→ one private composite replay
→ one StandingReplaySnapshot (private construction)
→ one same-call evaluation by one claim ID
→ one Sha256ClaimInlineBytesPredicateOutcomeV0
```

Conceptual future trusted query:

```rust
StandingReplaySnapshot::evaluate_sha256_claim_inline_bytes_equals_v0(
    &self, claim_id: &str,
) -> Sha256ClaimInlineBytesPredicateOutcomeV0
```

No caller supplies subject bytes, expected digest, predicate result,
parser output, resolved subject, descriptor, metadata, statement,
scope, receipt, outcome, authority table, policy, standing, evidence
ID, edge ID, closure, callback, content provider, snapshot fragment,
or store. The evaluator re-fetches both claim tables internally from
its one snapshot; mixing tables across snapshots is impossible by
construction. The contracted future evaluation hangs on `StandingReplaySnapshot` —
the layer that owns the existing standing-inert v0 verifier query —
because the predicate needs only replayed claim tables; it does not
consume a closure and asserts no verified-prefix provenance. The return is
total: no `Option`; a missing claim is an explicit `ResolutionFailed`
reason. Two evaluations of the same claim on the same snapshot produce
equal outcomes and byte-identical serialized audit output.

## 10. Exact parser and evaluation order

1. Re-fetch the typed claim and the legacy claim from the same verified
   snapshot by the one claim ID.
2. Strictly parse the outer metadata envelope with one duplicate-aware
   whole-envelope parse: exactly `claim_domain` and
   `machine_predicate_inline_bytes`, no other outer keys, no duplicate
   keys at either level, no descriptor fields at the outer level;
   `claim_domain` revalidated by the existing strict claim-domain
   parser and required to equal `ExactMachineCheckable`. Duplicate
   detection is post-unescape; the canonical statement is derived from
   decoded values; raw token bytes are not identity.
3. Strictly parse the nested descriptor in descriptor field order:
   schema; `predicate_id` (missing, wrong-type, empty, then beyond the
   compiled 64-byte bound — before character validation, statement
   construction, or allocation — then outside `[a-z0-9_]+`, then
   byte-unequal to the compiled identity); `subject_hex` (missing,
   wrong-type, then beyond 8192 encoded characters — rejected before
   decoding — then odd-length, uppercase, or non-hex);
   `expected_sha256` (missing, wrong-type, then not exactly 64
   lowercase hexadecimal characters — no prefix, no uppercase, no
   whitespace — validated before canonical statement construction).
4. Derive the canonical statement and require exact three-way
   statement equality by direct byte comparison —
   `StandingClaim.statement == TypedClaimNode.statement ==
   descriptor-derived canonical statement`. No normalization.
5. Require a non-empty `content_hash` and recompute it separately as
   the lowercase SHA-256 of the exact UTF-8 statement bytes. Collision
   resistance is assumed; hash equality is never identity.
6. Decode the subject from the claim only, enforcing the compiled
   `MAX_INLINE_SUBJECT_BYTES_V0 = 4096` decoded bound after decode.
7. Compute SHA-256 over the decoded subject bytes and compare the 32
   computed bytes with the 32 strictly decoded expected digest bytes:
   `DigestEqual(receipt)` or `DigestUnequal(receipt)`.

The evaluator is deterministic, total or fail-closed, commandless,
networkless, pluginless, callback-free, environment-independent, and
free of ambient mutable policy.

## 11. Exact failure order

`ResolutionFailed` carries one variant of
`ClaimInlineSubjectResolutionFailureV0`, closed and evaluated in this
frozen first-failure order:

```text
 1. MissingClaim
 2. MissingTypedClaim
 3. MalformedMetadata
 4. MissingClaimDomain
 5. WrongTypeClaimDomain
 6. DuplicateClaimMetadataKey
 7. UnknownClaimDomain
 8. ClaimDomainMismatch
 9. UnknownClaimMetadataKey
10. MissingInlineSubjectDescriptor
11. WrongTypeInlineSubjectDescriptor
12. DuplicateDescriptorKey
13. UnknownDescriptorKey
14. MissingSchema
15. WrongTypeSchema
16. UnknownSchema
17. MissingPredicateId
18. WrongTypePredicateId
19. EmptyPredicateId
20. PredicateIdTooLong
21. InvalidPredicateId
22. UnknownPredicateId
23. MissingSubjectHex
24. WrongTypeSubjectHex
25. SubjectHexTooLong
26. InvalidSubjectHex
27. MissingExpectedSha256
28. WrongTypeExpectedSha256
29. InvalidExpectedSha256
30. StatementBindingMismatch
31. MissingClaimContentHash
32. ClaimContentHashMismatch
33. DecodedSubjectTooLarge
```

The exact per-variant definitions are ratified in the design note's
failure-vocabulary section. Every variant is a closed audit failure
with no standing effect and no negative meaning. One evaluation returns
at most one reason, the first in this order. `DecodedSubjectTooLarge`
is unreachable when variants 25-26 hold and is retained as a compiled
guard, never as falsity.

## 12. Exact serialization boundary

Every future authority-bearing type in this family — the outcome, the
receipt, any application type — uses private construction, private
fields, read-only getters, `Serialize` only, no `Deserialize`, no
`Default`, no public constructor, no struct-literal construction, and
no public mutation. No resolver, evaluator, or composer ever accepts
such a value back as authority input. Cloning, serializing, or
deserializing any value yields audit material only. The internal
decoded-subject value is private, consumed in the same call, and never
published as a second authority source. Existing v0-v3
standing-resolution types keep their frozen public fields and APIs;
caller constructibility never confers authority.

## 13. Versioning boundary and compatibility invariants

`magpie-machine-predicate-v0`, `sha256_bytes_equals_v0`,
`magpie-verification-witness-v0` and its `witness_hex` field,
`DeterministicVerifierContextTraceV0` (including `Matched` and
`DigestMismatch`), and standing policies v0-v3 remain byte- and
behavior-identical. The boundary is fail-closed in both directions: an
inline-subject claim presented to the v0 predicate parser fails closed
as an unknown outer key; a v0-envelope claim presented to the inline
parser fails `ResolutionFailed`; a hybrid naming the new schema with
the v0 predicate identity fails `UnknownPredicateId`; a hybrid naming
the v0 schema fails `UnknownSchema`. No parser dispatches on
`claim_domain` alone. No L0 payload, tag, canonical encoding,
`docs/FORMAT.md`, golden vector, or Python verifier change is required
or permitted.

## 14. Polarity neutrality and standing inertia

No standing policy, support rule, refutation rule, conflict or
precedence rule, contradiction debt, invalidation, supersession or
currentness rule, evidence-kind assumption, edge-kind interpretation,
or attestation schema is ratified. The evaluation reads no evidence
node, edge, closure, or policy surface, so nothing about `supports` or
`contradicts` can alter the evaluated subject or the truth outcome.
`ResolutionContentClosureV0`, artifact-provenance verification, origin
admission, and the standing resolvers are not consumed. No ambient
lookup, second store read, network, filesystem, CAS, callback, or
loader is introduced. Later positive-support and direct-refutation
policies must invoke this same snapshot-only path internally and may
not accept caller-created outcomes or receipts.

## 15. Attestation decision and adjudication

Sol C compared three options — (1) predicate only, attestation deferred
entirely; (2) predicate plus routing-only attestation in this PR;
(3) predicate now, attestation as the next separately ratified
contract — and recommended option 3. K3 verified the material claims
against the repository and accepted:

- The predicate is a complete claim-local decision; an attestation is
  an independent evidence-format and binding decision (Ticket 0057
  ratified only its boundary, not a schema).
- Two load-bearing questions — the exact outer evidence-metadata
  envelope identity and the accepted evidence kind — lack enough
  evidence to freeze cleanly now.
- Bundling would ratify two independent decisions in one PR, against
  the smallest-sound-surface doctrine this repository applied to
  Tickets 0053 and 0057 themselves.

The ratified sequence:

```text
claim-inline SHA-256 predicate contract (this ticket)
→ routing-only attestation-binding contract
→ support/refutation lane contract(s)
→ direct-refutation runtime
```

No later direct-refutation or support/refutation contract may bundle
the attestation schema with polarity and standing rules in one PR. What
is binding for that next contract is only what is already ratified
elsewhere: Ticket 0057's boundary — routing and binding fields only,
named there as `schema`, `predicate_id`, `subject_claim_id`,
`scope_ref`, with `predicate_id` equal to the already validated claim
predicate identity, and no alternate subject byte source consumed by
the inline path — and, under Ticket 0057's authority boundary, one
same-replay, non-caller-substitutable binding path whose serialized
output is audit material only — and this contract's neutrality laws:
attestation binding is not predicate evaluation, predicate evaluation
is not evidence polarity, and evidence polarity is not standing
contribution. That inherited field boundary is Ticket 0057's ratified
text; changing it would require a separately reviewed amendment of
Ticket 0057, not silent reshaping. What remains genuinely open for the
attestation contract — the schema and outer-envelope identities,
duplicate and unknown-key handling with failure precedence, the exact
binding mechanics within the already-fixed same-replay path, field
requiredness, and the accepted evidence kind — is explicitly
**non-binding candidate material** recorded in the design note's
attestation section. This ticket ratifies none of that candidate
material.

Related boundary decision: `docs/design/standing-aggregation-independence-groups.md`
is a living doctrine note whose two status statements ("a predicate
contract ... remain[s] future") become stale on ratification. The
mission rule permits editing it only with explicit user approval of an
expanded allowlist; the user declined to grant that approval, so the
file stays untouched in this PR and the staleness fix is deferred to
the attestation-contract PR. Its historical Ticket 0056 statements
remain true regardless.

## 16. Candidate adjudication record

Three independent fresh read-only `gpt-5.6-sol` (xHigh) audits ran with
no shared inputs and no maker preference: Sol A (proposition and
semantics), Sol B (authority and substitution), Sol C (surface and
contract boundary). Sol A and Sol C returned `PROCEED WITH CONDITIONS`;
Sol B returned `SOUND WITH CONDITIONS`. K3 independently verified every
material claim against the repository, including recomputing every hash
vector quoted below. Their verdicts are review evidence, not epistemic
authority.

| Finding | Source | Decision | Repository evidence | Effect |
| --- | --- | --- | --- | --- |
| Proposition ranges over decoded claim-owned subject bytes only; encoded-hex and statement-byte readings explicitly excluded | A | accepted | v0 checker hashes decoded bytes and compares digest bytes; 0057 descriptor law | §5 formula; design note proposition |
| `expected_sha256` is proposition data read from the replayed claim, never evaluation input | A, B | accepted | 0057 authority boundary; author-fixed at assertion time | §5, §9 |
| Two-sided decidability holds after successful resolution; outcome proves only the digest relation | A | accepted | 0057 chose inline bytes as decidable in both directions; 0056 lacked claim-fixed subjects | §5 |
| Accept `sha256_claim_inline_bytes_equals_v0` (35 bytes, closed alphabet, new identity) | A | accepted | naming consistent with `sha256_bytes_equals_v0`; within `MAX_PREDICATE_ID_BYTES_V0 = 64`; absent repo-wide before this contract | §6 |
| Replace `Satisfied`/`Unsatisfied` with relation names `DigestEqual`/`DigestUnequal` plus `ResolutionFailed` | A | accepted | withdrawn 0056 vocabulary (`PredicateSatisfied`/`PredicateFalse`) is inactive; `Matched`/`DigestMismatch` stay evidence-local | §7 |
| Closed failure enum in frozen first-failure order covering every 0057 prerequisite; `DigestUnequal` only from the terminal unequal comparison | A, B | accepted | 0057 resolution procedure order; the family's own frozen structural order (the v0 claim-domain parser's encounter-order duplicate precedence is deliberately not mirrored) | §10, §11 |
| Evaluator takes only `&self` + one claim ID on `StandingReplaySnapshot`; total return, no `Option` | A, B | accepted | v0 verifier query lives on the snapshot; snapshot is private-construction; replay maps first-write-wins | §9 |
| Compiled predicate-identity equality before statement construction; mismatch is `UnknownPredicateId` | B | accepted | v0 checker compares parsed ID to its compiled constant and fails `UnknownPredicateId` | §6, §10, §11 |
| Receipt = exactly 8 fields; no verified-prefix identity (snapshot owns none), no subject length (recomputable from the statement), no raw subject copy, no standing/policy/evidence/edge/polarity fields | A, B | accepted | snapshot carries no prefix identity; v0 receipt needs `witness_len` only because witness bytes are absent from it; doctrine: claim ID plus digest is insufficient audit basis | §8 |
| Private-construction, `Serialize`-only outcome/receipt/failure types; serialized values are audit material only | A, B | accepted | v0 receipt and v3 private-construction doctrine | §12 |
| One duplicate-aware whole-envelope parse; post-unescape duplicate detection; canonical statement derived from decoded values | B | accepted | existing strict parser machinery; the lenient claim-domain parser alone is never descriptor authority | §10 |
| Bidirectional fail-closed versioning boundary; hybrids fail `UnknownSchema`/`UnknownPredicateId` | B | accepted | v0 parser rejects unknown outer keys; 0057 versioning boundary | §13 |
| Recreated Ticket 0056 arbitrary-byte attack yields `DigestEqual` for the true claim; evidence and edges are never evaluator inputs | B | accepted | the evaluator has no evidence/edge input; 0056 blocker record | §16 hostile table, design note hostile table |
| Deterministic repeated evaluation: equal outcomes, byte-identical serialized audit output | B | accepted | no candidate collection, clock, nonce, or traversal-dependent field exists | §9, §10 |
| Attestation deferred to a separate next contract (option 3); sequence predicate → attestation → polarity/standing; do-not-bundle law | C | accepted | 0057 ratified the attestation boundary but no schema; outer envelope identity and accepted evidence kind unresolved | §15 |
| Ceilings note needs both stale living-status spots updated; aggregation note left untouched without explicit user approval | C | accepted with narrowing | user declined allowlist expansion; staleness recorded as deferred debt | §4, §15, README and ceilings handoffs |
| 0057 design note's aside that `sha256_bytes_equals_v0` is 21 characters (it is 22) | A | accepted as recorded observation | counted directly; harmless rationale typo in a ratified document | not repeated in 0058 documents; 0057 text untouched per ratified-law rule |

Sol B's report quoted the zero-digest variant statement hash truncated
by two characters; K3 recomputed every hash vector quoted in these
documents independently (reference claim, empty-subject, and
unrelated-material vectors all confirmed). The verified full form of
that variant is
`1d5c96e14bb5116ef1dcf2481fac95a8b134cfc7434dc6054e69f919ca15ff93`,
recorded here for completeness; it is not itself a frozen vector in
either document.

### Hostile contract review (Sol D)

One fresh, read-only `gpt-5.6-sol` (xHigh) hostile review of the
candidate ran with no maker summary and no prior reviewers' reports,
with ten falsification targets. It returned `FAIL` with four findings;
K3 independently verified each against the repository and remediated
all four:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1. The design note's hostile table presented `DecodedSubjectTooLarge` as caller-reachable, contradicting the frozen first-failure contract (any valid ≤8192-char input decodes to ≤4096 bytes) | valid, must-fix | row replaced: 8194 encoded characters yield `SubjectHexTooLong`; variant 33 is documented as an internal defense-in-depth guard, unreachable after failures 25-26 |
| 2. README entry 28 used present tense implying the evaluator exists (`compiled into the evaluator`, `Evaluation hangs on`) while the slice ships no runtime | valid, must-fix | entry 28, ticket §9, and the design note's authority path now use contracted-future tense |
| 3. The attestation deferral was self-contradictory: it denied ratifying a schema, then froze a candidate identity and field set as non-reshapable | valid, must-fix | both documents now distinguish what is already binding (Ticket 0057's routing-only/no-byte-source boundary and this contract's neutrality laws) from explicitly non-binding candidate material the attestation contract owns and may reshape |
| 4. The adjudication record claimed the full zero-digest hash form was used "in both documents"; it appeared in neither | valid, should-fix | sentence corrected; the verified full form is recorded in this section only, and the empty-subject statement content hash (`afc617e9…d67aa`) is now quoted in the design note |

All other falsification targets survived Sol D's attack unchanged:
proposition identity, subject identity, one-path polarity neutrality,
bidirectional version separation, failure-never-falsity (including
double-failure winner pinning), standing inertia, changed-path scope,
constants, and ticket/design agreement. Sol D independently recomputed
every quoted hash vector and confirmed each one.

### Closure review round 1

A fresh, read-only `gpt-5.6-sol` (xHigh) closure review of the
remediated candidate ran cold, verifying the Sol D remediations and
re-attacking all ten falsification targets. It returned `FAIL` with two
findings; K3 independently verified each against the repository and
remediated both:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1. The Sol D remediation overshot: calling the four-field attestation set "freely reshapable" contradicts Ticket 0057's ratified boundary text, which names `schema`, `predicate_id`, `subject_claim_id`, `scope_ref` as the routing/binding carriers | valid, must-fix | both documents now treat the four-field routing boundary (with `predicate_id` equal to the validated claim predicate identity, and no alternate subject byte source) as inherited Ticket 0057 law, changeable only by a separately reviewed 0057 amendment; only the envelope identity, schema identity, failure rules, binding mechanics, requiredness, and evidence kind remain non-binding candidate material |
| 2. The claim that the failure-order prefix "mirrors the existing v0 outer-metadata order" is false for multi-fault metadata: the v0 claim-domain visitor records a duplicate key on encounter, so it can report a duplicate before a missing or wrong-typed `claim_domain` (`crates/magpie-claims/src/standing.rs`) | valid, should-fix | the frozen order is kept (it is deterministic and sound) but the mirror claim is removed from the design note and from this table's evidence column; both now state the prefix is the family's own frozen structural order that deliberately does not mirror the v0 parser |

The closure review verified the four Sol D remediations as landed,
mechanically confirmed the 33 failure variants are identical in name,
number, and order across both documents, and independently recomputed
every quoted hash vector and constant, including the tour's 1917 bytes
and pinned SHA-256.

### Closure review round 2

A second fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-1-remediated candidate. It returned `FAIL` with one
must-fix finding in two parts; K3 verified both parts against Ticket
0057's ratified text and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| 1a. The design note said this PR ratifies "no field set" (contradicting the inherited 0057 routing boundary named two paragraphs later) and classified the exclusion of `witness_hex`, subject bytes, digest overrides, results, receipts, standing contributions, and policy identities as reshapable candidate material — though those exclusions are exactly what 0057's "routing and binding fields only" law already forbids | valid, must-fix | the design note now says "no field set beyond Ticket 0057's already-ratified routing boundary"; the non-routing-field exclusions and the same-replay non-caller-substitutable binding-path doctrine are stated as inherited law; the non-binding list is narrowed to exactly the ticket's open dimensions (schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics, field requiredness, evidence kind); the reviewer checklist is aligned |
| 1b. Both hostile tables said a mismatching attestation `predicate_id` is deferred/unratified — but Ticket 0057 already ratified that the attestation `predicate_id` is the already validated claim predicate identity | valid, must-fix | both hostile tables now state the mismatch is rejected under 0057's inherited boundary; only the exact failure vocabulary, precedence, and binding mechanics are deferred |

The review verified all six prior remediations as landed, confirmed
the failure-enum identity and order across both documents, and
independently recomputed every quoted vector and constant.

### Closure review round 3

A third fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-2-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it against Ticket 0057's ratified text
and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| Both hostile tables attributed the `witness_hex` rejection to "the deferred attestation boundary" — misattributing inherited 0057 law to the deferred material — and neither checklist pinned the inherited/open split | valid, must-fix | both hostile tables now attribute the byte-source rejection to Ticket 0057's inherited routing-only boundary (only failure vocabulary, precedence, and mechanics deferred); both checklists now pin the exact split: inherited four routing fields, predicate-ID equality, no alternate/non-routing fields, same-replay binding path, versus the six open dimensions |

Every other falsification target survived the round unchanged.

### Closure review round 4

A fourth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-3-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it against Ticket 0057's authority
boundary and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| The ticket's §15 inherited list omitted the one same-replay, non-caller-substitutable binding path (present in the design note and both checklists), and both documents used unqualified "binding mechanics" as an open dimension — readable as reopening the same-replay requirement itself and permitting cross-prefix or caller-produced attestation binding | valid, must-fix | §15 now lists the same-replay, non-caller-substitutable binding path (audit-material serialized output) as inherited under Ticket 0057's authority boundary; every open-dimension and hostile-table use of "binding mechanics" in both documents is qualified as the exact mechanics within that already-fixed path |

Every other falsification target survived the round unchanged,
including the eight prior remediations, which the reviewer audited as
landed.

### Closure review round 5

A fifth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-4-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| Both checklists still listed bare `binding mechanics` among the genuinely open dimensions — the round-4 qualification had been applied to §15, the design-note candidate list, and all four hostile rows, but not to the two checklist surfaces, and the round-4 record's claim that every open-dimension use was qualified was therefore premature | valid, must-fix | both checklists now read "binding mechanics within that already-fixed same-replay path"; a full-document sweep confirms every normative open-dimension list and hostile row is now qualified (remaining bare uses are historical narratives inside this review record) |

Every other falsification target survived the round unchanged.

### Closure review round 6

A sixth fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran cold
on the round-5-remediated candidate. It returned `FAIL` with one
must-fix finding; K3 verified it and remediated:

| Finding | Verdict | Remediation |
| --- | --- | --- |
| All four hostile rows said "only the exact failure vocabulary, precedence, and binding mechanics ... are deferred", narrowing the deferred set to three items and contradicting the six open dimensions ratified in §15 and both checklists | valid, must-fix | all four hostile rows now defer exactly the six open dimensions — schema identity, envelope identity, duplicate/unknown-key handling and failure precedence, binding mechanics within the already-fixed same-replay path, field requiredness, and evidence kind — while preserving the inherited rejection sentences for `witness_hex` and mismatching `predicate_id` |

This six-dimension phrasing supersedes the narrower deferred-list
wording quoted in the round-2 and round-4 narratives above, which
describe each round's state at the time. Every other falsification
target survived the round unchanged, including all ten prior
remediations, which the reviewer audited as landed.

### Closure review round 7 (final)

A seventh fresh, read-only `gpt-5.6-sol` (xHigh) closure review ran
cold on the round-6-remediated candidate and returned `PASS` with no
findings. All ten falsification targets survived: proposition and
subject identity; one-path polarity neutrality; bidirectional version
separation and hybrid isolation; failure-never-falsity with the frozen
33-variant order; standing inertia and prefix-provenance honesty; the
six-surface attestation agreement closing the cross-prefix and
caller-produced binding attack; exact five-path scope; every quoted
hash vector and constant (independently recomputed); exact
ticket/design-note agreement; and all eleven prior remediations
verified landed. No further review rounds were run, per the stop rule
of one genuine PASS.

## 17. Hostile cases

The design note's hostile table is normative and agrees exactly with
this summary. Reference claim C: subject `616263` (`abc`), expected
`ba7816bf…15ad`, canonical statement
`magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad:616263`,
content hash `aa1b8f1c339bc8e53c7db9d432e85073cc1495284279ba1b2474b74d26381223`:

1. **Ticket 0056 arbitrary-byte attack recreated** — a well-formed
   `contradicts` evidence node carrying `witness_hex =
   756e72656c61746564206d6174657269616c` (digest `d42273bc…2dc`),
   same claim, same scope: `DigestEqual(receipt)` for C; the evidence
   bytes are never consumed; removing all support evidence changes
   nothing.
2. **`contradicts` edge on C** — no alteration of subject or outcome;
   the edge is not an evaluator input.
3. **`supports` edge on C** — same; no edge kind is interpreted.
4. **Subject `abc`, matching expected digest** — `DigestEqual`.
5. **Subject `abc`, different valid expected digest** —
   `DigestUnequal`, never `Refuted`.
6. **Empty subject, empty-byte digest** — `DigestEqual`.
7. **Empty subject, other digest** — `DigestUnequal`, never `Refuted`.
8. **Duplicate outer or nested key (including post-unescape
   duplicates)** — `ResolutionFailed`.
9. **Unknown outer or nested key** — `ResolutionFailed`.
10. **Wrong schema, including historical v0 schema** —
    `ResolutionFailed(UnknownSchema)`; no cross-family
    reinterpretation in either direction.
11. **Uppercase, prefixed, spaced, or wrong-length expected digest** —
    `ResolutionFailed(InvalidExpectedSha256)` before statement
    construction.
12. **Odd-length, uppercase, non-hex, or oversized subject hex** —
    `ResolutionFailed`, never `DigestUnequal`; the encoded bound is
    enforced before decode or allocation.
13. **Statement mismatch (either direct comparison)** —
    `ResolutionFailed(StatementBindingMismatch)`.
14. **Content-hash mismatch despite statement agreement** —
    `ResolutionFailed(ClaimContentHashMismatch)`.
15. **Matching content hash used instead of direct statement
    comparison** — rejected architecture; hash equality is never
    identity.
16. **Caller-created or deserialized resolved-subject value** — audit
    material only; never resolver authority.
17. **Caller-created, cloned, or deserialized receipt or outcome** —
    unconstructible; never accepted.
18. **Attestation carrying `witness_hex` or any byte source** —
    rejected by the inline path under Ticket 0057's inherited
    routing-only boundary (no alternate subject byte source); deferred
    to the attestation contract are exactly the six open dimensions —
    schema identity, envelope identity, duplicate/unknown-key handling
    and failure precedence, binding mechanics within the already-fixed
    same-replay path, field requiredness, and evidence kind.
    Historical v0 `witness_hex` handling is unchanged.
19. **Attestation `predicate_id` differing from the claim's** —
    rejected under Ticket 0057's inherited boundary: the attestation
    `predicate_id` must equal the already validated claim predicate
    identity. Deferred to the attestation contract are exactly the six
    open dimensions — schema identity, envelope identity,
    duplicate/unknown-key handling and failure precedence, binding
    mechanics within the already-fixed same-replay path, field
    requiredness, and evidence kind. The claim-side analogue —
    descriptor `predicate_id` unequal to the compiled identity — fails
    `UnknownPredicateId`.
20. **Same claim evaluated twice in one replay** — equal outcomes and
    byte-identical serialized audit output.
21. **Resolver mixing claim tables across snapshots** — impossible by
    construction; one `&self` snapshot re-fetches both tables
    internally.

## 18. Frozen surfaces

No change to: v0/v1/v2/v3 types, bytes, behavior, or fixtures;
`DeterministicVerifierContextTraceV0`; the support and refutation
policy matrices; L0 payloads or canonical encoding; `docs/FORMAT.md`;
golden vectors; the Python verifier; the tour (1917 bytes, SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`);
Cargo manifests or the lockfile; CI; release metadata; historical
tickets; ratified contracts, including Ticket 0057's design-note law.

## 19. Explicit non-goals

No Rust, tests, fixtures, Cargo, dependency, L0, canonical encoding,
golden vector, Python verifier, runtime, standing rule, support or
refutation lane, policy v4 or later identity, attestation schema,
evidence-kind assumption, edge-polarity meaning, conflict or
precedence rule, contradiction debt, invalidation,
supersession/currentness, `EpistemicGate`, ordinary writer, loader,
CAS, filesystem or network ingestion, model integration, librarian,
Deadbolt change, or production trust claim. The predicate evaluator and
the subject resolver are not implemented in this slice. Contract and
runtime are never combined.

## 20. Future sequence

```text
Ticket 0057: claim-inline subject binding contract — ratified
Ticket 0058: this predicate contract
→ routing-only attestation-binding contract
→ support/refutation lane contract(s)
→ direct-refutation runtime
→ contradiction debt, invalidation, supersession/currentness, ...
```

This ticket ratifies none of those steps.

## 21. Stop conditions

Stop rather than proceeding if the contract appears to require: Rust,
tests, fixtures, Cargo, or dependency changes; a new L0 payload, tag,
canonical byte, or golden vector; reinterpreting
`sha256_bytes_equals_v0` or the v0 trace; any evidence-, edge-,
closure-, caller-, or audit-supplied subject or expected digest; any
alternate subject byte source consumed by the inline path; any standing
rule, policy identity, attestation schema, support or refutation
semantics, conflict handling, or edge-polarity meaning; separate
positive and negative subject paths; verified-prefix identity claimed
from `StandingReplaySnapshot`; closure, filesystem, CAS, network, or
ambient lookup; or editing any path outside the five-path allowlist.

## 22. Validation commands

Run the following commands and verify explicit pass/fail gates:

```powershell
# Standard checks (must pass)
git diff --check
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --doc --locked

# Tour output invariant (hard gate: must be exactly 1917 bytes with pinned SHA-256)
cargo run --locked --example tour -p magpie-claims > tour_output.txt
if ((Get-Item tour_output.txt).Length -ne 1917) {
    Write-Error "FAIL: Tour output is not exactly 1917 bytes"
    exit 1
}
$tourHash = (Get-FileHash tour_output.txt -Algorithm SHA256).Hash.ToLower()
if ($tourHash -ne "48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f") {
    Write-Error "FAIL: Tour output SHA-256 mismatch (expected 48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f, got $tourHash)"
    exit 1
}
Remove-Item tour_output.txt

# Release metadata (may fail on dirty worktree; if so, run in VCS-free mirror)
python tools/check_release_metadata.py

# Changed-path allowlist enforcement (hard gate: only the five exact paths may change)
$allowlist = @(
    "README.md",
    "docs/design/claim-inline-subject-binding-v0.md",
    "docs/design/claim-inline-sha256-predicate-v0.md",
    "docs/design/standing-view-evidence-ceilings.md",
    "tickets/0058-claim-inline-sha256-predicate-contract.md"
)

# Collect all changed/untracked paths (union of git status and git diff)
$statusPaths = (git status --short | ForEach-Object { $_.Substring(3).Trim() })
$diffPaths = (git diff --name-only)
$allChanged = ($statusPaths + $diffPaths) | Sort-Object -Unique

# Verify each changed path is in the allowlist
foreach ($path in $allChanged) {
    if ($path -notin $allowlist) {
        Write-Error "FAIL: Unauthorized path changed: $path (only five allowlisted paths permitted)"
        exit 1
    }
}

# Report for review
git status --short
git diff --name-only
git diff --stat
```

Because this change is documentation-only, cargo outputs must remain
identical to the exact base. Validation passes only if: all standard
checks succeed, the tour output is exactly 1917 bytes with SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`,
and all changed paths are exactly within the five-path allowlist. The
release-metadata checker may refuse the intentionally dirty worktree;
the unchanged script is then run in a temporary VCS-free mirror. No
`--allow-dirty`. No golden or fixture regeneration.

## 23. Publication authority

At coordinator handoff the contract candidate is uncommitted and
unpushed; K3 performs no commit, push, or GitHub metadata action. The
user retains sole publication, readiness, and merge authority.

## 24. Suggested reviewer checklist

1. Exactly the five authorised paths changed; the subject-binding note
   carries only a narrow handoff/status clarification with its ratified
   law untouched; no runtime, test, fixture, Cargo, L0, format, golden,
   verifier, or CI change.
2. The proposition is exactly SHA-256 of the decoded claim-owned
   subject bytes compared with the claim-owned expected digest;
   encoded-hex and statement-byte readings are explicitly excluded.
3. `expected_sha256` is proposition data from the replayed claim; the
   evaluator accepts only `&self` and one claim ID.
4. The outcome vocabulary is exactly `DigestEqual`, `DigestUnequal`,
   `ResolutionFailed` with the neutrality laws; no v0 or withdrawn
   0056 outcome name is reused.
5. The failure enum is closed and ordered exactly as ratified;
   `DigestUnequal` is reachable only from the terminal unequal
   comparison.
6. The receipt carries exactly the eight ratified fields and no
   prefix identity, subject length, raw subject copy, or
   standing/policy/evidence/edge/polarity field.
7. Private-construction doctrine holds; serialized values are audit
   material only.
8. The versioning boundary is fail-closed in both directions; hybrids
   fail `UnknownSchema` or `UnknownPredicateId`.
9. The Ticket 0056 arbitrary-byte attack is answered by construction
   and reproduced as the central hostile case.
10. The attestation deferral states exactly: inherited from Ticket
    0057 — the four routing/binding fields (`schema`, `predicate_id`,
    `subject_claim_id`, `scope_ref`), `predicate_id` equal to the
    validated claim predicate identity, no alternate subject byte
    source and no non-routing fields, one same-replay
    non-caller-substitutable binding path; genuinely open — schema
    identity, envelope identity, duplicate/unknown-key handling and
    failure precedence, binding mechanics within that already-fixed
    same-replay path, field requiredness, evidence
    kind. No attestation schema or edge-polarity meaning is ratified;
    the sequence and do-not-bundle law are stated; the aggregation note
    is untouched and its staleness is recorded as deferred.
11. The ticket and the design note agree exactly.
12. Every reported validation actually ran.
