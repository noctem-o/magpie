# Ticket 0025: Exact Deadbolt occurrence verifier context

## Goal

Add an inert exact-match scaffold that can determine whether a structured
occurrence claim and typed `DeadboltAnchor` evidence reference identify an exact
identity in a supplied `DeadboltAnchorIndex`. Accepted-chain interpretation
requires the separate verified-replay construction precondition.

## Architectural law

```text
structured claim predicate
        ==
typed evidence anchor reference
        ==
identity present in supplied DeadboltAnchorIndex
```

An anchor-shaped assertion, evidence label, actor class, prose statement, or
caller-provided boolean is not verifier context. A context match is not
admission, aggregation, or achieved standing.

## Exact schemas

The claim predicate key is `deadbolt_occurrence`; the typed evidence reference
key is `deadbolt_anchor_ref`. Both contain the exact schema
`segment_anchored_v1` and all five identity fields:

```json
{
  "schema": "segment_anchored_v1",
  "bundle_kind": "kernel-decision-witness",
  "witness_root": "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b",
  "witness_algorithm": "sha256",
  "canonicalization_profile": "kernel-decision-witness-v1",
  "run_id": "portable-run-0001"
}
```

The target domain must be `Occurrence/Inclusion` and the evidence kind must be
`DeadboltAnchor`.

## Closed result vocabulary

- `Matched(DeadboltAnchorOccurrence)`;
- `WrongEvidenceKind`;
- `WrongClaimDomain`;
- `MissingClaimPredicate`;
- `MalformedClaimPredicate`;
- `MissingEvidenceReference`;
- `MalformedEvidenceReference`;
- `PredicateReferenceMismatch`;
- `AnchorNotFound`.

`Matched` means only that exact structured identities refer to an occurrence in
the supplied `DeadboltAnchorIndex`. Accepted-chain provenance additionally
requires construction through successful verified replay with the intended
verifying key.

## Acceptance criteria

- `DeadboltAnchorIdentity` preserves all five payload fields verbatim.
- `DeadboltAnchorOccurrence` retains sequence, event hash, and provenance as
  non-identity audit material.
- `DeadboltAnchorIndex` independently observes only `SegmentAnchored` events.
- Repeated identities retain all occurrences in sequence order under one key.
- Strict metadata parsing rejects duplicates, unknown nested fields, aliases,
  wrong types, nulls, invalid roots, empty strings, and unknown schema values.
- Exact claim/reference equality precedes exact supplied-index lookup.
- The pure resolver accepts no actor, prose, verifier flag, or ambient state.
- The public contract distinguishes exact membership in the supplied index from
  proof that the index was built by verified replay.
- Trusted callers derive `ClaimDomain` fail-closed from the same target claim
  metadata supplied to the occurrence resolver.
- Future standing policy must not accept an arbitrary caller-supplied anchor
  index. `StandingView` and `DeadboltAnchorIndex` must be derived from the same
  successful `LogReader::replay` of the same log under the same intended
  verifying key.
- Current standing and canonical resolution bytes remain unchanged.

## Tests

- Replay and audit the portable fixture.
- Pin deterministic index bytes and foreign-profile preservation.
- Ignore unrelated payloads and retain repeated anchors deterministically.
- Match the exact fixture identity.
- Change each identity field independently for mismatch and not-found outcomes.
- Exercise strict parsing on both authoritative metadata objects.
- Prove labels, prose, actor classes, booleans, wrong kinds/domains, and
  unanchored tuples cannot manufacture context.
- Preserve anchor-only standing inertness and existing canonical resolution
  tests.

## Frozen surfaces

No changes to `Payload`, L0 tags, `docs/FORMAT.md`, canonical encoding,
signatures, golden logs/hashes, fixture bytes, `tools/verify_chain.py`, policy
matrices, `StandingView`, `StandingResolution`, trace types/reasons, policy ID,
Deadbolt, Cogitator, MCP, or writer surfaces.

## Validation

```text
git diff --check
cargo fmt --all -- --check
cargo test -p magpie-claims --locked
cargo test -p magpie-log --locked
cargo test --workspace --locked
cargo clippy --locked -p magpie-claims --all-targets -- -D warnings
python tools/verify_chain.py fixtures/deadbolt-anchor-v1/anchor-log.jsonl d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
cargo test -p magpie-claims deadbolt_occurrence --locked
cargo test -p magpie-claims deadbolt_anchor --locked
cargo test -p magpie-claims standing_resolution --locked
```

## Non-goals

No achieved support, settlement, or refutation; admission tokens;
`EpistemicGate`; human or deterministic-verifier admission; foreign bundle
verification; Deadbolt execution/authority; aggregation; independence groups;
contradiction debt; invalidation; supersession; currentness; policy v1; writer
surfaces; cross-repository integration; Verus; checkpoint publication; network
service; or model/lens ingestion.

## Stop conditions

Stop rather than adding a payload, changing canonical format or fixture bytes,
adding anchors to `StandingView`, changing `StandingResolution` or its policy
ID, adding a trace reason, trusting actor/prose/caller flags, invoking Deadbolt,
or changing governed standing.

## Follow-up

The next phase is **Deadbolt occurrence/inclusion achieved-standing policy
v1**. This ticket does not settle anything.
