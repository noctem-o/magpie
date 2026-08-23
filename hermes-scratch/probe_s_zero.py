"""W3 analysis: why doesn't the S=0 fixed point converge?

Equation: [S]B = R + [k]A with S = 0 requires R = -[k]A, where
k = H(R_bytes || A || M) mod L. But k depends on R bytes, so we need a
fixed point R* = -[H(R*)...mod L]A. This is a random mapping on ~2^252
values — a fixed point exists only with probability ~L * (1/L^2)... i.e.
essentially never. My iteration cannot converge. That's EXPECTED, not a bug.

Correct W3 construction: pick R first as a genuinely random multiple of B,
compute k from it, then set S = k*a mod L so the equation holds — but that
makes S nonzero. For an S=0 ACCEPT witness we instead need the equation to
hold WITH S=0, which requires the (astronomically unlikely) fixed point.

=> The honest S=0 boundary claim is NOT "an S=0 signature can be accepted"
but the weaker, still-valuable property: "S=0 passes the canonicality gate
(range check) and fails later only at the equation if no fixed point exists."
We demonstrate that directly at the vsig_verify level by checking the gate
sequence: feed S=0 and confirm the rejection detail is 'equation false'
(NOT 'S out of range'), proving S=0 was treated as in-range/canonical.
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, _add, L, p

B = base_point()
A_good_pt = _mul(12345 % L, B)
A_GOOD_B = _encode(A_good_pt)
M = b"magpie-sig-v1" + bytes(32)

# Real random-R signature with S=0: equation will fail, but the failure must
# be AT THE EQUATION, proving the S=0 range gate passed.
r_rand = 0xC0FFEE % L
R_enc = _encode(_mul(r_rand, B))
res = vsig_verify(A_GOOD_B, R_enc + (0).to_bytes(32, "little"), M)
print("S=0, random R:", res["verdict"], "|", res["stage"], "|", res["detail"])
assert res["detail"] == "equation false", \
    f"S=0 must clear the range gate; got {res['detail']}"

# Contrast: S=L must fail EARLIER, at the range gate.
res_L = vsig_verify(A_GOOD_B, R_enc + L.to_bytes(32, "little"), M)
print("S=L, same R  :", res_L["verdict"], "|", res_L["stage"], "|", res_L["detail"])
assert res_L["detail"] != "equation false", "S=L must not reach the equation"

print("""
W3 (revised, honest form): S=0 is inside the strict range 0 <= S < L and
is treated as canonical — it reaches the equation gate ('equation false'),
unlike S >= L which is rejected earlier ('S out of range'). An actual
ACCEPTING S=0 signature requires a hash fixed point and is not constructible;
this weaker demonstration is the provable claim.
""")
