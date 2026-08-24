"""D4/S=0 boundary examination (Hermes) — corrected per Luna's audit.

Two claims, each stated at exactly the strength the evidence supports:

1. MIXED-TORSION R (prime-subgroup A): the uncofactored equation CANNOT
   hold. Proof by coset projection: [S]B always lies in <B>; for ANY k,
   RHS = R_mt + [k]A has the same non-<B> torsion component as R_mt, so
   [L]RHS != I while [L](LHS) == I. The [L]R gate is therefore
   defense-in-depth here (both gates fire; outcomes indistinguishable),
   load-bearing only for non-prime-subgroup A — unreachable after
   ExternalKey.

2. S=0: an ACCEPTING witness requires a fixed point of the map
   F(R) = -[H(R||A||M) mod L]A on ~L prime-subgroup points. Under the
   random-oracle heuristic a random mapping on N points has ~ (1-e^-1) N
   image points and ~1 EXPECTED fixed point overall (mean over mappings).
   So an accepting S=0 signature very likely EXISTS; no efficient search
   is claimed, and none was found here. Executed evidence covers only:
   S=0 passes the canonical range gate (rejection detail is 'equation
   false', never 'S out of range').
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, _add, L, p, V_SIG_PROFILE_ID

B = base_point()

# ---- Part 1: mixed torsion -------------------------------------------------
# decode(00*32): y=0 -> x^2 = -1/(d+1). This point T4 satisfies [4]T4 = I and
# [2]T4 != I, so it has ORDER 4 (not 8; earlier draft mislabeled this).
T4 = _decode(bytes(32))
print("decode(00*32) =", T4)
for n in (1, 2, 4):
    print(f"[{n}]T4 == I? {_mul(n, T4) == IDENTITY}")
assert _mul(4, T4) == IDENTITY and _mul(2, T4) != IDENTITY, "expected order 4"

R_mt = _add(B, T4)   # mixed-torsion point, order lcm(L,4) = 4L
print("\n[L]R_mt == I?", _mul(L, R_mt) == IDENTITY, "(False = outside prime subgroup)")

A_pt = _mul(12345 % L, B)
assert _mul(L, A_pt) == IDENTITY

# Coset argument verified concretely: for any k, RHS keeps R_mt's torsion.
all_outside = True
for trial in range(20):
    kk = (trial * 7919 + 3) % L
    rhs = _add(R_mt, _mul(kk, A_pt))
    if _mul(L, rhs) == IDENTITY:
        all_outside = False
        break
print("exists k putting RHS inside <B>? ", not all_outside)

# NOTE: the earlier k=42 'cancellation' print was WRONG — k=42 does not
# cancel T4's component ([42]T4 = [2]T4 != I); that line demonstrated
# nothing and has been removed.

# ---- Part 2: S=0 gate sequence ---------------------------------------------
A_GOOD_B = _encode(A_pt)
M = b"magpie-sig-v1" + bytes(32)
r_rand = 0xC0FFEE % L
R_enc = _encode(_mul(r_rand, B))
res = vsig_verify(A_GOOD_B, R_enc + (0).to_bytes(32, "little"), M, profile=V_SIG_PROFILE_ID)
print("\nS=0, random R:", res["verdict"], "|", res["stage"], "|", res["detail"])
assert res["detail"] == "equation false", "S=0 must clear the range gate"

res_L = vsig_verify(A_GOOD_B, R_enc + L.to_bytes(32, "little"), M, profile=V_SIG_PROFILE_ID)
print("S=L, same R  :", res_L["verdict"], "|", res_L["stage"], "|", res_L["detail"])
assert res_L["detail"] == "S out of range (not canonical)"

print("""
CONCLUSIONS (strength-matched to evidence):
1. With admissible prime-subgroup A, the equation cannot hold for
   mixed-torsion R; the normative [L]R gate fires alongside an inevitable
   equation failure (defense-in-depth at this stage).
2. S=0 passes the strict range gate and reaches the equation. An accepting
   S=0 witness likely exists (~1 expected fixed point under RO heuristic)
   but was not found; NO nonexistence or nonconstructibility claim is made.
""")
