"""Run the A21 named vector family through the independent checker.
Compute first, compare after — expected verdicts are asserted at the END.
"""
import hashlib
import json
import os
import sys

_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _HERE)
from magpie_canonical import encode_eventcore, content_hash, message, verify_history, Rejection
from vsig import vsig_verify, base_point, IDENTITY, _mul, _decode, _encode, _add, L, p
from vsig import V_SIG_PROFILE_ID

# Repo root = two levels above hermes-scratch/.
WT = os.path.dirname(_HERE)
results = []

# Expected-outcome oracle. Every recorded case MUST appear here; run_vectors
# exits nonzero if any actual outcome diverges. Stages: ExternalKey /
# Signature / None(ACCEPT) / Framing / JsonSyntax / Schema / Sequence /
# PreviousLink / ContentHash / Genesis / PayloadValidation.
EXPECTED = {
    "P0-golden":     ("ACCEPT", None),
    "P0-deadbolt":   ("ACCEPT", None),
    "A21-S4":        ("REJECT", "ExternalKey"),
    "A21-S5":        ("REJECT", "ExternalKey"),
    "A21-T2":        ("REJECT", "ExternalKey"),
    "A21-D2":        ("ACCEPT", None),
    "A21-D1":        ("REJECT", "ExternalKey"),
    "A21-S1":        ("REJECT", "Signature"),
    "A21-S2":        ("REJECT", "Signature"),
    "A21-S3":        ("REJECT", "Signature"),
    "A21-M1":        ("REJECT", "Signature"),
    "A21-K1":        ("REJECT", "ExternalKey"),
    "K-y>=p":        ("REJECT", "ExternalKey"),
    "A21-R1":        ("REJECT", "Signature"),
    "A21-R2":        ("REJECT", "Signature"),
    "A21-R3":        ("REJECT", "Signature"),
    "R-mixedtorsion": ("REJECT", "Signature"),
}


def record(vid, outcome, detail=""):
    results.append((vid, outcome, detail))
    print(f"{vid:12s} {outcome:28s} {detail}")


def run_case(vid, jsonl_text, key_hex):
    """Full-pipeline run; returns ('ACCEPT', summary) or ('REJECT(stage)', detail)."""
    try:
        r = verify_history(jsonl_text, key_hex, profile_identity=V_SIG_PROFILE_ID)
        return ("ACCEPT", f"count={r['event_count']} tip={r['tip'][:16]}…")
    except Rejection as e:
        return (f"REJECT({e.stage})", e.detail)


def genesis_core(ts, key_hex):
    return {
        "seq": 0, "timestamp_nanos": ts, "prev_hash": "00" * 32,
        "provenance": {"agent": "magpie-log", "source": "genesis"},
        "payload": {"kind": "Genesis", "canonicalization_profile": "magpie-core-v1",
                    "verifying_key": key_hex},
    }


def to_jsonl(core, sig_hex):
    ev = {"core": core, "hash": content_hash(core), "signature": sig_hex}
    return json.dumps(ev, separators=(",", ":"))


# ---------------------------------------------------------------- fixtures
golden_path = WT + "/crates/magpie-log/testdata/golden-v1.jsonl"
deadbolt_path = WT + "/fixtures/deadbolt-anchor-v1/anchor-log.jsonl"

for vid, path in [("P0-golden", golden_path), ("P0-deadbolt", deadbolt_path)]:
    text = open(path, encoding="utf-8").read()
    gen_key = json.loads(text.splitlines()[0])["core"]["payload"]["verifying_key"]
    outcome, detail = run_case(vid, text, gen_key)
    record(vid, outcome, detail)

# ---------------------------------------------------------------- S4
I_KEY = "01" + "00" * 31
core_s4 = genesis_core(1, I_KEY)
h_s4 = content_hash(core_s4)
sig_s4 = "01" + "00" * 31 + "00" * 32  # R=I, S=0
record("A21-S4", *run_case("S4", to_jsonl(core_s4, sig_s4), I_KEY))

# ---------------------------------------------------------------- S5
R_negB = _encode((p - base_point()[0], base_point()[1])).hex()
S5_bytes = (L - 1).to_bytes(32, "little").hex()
sig_s5 = R_negB + S5_bytes
record("A21-S5", *run_case("S5", to_jsonl(core_s4, sig_s5), I_KEY))

