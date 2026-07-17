# Ticket 0052: Support-contribution audit v0 implementation

## 1. Exact base

```text
base:
b07f33fcafb66494302b8b57dc83f30bacdd72f7
```

The base is the merge commit of PR #64 (Ticket 0051, the ratified
support-contribution audit v0 contract at `5c90328`).

## 2. Branch, commits and PR title

```text
branch:
agent/support-contribution-audit-v0-runtime

commits:
d845bc8 feat: add support-contribution audit v0 types and serde pins
5115dfd refactor: extract origin-binding grammar seams for support audit
145b55d feat: compose support-contribution audit through stage 15
41d89a2 feat: validate eighteen per-contribution invariants with boundary matrices
0f6821f docs: pin support-contribution authority surface with compile-fail doctests
137a518 test: add support-contribution fixtures and integration coverage
8086180 style: drop unused imports in support-contribution integration tests
docs: record support-contribution audit v0 implementation (this commit)

PR title:
feat: implement support-contribution audit v0

PR state:
draft
```

## 3. Classification

```text
correctness:
exact implementation of the ratified Ticket 0051 contract

authority:
internally derived AdmittedContributionAuditV0 only

positive effect:
typed standing-inert support input only

standing:
unchanged

aggregation:
not implemented

runtime:
implemented
```

## 4. Exact allowlist

Only:

```text
crates/magpie-claims/src/support_contribution_audit.rs   (new)
crates/magpie-claims/src/lib.rs                          (mod + re-exports)
crates/magpie-claims/src/origin_admission_replay.rs      (thin resolver method)
crates/magpie-claims/src/origin_binding_verifier.rs      (four pub(crate) helpers + delegation)
crates/magpie-claims/src/admitted_contribution_audit.rs  (#[cfg(test)] staging constructors only)
crates/magpie-claims/tests/support_contribution_audit.rs (new)
fixtures/support-contribution-audit-v0/*                 (new)
tickets/0052-support-contribution-audit-v0-implementation.md  (new)
README.md                                                (item 23 flip)
docs/design/standing-aggregation-independence-groups.md  (step 12 flip)
```

No `Cargo.toml`/`Cargo.lock` changes. No new dependencies. No changes to any
other file. No changes to Ticket 0050/0051 production behavior. No edits to
the ratified contract documents.

### Fixture-path note

The task brief's allowlist named `crates/magpie-claims/fixtures/...`, but the
same brief instructs "follow 0050's layout", and every existing fixture set
(`admitted-contribution-audit-v0`, `origin-admission-audit-v0`,
`origin-binding-v0`, `artifact-provenance-v0`) lives at repository root under
`fixtures/`. The fixtures were therefore placed at
`fixtures/support-contribution-audit-v0/` to preserve one fixture convention;
no crate manifest changes were required either way.

## 5. The one design decision, as implemented

There is exactly one composition path.

```text
OriginAdmissionReplayContextV0::resolve_support_contribution_audit_v0
-> derive AdmittedContributionAuditV0 internally (same context, same closure)
-> decompose through public getters only into AdmittedAuditCompositionInputV0
-> compose_support_contribution_audit_v0(expected, input)
```

`compose_support_contribution_audit_v0` is the single crate-private pure
composer. Production and every hostile test share it. No test switch, policy
override, runtime policy parameter, mock registry, caller-selected cell, or
alternate authority carrier exists.

`ExpectedIdentitiesV0` carries the current prefix identity, the closure
identity, and the two fixed compiled support-policy cells. The production
resolver fills the cells from the compiled policy
(`support_ceiling(ExternalSource, ExternalReport)` and
`support_context_requirement(ExternalSource, ExternalReport)`); hostile tests
stage drifted cells only through this crate-private view. This is the
smallest seam that makes the contract's reserved policy-drift hostile tests
implementable while the production cells remain compiled constants.

The resolver never panics on composed output. Every composition failure
becomes `UpstreamCompositionRejected { failure }` with zero support
contributions, still binding the expected current prefix and closure
identities. `assert!` is reserved for the 0050-style same-derivation
invariant (admitted key sets proven equal at stage 13).

