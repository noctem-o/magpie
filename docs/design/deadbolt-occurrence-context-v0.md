# Deadbolt Occurrence Context v0

## Purpose

This note defines the inert exact-match contract by which future standing
policy may determine that a typed `DeadboltAnchor` candidate refers to an
identity in a supplied `DeadboltAnchorIndex`. Treating that identity as an
accepted-chain occurrence requires the separate verified-replay construction
precondition defined below.

The contract establishes occurrence/inclusion context only. It does not admit
evidence, promote a claim, aggregate contributions, or produce achieved
standing.

## Exact anchor identity

`DeadboltAnchorIdentity` conserves exactly five `SegmentAnchored` fields:

- `bundle_kind`;
- `witness_root`;
- `witness_algorithm`;
- foreign `canonicalization_profile`;
- `run_id`.

Identity equality is case-sensitive and byte-for-byte over Rust strings. No
field is trimmed, case-folded, normalized, aliased, path-resolved, or
reinterpreted. In particular, the foreign canonicalisation profile remains
verbatim and is never replaced with `magpie-core-v1`.

Sequence, event hash, and provenance are retained as occurrence audit material.
They do not participate in the five-field identity and do not confer authority.

## Claim predicate schema

An occurrence/inclusion claim uses this authoritative nested object:

```json
{
  "claim_domain": "Occurrence/Inclusion",
  "deadbolt_occurrence": {
    "schema": "segment_anchored_v1",
    "bundle_kind": "kernel-decision-witness",
    "witness_root": "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b",
    "witness_algorithm": "sha256",
    "canonicalization_profile": "kernel-decision-witness-v1",
    "run_id": "portable-run-0001"
  }
}
```

The predicate asserts that an exact `SegmentAnchored` identity exists. The pure
resolver tests that assertion against the supplied anchor index. Claim prose is
not parsed for matching or authority.

## Evidence reference schema

A typed evidence node labelled `DeadboltAnchor` uses this authoritative nested
reference:

```json
{
  "deadbolt_anchor_ref": {
    "schema": "segment_anchored_v1",
    "bundle_kind": "kernel-decision-witness",
    "witness_root": "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b",
    "witness_algorithm": "sha256",
    "canonicalization_profile": "kernel-decision-witness-v1",
    "run_id": "portable-run-0001"
  }
}
```

The reference remains asserted material until the exact replay lookup succeeds.
An evidence-kind label, actor class, summary, or `content_hash` cannot replace
the full identity.

## Parsing rules

Both authoritative objects fail closed unless:

- the outer metadata is valid JSON;
- the authoritative value is an object present exactly once;
- `schema` is exactly `segment_anchored_v1`;
- all five identity fields are present exactly once and are strings;
- every identity string is non-empty;
- `witness_root` is exactly 64 lowercase hexadecimal characters;
- no unknown field appears inside the authoritative object.

Nulls, arrays, numbers, booleans, aliases, coercion, duplicate keys, unknown
schemas, and unknown nested fields fail. Values are retained verbatim. Unrelated
top-level advisory metadata is ignored and cannot affect the match.

## Three-way equality law

```text
structured claim predicate
        ==
typed evidence anchor reference
        ==
identity present in supplied DeadboltAnchorIndex
```

`Matched` requires `DeadboltAnchor × Occurrence/Inclusion`, successful strict
parsing on both metadata surfaces, exact predicate/reference equality, and an
exact identity present in the supplied `DeadboltAnchorIndex`.

## Trusted construction precondition

`DeadboltAnchorIndex` is a generic projection, not a verified-container type.
It records whatever `SegmentAnchored` events are passed to
`Projection::apply`. Direct application does not validate signatures, event
hashes, sequence or previous-hash links, genesis key binding, chain membership,
or trust in a verifying key. Likewise, `LogReader::events` parses stored events
without verifying them.

The pure resolver therefore proves exact membership only in the supplied
index. Treating a match as accepted-chain occurrence/inclusion requires the
index to have been populated exclusively through a successful
`LogReader::replay` using the intended verifying key. `LogReader::replay`
verifies the chain before folding its events, while trust in the supplied
verifying key remains external to the projection.

Future standing integration must derive `StandingView` and
`DeadboltAnchorIndex` from the same successfully verified replay of the same log
under the same intended verifying key. It must not accept an arbitrary
caller-constructed index, an index populated by manual `Projection::apply`, an
index populated from unverified `LogReader::events`, or projections produced
from different logs or keys.

The future policy-bearing caller must also derive `claim_domain` fail-closed
from the same target claim metadata passed as `claim_metadata_json`. A caller
must not select `Occurrence/Inclusion` independently merely to enter the
Deadbolt matching path.

This inert phase intentionally introduces no verified wrapper, combined replay
object, typestate, construction token, or achieved-standing API.

## Matched-context meaning

`Matched` means only:

> The exact structured claim and exact typed evidence reference identify a
> `SegmentAnchored` occurrence present in the supplied anchor index.

Only when the trusted construction precondition holds may that membership be
interpreted as occurrence/inclusion in the accepted Magpie chain. Neither the
projection nor the resolver itself proves that provenance.

It does not establish interpretation truth, policy wisdom, general safety,
scientific correctness, action success beyond the exact proposition, admission,
aggregation, or achieved standing. Magpie verifies its chain, signature, order,
and inclusion; it does not replace the foreign bundle verifier.

## Authority limitations

The resolver has no inputs for actor class, claim prose, evidence summary,
caller-provided `verified` or `admitted` booleans, filesystem state, network
state, or runtime-selected policy. Those values cannot manufacture context.
The resolver also does not verify the Magpie chain, the construction history of
the supplied index, or how its `EvidenceKind` and `ClaimDomain` arguments were
derived.

## Portability

`DeadboltAnchorIndex` is a pure `magpie-log::Projection` and requires no live
Deadbolt installation or Deadbolt dependency. The checked-in portable fixture
exercises the contract independently of the foreign runtime.

## Duplicate anchors

The index retains every occurrence of an identical identity in sequence order.
The identity remains one key, and resolution returns its earliest occurrence
deterministically. Repetition does not strengthen the evidence epistemically.

## Non-goals

No `StandingView` anchor table, `StandingResolution` integration, new trace
reason, policy ID change, L0 or canonical change, fixture regeneration, foreign
bundle verification, admission, support/refutation application, aggregation,
writer surface, or Deadbolt execution is introduced here.

## Follow-up

The narrowly scoped follow-up is **Deadbolt occurrence/inclusion
achieved-standing policy v1**. That phase, not this scaffold, must decide how a
matched context participates in standing.
