# Ticket 0057: Claim-inline subject binding contract

## 1. Exact base

```text
4d5520a879f021dc82a8ce129d6d2a35dd02e7ed
```

That commit is the merge of PR #71
(`docs: record direct-refutation subject-binding blocker`) into `main`.

## 2. Branch, commits and PR title

Suggested branch:

```text
agent/claim-inline-subject-binding-contract
```

Suggested commit title and draft PR title:

```text
docs: ratify claim-inline subject binding v0
```

`AGENTS.md` is binding. K3 does not commit, push, open or edit a PR, mark
readiness, merge, resolve review threads, or enable auto-merge. The user
retains sole publication and merge authority.

## 3. Documentation-only classification

```text
correctness:
ratifies the claim-invariant byte-subject identity law and one
concrete subject representation; no runtime

authority:
documentation only; no authority path changes

standing:
unchanged

policy:
none ratified (no policy identity introduced)

predicate:
none ratified (versioning boundary defined only)

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
the Ticket 0056 prerequisite is met by construction — no evidence node
can supply subject bytes to the new inline-subject path, while existing
v0 evidence formats — including magpie-verification-witness-v0 and
witness_hex — remain unchanged under their own historical path
```

This ticket implements no runtime capability. It changes no Rust, tests,
fixtures, formats, vectors, public APIs, policy matrices, standing
behavior, writer behavior, Deadbolt behavior, origin admission, or
aggregation.

## 4. Exact changed-path allowlist

Exactly these five paths may change:

```text
README.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
docs/design/claim-inline-subject-binding-v0.md  (new)
tickets/0057-claim-inline-subject-binding-contract.md  (new)
```

No other path is permitted. Specifically prohibited: Rust source, tests,
fixtures, Cargo manifests, the lockfile, CI, release metadata, changelog,
`docs/FORMAT.md`, `AGENTS.md`, golden vectors, the Python verifier,
historical tickets including Ticket 0056, the v4 blocker record beyond
its living-status cross-references, generated files, and GitHub
metadata.

## 5. The subject-binding law

Ratified in full in
[`docs/design/claim-inline-subject-binding-v0.md`](../docs/design/claim-inline-subject-binding-v0.md);
this ticket and that note agree exactly:

```text
1. Byte equality is the identity relation.
2. The claim owns the complete subject descriptor.
3. The canonical statement carries the descriptor. Binding is the
   exact direct three-way comparison StandingClaim.statement ==
   TypedClaimNode.statement == the descriptor-derived canonical
   statement; ClaimAssertedV2.content_hash is separately recomputed as
   the SHA-256 of the exact statement bytes, with collision resistance
   assumed and hash equality never used as identity.
4. One resolution path serves both polarities; no separate positive or
   negative subject suppliers.
5. Resolution failure is never falsity.
6. No routing value is content identity.
7. Evidence supplies no candidate-selectable subject bytes to the
   inline family; existing v0 evidence formats — including
   magpie-verification-witness-v0 and witness_hex — remain unchanged
   under their own historical path.
```

## 6. The chosen mechanism

```text
claim-owned inline immutable subject bytes

outer metadata:  { claim_domain: "ExactMachineCheckable",
                   machine_predicate_inline_bytes: { ... } }
                 — exactly these two outer keys and no others, no
                 duplicate keys at either level, no descriptor fields
                 at the outer level; claim_domain revalidated by the
                 existing strict parser
schema:          magpie-machine-predicate-inline-bytes-v0
fields:          schema, predicate_id, subject_hex, expected_sha256
                 (only, nested; never at the outer level)
subject form:    strict lowercase even-length hex; empty permitted
bound:           4096 decoded bytes (encoded checked before decode)
predicate_id:    closed [a-z0-9_]+, at most
                 MAX_PREDICATE_ID_BYTES_V0 = 64 bytes (checked after
                 missing/wrong-type/empty and before character
                 validation, statement construction, or allocation)
canonical form:  magpie-machine-predicate-inline-bytes-v0:<predicate_id>:<expected_sha256>:<subject_hex>
binding:         StandingClaim.statement == TypedClaimNode.statement
                 == descriptor-derived canonical statement (exact
                 direct comparison); ClaimAssertedV2.content_hash
                 separately recomputed == SHA-256(UTF-8(exact
                 statement)) — collision resistance assumed; hash
                 equality is never identity and never replaces a
                 direct comparison
```

The checked bytes come only from the replayed claim. The Ticket 0056
counterexample class — an arbitrary `contradicts` witness supplying
unrelated bytes — is closed by construction: the new inline family
consumes no evidence-supplied subject bytes, while existing v0
evidence formats — including `magpie-verification-witness-v0` and
`witness_hex` — remain unchanged under their own historical path.

