"""S=0 gate-sequence probe (Hermes) — corrected per Luna's audit.

Executed claim ONLY: S=0 passes the canonical range gate and reaches the
equation; S>=L is rejected earlier at the range gate.

An ACCEPTING S=0 witness requires a fixed point of F(R) = -[H(R||A||M) mod L]A.
Under the random-oracle heuristic, a random mapping on N ~ L points has
roughly ONE EXPECTED fixed point (mean over random mappings), so such a
witness likely EXISTS. Finding it efficiently is a separate (open for this
reference) problem. No nonexistence or nonconstructibility claim is made.
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, _add, L, p, V_SIG_PROFILE_ID

B = base_point()
A_good_pt = _mul(12345 % L, B)
A_GOOD_B = _encode(A_good_pt)
M = b"magpie-sig-v1" + bytes(32)

# Real random-R signature with S=0: equation will fail, but the failure must
# be AT THE EQUATION, proving the S=0 range gate passed.
r_rand = 0xC0FFEE % L
R_enc = _encode(_mul(r_rand, B))
res = vsig_verify(A_GOOD_B, R_enc + (0).to_bytes(32, "little"), M, profile=V_SIG_PROFILE_ID)
print("S=0, random R:", res["verdict"], "|", res["stage"], "|", res["detail"])
assert res["detail"] == "equation false", \
    f"S=0 must clear the range gate; got {res['detail']}"

# Contrast: S=L must fail EARLIER, at the range gate.
res_L = vsig_verify(A_GOOD_B, R_enc + L.to_bytes(32, "little"), M, profile=V_SIG_PROFILE_ID)
print("S=L, same R  :", res_L["verdict"], "|", res_L["stage"], "|", res_L["detail"])
assert res_L["detail"] == "S out of range (not canonical)", \
    f"S=L must fail at the range gate; got {res_L['detail']}"

print("""
W3 boundary conclusion (strength-matched):
  S=0 passes the strict range gate 0 <= S < L and reaches the equation.
  An accepting S=0 signature corresponds to a fixed point of a hash-driven
  random mapping (~1 expected fixed point under the RO heuristic); its
  existence is likely but was NOT demonstrated here, and no efficient
  construction is claimed.
""")
