# Magpie

A local-first, provenance-first research mind. **One signed append-only log is the
sole source of truth; one verified capability gate is the only thing that may write
to it; everything else is a derived, regenerable projection.**

The skeleton walks: the bottom of the stack (`magpie-log`, L0, format-frozen and
golden-pinned), two worked projections (`magpie-claims`, `magpie-episodic`), an
independent Python verifier, a ratified live seam to `deadbolt`, and the first
ADR-0002 claim-memory vocabulary. The kernel can anchor sealed evidence into the
chain; Magpie can now name typed claim, evidence, and justification events
without yet exposing ordinary writer surfaces.

## Run it

```sh
cargo test                                   # whole workspace, incl. the thesis test
cargo run --example tour -p magpie-claims    # a 30-second end-to-end tour
```

The thesis, as a test (`crates/magpie-claims/tests/regenerable.rs`):
> build a projection → **drop all derived state** → replay from the log alone →
> assert the result is **byte-for-byte identical**.

## Layout

```
magpie/
  crates/
    magpie-log/        L0 — the signed, hash-chained, append-only event log (the hoard)
      src/event.rs         Event model: Provenance, Status spectrum, Payload, EventCore, SignedEvent
      src/canonical.rs     magpie-core-v1 — the canonical byte encoding (FROZEN; see docs/FORMAT.md)
      src/hashing.rs       SHA-256 ContentHash (the chain link)
      src/store.rs         LogStore trait + FileStore (durable) + MemStore (tests/embedding)
      src/logimpl.rs       LogWriter (the write capability) / LogReader (read-only) / Projection / replay
      tests/chain.rs       chain integrity, tip recovery, tamper detection, genesis rules
      tests/golden.rs      golden vectors: pinned hashes, signatures, canonical preimages
      testdata/            golden-v1.jsonl — the committed fixture chain (nine events, incl. ADR-0002 tags)
      examples/regen_golden.rs   regenerates the fixture for reviewed format-surface changes
    magpie-claims/     projection #1: the epistemic claim store, folded from the log
      src/lib.rs           ClaimsView : Projection (claims move along open→settled→refuted)
      src/standing.rs      StandingView : ADR-0002 read-only standing projection
      tests/regenerable.rs the thesis test
      examples/tour.rs     write → verify → replay → drop → replay → byte-identical
    magpie-episodic/   projection #2: SQLite + FTS5 full-text search over events
      src/lib.rs           EpisodicView : Projection (the only crate that may depend on SQLite)
      tests/episodic.rs    incl. rebuilt-from-zero == incrementally-built
  docs/
    FORMAT.md          the normative format spec — a reimplementation from this page
                       alone must reproduce the golden vectors
    adr/0001-deadbolt-seam.md   the anchored-hierarchy decision (see seam section below)
    adr/0002-governed-claim-memory.md   how memory events earn standing
  tools/
    verify_chain.py    independent verifier, written from FORMAT.md alone; CI runs it
                       against the golden fixture with a pinned trust root
```

## The two invariants, enforced by types (not by convention)

1. **Append-only.** There is no `update` or `delete`. A correction is a new event that
   supersedes — which is exactly how the epistemic spectrum records a claim moving
   `Supported → Refuted`.
2. **The gate is the only writer.** Only `LogWriter` has `.append()`. Holding a
   `LogWriter` *is* the write capability. Memory layers are handed a `LogReader`, which
   exposes no way to write. Projections are pure folds and stay structurally unable
   to write.

## How this meets `deadbolt` (the seam — ratified and live)

ADR-0001 (**anchored hierarchy**, ratified 2026-07-02): deadbolt keeps its internal
witness/evidence machinery unchanged; after each successful **seal-and-verify**, the
kernel appends one `SegmentAnchored` event (payload tag 5) to this chain, carrying
the bundle kind, the witness root (verbatim), the hash algorithm, the **foreign**
canonicalization profile that produced the root, and the run id.

- **A segment that is not anchored is not part of the record.** The Magpie chain is
  the record of what execution history exists, and in what order.
- **Two-step verification, one root of trust.** The chain verifies order, signature,
  and inclusion; a bundle's contents verify locally against its anchored root using
  deadbolt's own verifier for that profile.
- **Kind-agnostic by construction.** New deadbolt bundle kinds are new `bundle_kind`
  strings — never new Magpie payload variants.