## 7. Versioning boundary

`magpie-machine-predicate-v0`, `sha256_bytes_equals_v0`,
`magpie-verification-witness-v0` and its `witness_hex` field,
`DeterministicVerifierContextTraceV0`, and standing policies v0-v3
remain byte- and behavior-identical. The new
schema is unknown and fail-closed to the v0 strict parser and has its own
strict parser. No L0 payload, tag, canonical encoding, `docs/FORMAT.md`,
golden vector, or Python verifier change is required or permitted.
`<predicate_id>` is a placeholder for a later predicate contract; this
ticket ratifies no predicate identity.

## 8. Replay resolution law

A future resolver must: re-fetch the typed claim from the same verified
replay snapshot; strictly parse the outer metadata envelope — exactly
`claim_domain` and nested `machine_predicate_inline_bytes`, no other
outer keys, no descriptor fields at the outer level, duplicate keys
rejected at both levels — with `claim_domain` revalidated as
`ExactMachineCheckable` through the existing strict claim-domain
parser; strictly parse the nested descriptor with closed failures
(malformed or trailing JSON; missing or wrong-typed descriptor;
unknown nested keys; unknown schema; `predicate_id` missing,
wrong-typed, or empty, then beyond the compiled
`MAX_PREDICATE_ID_BYTES_V0 = 64` bound — enforced before character
validation, statement construction, or allocation — then characters
outside closed `[a-z0-9_]+`; missing/wrong-typed/invalid
`expected_sha256`; invalid or oversized subject hex); derive the
canonical statement and require exact three-way statement equality by
direct byte comparison — `StandingClaim.statement ==
TypedClaimNode.statement == descriptor-derived canonical statement` —
then separately recompute and verify `ClaimAssertedV2.content_hash ==
SHA-256(UTF-8(exact canonical statement))`, with collision resistance
assumed and hash equality never used as identity; decode the subject
from the claim only; and only then evaluate, through one shared path
for both polarities. Every prerequisite failure is a closed audit
failure with no standing effect and no negative meaning. Subject
resolution grants no standing.

## 9. Authority and construction boundary

- No caller-provided claim bytes, metadata, subject, receipt, or
  evaluation result ever enters a resolver.
- Every authority-bearing value is private-construction,
  `Serialize`-only, no `Deserialize`, no `Default`, no public
  constructor, no public mutation; serialized or cloned values are audit
  material only.
- Evidence attestation may carry routing fields only; any alternate
  subject byte source offered to the inline path is rejected by that
  path, while existing v0 evidence formats — including
  `magpie-verification-witness-v0` and `witness_hex` — remain unchanged
  under their own historical path.
- A claim author choosing subject and expected digest at assertion time
  defines the proposition; it is not after-the-fact substitution.

## 10. Standing inertia

No standing policy, standing rule, predicate checker, evaluation
outcome, receipt, resolver, conflict rule, or runtime is ratified or
introduced. `ResolutionContentClosureV0`, artifact-provenance
verification, and origin admission are not consumed. No ambient lookup,
second store read, network, filesystem, CAS, callback, or loader is
introduced.

## 11. Candidate adjudication record

Two independent fresh read-only `gpt-5.6-sol` (xHigh) audits ran with no
shared inputs and no maker preference. K3 independently verified every
material claim against the repository.

| Finding | Source | Decision | Repository evidence | Effect |
| --- | --- | --- | --- | --- |
| Byte equality is the decisive identity relation; the canonical statement carries the complete subject descriptor; binding is the exact direct three-way comparison `StandingClaim.statement == TypedClaimNode.statement == the descriptor-derived canonical statement`, with `ClaimAssertedV2.content_hash` recomputed separately as the SHA-256 of the exact statement bytes — collision resistance assumed, hash equality never identity | A, B | accepted | v0 statement carries only the digest; content_hash = SHA-256(statement); metadata alone is not subject-canonical | §5-§6 law and binding |
| Claim-inline bytes are the smallest sound mechanism, decidable in both directions, no L0/closure/acquisition dependency | A, B | accepted | existing ExactMachineCheckable usage is small inline witnesses; no replay byte index exists; closure validates no digests | chosen mechanism (§6) |
| External acquired-object subjects are sound only with a compound selector, claim-key == receipt-key equality, injective canonical encoding, and one closure-aware internal path | B (A concurring) | accepted as bounded future extension, unratified | acquisition verifier checks canonical bundle, selector fields, witness root, same-replay anchor, artifact digest (artifact_provenance_verifier.rs:431-511); content-addressed subjects make digest equality near-degenerate | §12 of design note |
| Replay-owned event-core subjects are narrow, need a new byte-retaining replay projection, and are near-tautological by content-hash reference | A, B | accepted as bounded future extension, unratified | snapshot retains no event bytes; EventCore::canonical_bytes/hash fix the encoding | design note §Bounded future extensions |
| Closure-key M-relative predicate proves no external semantics | B | rejected for this contract | closure construction performs no digest validation; membership != validity | design note §Candidate architectures |
| Separate schema/predicate identities for new forms; v0 untouched and fail-closed | A, B | accepted | v0 strict parser rejects unknown keys/schemas; FORMAT evolution is additive-only | §7 |
| One resolution path for both polarities; failures never false; no evidence-supplied subject bytes to the inline path | A, B | accepted | the Ticket 0056 failure mode was exactly a second (negative) byte source | §5, §8, §9 |

