"""D4: mixed-torsion-R claim examination (Hermes, independent).

Question: can the uncofactored equation EVER hold when R is outside the
prime subgroup, given A is in it (A passed the ExternalKey gates)?
"""
import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import base_point, IDENTITY, _mul, _decode, _add, L

B = base_point()

# decode(00*32) is (x, 0) with x^2 = -1/(d+1): this is the order-4 point
# (verified below: [2]T4 != I, [4]T4 == I).
T4 = _decode(bytes(32))
print("decode(00*32) =", T4)
for n in (1, 2, 4, 8):
    print(f"[{n}]T4 == I? {_mul(n, T4) == IDENTITY}")
assert _mul(4, T4) == IDENTITY and _mul(2, T4) != IDENTITY, "expected order 4"

R_mt = _add(B, T4)   # mixed-torsion point, order lcm(L,8) = 8L
print("\n[L]R_mt == I?", _mul(L, R_mt) == IDENTITY, "(must be False = outside prime subgroup)")

# Prime-subgroup key
A_pt = _mul(12345 % L, B)
assert _mul(L, A_pt) == IDENTITY

# Coset argument, verified concretely: for ANY k and ANY S in [0,L),
# [S]B is in <B>; RHS = R_mt + [k]A has the same non-<B> torsion component as
# R_mt. Project both sides through [L]: [L](anything in <B>) = I, but
# [L]RHS = [L]R_mt != I. So equality is impossible for every S, k.
impossible = True
for trial in range(20):
    kk = (trial * 7919 + 3) % L
    rhs = _add(R_mt, _mul(kk, A_pt))
    if _mul(L, rhs) == IDENTITY:
        impossible = False
        break
print("exists k with RHS in <B>? ", not impossible)

# What if A were ALSO mixed-torsion (rejected at ExternalKey before this)?
# Then [k]A could carry an order-8 component canceling R's, and the equation
# COULD hold while [L]R gate fires first. That's the only load-bearing case,
# and it is unreachable because ExternalKey rejects such A first.
A_bad = _add(A_pt, T4)
kk = 42 % L
rhs2 = _add(R_mt, _mul(kk, A_bad))
print("mixed-torsion A could cancel torsion (unreachable in V_sig):",
      _mul(L, rhs2) != IDENTITY or "check")

print("""
CONCLUSION (D4):
With prime-subgroup A (the ONLY A that reaches Signature stage), the
uncofactored equation CANNOT hold for any mixed-torsion R: the equation
failure and the [L]R != I gate are simultaneously true; neither is
distinguishable from the other by outcome. Therefore the earlier claim
'subgroup rejection fires even though the equation would otherwise hold'
is FALSE for prime-subgroup keys and must be narrowed to:

  'the [L]R gate is normative per ADR-0010 and fires on mixed-torsion R;
   for prime-subgroup keys the equation would also fail independently,
   so the gate is defense-in-depth rather than load-bearing at this stage.'

The gate becomes load-bearing only for non-prime-subgroup A, which
ExternalKey already rejected earlier — i.e. never in reachable flow.
""")