# ---------------------------------------------------------------- T2
T2_KEY = "5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea"
core_t2 = genesis_core(2, T2_KEY)
h_t2 = content_hash(core_t2)
R_B = _encode(base_point()).hex()
# S = k + 1 as derived in the contract; recompute k ourselves
M_t2 = b"magpie-sig-v1" + bytes.fromhex(h_t2)
k_t2 = int.from_bytes(hashlib.sha512(bytes.fromhex(R_B) + bytes.fromhex(T2_KEY) + M_t2).digest(), "little") % L
assert k_t2 % 4 == 0
S_t2 = (k_t2 + 1).to_bytes(32, "little").hex()
record("A21-T2", *run_case("T2", to_jsonl(core_t2, R_B + S_t2), T2_KEY))

# ---------------------------------------------------------------- D2: ordinary golden A, R = I, keyholder signature
# Take the golden chain's own key and genesis event; construct S = k*a mod L
# requires signing scalar — instead use the documented property: build a fresh
# single-event history signed by a fresh ordinary key with R = I.
seed = hashlib.sha512(b"hermes-d2-seed").digest()[:32]
h = hashlib.sha512(seed).digest()
a = int.from_bytes(h[:32], "little") % L   # unclamped scalar is fine for verification math
A_pt = _mul(a, base_point())
A_D2 = _encode(A_pt).hex()

core_d2 = genesis_core(7, A_D2)
h_d2 = content_hash(core_d2)
M_d2 = b"magpie-sig-v1" + bytes.fromhex(h_d2)
R_I = "01" + "00" * 31
k_d2 = int.from_bytes(hashlib.sha512(bytes.fromhex(R_I) + bytes.fromhex(A_D2) + M_d2).digest(), "little") % L
S_d2 = ((k_d2 * a) % L).to_bytes(32, "little").hex()
record("A21-D2", *run_case("D2", to_jsonl(core_d2, R_I + S_d2), A_D2))

# ---------------------------------------------------------------- D1: order-two key (0, p-1)
D1_KEY = "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"
core_d1 = genesis_core(3, D1_KEY)
h_d1 = content_hash(core_d1)
# equation-true signature under this key: [0]B = I + [k]*order2? use S=0, R=I:
# equation: LHS [0]B = I ; RHS R + [k]A = I + [k](0,p-1) = I + I = I (since [k]A has order<=2 -> in {-A, I}... compute properly)
k_d1 = int.from_bytes(hashlib.sha512(bytes.fromhex(R_I) + bytes.fromhex(D1_KEY) +
                     b"magpie-sig-v1" + bytes.fromhex(h_d1)).digest(), "little") % L
# choose R so equation holds: R = -[k]A  (then encode canonically)
A_o2 = _decode(bytes.fromhex(D1_KEY))
R_pt = _add(IDENTITY, (p - _mul(k_d1, A_o2)[0], _mul(k_d1, A_o2)[1]))  # negate
R_d1 = _encode(R_pt).hex()
sig_d1 = R_d1 + "00" * 32
record("A21-D1", *run_case("D1", to_jsonl(core_d1, sig_d1), D1_KEY))

# ---------------------------------------------------------------- S-family scalar boundaries on an ordinary-key history
# Build an ordinary valid genesis signed correctly, then mutate S.
def sign_event(core, a):
    h = content_hash(core)
    M = b"magpie-sig-v1" + bytes.fromhex(h)
    r = int.from_bytes(hashlib.sha512(b"nonce" + M).digest(), "little") % L
    R_pt = _mul(r, base_point())
    R_b = _encode(R_pt)
    k = int.from_bytes(hashlib.sha512(R_b + _encode(_mul(a, base_point())) + M).digest(), "little") % L
    S = (r + k * a) % L
    return h, (R_b + S.to_bytes(32, "little")).hex()


ORD_KEY_PT = _mul(12345 % L, base_point())
ORD_KEY = _encode(ORD_KEY_PT).hex()
core_ord = genesis_core(9, ORD_KEY)
h_ord, good_sig = sign_event(core_ord, 12345 % L)

# S1: S = L
sig_s1 = good_sig[:64] + L.to_bytes(32, "little").hex()
record("A21-S1", *run_case("S1", to_jsonl(core_ord, sig_s1), ORD_KEY))

# S2: S = L+1 (non-canonical)
sig_s2 = good_sig[:64] + (L + 1).to_bytes(32, "little").hex()
record("A21-S2", *run_case("S2", to_jsonl(core_ord, sig_s2), ORD_KEY))

# S3: S = 0 boundary with non-matching equation
sig_s3 = good_sig[:64] + "00" * 32
record("A21-S3", *run_case("S3", to_jsonl(core_ord, sig_s3), ORD_KEY))

# M1: correct signature bytes under wrong message — a signature valid for a
# DIFFERENT event, stored on ours. The challenge binds M, so the equation
# must fail.
core_other = genesis_core(10, ORD_KEY)
_, other_sig = sign_event(core_other, 12345 % L)
record("A21-M1", *run_case("M1", to_jsonl(core_ord, other_sig), ORD_KEY))