Status of the deadbolt side (in the deadbolt repo): `cog-anchor` is the only crate
there that may depend on `magpie-log`, and its `Anchorer::open` is the sole
`LogWriter` construction site in that estate. Identity creation is explicit and
fail-closed (`cog anchor init`, refuses to overwrite); emission is fail-open —
anchoring failure writes a `pending-anchor.json` marker beside the bundle and never
fails a seal. All four production seal points emit anchors today — kernel witness,
observation, codelet, and desktop-transaction dry-run: every sealed bundle either
has an anchor in the log or a `pending-anchor.json` beside it. `cog anchor
reconcile` (deadbolt ticket 0009, merged) pays the recorded debt idempotently,
re-verifying every bundle with its own kind's verifier before anchoring — reconcile
is deliberately the first consumer of anchors, folding an `AnchorSet` projection
over a chain-verified replay of this log.

## Governed claim memory (ADR-0002 — additive vocabulary, no new authority)

ADR-0002 answers the question the anchor seam intentionally does not answer:
Deadbolt can prove or refuse occurrence, but what may Magpie believe about that
evidence?

The rule is deliberately conservative: a memory event earns standing through a
governed write path and deterministic replay into `StandingView`. Standing is not
stored as mutable truth, and `SegmentAnchored` remains occurrence/inclusion
evidence, not interpretation truth.

The log now knows three additive payload tags:

- tag 6, `ClaimAssertedV2`: registers a scoped claim node;
- tag 7, `EvidenceRegistered`: registers typed evidence under a deterministic
  ceiling;
- tag 8, `JustificationEdgeRecorded`: records a closed-vocabulary support,
  contradiction, lineage, invalidation, supersession, or ratification edge.

These are format vocabulary, not a permission system. `StandingView` gives the
first deterministic read surface. `EpistemicGate` and all writer-facing surfaces
remain future work.

## Development workflow

Magpie uses a maker/checker loop for ordinary agent work:

- Claude decides the bounded change and writes a ticket under `.agent-runs\pending\`;
- Codex implements that ticket through `scripts\codex-delegate.ps1`, the delegation boundary;
- Claude reviews the resulting diff and tests;
- a human holds sole merge authority.

The wrapper is not edited during ordinary work, and Codex output is never committed or
merged automatically.

On native Windows, validation commands are normally run by the human reviewer if Codex
shell execution is unavailable.

## Honest notes

- **Canonical encoding is `magpie-core-v1`** — a fixed binary codec (big-endian
  integers, length-prefixed UTF-8, tagged enums, magic-prefixed), spec'd in
  `docs/FORMAT.md` and pinned by golden vectors in CI. The format is **frozen**:
  tags 5-8 were added under §3's additive-evolution rule, with older golden hashes
  and signatures unchanged. The store stays JSON-lines; stored bytes are never the
  hash preimage. Every chain begins with a **genesis event** (seq 0) declaring the
  profile and the verifying key, written automatically when a writer opens an
  empty store. Signatures are made over `magpie-sig-v1 || hash` — domain-separated,
  so this key's log signatures can't be confused with anything else it signs.
- **`MemStore` is single-threaded** (`Rc`/`RefCell`). `FileStore` is the durable one; swap
  to `Arc`/`Mutex` only after a design conversation — single-threaded is currently a choice.
- **Keys are dev-grade everywhere.** `magpie-log` takes a `SigningKey` from the
  caller; deadbolt's anchor identity is a bare hex seed file created by
  `cog anchor init`. Real custody (keyring/HSM, rotation) is deliberately deferred
  and honestly labelled wherever it appears.
- **No knowledge graph.** The wiki, when it exists, is a downstream projection.

## Next steps (from the architecture ledger; kept honest as things land)

1. ~~Canonical codec~~ — **done**: `magpie-core-v1`, genesis, signature domain
   separation, golden vectors, `docs/FORMAT.md`, independent Python verifier in CI.
2. ~~Episodic projection~~ — **done**: `magpie-episodic`, SQLite + FTS5, pure fold.
3. ~~Deadbolt seam~~ — **ratified and live**: ADR-0001, `SegmentAnchored` tag 5,
   deadbolt-side `Anchorer` emitting at the kernel witness seal point.
4. ~~Reconcile pass~~ — **done** (deadbolt PR #320): verify-then-anchor,
   idempotent, first consumer of anchors. The seam is complete end to end.
5. ~~ADR-0002 groundwork~~ — **landed**: governed claim memory ADR, `StandingView`
   v0 skeleton, boundary tests, and additive tags 6-8 with verifier/golden coverage.
6. ~~Standing replay structure~~ — **in progress**: typed claim, evidence, and
   justification edge tables make ADR-0002 graph material regenerable.
7. **Next:** implement standing semantics behind those tables — evidence
   ceilings, support, contradiction debt, invalidation, supersession, and
   ratification, each with enforcement tests.
8. Then add `EpistemicGate`; only after that should writer-facing surfaces, MCP
   write paths, lens ingestion, typed stores, rerankers, or evaluators appear.

Conserve the log. Derive the rest.