`#[cfg(test)] pub(crate)` staging constructors live in
`admitted_contribution_audit.rs` and nowhere else. Production bytes and
public API are untouched; the constructors are visible to `#[cfg(test)]`
modules crate-wide. Hostile tests are in-crate unit tests; `tests/`
integration tests exercise the public resolver and fixtures.

## 6. Unreachable-by-construction defenses (honest caveat)

Three frozen checks cannot be exercised through any permitted staging path:

```text
DuplicateCandidateAdmission (stage 11):
trace alignment (stage 10, field 1) binds each candidate edge ID to the
identity's justification_edge_id, so two trace-aligned candidates carrying
one identity necessarily share an edge ID (caught at stage 9), and carrying
one identity under a second edge ID is a trace mismatch (caught at stage
10). The stage-11 check remains as defense-in-depth.

ArtifactAlgorithmMismatch (invariant 10):
ContributionIdentityV0's only constructor pins artifact_algorithm to
sha256, so no drifted identity is constructible outside
origin_binding_verifier.rs.

NamespacePolicyMismatch (invariant 12):
OriginComparisonNamespaceV0's only constructor pins
origin_admission_policy_id, so no drifted namespace is constructible
outside origin_binding_verifier.rs.
```

Staging those two would require a constructor seam inside
`origin_binding_verifier.rs`, which the contract forbids: the four
crate-private grammar helpers and the `validate_replay_reference` delegation
are the only permitted origin-binding module changes. The three checks
remain in the closed vocabularies exactly as frozen. Every other frozen
composition failure and invariant reason is staged and asserted by name.

## 7. Grammar reuse seams

`crates/magpie-claims/src/origin_binding_verifier.rs` gained exactly:

```text
valid_origin_binding_replay_reference_v0 (1..=1024 byte-length grammar)
valid_origin_group_v0                      (-> valid_identifier)
valid_origin_binding_artifact_digest_v0    (-> is_lowercase_hex_64)
valid_origin_binding_selector_v0           (two outer origin-binding kinds)
```

The existing private `validate_replay_reference` now delegates to
`valid_origin_binding_replay_reference_v0`, so the replay-reference grammar
has one implementation source. All pre-existing origin-binding tests pass
unchanged; that pass is the behavior-preservation proof. No origin-binding
behaviour, output, canonical bytes, or public API changed.

## 8. Fixture inventory

Literal canonical fixtures under `fixtures/support-contribution-audit-v0/`.
Tests compare `canonical_bytes()` against these files byte for byte and pin
length and sha256 independently:

```text
one-support-contribution.json
len=1831
sha256=8da9cfd4e15006098909e8221969a965a2d103451f23a791a64878ff16995485

complete-empty.json
len=732
sha256=5be0f96921f5693dfdd40ac2ebb1e2b7f12e5945d2529b44932e6bc71c9e4658

admitted-contribution-incomplete.json
len=1083
sha256=2286c05b2459cbb3bfe1977d7797a8a9150dd076468b454ea4fadeb73a807450

same-origin-distinct-contributions.json
len=2946
sha256=47fbb1f0c0e33b5e338d3d30ab90c8a52fbc91e89d7a0af2774491e2bb80782d

distinct-origin-distinct-contributions.json
len=2889
sha256=8b95d741ee7ac8ca099c094a03fe5811b4581017767a3213420f5a0ecc7a4089
```

The fixtures were generated once from the production resolver through the
same case builders the tests run, then frozen; the tests re-derive each case
and compare bytes, length, and hash.

## 9. Hostile-test coverage

In-crate unit tests in `support_contribution_audit.rs`, one module per stage:

