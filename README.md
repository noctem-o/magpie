# Magpie

A local-first, provenance-first research mind. **One signed append-only log is the
sole source of truth; one verified capability gate is the only thing that may write
to it; everything else is a derived, regenerable projection.**

This repo is the **walking skeleton**: the bottom of the stack (`magpie-log`, L0) and
one worked projection (`magpie-claims`) that proves the central promise — drop every
derived view, replay the log, and you are back exactly where you were.

## Run it

```sh
cargo test                                   # all tests, incl. the thesis test
cargo run --example tour -p magpie-claims    # a 30-second end-to-end tour
```

The thesis, as a test (`crates/magpie-claims/tests/regenerable.rs`):
> build a projection → **drop all derived state** → replay from the log alone →
> assert the result is **byte-for-byte identical**.

## Layout

```
magpie/
  crates/
    magpie-log/      L0 — the signed, hash-chained, append-only event log (the hoard)
      src/event.rs       Event model: Provenance, Status spectrum, Payload, EventCore, SignedEvent
      src/canonical.rs   magpie-core-v1 — the canonical byte encoding (see docs/FORMAT.md)
      src/hashing.rs     SHA-256 ContentHash (the chain link)
      src/store.rs       LogStore trait + FileStore (durable) + MemStore (tests/embedding)
      src/logimpl.rs     LogWriter (the write capability) / LogReader (read-only) / Projection / replay
      tests/chain.rs     chain integrity, tip recovery, tamper detection, genesis rules
      tests/golden.rs    golden vectors: pinned hashes, signatures, canonical preimages
      testdata/          golden-v1.jsonl — the committed fixture chain
      examples/regen_golden.rs   regenerates the fixture (only when cutting a NEW profile)
    magpie-claims/   A projection: the epistemic claim store, folded from the log
  docs/FORMAT.md     the normative format spec — a reimplementation from this page
                     alone must reproduce the golden vectors
      src/lib.rs         ClaimsView : Projection (claims move along open→settled→refuted)
      tests/regenerable.rs   the thesis test
      examples/tour.rs   write → verify → replay → drop → replay → byte-identical
```

## The two invariants, enforced by types (not by convention)

1. **Append-only.** There is no `update` or `delete`. A correction is a new event that
   supersedes — which is exactly how the epistemic spectrum records a claim moving
   `Supported → Refuted`.
2. **The gate is the only writer.** Only `LogWriter` has `.append()`. Holding a
   `LogWriter` *is* the write capability. Memory layers are handed a `LogReader`, which
   exposes no way to write. In the full system the gate (`deadbolt`) is the only holder
   of the `LogWriter`.

## How this meets `deadbolt` (the integration seam)

`magpie` and the kernel relate over a **runtime boundary, not a `use` statement**. The
shared foundation is the log crate, used **asymmetrically**:

- the kernel (`deadbolt`) holds the `LogWriter` — it is the sole writer, after its
  capability + policy check, and it writes the witness event;
- memory and every other projection hold a `LogReader` — read-only by construction.

So you don't merge codebases. `magpie-log` becomes the crate both repos depend on; the
kernel adapts to *its* boundary, never the reverse.

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

- **Toolchain pins.** `Cargo.toml` pins `ed25519-dalek = "=2.1.1"`, `base64ct = "=1.6.0"`,
  and `zeroize = "=1.8.1"` solely because this was built on Ubuntu's cargo 1.75; newer
  releases of those crates require edition 2024 / rustc ≥ 1.81. On a current toolchain you
  can drop all three pins.
- **Canonical encoding is `magpie-core-v1`** — a fixed binary codec (big-endian
  integers, length-prefixed UTF-8, tagged enums, magic-prefixed), spec'd in
  `docs/FORMAT.md` and pinned by golden vectors in CI. The store stays JSON-lines;
  stored bytes are never the hash preimage. Every chain begins with a **genesis
  event** (seq 0) declaring the profile and the verifying key, written
  automatically when a writer opens an empty store. Signatures are made over
  `magpie-sig-v1 || hash` — domain-separated, so this key's log signatures can't
  be confused with anything else it signs. Logs written by the pre-v1 scaffold
  (serde_json preimages, no genesis) do not verify under v1 — by design; they
  were throwaways.
- **`MemStore` is single-threaded** (`Rc`/`RefCell`). `FileStore` is the durable one; swap
  to `Arc`/`Mutex` if you need threads.
- Keys: the scaffold takes a `SigningKey` from the caller and uses a fixed seed in tests.
  In production generate it once with a CSPRNG: `SigningKey::generate(&mut OsRng)`.

## Next steps (from the architecture ledger, Tier 1)

1. ~~Swap `canonical_bytes()` for a canonical codec~~ — **done**: `magpie-core-v1`,
   genesis event, signature domain separation, golden vectors, `docs/FORMAT.md`.
2. Add a SQLite + FTS5 episodic projection alongside `ClaimsView` (same `Projection` trait,
   built purely by replay, read-only).
3. Wire the kernel's capture/provenance into `append`, and hand the `LogWriter` to `deadbolt`.
4. Then, and only then: the typed stores, the reranker, the evaluator. Earn each from use.

Conserve the log. Derive the rest.
