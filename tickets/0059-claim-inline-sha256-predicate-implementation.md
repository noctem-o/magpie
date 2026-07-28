# Ticket 0059: Claim-inline SHA-256 predicate implementation

## 1. Exact base and authority

```text
base:
c4a824fe1e781a39eb8eacaa8f3901544956b90f

branch:
agent/claim-inline-sha256-predicate-runtime

suggested commit / PR title:
claims: implement claim-inline SHA-256 predicate v0

contract authority:
Ticket 0057
docs/design/claim-inline-subject-binding-v0.md
Ticket 0058
docs/design/claim-inline-sha256-predicate-v0.md
merged PR #73
```

This ticket implements the ratified neutral predicate literally. It does not
reopen either contract. The human retains sole publication and merge
authority; no commit, push, or GitHub mutation belongs to this implementation
run.

## 2. Exact changed-path allowlist

Only:

```text
crates/magpie-claims/src/claim_inline_sha256_predicate.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/tests/claim_inline_sha256_predicate.rs
tickets/0059-claim-inline-sha256-predicate-implementation.md
README.md
docs/design/claim-inline-subject-binding-v0.md
docs/design/claim-inline-sha256-predicate-v0.md
docs/design/standing-view-evidence-ceilings.md
docs/design/standing-aggregation-independence-groups.md
```

No Cargo manifest, lockfile, L0, format, historical ticket, fixture, policy,
standing, verifier, tour, release, CI, loader, CAS, filesystem, network, or
Deadbolt path changes.

## 3. Implemented proposition and authority path

```text
one &StandingReplaySnapshot
+ one requested claim ID
-> re-fetch StandingClaim and TypedClaimNode from that snapshot
-> resolve both operands from that exact claim
-> SHA-256(exact decoded claim-owned inline subject bytes)
   == exact claim-owned expected_sha256
-> DigestEqual | DigestUnequal | ResolutionFailed
```

The evaluator never reads typed evidence or justification edges. It accepts no
claim object, descriptor, subject, digest, receipt, outcome, serialized audit
value, closure, callback, store, evidence ID, edge ID, policy, or alternate
authority table. It performs no mutation or ambient lookup.

## 4. Public API and opaque representation

The sole evaluator is:

```rust
impl StandingReplaySnapshot {
    pub fn evaluate_sha256_claim_inline_bytes_equals_v0(
        &self,
        claim_id: &str,
    ) -> Sha256ClaimInlineBytesPredicateOutcomeV0;
}
```

The exported surface is limited to:

```text
ClaimInlineSubjectResolutionFailureV0
Sha256ClaimInlineBytesPredicateOutcomeKindV0
Sha256ClaimInlineBytesPredicateOutcomeV0
Sha256ClaimInlineBytesPredicateReceiptV0
MAX_PREDICATE_ID_BYTES_V0
MAX_INLINE_SUBJECT_BYTES_V0
```

The outcome is a public struct containing a private enum. Terminal private
states contain one receipt; the failure state contains the exact requested
query ID and one failure reason. A successful outcome stores its claim ID only
inside the receipt. The outcome exposes only `kind()`,
`requested_claim_id()`, `receipt()`, `failure()`, and the separately required
one-way `canonical_bytes()`.

The receipt has private fields, private construction, `Serialize`, and
read-only getters for exactly:

```text
predicate_schema
predicate_id
claim_id
scope_ref
canonical_statement
claim_content_hash
expected_sha256
computed_sha256
```

Neither authority-bearing type implements `Deserialize`, `Default`, `From`,
`TryFrom`, public mutation, or public construction. Compile-fail documentation
pins construction, reclassification, replacement, deserialization,
defaulting, conversion, mixed-snapshot input, and audit reinjection failures.

## 5. Parser and resource-law implementation

The module uses no generic descriptor `String` deserializer. A purpose-built
borrowed parser first validates the complete JSON document with a
constant-state multi-pass scanner:

