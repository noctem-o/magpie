# Ticket 0054: Standing policy v3 external-report corroboration implementation

## 1. Exact base and authority

```text
base:
757b76b9cc19a6dbdfbbc6af6940127d239c656e

contract authority:
Ticket 0053
docs/design/standing-policy-v3-external-report-corroboration-v0.md
merged PR #66
```

This ticket implements the already-ratified contract without reopening or
expanding it. The user retains sole publication and merge authority.

## 2. Classification

```text
correctness:
exact Ticket 0053 runtime implementation

authority:
one OriginAdmissionReplayContextV0 and one immutable closure

positive effect:
complete exact ExternalReport support from at least two distinct admitted
origin groups inside one exact OriginComparisonNamespaceV0 lane
-> governed Supported

standing ceiling:
never Settled
```

## 3. Exact changed-path allowlist

Only:

```text
crates/magpie-claims/src/standing_v3.rs
crates/magpie-claims/src/origin_admission_replay.rs
crates/magpie-claims/src/lib.rs
crates/magpie-claims/src/support_contribution_audit.rs
crates/magpie-claims/src/origin_binding_verifier.rs
crates/magpie-claims/tests/standing_v3.rs
fixtures/standing-policy-v3-corroboration-v0/*
docs/design/standing-aggregation-independence-groups.md
README.md
tickets/0054-standing-policy-v3-external-report-corroboration-implementation.md
```

No Cargo, dependency, L0, policy-matrix, standing-v0/v1/v2 production,
support-audit production, existing fixture, CI, release, tool, example, or
ratified-contract path may change.

## 4. Architectural law

```text
OriginAdmissionReplayContextV0
+ exact requested claim
+ immutable ResolutionContentClosureV0
-> internally derived StandingResolutionV2
-> internally derived SupportContributionAuditV0
-> one crate-private pure v3 composer
-> StandingResolutionV3
```

No caller supplies an audit, contribution, group, threshold, policy, lane,
snapshot, callback, serialized output, or alternate authority table.

## 5. Public API

The implementation adds only the Ticket 0053 constants and closed public
vocabulary plus:

```rust
OriginAdmissionReplayContextV0::resolved_standing_v3
OriginAdmissionReplayContextV0::resolved_standing_with_trace_v3
```

Every new public struct has private fields and read-only getters. New types are
`Serialize` only: no `Deserialize`, `Default`, public constructor, or public
mutation. Only `StandingResolutionV3` exposes direct typed JSON
`canonical_bytes()`.

## 6. One-composer stage correspondence

The single composer implements, in order:

1. inherited claim identity;
2. inherited v2 policy identity;
3. support-audit schema;
4. support-audit canonicalization profile;
5. support policy identity;
6. admitted policy identity;
7. origin policy identity;
8. verified-prefix identity;
9. closure identity;
10. completion and inherited-v2 blockers;
11. exact requested-claim selection;
12. whole-set lane-invariant validation in contribution-identity order;
13. whole-set target/scope validation in contribution-identity order;
14. duplicate contribution-identity rejection;
15. exact lane/group construction;
16. application and explicit standing precedence.

Outer failures return one exact first failure, bind the expected current
identities, retain both nested inputs verbatim, and expose no evaluation
material. Inherited claim/policy mismatch clears top-level governed and legacy
standing and sets currentness to `Unknown`; later outer failures preserve valid
inherited values without promotion.

## 7. Completion, blockers, and precedence

Only a `Complete` support audit may select and group contributions.
`AdmittedContributionIncomplete` and `UpstreamCompositionRejected` produce
their closed blockers and no lane, application, or standing effect.

An inherited-v2 resolution failure is a blocker, not an outer failure. It
produces no v3 application and no promotion. Under failure-free inherited v2,
standing precedence is exact: inherited `Settled`, then `Refuted`, then
`Supported`, then successful v3 corroboration to `Supported`, otherwise the
inherited standing unchanged. Legacy raw standing never participates.

## 8. Deterministic ordering

```text
lane: OriginComparisonNamespaceV0::Ord
groups: String::Ord
contributions: ContributionIdentityV0::Ord
ignored contributions: ContributionIdentityV0::Ord
blockers: declaration order
outer failure: frozen first-failure order
same-stage contribution failure: first identity by ContributionIdentityV0::Ord
```

