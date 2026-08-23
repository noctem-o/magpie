# Hermes Phase 1 independent baseline — PR #133

Date: 2026-08-23. Recorded BEFORE reading Qwen's #134 conclusions or hostile review.

## Artifact inspected
- Branch/head: `hermes/a021-vsig-independent` @ `968f503be397af5b84fe866bf8b09a3e677bc8cb`
- Base: main @ `2751ac41748d6aba05c503c721dad84a2c89f916`
- Files: `hermes-scratch/{vsig.py, magpie_canonical.py, calibrate.py, run_vectors.py}`

## Governing sources consulted
- `docs/adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md`
  (normative gates §profile)
- `docs/design/a021-ed25519-verification-semantics-contract.md` (ticket 0073)
- portable verifier input-language contract (title only this phase; deep read deferred)

## Code actually executed
1. `python calibrate.py` → exit 0, all 13 checks PASS.
   Scope: RFC 8032 **Test 1 only** (empty message) plus structural probes
   (base point identity, [L]B, order-2/y=p/y=p+1 decodes, canonical-identity rules).
   NOT all official RFC vectors.
2. `python run_vectors.py` → exit 0, 17 named outcomes printed:
   P0-golden ACCEPT(9 events), P0-deadbolt ACCEPT(2), S4/S5/T2/D1 ExternalKey,
   D2 ACCEPT(1), S1/S2/S3/M1 Signature, K1 ExternalKey(transport),
   K-y>=p Signature(!), R1/R2/R3 Signature, R-mixedtorsion Signature([L]R!=I).

## Failures / defects observed (self-audit)

### D1 [semantic defect] — wrong-stage classification for undecodable A
`magpie_canonical.py:170` remaps V_sig rejections to stages by matching
detail strings `("A == I", "[L]A != I")`. A non-canonically-encoded external
key (`y >= p`, non-square, x=0/sign=1) yields detail
"non-canonical / undecodable key", which falls through to **Signature**.
ADR-0010 puts ALL failed external-key admissibility (transport, decode
canonicity, A!=I, [L]A=I) in ExternalKey BEFORE any record processing.
Observed: `K-y>=p → REJECT(Signature)`; correct answer is ExternalKey.
Fix: use `res["stage"]` (already computed correctly inside vsig_verify)
instead of detail-string matching.

### D2 [test-oracle defect] — run_vectors.py asserts nothing
All 17 outcomes are printed, never compared to an expected-outcome oracle;
exit code is always 0. Divergences would pass silently.

### D3 [portability defect] — machine-specific absolute paths
`run_vectors.py` lines 8/12 hardcode `C:/Users/herpe/...`.
`calibrate.py` line 7 splits `__file__` on backslash — breaks on POSIX.

### D4 [evidence gap] — mixed-torsion-R claim not proven
Comment at run_vectors.py ~line 193-200 implies the crafted case shows
"subgroup rejection fires even though the equation would otherwise hold".
The constructed S is arbitrary (r=777-based), so nothing shows the equation
would hold absent the [L]R gate. Mathematical analysis suggests the opposite:
for prime-subgroup A, RHS = R+[k]A carries R's order-8 component while
LHS=[S]B never does, so the equation can NEVER hold for mixed-torsion R.
Must verify numerically and NARROW the claim either way.

### D5 [documentation overclaim risk] — calibration wording
PR body says "RFC 8032 calibration passes"; honest phrasing is
"RFC 8032 Test 1 plus structural calibration probes".

### D6 [hygiene] — dead code
run_vectors.py line 144-145 contains a no-op `if False else` expression;
line 172 uses an inline-walrus `__import__` hack; `V` import unused.

## Claims the evidence DOES support
- The from-scratch arithmetic layer passes RFC 8032 Test 1 end-to-end
  (derive key, verify signature, accept) plus structural decode probes.
- The V_sig gate ORDER inside vsig_verify matches ADR-0010 §normative gates:
  len→canonical-decode→A!=I→[L]A=I precede len64→R-decode→[L]R→S-range→equation.
- Golden/deadbolt frozen corpora verify ACCEPT under the independent encoder.
- Synthetic boundary witnesses (S=L, L+1, S<L−1-equation-false, altered M,
  non-decodable R encodings) reject at the expected gate *within vsig_verify*.

## Claims the evidence DOES NOT support
- Portable verifier conformance (input language, framing, duplicate/unknown
  members, lexical hex/integer law, schema closure, precedence chain) —
  verify_history uses ordinary json.loads/split/lower and implements NONE
  of the frozen lexical laws.
- All-RFC-vector calibration.
- Cross-language agreement (no Rust differential run yet).
- Production-path integration (this is scratch, not wired anywhere).
- That iterating fixture files = verifying portable cases (harness here
  builds synthetic histories; consumes only golden/deadbolt bytes verbatim).

## Relation-only verdict (my artifact)
PASS — V_sig mathematical relation only, modulo D1 being a WRAPPER bug
(vsig_verify itself classifies correctly; the history wrapper mislabels).
NOT YET — portable verifier conformance. NOT YET — A-021 closure.
