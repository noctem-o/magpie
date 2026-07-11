# Deadbolt Anchor Contract

## Purpose

This document defines the portable Deadbolt seam contract for Magpie. It is a
protocol boundary, not a runtime dependency.

Deadbolt is optional for Magpie core. Core tests, examples, verification, and
projections must remain runnable without a live Deadbolt installation, private
machine topology, or maintainer-local paths. Later seam tests should use
portable fixtures, not the user's live machine.

## Anchor Event

After a successful Deadbolt seal-and-verify step, Deadbolt may append one
`SegmentAnchored` event to a Magpie log.

A `SegmentAnchored` event records:

- `bundle_kind`;
- `witness_root`;
- `witness_algorithm`;
- foreign `canonicalization_profile`;
- `run_id`.

The foreign `canonicalization_profile` must be preserved verbatim. Magpie must
not relabel it as `magpie-core-v1`. `magpie-core-v1` describes Magpie event
canonical bytes; it is not the bundle profile that produced a foreign witness
root.

New Deadbolt bundle kinds are represented by new `bundle_kind` strings, not new
Magpie payload variants.

## Verification Split

Magpie verifies the chain: event order, chain inclusion, and signature.

The bundle's contents are verified by the bundle's own verifier and named
profile. Magpie does not reinterpret a foreign bundle by changing its
canonicalization profile or by replacing the bundle verifier with Magpie's log
verifier.

An unanchored segment is not part of Magpie's record. A sealed but unanchored
bundle may exist outside Magpie, but Magpie has not recorded its occurrence.

## Authority Boundary

An anchor is occurrence/inclusion evidence only.

An anchor does not settle interpretation truth. It records that a specific
foreign witness root was included in a specific Magpie chain position under a
specific provenance path. It does not decide what the bundle means.

An anchor does not create a typed claim by itself.

An anchor does not promote ADR-0002 standing by itself. In particular, it does
not grant ADR-0002 standing unless a later `StandingView` policy admits a
specific claim/evidence/edge relation with the required verifier context.

This preserves the base invariant:

> Magpie runs alone; integrations add evidence, not authority.

## Portability Requirements

The seam must remain testable without requiring the user's live Deadbolt
machine. Future fixture work should provide portable logs and anchor material
that exercise the `SegmentAnchored` contract deterministically.

The portable fixture at `fixtures/deadbolt-anchor-v1/` contains a deterministic
Magpie log with one `SegmentAnchored` event. It proves Magpie-side
verification and replay of anchor fields without requiring Deadbolt or live
machine artifacts.

The inert exact-match scaffold and its structured claim/evidence metadata
contract are defined in `docs/design/deadbolt-occurrence-context-v0.md`. A match
establishes exact membership in the supplied anchor index; accepted-chain
interpretation additionally requires the verified-replay construction
precondition defined there. A match does not change standing.

Magpie core must not depend on Deadbolt being installed for:

- core tests;
- examples;
- `tools/verify_chain.py`;
- `magpie-claims`;
- `magpie-episodic`;
- deterministic fixtures.

Deadbolt may depend on `magpie-log` at its own seam layer. That does not make
Deadbolt part of Magpie core.
