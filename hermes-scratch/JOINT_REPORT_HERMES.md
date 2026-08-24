# Hermes — Rust↔Python differential matrix + joint report sections

PR #133 (Python) head at time of writing: `1f1fde7482bd93704bd933f6c185e50049e38401`
PR #134 (Rust) head under review: `0289dcc8741fa5866c56ffdebf8b09a3e677bc8cb`
main baseline: `2751ac41748d6aba05c503c721dad84a2c89f916`
Frozen corpus: **432 cases (19 ACCEPT, 413 REJECT)** per the manifest checker.

## Differential matrix — V_sig relation surface

Legend: ✅ = agreement verified by executed evidence on both sides;
🧮 = agreement by code-reading plus one-side execution only; ⬜ = not jointly
tested. Per Luna's finding F6, the defensible summary claim is restricted to
the directly executed subset.

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
| identity R / equation true | ACCEPT (frozen a21-d2) | intended ACCEPT (literal broken @0289dcc) | 🧮 |
| identity R / equation false | REJECT/Signature (frozen, manifest-driven) | REJECT/Signature (test) | ✅ |
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
| altered M | REJECT/Signature | test broken @0289dcc — and per Luna's audit the committed vector verifies for the ALTERED hash and rejects for the ORIGINAL, i.e. the constant encodes the wrong direction | ⬜ needs Qwen fix |
| altered sig bit | REJECT/Signature | REJECT/Signature (test) | ✅ |

### Profile

| case | Python | Rust | status |
|---|---|---|---|
| exact ID | selects (`profile_from_identity`) | const + from_identity PASS tests | ✅ |
| unknown/case/space/newline/"latest" | fail closed (verified, verify_f8_profile.py) | fail closed (tests) | ✅ |

### Summary claim (F6-corrected)

**"No mismatch in the directly executed relation subset."** Rows marked 🧮
(code-read on one side) and ⬜ (altered-message, broken on the Rust side)
are evidence gaps, not verified agreements.

## Post-audit fixes applied to #133 (Luna F1–F10)

- **F1 [semantic]** — complete composed ExternalKey gate now runs BEFORE
  framing/JSON handling in `verify_history`; identity/y=p keys with empty or
  malformed input reject ExternalKey, not Framing/JsonSyntax
  (`verify_f1_precedence.py`: 5/5 PASS).
- **F2 [oracle]** — `check_qwen_constants.py` QWEN_A literal repaired
  (missing `e`); rerun: A_KEY and HASH both match fixture bytes True.
- **F3 [overclaim]** — S=0 claims restated: ~1 expected hash-mapping fixed
  point under the RO heuristic (existence likely), none found here; only the
  gate-sequence property is claimed as executed evidence. Stale W3 ACCEPT
  expectation removed.
- **F5 [oracle]** — mixed-torsion probe repaired: T4 has ORDER 4 (mixed point
  order 4L, not 8L); bogus k=42 "cancellation" print removed.
- **F7 [gap]** — `frozen_differential.py` rewritten to load expectations from
  the manifest directly, verify case SHA-256s, and classify every one of the
  432 cases honestly: 37 relation-executed / 20 matched manifest expectations,
  0 divergences, 0 SHA mismatches, 19 key-only-or-transport cases listed
  NO-RECORDS, 376 out-of-relation-scope skipped. Explicit scope statement
  emitted.
- **F8 [portability]** — explicit fail-closed profile-selection seam added to
  `vsig.py` (`V_SIG_PROFILE_ID`, `profile_from_identity`,
  `require_profile`, profile argument on `vsig_verify`);
  `verify_f8_profile.py`: 9/9 PASS.
- **F10 [overclaim]** — this report regenerated against final head with the
  432-case count.
- F4 (W1/W2 executed in Python only) is acknowledged and left open: running
  the witness through Rust requires editing #134, which this mission forbids;
  recommended for Qwen's repair tranche.
- F9 (Rust pub unit struct bypassable) is acknowledged as valid; fix belongs
  in #134.

## What V_sig agreement does NOT establish

exact portable framing; restricted JSON language; duplicate-member
rejection; unknown-member rejection; lexical hex law; integer grammar;
schema closure; payload validation beyond canonical encoding of known
kinds; genesis binding; sequence; previous link; content-hash pipeline in
the production reader; exact failure precedence across stages; production
path integration; producing-context carriage; Go independence; whole-corpus
432-case differential evidence (relation harness covers 37).

## Decision gate answers (Hermes, post-audit)

1. ADR-0010 implementable without new doctrine? YES.
2. Qwen's Rust relation semantically correct? YES (H1–H7); tests-as-committed fail (H8).
3. My Python relation semantically correct within its narrow boundary? YES.
   History wrapper now precedence-complete but remains NOT a portable conformer.
4. Agreement on jointly tested surface? No mismatch in the directly executed
   subset; code-read cells remain gaps.
5. Unresolved disagreements? None semantic. Open: H8 literals+fmt, F4
   Rust-side witness run, H7/F9 hardening decision.
6. Disposition: #133 CLEAN THEN KEEP AS REFERENCE (post-audit fixes landed);
   #134 CLEAN THEN KEEP AS REFERENCE (after Qwen repairs).
7. Before promotion: see Luna §5/§6 — corrected tests, opaque profile seam,
   portable/history integration, environment-successful validation.
8. Smallest authoritative Rust tranche: vsig.rs + fixed unit tests behind an
   opaque explicit profile-selection API; no production wiring yet; harness
   moved out of .scratch.
9. Python portable-conformer requires: exact input language, full pre-framing
   key gate (done), explicit selection (done), manifest-driven full-corpus
   execution of ALL classes, pinned reproducible deps.
10. Go conformer still required per the accepted sequence.
11. A-021 blockers unchanged: hostile-review completion, authoritative merge,
    second-language conformance, owner ledger reconciliation.
12. A-004/RQ-006: separate Confirmed finding; untouched.
13. Owner decisions: promotion authorization, tranche selection, merge
    authorization (neither draft merges during this mission).
