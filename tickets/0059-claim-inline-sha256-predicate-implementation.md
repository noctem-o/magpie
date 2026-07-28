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
`TryFrom`, `Debug`, public mutation, or public construction. `Debug` is
deliberately absent because derived formatting would expose the opaque
outcome's private variant topology and the receipt's private representation as
an additional semantic inspection channel beyond the ratified accessors and
one-way serialization. The public fieldless kind and failure vocabularies may
retain `Debug`; they are explicitly caller-constructible non-authority
classifications. Compile-fail documentation pins construction,
reclassification, replacement, deserialization, defaulting, conversion,
mixed-snapshot input, audit reinjection, and the absence of `Debug` on both
authority-bearing types.

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

### 5.1 CPU and memory complexity

Constant retained parser state does not imply linear CPU time. Let `n` be the
raw `metadata_json` byte length, `d` the maximum container depth, and `m` the
member count in an outer or descriptor object:

| Component | Worst-case time | Retained parser state |
| --- | --- | --- |
| `validate_json_strings_and_delimiters` | `O(n²)`: each closer can rescan its prefix through `opening_delimiter_at_depth` | `O(1)` |
| `opening_delimiter_at_depth` | `O(stop)` per call | `O(1)` |
| `validate_every_json_container` | `O(n d)`, therefore `O(n²)` when `d = O(n)` | `O(1)` |
| `find_matching_json_close` | linear in the scanned container span per call | `O(1)` |
| one complete `ObjectMemberCursor` traversal | linear in that object's bytes | `O(1)` |
| `key_has_equal_predecessor` | linear in the preceding object prefix plus decoded-key comparison work per key | `O(1)` |
| exact duplicate detection over `m` keys | `O(m n)`; `Θ(m²)` for fixed-size members and at most `O(n²)` overall | `O(1)` |
| deeply nested unknown arrays/objects | `O(n d)`, worst-case `O(n²)` | `O(1)` |
| complete descriptor resolution/evaluation | `O(n²)` in metadata, plus linear bounded accepted-field decoding, statement hashing, subject hashing, and exact output/query-field copies | constant parser state; bounded descriptor/subject/statement allocations plus exact copies of upstream claim ID, scope, and content-hash strings |

The composed rescans add; they do not multiply into a cubic or exponential
path. The decoded predicate ID (64 bytes), expected digest (64 characters),
encoded subject (8192 decoded JSON characters), decoded subject (4096 bytes),
and descriptor-derived statement are contract-bounded. Raw `metadata_json`,
container depth, outer/descriptor key count, the already-replayed input
allocation, and the exact query/receipt routing strings are not globally
bounded upstream by `magpie-core-v1`.

### 5.2 Optimized measurement

A temporary ignored integration test measured the complete public evaluator
with `cargo test --release` on an AMD Ryzen 7 5800X3D, Windows
10.0.26200, Rust 1.94.1. Each row is the median of 11 samples after three
warm-ups; samples batched repeated evaluations and report per-evaluation time.
There was no timing assertion. The harness was deleted after the run and the
integration test returned byte-for-byte to its starting SHA-256
`6c53460652fdcb2e7be832d81a98d457af69018ff9af8011c5f13ea9f9d39ec9`.

| Case | Depth / keys / subject bytes | Input bytes | Median | Growth from prior row |
| --- | ---: | ---: | ---: | ---: |
| nested valid array in unknown outer field | 32 | 364 | 9.078 us | — |
| nested valid array in unknown outer field | 64 | 428 | 24.866 us | 2.74x |
| nested valid array in unknown outer field | 128 | 556 | 81.418 us | 3.27x |
| nested valid array in unknown outer field | 256 | 812 | 294.560 us | 3.62x |
| nested valid array in unknown outer field | 512 | 1324 | 1.121 ms | 3.81x |
| nested valid array in unknown outer field | 1024 | 2348 | 4.329 ms | 3.86x |
| unique literal unknown outer keys | 16 | 560 | 14.290 us | — |
| unique literal unknown outer keys | 32 | 832 | 43.337 us | 3.03x |
| unique literal unknown outer keys | 64 | 1376 | 153.030 us | 3.53x |
| unique literal unknown outer keys | 128 | 2464 | 578.720 us | 3.78x |
| unique literal unknown outer keys | 256 | 4640 | 2.201 ms | 3.80x |
| unique literal unknown outer keys | 512 | 8992 | 8.646 ms | 3.93x |
| unique escaped unknown outer keys | 16 | 1120 | 27.324 us | — |
| unique escaped unknown outer keys | 32 | 1952 | 88.746 us | 3.25x |
| unique escaped unknown outer keys | 64 | 3616 | 316.925 us | 3.57x |
| unique escaped unknown outer keys | 128 | 6944 | 1.165 ms | 3.68x |
| unique escaped unknown outer keys | 256 | 13600 | 4.482 ms | 3.85x |
| unique escaped unknown outer keys | 512 | 26912 | 17.589 ms | 3.92x |
| maximum accepted subject | 4096 | 8474 | 149.500 us | n/a |

No panic, recursive stack growth, excessive retained parser allocation, or
timeout occurred. The depth and key series converge toward the approximately
4x cost expected when doubling a quadratic dimension; the maximum accepted
subject follows the bounded linear field path.

**Disposition: ACCEPTABLE TRADEOFF.** For this explicit, local,
standing-inert evaluator, the observed costs are acceptable and preserve the
frozen 33-reason acceptance and precedence surface. No global metadata or
depth cap, 34th failure, changed failure, truncation, or silent acceptance
change is introduced. The parser is bounded in retained parser state and in
its accepted predicate/subject fields; it is not generally CPU-resource
bounded. Future ingestion/resource governance may impose an outer operational
limit, but this evaluator does not silently invent one. Linearizing exact
post-unescape duplicate detection would require an unbounded retained key
collection or a separately ratified visible limit; changing delimiter
tracking alone would not remove the duplicate-detection quadratic bound.

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