All lane-invariant checks complete before target/scope checks, and all
target/scope checks complete before duplicate detection. Caller/vector order
never selects a result.

## 9. Hostile staging seams

Exactly three `#[cfg(test)] pub(crate)` helpers stage:

- a complete `SupportContributionV0`;
- a complete `SupportContributionAuditV0`;
- an arbitrary `OriginComparisonNamespaceV0`.

They do not exist in production builds, change production bytes, add public
authority, or create a second composition path.

## 10. Fixture authority split

Human-approved on 2026-07-19:

- states reachable from honest signed replay and immutable closure input are
  regenerated through the public context resolver;
- support-audit rejection, inherited-v2 failure, and outer composition
  failures are unreachable-by-construction defenses and are regenerated by
  the same single pure composer through the test-only staging seams.

No public or alternate authority-injection route is added merely to generate
defensive fixtures. Every literal fixture pins bytes, length, SHA-256, JSON
shape, endpoints, BOM absence, and newline absence.

| Fixture | Regeneration authority | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `corroborated-two-groups.json` | public resolver | 6777 | `2a4eb437e8a39574e7abfc1d32d380f2b68487ca6b42513e5af054bb9e286132` |
| `insufficient-one-group.json` | public resolver | 6665 | `5bf0902c0c64718c12c798517675ce23777bca885e2ada7fa05adbb7dfadbe9a` |
| `support-audit-incomplete.json` | public resolver | 2300 | `12ae03f449a153f4665e8180126e5f0f7a559c968581559fb8c5cc654760e6e5` |
| `support-audit-rejected.json` | test-only staging into the single composer | 1668 | `49281b375cc57c09ec4c8345f31c1b241ebc841d70819f08d9729d79f1cad897` |
| `inherited-v2-failure.json` | test-only staging into the single composer | 4788 | `d71f54f2646372f71a5714b55c7492bb15dcd7b5a71483e52578040e684a1931` |
| `outer-failure.json` | test-only staging into the single composer | 1607 | `ad5dce19f5f3a165d2e7acb6736e0032a278fbddcb0caa0a13ec981aa72a3458` |
| `outer-failure-lane-invariant.json` | test-only staging into the single composer | 2614 | `6a10c3559313cef0ed7c16c227759f016c8261d31f86e47d0cc91793ad90527c` |
| `outer-failure-target-scope.json` | test-only staging into the single composer | 2630 | `032ce9ce6faa3971882cc0605ab69df97d2c2145f8f6c63b407d32b7e723dc6e` |

## 11. Test and compatibility coverage

The source unit module has 19 focused tests for closed serialization, canonical
field order, every outer identity failure, whole-set failure-class order,
identity-order selection, duplicate rejection, completion/blocker order,
standing precedence, legacy quarantine, fixture regeneration, and hostile
defensive states. The public integration target has 13 tests for honest replay,
zero/one/two/many-group boundaries, same-origin multiplicity, ignored targets,
case-distinct groups, same textual groups across claim namespaces, empty and
absent requests, malformed/wrong-domain requests, exact fixture regeneration,
and v0/v1/v2/support-audit/closure/policy-table inertia.

The standalone documentation suite executed 123 tests, including all 38 new
v3 compile-fail examples. Full-field literals independently pin all four
private structs. Every new type independently rejects `Deserialize` and
`Default`. Both public resolvers independently reject an audit, one support
contribution, a contribution list, an origin group, a lane value, a lane ID, a
threshold, and a policy ID. A standalone snapshot resolver and a
`StandingView` v3 resolver also remain compile-time impossible.

The tour must remain byte-identical to the captured base output:

```text
base length: 1917
base sha256: 48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f
```

## 12. Validation

Observed on the uncommitted candidate:

| Command | Observed result |
| --- | --- |
| `git diff --check` | pass |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | pass |
| `cargo test --workspace --locked` | pass; all workspace unit, integration, and doc targets green |
| `cargo test --doc --locked` | pass; 120 `magpie-claims` plus 3 `magpie-log`, zero failures |
| `cargo run --locked --example tour -p magpie-claims` | pass; 1917 bytes, frozen SHA-256, byte-identical to exact-base capture |
| independent Python verification of `golden-v1.jsonl` | pass; all records `OK` |
| independent Python verification of `deadbolt-anchor-v1/anchor-log.jsonl` | pass; both records `OK` |
| `python tools/check_release_metadata.py` | direct worktree invocation reached Cargo and refused the intentionally dirty `README.md`; unchanged script passed in a VCS-free mirror of the exact candidate: inventories 18/39/8 and metadata/licenses consistent |
| focused v3 unit module after workspace suite | pass; 19/19 |
| focused public v3 integration target after workspace suite | pass; 13/13 |

The tour result was exactly:

```text
length: 1917
sha256: 48427d488c501c577c4ba9cf8c44bd89e20b84df8e663dee2c7dc4dbf1e25c2f
byte-identical to base: true
```

## 13. Mutation check

One bounded uncommitted mutation replaced distinct-group counting with the sum
of contributions across groups. The exact
`same_origin_multiplicity_is_visible_but_counts_once` integration test then
failed because the mutant returned `Some(Supported)` instead of inherited
`Some(Conjectured)`. Restoring group counting made the test pass. The
pre-mutation and post-restoration SHA-256 of `standing_v3.rs` was identically:

```text
fac97ad2b1f8166e8bdd6ef5925b5392c38225c5e4286df2ce6d00ba211e106e
```

## 14. Honest caveats

Fresh Checker A (`gpt-5.6-sol`, xHigh) returned `PROCEED` after a read-only
hostile contract-to-code preflight. Its stage-order, type-surface, authority,
ordering, and test recommendations were adopted except where higher authority
controlled:

- Checker A allowed an application to remain visible on an inherited-v2
  resolution failure; the ratified task and human instruction require no
  application and no promotion, which this implementation enforces.
- Checker A recommended staging unreachable defensive fixtures. The human
  explicitly approved the split recorded in section 10, so this is not an
  alternate production authority path.

The defensive fixtures are necessarily regenerated in the source unit module,
because integration crates cannot access `#[cfg(test)] pub(crate)` staging.
That is a test-visibility constraint, not production authority. The release
metadata checker also cannot complete directly in a deliberately uncommitted
worktree because Cargo's package inventory command enforces cleanliness; the
exact candidate passed through the unchanged checker in a temporary VCS-free
mirror instead.

The first fresh Checker B returned `FAIL` with two medium test-contract
findings. Both were independently confirmed and remediated without changing
production behavior or fixture bytes:

- false-positive combined compile-fail examples were replaced with full-field
  struct literals and independent per-type/per-input cases for both resolvers;
- explicit hostile assertions were added for complete-empty standing and its
  zero-count blocker, many contributions in one group, the same textual group
  across claim namespaces, the empty requested claim ID, and inherited
  `Supported` preservation under incomplete and rejected audits.

The complete validation ladder and the bounded mutation check were rerun after
that amendment. The first Checker B verdict is stale by construction; closure
requires an entirely fresh Checker B over the amended raw diff.

## 15. Stop conditions and future work

Stop rather than adding a second rule/lane/threshold/composer, caller-provided
authority, `Deserialize`, standing v0/v1/v2 or support-audit production change,
policy-matrix/Cargo/L0 change, ambient lookup, unsafe code, or any result that
settles an external report.

Refutation aggregation, contradiction debt, invalidation, supersession,
currentness policy, `EpistemicGate`, writers, loaders, storage, network,
models, and librarian integration remain future work.

## 16. Publication authority

The implementation remains uncommitted and unpushed. No PR or GitHub metadata
action is authorised. The user retains sole publication and merge authority.

## 17. Suggested reviewer checklist

1. One production composer and one context-owned authority path.
2. Exact outer-failure and blocker order.
3. Identity-mismatch clearing and later-failure preservation.
4. Inherited-v2-failure no-application/no-promotion law.
5. Whole-set deterministic validation before duplicate rejection.
6. Full namespace lane key and separate origin-policy invariants.
7. Requested target and exact replayed scope binding.
8. Distinct group count rather than contribution count.
9. Never-`Settled` standing ceiling and legacy raw quarantine.
10. Test-only staging boundaries and fixture provenance.
11. Private construction and deserialization impossibility.
12. Exact changed-path allowlist and v0/v1/v2/support-audit inertia.