# ---------------------------------------------------------------- K1: representation-invalid key (uppercase hex)
record("A21-K1", *run_case("K1", to_jsonl(core_ord, good_sig), ORD_KEY.upper()))

# K-noncanonical: y >= p key encoding
BAD_KEY = (p + 1).to_bytes(32, "little").hex()
record("K-y>=p", *run_case("Ky", to_jsonl(genesis_core(11, BAD_KEY), "00"*64), BAD_KEY))

# ---------------------------------------------------------------- R-family (mutate R of a good signature)
good_R = good_sig[:32]  # hex of R bytes... note good_sig is hex string; first 64 hex chars = R
R_b_hex = good_sig[:64]
R_b = bytes.fromhex(R_b_hex)

# R1: non-decompressing R (y with no square root). Search y values near the
# good R for one where xx = (y^2-1)/(d*y^2+1) is a quadratic non-residue.
import vsig as _v
found = None
for delta in range(1, 300):
    cand_y = ((int.from_bytes(R_b, "little") & ((1 << 255) - 1)) + delta) % p
    if cand_y >= p:
        continue
    xx = ((cand_y * cand_y - 1) * pow(_v.d * cand_y * cand_y + 1, p - 2, p)) % p
    x8 = pow(xx, (p + 3) // 8, p)
    if (x8 * x8 - xx) % p != 0:
        x8q = (x8 * pow(2, (p - 1) // 4, p)) % p
        if (x8q * x8q - xx) % p != 0:
            found = cand_y.to_bytes(32, "little")
            break
if found:
    sig_r1 = found.hex() + good_sig[64:]
    record("A21-R1", *run_case("R1", to_jsonl(core_ord, sig_r1), ORD_KEY))
else:
    record("A21-R1", "SKIPPED", "no non-residue found in range")

# R2: non-canonical encoding (y = p+1 style) of R
sig_r2 = ((p + 1).to_bytes(32, "little")).hex() + good_sig[64:]
record("A21-R2", *run_case("R2", to_jsonl(core_ord, sig_r2), ORD_KEY))

# R3: x=0 sign=1 identity encoding as R
sig_r3 = ("01" + "00" * 30 + "80") + good_sig[64:]
record("A21-R3", *run_case("R3", to_jsonl(core_ord, sig_r3), ORD_KEY))

# Non-subgroup R (mixed torsion): R' = B + T4 with recomputed S to make equation true
# except we keep S fixed -> equation false AND subgroup gate fires. Order matters:
# [L]R check must fire before equation. Use mixed-torsion point as R with correct-S-for-it:
import vsig as V
T4 = _decode(b"\x00" * 32)
R_mt_pt = _add(base_point(), T4)
R_mt = _encode(R_mt_pt).hex()
# craft S making equation true would still be REJECT via [L]R != I — do exactly that
r = 777 % L
k_mt = int.from_bytes(hashlib.sha512(bytes.fromhex(R_mt) + bytes.fromhex(ORD_KEY) +
                      b"magpie-sig-v1" + bytes.fromhex(h_ord)).digest(), "little") % L
S_mt = ((r + k_mt * (12345 % L)) % L).to_bytes(32, "little").hex()
record("R-mixedtorsion", *run_case("Rmt", to_jsonl(core_ord, R_mt + S_mt), ORD_KEY))

# ---------------------------------------------------------------- oracle pass
# Compute-first, compare-second: every recorded outcome must match EXPECTED.
divergences = []
for vid, actual, detail in results:
    exp = EXPECTED.get(vid)
    if exp is None:
        divergences.append((vid, "no expected outcome registered", actual))
        continue
    exp_verdict, exp_stage = exp
    if exp_verdict == "ACCEPT":
        ok = (actual == "ACCEPT")
    else:
        ok = (actual == f"REJECT({exp_stage})")
    if not ok:
        divergences.append((vid, f"{exp_verdict}" + (f"({exp_stage})" if exp_stage else ""), actual))

missing = set(EXPECTED) - {r[0] for r in results}
if missing:
    divergences.append((sorted(missing), "expected case never ran", None))

print("\n--- oracle ---")
if divergences:
    for vid, want, got in divergences:
        print(f"DIVERGENCE {vid}: expected {want}, got {got}")
    print(f"ORACLE FAILED: {len(divergences)} divergence(s)")
    sys.exit(1)
print(f"ORACLE PASSED: {len(results)}/{len(EXPECTED)} cases match expected outcomes")

print("\n--- machine-readable ---")
print(json.dumps([list(r) for r in results]))
