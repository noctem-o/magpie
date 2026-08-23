"""Hermes independent adversarial witness against Qwen's Rust V_sig (#134).

Target under test: `prime_subgroup()` computes [L]P as `(L-1)*P + P`
because `Scalar::from_bytes_mod_order(L)` would reduce to zero.

Adversarial angle (independent, not copied from any Qwen test):
**the S = 0 / self-canceling-equation degenerate corner.**

  For an order-2 key A2 = (0,-1): [k]A2 has order <= 2. With R = -[k]A2 and
  S = 0 the uncofactored equation HOLDS:  [0]B = I = (-[k]A2) + [k]A2.
  Both R-decode and the equation pass; ONLY
      (a) A != I and [L]A == I   (ExternalKey stage), or
      (b) [L]R == I              (Signature stage)
  can reject. This witness therefore simultaneously exercises:
    - prime_subgroup() on a non-identity small-order point,
    - canonical_decode() of the order-2 encoding,
    - gate ORDER (ExternalKey must fire before Signature),
    - the S = 0 boundary (S must be treated as canonical).

Expected per ADR-0010:
  W1 REJECT at ExternalKey (order-2 key fails [L]A == I);
  W2 REJECT at Signature   (same order-2 bytes as R under a valid key:
     [L]R != I fires — proving the gate is load-bearing, not vacuous);
  W3 ACCEPT                (legitimately constructed S=0 signature).
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, _add, L, p

B = base_point()
ZERO_S = (0).to_bytes(32, "little")


def edwards_neg(pt):
    """Negation on twisted Edwards is (-x, y)."""
    return ((-(pt[0])) % p, pt[1])


M = b"magpie-sig-v1" + bytes(32)

# ---- W1: order-2 key, self-canceling signature, S=0 ----------------------
A2_ENC = bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f")
A2 = _decode(A2_ENC)
assert _mul(2, A2) == IDENTITY and A2 != IDENTITY

R1 = edwards_neg(_mul(1, A2))          # -[k]A2 for odd k; order-2 either way
R1_ENC = _encode(R1)
# verify by hand that the equation would hold: [0]B == R + [k]A2
lhs = IDENTITY                          # [0]B
rhs = _add(_decode(R1_ENC), _mul(1, A2))
print("W1 equation would hold:", lhs == rhs)
res = vsig_verify(A2_ENC, R1_ENC + ZERO_S, M)
print("W1 order-2 key, self-cancelling sig, S=0 ->", res["stage"], "|", res["detail"])
assert res["verdict"] == "REJECT" and res["stage"] == "ExternalKey", \
    f"W1 FAILED: expected ExternalKey rejection, got {res}"

# ---- W2: valid key, same order-2 bytes as R, S=0 --------------------------
A_good_pt = _mul(12345 % L, B)
A_GOOD_B = _encode(A_good_pt)   # bytes
res2 = vsig_verify(A_GOOD_B, A2_ENC + ZERO_S, M)
print("W2 good key, order-2 R, S=0              ->", res2["stage"], "|", res2["detail"])
assert res2["verdict"] == "REJECT" and res2["stage"] == "Signature", \
    f"W2 FAILED: expected Signature rejection via [L]R, got {res2}"

# ---- W3: legitimately constructed S=0 signature must ACCEPT ---------------
# Equation with S=0: [0]B = R + [k]A  =>  R = -[k]A. k depends on the FINAL
# R bytes, so iterate to a fixed point (R encoding stable).
A_GOOD_B = _encode(A_good_pt)
prev = None
R_enc = _encode(edwards_neg(_mul(1, A_good_pt)))   # any starting guess
for _ in range(10):
    k = int.from_bytes(hashlib.sha512(R_enc + A_GOOD_B + M).digest(), "little") % L
    R_new = _encode(edwards_neg(_mul(k, A_good_pt)))
    if R_new == R_enc:
        break
    R_enc = R_new
res3 = vsig_verify(A_GOOD_B, R_enc + ZERO_S, M)
print("W3 good key, R=-[k]A attempt, S=0        ->", res3["verdict"], "|", res3["detail"])
# Honest claim (see probe_s_zero.py): a genuinely ACCEPTING S=0 signature
# requires a hash fixed point R = -[H(R)...]A and is not constructible.
# The provable S=0 property is that S=0 passes the canonicality/range gate
# and reaches the equation gate — demonstrated in probe_s_zero.py by the
# rejection detail being 'equation false', never 'S out of range'.
assert res3["detail"] == "equation false", \
    f"W3 FAILED: S=0 must clear the range gate; got {res3['detail']}"

print("""
WITNESS SUMMARY (all three must hold simultaneously):
  W1 REJECT/ExternalKey — order-2 key rejected before signature work even
     though the uncofactored equation would VERIFY ([0]B = -[k]A2 + [k]A2).
  W2 REJECT/Signature   — order-2 bytes as R under a valid key: [L]R != I
     fires, proving prime_subgroup()/[L]R is load-bearing here.
  W3 equation-false     — S = 0 passes the strict range gate and reaches
     the equation (never 'S out of range'); an ACCEPTING S=0 witness is
     not constructible (needs a hash fixed point), so this weaker form is
     the honest claim.

Any Rust implementation that (a) reduces L mod L to compute [L]P (predicate
vacuously true -> W1/W2 accept small-order points), or (b) orders gates
wrongly fails one of these witnesses.
""")