```text
identity_and_policy_cells:
six exact upstream identity mismatches; ceiling and context drift with
both admitted maps empty; cell rejection before completion mapping;
complete-empty valid; origin-incomplete mapping with exact inherited
selector order; incomplete does not hide context drift

edge_id_uniqueness:
admitted+admitted, non-admitted, mixed duplicate edge IDs; lexically
first of several duplicates; duplicate outranks trace mismatch;
duplicate detected before map insertion

admitted_candidate_trace:
all eight fields individually, plus a missing optional field; first
malformed candidate in exact edge-ID order under vector permutation;
trace before top-level duplicate detection

admitted_alignment:
repeated candidate identity rejected before insertion (stage 9/10
routes; stage 11 unreachable, see §6); duplicate top-level admission;
candidate-only and top-level-only set differences; value mismatch,
first in identity order

completion_shape_and_disposition:
Complete forbids incomplete candidates; incomplete forbids terminal
dispositions and admissions; first inconsistent in edge-ID order;
incomplete with only permitted dispositions accepted; empty-vector
incomplete rejected with only PolicyIneligible, with an empty universe,
and by precedence over a disposition violation; non-empty incomplete
with only PolicyIneligible accepted (regression guard)

projection:
one-to-one field-exact projection; same-origin and distinct-origin
multiplicity without collapse or corroboration claims; determinism
under candidate/top-level vector permutation

invariants:
all fifteen constructible reasons (see §6 for the other three);
1/1024/1025-byte and multibyte UTF-8 reference boundaries for all four
references; equality-does-not-replace-validity; reference precedence
over artifact and namespace failures; 63/64/65/uppercase/mixed/non-hex/
0x-prefixed/whitespace digest matrix; 1/128/129-byte group and run-ID
matrices; punctuation-first, whitespace, unsupported-punctuation and
non-ASCII rejections; both exact selector bundle kinds accepted, inner
artifact-provenance kinds rejected; empty selector vector is missing,
not invalid; strict-order and duplicate rejection; individual validity
before vector canonicality; first failing contribution in identity
order
```

Integration tests in `crates/magpie-claims/tests/support_contribution_audit.rs`:

```text
exact public surface and fixed identities
upstream Ticket 0048 pin: origin constructor yields Complete iff the
unavailable-selector vector is empty (both directions, and the admitted
audit inherits the non-empty vector exactly)
deterministic closure order and repeated resolution (byte-identical)
different signed history -> different prefix identity and audit bytes
support-audit execution preserves every existing projection, trace,
audit and policy surface byte for byte (standing v0/v1/v2, deterministic
verifier, origin binding, origin admission, admitted contribution,
support/refutation ceilings, support-context requirements)
serialized output is compact typed audit material only
five literal fixtures match production bytes, length and sha256
```

Compile-fail doctests in the module doc pin: no struct-literal construction
of the public types, no `Deserialize` on any of the five types, no resolver
accepting an admitted audit, an admitted-contribution list, a claim ID, an
origin group, or a policy ID, no serialized-or-cloned output as authority,
and no `StandingView` or standalone `StandingReplaySnapshot` resolver.

## 10. Serde shape pins

Unit variants serialize as `{"outcome":"..."}` with no `details` key;
single-field struct variants include `details`. Every completion variant,
every composition-failure variant (including
`empty_unavailable_binding_selectors` and the nested
`contribution_invariant_mismatch` reason), and every invariant-reason
variant has its exact serialized bytes pinned by test. Declaration order is
canonical byte order.

## 11. Self-mutation check (not committed)

Mutation tried: stage 15's empty-selector shape check moved below the
disposition-agreement loop (disposition checked before shape).

Caught by:
`composition_tests::completion_shape_and_disposition::empty_vector_incomplete_outranks_any_disposition_violation`

Observed failure: the mutated composer returned
`UpstreamCompletionDispositionMismatch { edge_id }` where the contract
requires `EmptyUnavailableBindingSelectors`. The mutation was reverted and
the suite re-verified green.

## 12. Validation

All commands actually ran on the final head; results reported as observed.