Sol A's verdict was `NEEDS-NARROWING` (which byte universe the
proposition ranges over); Sol B's was `CHOOSE` the inline + acquired
pair. K3 narrowed the pair to the inline mechanism alone as the smallest
sound mechanism with sufficient evidence, recording the external and
replay-owned forms as bounded unratified extensions rather than
ratifying a second family this contract does not need.

## 12. Bounded future extensions — explicitly unratified

- **External acquired-object subject**: compound
  `(ArtifactProvenanceAnchorSelectorV0, ResolutionArtifactObjectKeyV0)`
  selector, injectively encoded; acquisition receipt's parsed artifact
  identity must equal the claim-owned key; one crate-private
  closure-aware internal path; unavailability is never falsity.
- **Replay-owned event-core subject**: pinned to
  `EventCore::canonical_bytes()` under `magpie-core-v1`; requires a
  private same-replay byte index that does not exist today.
- **Stop conditions**: no direct-refutation contract may proceed
  without injective canonical subject encoding, one internal-only
  authority path, missing/malformed/unavailable never false, no
  alternate subject byte source consumed by the inline-subject path
  (existing v0 evidence formats — including
  `magpie-verification-witness-v0` and `witness_hex` — preserved under
  their own historical path), and brand-new
  schema/predicate identities leaving
  `sha256_bytes_equals_v0` and policies v0-v3 untouched.

## 13. Explicit non-goals

No Rust, tests, fixtures, Cargo, dependency, L0, canonical encoding,
golden vector, Python verifier, runtime, predicate checker, evaluation
outcome, receipt, standing rule, policy v4 or later identity, negative
receipt, conflict or precedence rule, resolver signature, attestation
schema, contradiction debt, invalidation, supersession/currentness,
`EpistemicGate`, ordinary writer, loader, CAS, filesystem or network
ingestion, model integration, librarian, Deadbolt change, or production
trust claim.

## 14. Frozen surfaces

No change to: v0/v1/v2/v3 types, bytes, behavior, or fixtures;
`DeterministicVerifierContextTraceV0`; support and refutation policy
matrices; L0 payloads or canonical encoding; `docs/FORMAT.md`; golden
vectors; the Python verifier; the tour (1917 bytes, SHA-256
`48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f`);
Cargo manifests or the lockfile; CI; release metadata; historical
tickets.

## 15. Future sequence

```text
Ticket 0057: this subject-binding contract
→ predicate contract over inline-subject claims (positive and/or
  negative deterministic evaluation)
→ direct-refutation contract
→ direct-refutation runtime
→ contradiction debt, invalidation, supersession/currentness, ...
```

This ticket ratifies none of those steps.

## 16. Stop conditions

Stop rather than proceeding if the contract appears to require: Rust,
tests, fixtures, Cargo, or dependency changes; a new L0 payload, tag,
canonical byte, or golden vector; reinterpreting `sha256_bytes_equals_v0`;
any alternate subject byte source consumed by the inline-subject path
(existing v0 evidence formats — including `magpie-verification-witness-v0`
and `witness_hex` — must remain unchanged); any
caller-substitutable authority;
any standing rule, policy identity, predicate identity, receipt, or
resolver; ratifying the external or replay-owned subject forms in this
slice; treating resolution failure as falsity; or editing any path
outside the five-path allowlist.

## 17. Validation commands

Run and report actual results only:

```powershell
git diff --check
cargo fmt --all --check
cargo test --workspace --locked
cargo test --doc --locked
cargo run --locked --example tour -p magpie-claims
python tools/check_release_metadata.py
git status --short
git diff --name-only
git diff --stat
```