- exact JSON strings, escapes, controls, Unicode escapes and surrogate pairs;
- exact numbers, literals, arrays, objects, separators and JSON whitespace;
- one complete value, opener/closer type agreement and no trailing content;
- no recursion, retained container stack, depth cap, or global metadata cap.

Known keys are classified by streaming decoded-scalar comparison. Every key is
compared with prior keys through pairwise rescans of borrowed raw tokens, so
post-unescape duplicates—including arbitrary unknown keys—are exact without
owned key strings, hashes, collision risk, or an unbounded retained key list.
Fixed field slots collect structural facts; semantic validation then applies
the frozen order independently of JSON source order.

`predicate_id` remains only two offsets into upstream-owned metadata until its
bound succeeds. The first pass logically unescapes and counts decoded UTF-8
bytes using scalar decoder/count state only. It copies no identifier byte,
prefix, suffix, partial value, complete value, or fixed-buffer fragment.
Decoded byte 65 returns `PredicateIdTooLong`. Only the second pass allocates
and materializes an accepted identifier, followed by `[a-z0-9_]+` validation,
compiled-identity equality, and later statement construction.

`expected_sha256` and `subject_hex` are likewise streaming-validated before
materialization. The subject check applies 8192 to the logically decoded
descriptor characters, then validates even lowercase hex. The decoded subject
`Vec` is allocated only after exact statement binding and content-hash
rebinding. The separate 4096-byte guard remains defense in depth.

## 6. Exact failure order

The implementation applies all 33 reasons in declaration and canonical
encoding order:

```text
1-2   same-snapshot legacy and typed claim lookup
3     whole-document malformed/trailing/non-object
4-9   domain and outer-envelope structure
10-13 nested descriptor structure
14-16 schema
17-22 predicate_id
23-25 expected_sha256
26-29 subject_hex
30    three-way statement binding
31-32 content-hash rebinding
33    decoded subject defense guard
then  terminal digest comparison
```

Whole-document syntax is known before structural failures. Cumulative fixed
flags and semantic staging ensure map/source/encounter order never selects the
failure. In particular, expected-digest failure precedes subject failure and
predicate length precedes invalid-character classification.

## 7. Canonical serialization

`Sha256ClaimInlineBytesPredicateOutcomeV0::Serialize` borrows a private
fixed-field adjacent-tag wire view and delegates directly to
`serde_json::to_vec`. The receipt itself derives `Serialize` in its exact field
declaration order.

```text
profile:
magpie-claim-inline-sha256-predicate-outcome-json-v0

outer order:
outcome, details

success details:
receipt

failure details:
claim_id, reason
```

There is no `serde_json::Value`, map, JCS/RFC 8785, generic canonical JSON,
L0 encoding, pretty printing, omission, null, BOM, or newline path.

The three ratified vectors reproduce exactly:

| Outcome | Bytes | SHA-256 |
| --- | ---: | --- |
| `DigestEqual` | 639 | `d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a` |
| `DigestUnequal` | 641 | `cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291` |
| `ResolutionFailed(MissingClaim)`, `claim-missing` | 95 | `b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d` |

Tests compare literal bytes, lengths, endpoints, BOM/newline absence, hashes,
clone/`Serialize` identity, and the exact escaping law.

## 8. Tests and compatibility

Focused source tests cover all 33 failure encodings, constant-state complete
JSON validation, deep nesting without a hidden cap, escaped/literal Unicode,
surrogate pairs, post-unescape duplicate keys, exact failure precedence,
decoded predicate-ID boundaries 63/64/65, escaped ASCII and multibyte
boundaries, invalid-after-length ordering, subject decoded-character bounds,
and the unreachable decoded-size guard.

The public integration suite covers equal/unequal `abc`, empty subjects,
the 4096-byte boundary, repeated and replayed determinism, exact receipts,
same-snapshot lookup/binding/content-hash failures, every reachable parser
failure, hostile spellings and double faults, exact vectors and escaping,
unbounded exact query attribution, cross-family refusal, historical v0 trace
bytes, snapshot and standing v0/v1/v2/v3 inertia, policy-matrix inertia, and
the Ticket 0056 unrelated-byte-evidence plus `contradicts` substitution
attack.

