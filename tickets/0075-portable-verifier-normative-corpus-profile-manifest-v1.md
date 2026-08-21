# Ticket 0075: Portable verifier normative corpus/profile manifest v1

## Status

Candidate exact-byte corpus and profile manifest pending hostile owner review
and merge. This ticket does not implement a Rust, Python, or Go conformer and
does not close A-021, A-004, RQ-006, or RQ-013.

## Satisfied upstream dependencies

- PR #120 / `docs/design/portable-verifier-input-language-contract.md` fixes
  the reference JSONL language, result vocabulary, coordinates, and coverage
  obligations.
- Accepted ADR-0010 fixes
  `V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"` and the exact
  cryptographic acceptance relation.

The ratified signature profile remains outside signed `magpie-core-v1` bytes.
Textual external-key spelling remains owned by the enclosing input language,
and key trust remains external.

## Scope

This tranche adds:

- `magpie-portable-verifier-corpus-v1` as a candidate, immutable-on-approval
  manifest identity;
- one SHA-256-bound machine-readable manifest;
- 430 new exact-byte case artifacts plus in-place references to the existing
  golden and Deadbolt fixtures, for 432 total cases;
- one complete ACCEPT/REJECT result per case;
- mechanical schema, numeric, vocabulary, hostile-family, and A-021
  inventories;
- exact S4/S5/D1/D2/T2 classifications under Accepted ADR-0010;
- a path-specific binary/no-text Git rule for hostile corpus bytes; and
- a metadata/hash/inventory checker that is not a verifier implementation; and
- one owner-authorized CI invariant that runs that checker on GitHub's normal
  clean checkout.

The corpus does not mint an ADR-0008 `G_history`. Its manifest directly fixes
`V_sig` and commits to the cited input-language/FORMAT/ADR revisions while
leaving the eventual complete history-verification derivation profile as a
separate coordinate-design and implementation matter.

## Exact result boundary

Every normative case has exactly one result:

```text
ACCEPT(event_count, tip, ordered_recomputed_hashes)
REJECT(class, line?, record_index?)
```

No dependency sentinel or third semantic result remains. Operational failure
to run a conformer is not a corpus verdict.

## Protected surfaces

This ticket does not change:

- `docs/FORMAT.md`;
- Rust, Python, or Go verification behavior;
- Cargo dependencies or lockfiles;
- existing golden or Deadbolt fixture bytes;
- canonical encoding or signature bytes;
- standing, admission, currentness, persistence, checkpoint, or release
  semantics; or
- CI other than the explicitly owner-authorized step that runs
  `tools/check_verifier_corpus_manifest.py`.

That step is not a conformer, does not make CI a second specification, does not
close A-021/A-004/RQ-006, and does not authorize broader workflow changes.

A-021 remains Confirmed because ratification and a candidate corpus are not
conformer implementation/conformance evidence. A-004 and RQ-006 remain
Confirmed. The corpus enriches RQ-013 evidence but does not satisfy its wider
resource, crash, platform, or exact-boundary assurance scope.

## Review gate

Before implementation, hostile review must independently check:

1. every input SHA-256 and clean-checkout byte identity;
2. all 57 required member coordinates and 12 object shapes;
3. positive coverage of every closed vocabulary value;
4. exact first-failure classification and coordinates for multi-defect cases;
5. full-pipeline A21-D1/D2 reconstruction and all subgroup/cofactor claims;
6. unchanged frozen fixtures; and
7. absence of any implied trust, freshness, completeness, non-equivocation,
   or authority claim.

## Next gate

```text
hostile corpus review
-> owner merge of the candidate corpus
-> typed Rust V_sig/profile-selection implementation
-> matching Python profile path
-> independent Go conformer
-> differential conformance
-> separate finding-status reconciliation
```

Human owner retains merge authority. Do not mark A-021, A-004, or RQ-006
closed merely because this corpus is complete or merged.