The earlier read-only parser/resource-law plan audit initially returned
`FAIL` on three underspecified points: arbitrary key allocation,
recursive/depth-capped JSON traversal, and applying the subject bound to raw
rather than logically decoded characters. The design was revised to
fixed-state pairwise rescans, constant-state complete grammar validation, and
decoded-character subject preflight, then received a genuine plan `PASS`.

Two genuinely fresh independent read-only final-diff reviews then ran cold on
starting head `1a3cf4897524f77791c129d3b0b142d012dc1635`, with the pinned
contracts, complete parser and tests, exact base diff, and no maker verdict:

- **Checker A — parser / resource / failure order: `VERDICT: PASS`.** It
  attacked complete JSON grammar, delimiter types, Unicode and surrogate
  handling, post-unescape known and arbitrary-key duplicates, double-fault
  precedence, decoded predicate-ID byte 65, pre-bound copying, generic parser
  reachability, construction/allocation staging, version isolation, forbidden
  inputs, attacker-reachable panics, and CPU amplification. It found no
  contract-fidelity defect and independently identified quadratic worst-case
  CPU as the remaining non-blocking resource concern.
- **Checker B — API / authority / serialization: `VERDICT: FAIL`.** All
  construction, reclassification, attribution, same-snapshot, reinjection,
  ordering, vector, escaping, and standing-inertia attacks survived, but
  derived public `Debug` on the outcome and receipt exposed the private
  variant topology and field representation as an additional semantic
  inspection surface beyond the contract's exhaustive named accessors and
  one-way serialization.

The `Debug` finding was accepted. `Debug` was removed from the public outcome
and receipt and, as required for compilation, the private outcome-data enum.
The fieldless kind and failure vocabularies retain diagnostic formatting
because their variants are already public non-authority classifications. Two
compile-fail barriers now prove the authority-bearing types do not implement
`Debug`.

After the complete remediation and validation record was available, two new
fresh independent read-only final-diff reviews ran cold:

- **Checker A — parser / resource / failure order: `VERDICT: PASS`.** It found
  no parser, failure-order, panic, allocation-staging, authority-input, or
  contract-fidelity defect. It independently confirmed that the composed
  rescans are worst-case quadratic rather than cubic or exponential and
  identified the disclosed upstream-unbounded CPU amplification as the
  non-blocking residual risk.
- **Checker B — API / authority / serialization: `VERDICT: PASS`.** It
  confirmed that the authority-bearing outcome and receipt no longer expose
  `Debug`, that the remaining private and fieldless-enum diagnostic derives
  add no authority surface, and that every construction, reclassification,
  reinjection, same-snapshot, attribution, ordering, escaping, vector, and
  standing-inertia attack survived. It independently recomputed all three
  canonical vector lengths and hashes exactly.

The closure record is therefore not the earlier contradictory pair of plan
`PASS` claims nor a concealed failure: the first final-diff round was
`PASS`/`FAIL`, the demonstrated `Debug` defect was remediated, and the complete
post-remediation final-diff round was `PASS`/`PASS`.

## 10. Validation

Observed on the complete post-remediation candidate:

| Command | Observed result |
| --- | --- |
| `cargo test -p magpie-claims --lib claim_inline_sha256_predicate --locked` | pass; 12/12 focused source tests |
| `cargo test -p magpie-claims --test claim_inline_sha256_predicate --locked` | pass; 19/19 focused integration tests |
| `git diff --check` and `git diff --check c4a824fe1e781a39eb8eacaa8f3901544956b90f --` | pass |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass |
| `cargo test --workspace --locked` | pass; all workspace unit, integration, and documentation targets green |
| `cargo test --doc --locked` | pass; 137 `magpie-claims` plus 3 `magpie-log` documentation tests, zero failures, including both new no-`Debug` compile-fail barriers |
| optimized temporary characterization harness | pass; one ignored release target, 19 ordinary integration tests filtered; medians reported in section 5.2; no wall-clock assertion retained; harness removed and original test-file hash restored |
| exact binary tour gate | pass; 1917 stdout bytes, SHA-256 `48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f` |
| independent `golden-v1.jsonl` verification | pass; all nine records `OK` |
| independent Deadbolt anchor-chain verification | pass; both records `OK` |
| `python tools/check_release_metadata.py` | direct invocation checked the 19-file `magpie-log` inventory, then Cargo refused the intentional uncommitted `magpie-claims` source patch; the unchanged exact checker passed in a temporary VCS-free mirror with inventories 18/41/8; no `--allow-dirty`; cleanup verified |
| `cargo package -p magpie-log --locked` | pass directly; packaged 19 files and verified the crate |
| changed-path and index-flag audit | pass; the pinned-base-to-working-tree union is exactly the nine allowlisted paths; only the predicate source and Ticket 0059 are unstaged; no staged or untracked files, hidden index flags, Cargo/L0/fixture/tool drift, or unauthorized files |

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

Both focused suites passed after restoration, and the mutation left no
worktree change.

## 11. Remaining sequence

The next separately reviewed slice is a routing-only attestation-binding
contract. Then follow its implementation, support/refutation lane contracts,
standing-inert lane/input implementations, a direct-refutation policy
contract, and only then any direct-refutation runtime. Contradiction debt,
invalidation, supersession/currentness, `EpistemicGate`, ordinary writer
surfaces, loader, CAS, filesystem, and network ingestion remain future.
