# Hermes — Rust↔Python differential matrix + joint report sections

PR #133 (Python) head at time of writing: `5a3d63f`
PR #134 (Rust) head under review: `0289dcc`
main baseline: `2751ac4`

## Differential matrix — V_sig relation surface

Legend: ✅ = agreement verified by executed evidence on both sides;
🧮 = agreement by code-reading + one-side execution; ⬜ = not jointly tested.

### External key

| case | Python result | Rust result | status |
|---|---|---|---|
| ordinary valid | ACCEPT (golden, RFC8032-T1) | ACCEPT (golden rec0 test) | ✅ |
| identity A | REJECT/ExternalKey | REJECT/ExternalKey (test) | ✅ |
| order-2 A | REJECT/ExternalKey (W1 witness + S4/S5/D1) | REJECT/ExternalKey (test) | ✅ |
| order-4 A | REJECT/ExternalKey | REJECT/ExternalKey (test) | ✅ |
| mixed-torsion A | REJECT/ExternalKey (T2) | REJECT/ExternalKey (test) | ✅ |
| y=p | REJECT/ExternalKey | REJECT/ExternalKey (test) | ✅ |
| y=p+1 | REJECT/ExternalKey | REJECT/ExternalKey (test) | ✅ |
| y=0xffed false-reject trap | accepted as below-p (probe) | accepted (`y_below_p` unit test) | ✅ |
| non-square-y | REJECT/ExternalKey | REJECT/ExternalKey (test k4/k5/k11) | 🧮 |
| x=0/sign=1 | REJECT/ExternalKey | REJECT/ExternalKey (test k12) | 🧮 |
| wrong length | REJECT/ExternalKey | REJECT/ExternalKey (31/33/0 bytes) | ✅ |

### R

| case | Python | Rust | status |
|---|---|---|---|
| ordinary R | ACCEPT (golden) | ACCEPT (golden rec0 test) | ✅ |
| identity R / equation true | ACCEPT (D2 frozen) | intended ACCEPT (literal broken @0289dcc) | 🧮 |
| identity R / equation false | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |
| order-2 R | REJECT/Signature (W2 witness) | REJECT/Signature (test) | ✅ |
| order-4 R | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |
| mixed-torsion R | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |
| y=p R | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |
| x=0/sign=1 R | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |
| non-decompressible R | REJECT/Signature (frozen) | REJECT/Signature (test) | ✅ |

### S

| case | Python | Rust | status |
|---|---|---|---|
| S=0 | passes range gate, reaches equation | `from_canonical_bytes` Some(0) (code-read) | 🧮 |
| S=L−1 equation-false | REJECT/Signature | intended (literal broken) | 🧮 |
| S=L | REJECT/Signature | intended (literal broken) | 🧮 |
| S=L+1 | REJECT/Signature | intended (literal broken) | 🧮 |
| all-FF S | REJECT/Signature | REJECT/Signature (test passes) | ✅ |

### Binding

| case | Python | Rust | status |
|---|---|---|---|
| correct M | ACCEPT | ACCEPT | ✅ |
| altered M | REJECT/Signature | test broken @0289dcc (got Ok) | ⬜ needs Qwen fix |
| altered sig bit | REJECT/Signature | REJECT/Signature (test) | ✅ |

### Profile

| case | Python | Rust | status |
|---|---|---|---|
| exact ID | only exact matches in wrapper design | const + from_identity PASS tests | ✅ |
| unknown/case/space/newline/"latest" | fail closed (wrapper) | fail closed (tests) | ✅ |

**No semantic disagreements found.** The only divergences are the five
broken Rust test literals (H8) and wording scope (H9).

## What V_sig agreement does NOT establish (jointly)

exact portable framing; restricted JSON language; duplicate-member
rejection; unknown-member rejection; lexical hex law; integer grammar;
schema closure; payload validation beyond canonical encoding of known
kinds; genesis binding; sequence; previous link; content-hash pipeline
in production reader; exact failure precedence across stages; production
path integration; producing-context carriage; Go independence; whole-corpus
(430-file) differential evidence.

## Decision gate answers (Hermes's artifacts)

1. **ADR-0010 implementable without further doctrine?** YES — both a Rust
   and an independent Python implementation exist and agree everywhere
   jointly exercised. No ambiguity requiring new doctrine surfaced.
2. **Qwen's Rust relation semantically correct?** YES on every reviewed
   surface (H1–H7). Tests-as-committed fail (H8), but those are oracle
   defects, not relation defects.
3. **My Python relation semantically correct?** YES within its claimed
   boundary: RFC 8032 Test 1 + structural calibration; 17-case synthetic
   family with real oracle; 10 frozen corpus cases match manifest-derived
   expectations.
4. **Agreement on independently tested V_sig surface?** YES — see matrix;
   zero semantic disagreements.
5. **Unresolved disagreements?** NONE semantic. Open items: Qwen's 5 test
   literals + fmt; H7 API-hardening choice.
6. **Either artifact suitable as reference?**
   - **#133: CLEAN THEN KEEP AS REFERENCE** — hardening landed (37635b2,
     a1eab82, e108ab2, 5a3d63f); remaining gap is full-RFC-vector
     calibration if desired.
   - **#134: CLEAN THEN KEEP AS REFERENCE** — fix 5 literals from frozen
     fixtures/math, run fmt/clippy, then it is a sound reference.
7. **Before promotion:** #134 must pass `fmt --check`, `clippy -D warnings`,
   `cargo test -p magpie-log --locked`; harness output phrased as
   "V_sig-relation cases"; decide H7 profile construction hardening;
   owner selects the authoritative tranche location (not hermes-scratch /
   .scratch).
8. **Smallest authoritative Rust tranche:** vsig.rs module + unit tests
   (fixed) re-exported behind an explicit profile-selection API, wired into
   NO production path yet; plus the corpus harness moved out of .scratch
   into tools/ or crates/*-tests.
9. **Before Python is a portable conformer:** implement the frozen input
   language exactly (framing, duplicate/unknown members, lexical hex/
   integer laws, schema closure, precedence chain, empty-input semantics),
   replace convenience parsing, then run the full 430-case corpus.
10. **Go conformer still required?** For A-021 closure as written
    (cross-language conformance evidence): yes, per the disposition doc.
11. **Blocks A-021:** hostile review completion, authoritative conformer
    merged (Rust), independent second-language conformance (Go per ledger),
    audit-ledger reconciliation by owner.
12. **Blocks A-004/RQ-006:** untouched this mission; separate finding.
13. **Owner decisions needed:** (a) H7 profile-construction hardening or
    accept-as-is; (b) select authoritative implementation location/tranche;
    (c) whether Python track continues to portable-conformer investment or
    remains hostile-check reference.