Because this change is documentation-only, cargo outputs must remain
identical to the exact base. The release-metadata checker may refuse the
intentionally dirty worktree; the unchanged script is then run in a
temporary VCS-free mirror. No `--allow-dirty`. No golden or fixture
regeneration.

## 18. Publication authority and record

At coordinator handoff the contract candidate was uncommitted and
unpushed. The user subsequently exercised sole publication authority:
committed the candidate as `cf437715ff6f2e2be9ec353363bbf17e5e3b0cf0`
on branch `agent/claim-inline-subject-binding-contract`, pushed it, and
opened draft PR #72. Pre-merge review then produced the four
remediations recorded in section 20. The user retains sole publication,
readiness, and merge authority.

## 19. Suggested reviewer checklist

1. The ratification commit `cf43771` changed exactly the five
   authorised paths of section 4, and the PR #72 remediation diff
   changed exactly the three files named in section 20; no runtime,
   test, fixture, Cargo, L0, format, golden, verifier, or CI change.
2. For the new inline family, the subject bytes come only from the
   claim; existing v0 evidence formats — including
   `magpie-verification-witness-v0` and `witness_hex` — remain
   unchanged under their own historical path.
3. The canonical statement carries the complete subject descriptor;
   binding is the exact direct three-way statement comparison
   `StandingClaim.statement == TypedClaimNode.statement == the
   descriptor-derived canonical statement`, with
   `ClaimAssertedV2.content_hash` recomputed separately as the SHA-256
   of the exact statement bytes — collision resistance assumed, hash
   equality never identity.
4. Subject resolution re-fetches the typed claim from the same
   verified replay snapshot, parses the outer envelope before the
   nested descriptor in that order, and decodes the subject from the
   claim alone.
5. The outer envelope carries exactly `claim_domain` and nested
   `machine_predicate_inline_bytes` and no other outer keys, duplicate
   keys are rejected at both levels, no descriptor fields appear at the
   outer level, and `claim_domain` is revalidated by the existing
   strict claim-domain parser.
6. `predicate_id` is closed `[a-z0-9_]+`, bounded by the compiled
   `MAX_PREDICATE_ID_BYTES_V0 = 64` constant, checked after
   missing/wrong-type/empty and before character validation, canonical
   statement construction, or allocation.
7. `sha256_bytes_equals_v0`, its parser and trace, policies v0-v3, L0,
   and FORMAT are untouched; the new schema is fail-closed to v0.
8. Missing, malformed, oversized, or unparseable subject material is
   never falsity.
9. No predicate, policy, standing rule, refutation, resolver, receipt,
   or runtime is ratified or claimed.
10. Private-construction and no-caller-receipt requirements hold for
    every authority-bearing value.
11. The external and replay-owned extensions are recorded as unratified,
    with prerequisites and stop conditions.
12. The Ticket 0056 counterexample is answered by construction.
13. The ticket and the design note agree exactly.
14. Every reported validation actually ran.

## 20. Review remediations (PR #72)

Pre-merge review of the published candidate produced four required
remediations, all applied:

1. **Outer metadata shape.** The descriptor is nested as
   `machine_predicate_inline_bytes` beside `claim_domain` in the
   established machine-predicate pattern; the outer envelope carries
   exactly those two keys and no others, duplicate keys are rejected at
   both levels, `claim_domain` is revalidated by the existing strict
   parser, and no descriptor field may appear at the outer level. The
   earlier draft had framed the descriptor as the whole metadata
   object.
2. **Hash claims.** The earlier draft claimed two different descriptors
   could never share one statement hash. SHA-256 is not injective;
   binding is the exact direct three-way comparison
   `StandingClaim.statement == TypedClaimNode.statement == the
   descriptor-derived canonical statement`, with
   `ClaimAssertedV2.content_hash` recomputed separately as the SHA-256
   of the exact statement bytes. Collision resistance is assumed, and
   hash equality is never identity and never replaces descriptor or
   statement comparison.
3. **Evidence-byte scope.** The earlier draft's rule read as a global
   ban on byte-bearing evidence. The ratified rule is narrow: existing
   v0 evidence formats — including `magpie-verification-witness-v0`
   and `witness_hex` — are unchanged under their own historical path,
   and only an alternate subject byte source offered to the new inline
   path is rejected.
4. **`predicate_id` bound.** A compiled `MAX_PREDICATE_ID_BYTES_V0 = 64`
   bound over the closed `[a-z0-9_]+` alphabet is now frozen, checked
   after the missing/wrong-type/empty checks and before character
   validation, canonical statement construction, or allocation; the
   rationale (current identifiers are about 21-35 characters) is
   recorded in the design note.

These remediations were verified against the repository and are
reflected identically in the design note, this ticket, and README
entry 27.
