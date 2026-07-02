# Ticket 0003 — SQLite + FTS5 episodic projection (`magpie-episodic`)

## 1. Goal

Create `crates/magpie-episodic`: the second projection over the log — a
time-ordered, full-text-searchable record of everything that happened.
Same `Projection` trait as `ClaimsView`, built purely by `LogReader::replay`,
structurally unable to write to the log, droppable and rebuildable at any time.

Deliverables:

1. New workspace member `crates/magpie-episodic` with an `EpisodicView` type
   implementing `magpie_log::Projection`.
2. SQLite storage via `rusqlite` with the `bundled` feature (decision made:
   vendored SQLite, FTS5 enabled, no system dependency).
3. An FTS5 index over event text, with a deterministic search API.
4. Tests mirroring the house pattern: regeneration equality, adversarial
   replay, and search behaviour.

## 2. Design decisions (already made — implement, do not revisit)

- **Crate layout.** Follow `magpie-claims` conventions: one `src/lib.rs`,
  doc comments carrying rationale, honest caveats stated plainly.
- **Construction.** `EpisodicView::in_memory()` (SQLite `:memory:`, default)
  and `EpisodicView::at_path(path)` for a file-backed index. Both are derived
  stores: deleting the file must never lose information.
- **Schema.** One base table plus one FTS5 external-content table:

  ```sql
  CREATE TABLE events (
      seq             INTEGER PRIMARY KEY,
      timestamp_nanos INTEGER NOT NULL,
      agent           TEXT NOT NULL,
      source          TEXT NOT NULL,
      kind            TEXT NOT NULL,
      claim_id        TEXT,
      body            TEXT NOT NULL
  );
  CREATE VIRTUAL TABLE events_fts USING fts5(
      body, content='events', content_rowid='seq'
  );
  ```

  Populate `events_fts` by explicit dual-insert inside `apply` — no triggers.
  Boring and visible beats clever and hidden.
- **Payload mapping.** The `Payload` match is exhaustive — every variant has
  an explicit arm, never `_ => {}` (freeze rule):

  | variant              | kind                   | claim_id | body                       |
  |----------------------|------------------------|----------|----------------------------|
  | `Genesis`            | `genesis`              | NULL     | canonicalization_profile   |
  | `ClaimAsserted`      | `claim_asserted`       | claim_id | statement                  |
  | `EvidenceRecorded`   | `evidence_recorded`    | claim_id | summary                    |
  | `ClaimStatusChanged` | `claim_status_changed` | claim_id | reason                     |
  | `Note`               | `note`                 | NULL     | text                       |

  Statuses (`from`/`to`) are deliberately NOT columns yet — the claims
  projection owns claim state; episodic owns "what happened, findable by
  text". Document this as provisional.
- **Timestamps.** `timestamp_nanos` is `u64`; SQLite INTEGER is `i64`.
  Convert with `i64::try_from` and treat overflow as a hard error (panic with
  a clear message + doc caveat). Do not silently truncate.
- **Error stance.** `Projection::apply` returns `()`. SQLite errors inside
  `apply` panic with a clear message. Rationale (put it in the doc comment):
  a derived view that failed mid-fold has no valid state to limp along with —
  drop it and rebuild from the log. Changing the `Projection` trait to return
  `Result` is a core-seam decision explicitly out of scope.
- **Search API.**
  `pub fn search(&self, query: &str) -> Result<Vec<u64>, rusqlite::Error>` —
  seqs of events whose body matches the FTS5 `MATCH` query, **ordered by seq
  ascending**. Not bm25 rank: determinism outranks relevance until a reranker
  exists (queued work #3). Also provide `len()`, `is_empty()`, and
  `get(seq) -> Option<EpisodicEvent>` returning a plain struct of the row.
- **Regeneration equality.** `canonical_bytes()` = deterministic JSON
  serialization of all rows `ORDER BY seq` (serde on a row struct — derive
  `Serialize`; use `BTreeMap` if any map appears). The SQLite *file* bytes are
  explicitly NOT the comparison surface — page layout is not deterministic.
  State this in a doc comment.

## 3. Scope

- May create: `crates/magpie-episodic/**` (Cargo.toml, src, tests).
- May edit: workspace `Cargo.toml` — add the member and
  `rusqlite = { version = "<current stable>", features = ["bundled"] }` to
  `[workspace.dependencies]`. `rusqlite` stays OUT of `magpie-log` — the hash
  path remains dependency-minimal.
- Cargo.lock: do NOT hand-edit. The reviewer refreshes it with cargo after
  applying the diff (CI enforces `--locked`, so the PR will carry the
  reviewer-generated lockfile).

## 4. Non-goals

- No changes to `magpie-log` or `magpie-claims` source.
- No new `Payload` variants, no reranker, no embeddings, no typed stores,
  no wiki, no CLI.
- No `Arc`/`Mutex` — single-threaded by design, same as the rest.
- No triggers, views, migrations framework, or connection pooling.

## 5. Constraints

- Rust 2021, workspace conventions; explicit types at API boundaries.
- Tests must include, at minimum:
  1. **Regenerable (the thesis test):** apply events one-by-one alongside a
     writer vs. rebuild-from-zero via `replay` — `canonical_bytes()` equal,
     byte for byte. Mirror `tests/regenerable.rs` in magpie-log.
  2. **Adversarial:** tamper a stored record in a `MemStore`, then attempt
     `replay` into a fresh `EpisodicView` — must error (chain verification
     fails) and the view must remain empty. New chain-level behaviour needs a
     hostile test, not just a happy path.
  3. **Search:** seed the golden-fixture events; assert an FTS query (e.g.
     `semiclassical`) returns exactly the expected seqs in ascending order;
     assert a non-matching query returns empty.
  4. **Drop-and-rebuild:** build, drop, rebuild from the same store, equal
     `canonical_bytes()`.
- Use `magpie_log::{LogWriter, MemStore}` with `open_with_clock` and a fixed
  seed/clock for deterministic test chains (see `tests/golden.rs` for the
  pattern).

## 6. Windows execution constraint

Codex must not run `cargo` or any shell command as a precondition — file
edits only. If shell execution fails with `CreateProcessAsUserW failed: 5`,
continue with safe, bounded file edits and report every skipped validation
command. Never report tests as passing unless they actually ran and passed.

## 7. Tests / reviewer validation (run by the reviewer, not Codex)

```powershell
cargo update            # refresh lockfile for the new dependency
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Plus a golden-suite explicit run (`cargo test -p magpie-log --test golden`) —
this ticket must not move a single pinned byte.

## 8. Acceptance criteria

- `cargo test --workspace` green, suite strictly larger than before (grow the
  suite, never shrink it; 17 tests at format freeze is the floor).
- The four test categories in §5 all present and passing.
- No frozen surface touched: `canonical.rs`, `docs/FORMAT.md`,
  `testdata/golden-v1.jsonl`, `tests/golden.rs`, `tools/verify_chain.py`.
- `magpie-log`'s dependency list unchanged.
- Doc comments explain the *why* (error stance, seq-order search,
  canonical-bytes-not-file-bytes) — not just the API.

## 9. Frozen surfaces

`crates/magpie-log/src/canonical.rs`, `docs/FORMAT.md`,
`crates/magpie-log/testdata/`, `crates/magpie-log/tests/*.rs`,
`tools/verify_chain.py`, `scripts/codex-delegate.ps1`. Read-only; no edits
under any circumstances.
