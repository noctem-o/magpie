# ADR-0001 — Deadbolt seam: anchored hierarchy

**Status:** Accepted (ratified by George, 2026-07-02)
**Deciders:** George; framing and drafting by Fable (Claude); evidence by
read-only survey of the deadbolt repository (`deadbolt-witness-survey.md`).

## Context

Deadbolt (the kernel; the trust boundary) will hold Magpie's `LogWriter` — the
sole write capability over the L0 log. Deadbolt also has complete internal
witness/evidence machinery of its own: kernel witness builds, observation,
codelet, and desktop-transaction evidence bundles, each with manifests, trace
events, and roots. The question: when the kernel can write to the one log,
what happens to its internal record-of-what-happened?

The spine sentence constrains the answer: *one* signed, append-only log is the
sole source of truth; everything else is derived — or, this ADR adds,
anchored.

## Evidence (from the survey; citations therein)

1. **Deadbolt seals, it does not stream.** Every evidence subsystem closes
   into a bundle at a natural completion boundary with fixed cardinality —
   exactly one trace event, one witness manifest, one root per
   observation/codelet/transaction bundle; ~4 events + one root per kernel
   witness build. Verifiers reject other cardinalities. The only
   stream-shaped artifact (kernel transcripts) is explicitly diagnostic-only
   and outside the evidence system.
2. **Roots are currently unsigned local commitments** — in the survey's own
   words, "until separately exported through DSSE or another anchor." There
   is no signed, totally ordered record of which segments occurred, in what
   order.
3. **Canonicalization plurality is real and load-bearing.** At least five
   named profiles are active (`phase5-interim-jcs-like-v1`,
   `codelet-interim-jcs-like-v1`, `desktop-transaction-interim-jcs-like-v1`,
   `cogitator-rfc8785-jcs-v1`, `phase9a-capability-lease-canonical-v1`),
   fit-for-purpose per domain.
4. **Coupling asymmetry.** Wholesale re-emission of witness events as Magpie
   events touches seven crates including `cog-canonical`; exporting segment
   roots touches only surfaces that already exist and requires no
   `cog-canonical` change.
5. All active root builders are SHA-256 over canonical JSON (no active BLAKE3
   chain — correcting an earlier assumption).

## Decision

**Anchored hierarchy.** Deadbolt keeps its internal evidence machinery
unchanged. After each successful seal-and-verify, the kernel — as holder of
the `LogWriter` — appends one `SegmentAnchored` event (payload tag 5) to the
Magpie log, carrying: `bundle_kind`, `witness_root` (64 lowercase hex,
verbatim), `witness_algorithm`, `canonicalization_profile` (the **foreign**
profile that produced the root, named explicitly), and `run_id`.

Semantics:

- **A segment that is not anchored is not part of the record.** The Magpie
  chain is the record of what execution history exists and in what order.
- **Two-step verification, one root of trust.** The Magpie chain verifies
  order, signature, and inclusion; a bundle's contents verify locally against
  its anchored root using deadbolt's existing verifier for that
  (`witness_algorithm`, `canonicalization_profile`) pair.
- **Kind-agnostic by construction.** New deadbolt bundle kinds are new
  `bundle_kind` strings; they must never require a Magpie format change.
- **DSSE is positioned, not displaced.** DSSE/in-toto remains the export
  vehicle for third parties; Magpie anchoring is the internal
  system-of-record. Complementary directions: DSSE speaks to the world,
  Magpie remembers for ourselves.
- **Excluded from anchoring:** kernel transcripts (diagnostic-only) and
  verify reports (verifier output — repeatable, hence regenerable, hence not
  chain-worthy).

Ratified parameters:

- **Cadence:** per-seal — one anchor per bundle. Survey cardinality implies
  tens per active session, comfortably chain-scale.
- **Topology:** anchors live in the *same* chain as claims. This is the
  spine, and it is what lets a future claim cite the execution that produced
  it.
- **Cross-references:** v1 informal (e.g. an `EvidenceRecorded` summary may
  mention a root in prose). A typed cross-reference field is a future
  vocabulary addition, earned from observed usage.

## Alternatives considered

- **A — full convergence** (witness events re-emitted as Magpie events;
  internal transcripts ephemeral): rejected. Seven-crate coupling; five
  foreign canonicalization profiles colliding with a frozen vocabulary;
  deadbolt's verified core would take a dependency on Magpie format
  evolution; and the granularity concern it was meant to answer turned out
  not to exist (evidence point 1).
- **B — parallel truth systems** (each keeps its chain): rejected. Two
  codecs, two verification stories, no principled citation path from claims
  to executions — the duplicated-truth failure the spine sentence exists to
  forbid.

## Consequences

Positive: deadbolt's roots gain what they lack — signature and total order —
without deadbolt changing its evidence machinery; one verification story;
`cog-canonical` untouched; Magpie's format stays frozen (tag 5 is an additive
vocabulary change under §3's evolution rule, with golden coverage appended at
seq 5 and seqs 0–4 hashes provably unchanged).

Negative / deferred: verification is two-step by design; a
**sealed-but-unanchored** bundle (append failure after seal) needs a
reconciliation story — visible set, retry policy — which is ops design,
deliberately deferred; anchor events grow the log, bounded by seal
cardinality.

Forcing function: every projection must decide what an anchor means to it
(exhaustive `Payload` matches, by design). `ClaimsView` ignores anchors; the
episodic store should surface them as timeline rows.

## Implementation

The Magpie-side slice ships with this ADR as `seam-slice.patch`: the variant,
its tag-5 canonical encoding, the FORMAT.md §3 addition, the `ClaimsView`
arm, and extended golden vectors (six events; original five hashes
unchanged). The deadbolt-side emitter (gate appends one anchor after each
successful seal-and-verify, plus the reconciliation set) is the next ticket,
written after this lands.