```text
git diff --check                                   clean
cargo fmt --all --check                            pass
cargo clippy --workspace --all-targets --locked -- -D warnings
                                                   pass
cargo test --workspace --locked                    pass (every suite ok)
cargo test --doc --locked                          pass (82 doctests)
cargo run --locked --example tour -p magpie-claims pass (byte-identical)

python tools/verify_chain.py
  crates/magpie-log/testdata/golden-v1.jsonl
  ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
                                                   all events OK

python tools/verify_chain.py
  fixtures/deadbolt-anchor-v1/anchor-log.jsonl
  d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
                                                   all events OK

python tools/check_release_metadata.py             consistent
git status --short                                 clean
```

The tour output is byte-identical before and after this change: the new
module is projection-inert and no existing projection reads it. The
integration inertia test additionally proves support-audit execution leaves
standing v0/v1/v2 bytes, deterministic-verifier output, origin-binding
output, origin-admission and admitted-contribution audit bytes, and both
policy matrices unchanged.

## 13. Explicit non-goals

This ticket does not define or implement:

- aggregation lane, lane key, lane partition, group count, threshold,
  quorum, majority, weight, score, minimum reports, or corroboration;
- standing policy v3 or any standing change;
- same-origin collapse or distinct-group counting;
- refutation contribution, contradiction debt, invalidation, or
  supersession/currentness;
- source-standing propagation or any non-`ExternalSource × ExternalReport`
  lane;
- `EpistemicGate`, writer authority, loader, CAS, filesystem or network
  ingestion;
- any L0 payload, format, or dependency change; or
- Deadbolt changes.

## 14. Authority

`AGENTS.md` remains binding except that George authorizes, only after the
exact implementation and every prescribed validation pass:

- branch `agent/support-contribution-audit-v0-runtime` from the PR #64
  merge commit;
- the commits listed in §2;
- pushing that branch without force; and
- opening exactly one draft PR titled
  `feat: implement support-contribution audit v0`.

The worker may not modify `main`, create another branch, mark the PR ready,
merge, enable auto-merge, force-push, reply to or resolve review threads,
edit the ratified contract documents, add dependencies, or change any
existing production behavior.

## 15. Reviewer checklist

1. Confirm the base is the PR #64 merge commit and the branch contains
   exactly the commits listed in §2.
2. Confirm the five public types, three constants, field order, enum
   tagging, and derive sets match the contract (no `Deserialize`, no
   `Default`, no public constructors, private fields).
3. Confirm every upstream constant is reused by import and none is
   redefined.
4. Confirm the resolver is the single public method on
   `OriginAdmissionReplayContextV0` and accepts only `&self` and the
   closure.
5. Confirm there is exactly one composition path and the production
   resolver decomposes the internally derived audit through public getters
   only.
6. Confirm the eighteen-stage order in the composer matches the contract,
   including stage 15 shape-before-disposition.
7. Confirm the policy cells are checked before any candidate work and even
   when both admitted maps are empty.
8. Confirm duplicate edge IDs are checked across all dispositions and
   report the lexically first duplicate.
9. Confirm all trace checks complete before any map insertion.
10. Confirm deterministic first-failure selection is independent of vector
    order everywhere (edge-ID order or exact `ContributionIdentityV0`
    order).
11. Confirm stage 15 rejects empty-vector incomplete completions as
    `EmptyUnavailableBindingSelectors`, never normalized or repaired.
12. Confirm the eighteen invariants run in the frozen order through the
    four crate-private grammar seams, and `validate_replay_reference`
    delegates to the replay-reference helper.
13. Confirm completion mapping happens only after all checks, and any
    rejection yields zero support contributions with expected identities.
14. Confirm the unreachable-by-construction caveat in §6 and that no
    staging seam was added to the origin-binding module.
15. Confirm the five fixtures match production bytes and the recorded
    lengths and hashes.
16. Confirm the self-mutation check in §11 was performed and reverted.
17. Confirm no Cargo, dependency, L0, standing, policy-matrix,
    origin-binding behavior, origin-admission, or admitted-contribution
    production change; tour output byte-identical.
18. Confirm no aggregation, group counting, threshold, or corroboration
    vocabulary entered the runtime.