Historical `sha256_bytes_equals_v0` remains evidence-local. The new family is
unknown to the old parser and the old family is unknown to the new parser.
No support/refutation ceiling, standing policy, deterministic verifier trace,
tour, L0, chain fixture, or release surface changes.

## 9. Review findings and disposition

The read-only parser/resource-law plan audit initially returned `FAIL` on
three underspecified points: arbitrary key allocation, recursive/depth-capped
JSON traversal, and applying the subject bound to raw rather than logically
decoded characters. The primary agent independently verified all three
against Tickets 0057/0058, revised the design to fixed-state pairwise rescans,
constant-state complete grammar validation, and decoded-character subject
preflight, then received a genuine plan `PASS`.

The read-only API/serialization plan audit returned `PASS` and independently
recomputed all three canonical vectors. It emphasized that both receipt and
outcome must remain `Serialize`-only; the implementation does so.

Final-diff hostile review and closure results are recorded after the complete
validation ladder.

## 10. Validation

Observed on the complete pre-review candidate:

| Command | Observed result |
| --- | --- |
| `cargo test -p magpie-claims --lib claim_inline_sha256_predicate --locked` | pass; 12/12 focused source tests |
| `cargo test -p magpie-claims --test claim_inline_sha256_predicate --locked` | pass; 19/19 focused integration tests |
| `git diff --check` | pass |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass |
| `cargo test --workspace --locked` | pass; all workspace unit, integration, and documentation targets green |
| `cargo test --doc --locked` | pass; 135 `magpie-claims` plus 3 `magpie-log` documentation tests, zero failures |
| exact binary tour gate | pass; 1917 stdout bytes, SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f` |
| independent `golden-v1.jsonl` verification | pass; all nine records `OK` |
| independent Deadbolt anchor-chain verification | pass; both records `OK` |
| `python tools/check_release_metadata.py` | direct invocation reached Cargo and refused the intentionally dirty `README.md`; unchanged checker passed in a temporary VCS-free mirror with inventories 18/41/8; no `--allow-dirty`; cleanup verified |
| `cargo package -p magpie-log --locked` | direct invocation refused the intentionally dirty `README.md`; exact command passed in a temporary VCS-free mirror, packaged 18 files and verified the crate; no `--allow-dirty`; cleanup verified |
| changed-path and index-flag audit | pass; exactly the nine allowlisted paths, no staged files, hidden index flags, Cargo/L0/fixture/tool drift, or unauthorized files |

The three predicate vectors are pinned in the public integration suite and
passed in both focused and workspace execution:

```text
DigestEqual:
639 bytes
d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a

DigestUnequal:
641 bytes
cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291

ResolutionFailed(MissingClaim), claim-missing:
95 bytes
b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d
```

### Mutation check

One bounded uncommitted mutation moved `subject_hex` validation ahead of
`expected_sha256`. The exact
`expected_digest_precedes_subject_and_source_key_order_is_irrelevant` test
failed as required:

```text
mutant: InvalidSubjectHex
required: InvalidExpectedSha256
```

The source was restored without retaining the mutation. Its pre-mutation and
post-restoration SHA-256 was exactly:

```text
7ac9e11be0ef8eb03b8e6e27ed0ddc67bddb5cd736c30c1409cdb272a3f9d61e
```

Both focused suites passed after restoration. Final read-only diff review is
the remaining closure gate.

## 11. Remaining sequence

The next separately reviewed slice is a routing-only attestation-binding
contract. Then follow its implementation, support/refutation lane contracts,
standing-inert lane/input implementations, a direct-refutation policy
contract, and only then any direct-refutation runtime. Contradiction debt,
invalidation, supersession/currentness, `EpistemicGate`, ordinary writer
surfaces, loader, CAS, filesystem, and network ingestion remain future.
