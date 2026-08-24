"""RFC 8032 calibration of the from-scratch checker. MUST pass before any
corpus vector is evaluated (magpie-development skill: ed25519 traps).

Calibration scope (honest statement): RFC 8032 Test 1 (empty message) plus
structural probes — base-point identity/encoding, [L]B = I, canonical
identity/order-2 decoding, sign-bit and y >= p rejection rules.
This is NOT the full set of official RFC 8032 test vectors.
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, L, p, V_SIG_PROFILE_ID

failures = []


def check(name, cond):
    print(f"{'PASS' if cond else 'FAIL'}  {name}")
    if not cond:
        failures.append(name)


# --- Trap 1: base point must be the RFC 8032 one (By = 4/5), not y=9
B = base_point()
check("base point y = 4/5 mod p", B[1] == (4 * pow(5, p - 2, p)) % p)
check("enc(B) == 5866..6666", _encode(B).hex() ==
      "5866666666666666666666666666666666666666666666666666666666666666")
check("[L]B == I", _mul(L, B) == IDENTITY)
check("[L-1]B == -B", _mul(L - 1, B) == (p - B[0], B[1]) if True else False)

# --- Trap 2/3: RFC 8032 Test 1 (empty message), all three checks
seed_hex = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
A_exp = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
R_hex = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155"
S_hex = "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"

h = hashlib.sha512(bytes.fromhex(seed_hex)).digest()
clamped = bytearray(h[:32])
clamped[0] &= 248
clamped[31] &= 127
clamped[31] |= 64
s_int = int.from_bytes(bytes(clamped), "little") % L
A_derived = _mul(s_int, B)
check("derived A matches RFC 8032 Test 1 public key", _encode(A_derived).hex() == A_exp)

A_b = bytes.fromhex(A_exp)
R_b = bytes.fromhex(R_hex)
S_b = bytes.fromhex(S_hex)
k = int.from_bytes(hashlib.sha512(R_b + A_b + b"").digest(), "little") % L
lhs = _mul(int.from_bytes(S_b, "little"), B)
rhs = (_decode(R_b), _mul(k, _decode(A_b)))[0]
from vsig import _add
rhs = _add(_decode(R_b), _mul(k, _decode(A_b)))
check("RFC 8032 Test 1 equation holds", lhs == rhs)
check("[L]A_test1 == I", _mul(L, _decode(A_b)) == IDENTITY)

# V_sig accepts this ordinary signature (A prime-subgroup non-identity,
# R prime-subgroup, S canonical).
res = vsig_verify(A_b, R_b + S_b, b"", profile=V_SIG_PROFILE_ID)
check("vsig ACCEPT on RFC 8032 Test 1 signature", res["verdict"] == "ACCEPT")

# --- Trap 4: identity normalization — decode(01||00*31) == (0,1); order-2 point distinct
I_enc = bytes.fromhex("01" + "00" * 31)
check("canonical identity decodes to (0,1)", _decode(I_enc) == (0, 1))
order2 = _decode(bytes.fromhex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"))
check("order-2 point (0,p-1) decodes and != identity", order2 == (0, p - 1))

# Non-canonical identity encoding (y=1 with x sign bit set: 01 00*30 80)
# must fail canonical decode.
check("identity with sign=1 rejected by canonical decode",
      _decode(bytes.fromhex("01" + "00" * 30 + "80")) is None)

# y >= p must be rejected (no reduction)
check("y = p encoding rejected", _decode((p).to_bytes(32, "little")) is None)
check("y = p+1 encoding rejected", _decode((p + 1).to_bytes(32, "little")) is None)

print()
if failures:
    print(f"CALIBRATION FAILED: {len(failures)}: {failures}")
    sys.exit(1)
print("ALL CALIBRATION CHECKS PASSED — checker outputs are now trustworthy")
