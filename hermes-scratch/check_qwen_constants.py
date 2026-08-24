"""Verify Qwen's frozen test constants (A_KEY, HASH) against the actual
frozen fixture bytes in crates/magpie-log/testdata/golden-v1.jsonl.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from magpie_canonical import content_hash

WT = os.path.dirname(HERE)
path = os.path.join(WT, "crates", "magpie-log", "testdata", "golden-v1.jsonl")
with open(path, encoding="utf-8") as f:
    first = json.loads(f.readline())

core = first["core"]
key_hex = core["payload"]["verifying_key"]
print("genesis verifying_key :", key_hex)

# Qwen's A_KEY constant: ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c
QWEN_A = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c"
print("matches Qwen A_KEY?   :", key_hex == QWEN_A)

# Qwen's HASH constant: cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f
h0 = first["hash"]
print("rec0 stored hash      :", h0)
recomputed = content_hash(core)
print("recomputed via my encoder:", recomputed, "| match:", h0 == recomputed)
QWEN_HASH = "cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f"
print("matches Qwen HASH?    :", h0 == QWEN_HASH)

# Also verify the golden rec0 signature against MY checker with these bytes:
sys.path.insert(0, HERE)
from vsig import vsig_verify, V_SIG_PROFILE_ID

sig_hex = first["signature"]
res = vsig_verify(
    bytes.fromhex(key_hex),
    bytes.fromhex(sig_hex),
    b"magpie-sig-v1" + bytes.fromhex(h0),
    profile=V_SIG_PROFILE_ID)
print("golden rec0 under my checker:", res["verdict"], "|", res["detail"])
