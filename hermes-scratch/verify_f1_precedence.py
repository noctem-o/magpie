"""F1 verification: complete ExternalKey precedence (Luna's finding 1).

Probes:
  - identity key + empty input   -> ExternalKey (was Framing)
  - identity key + "{"           -> ExternalKey (was JsonSyntax)
  - y=p key + empty input        -> ExternalKey (was Framing)
  - valid key + empty input      -> Framing (unchanged)
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from magpie_canonical import verify_history, Rejection

I_KEY = "01" + "00" * 31
Y_P_KEY = "ed" + "ff" * 30 + "7f"
VALID_KEY = None
# derive a valid key from the golden fixture
import json
golden_path = os.path.join(os.path.dirname(HERE), "crates", "magpie-log", "testdata", "golden-v1.jsonl")
with open(golden_path, encoding="utf-8") as f:
    VALID_KEY = json.loads(f.readline())["core"]["payload"]["verifying_key"]

probes = [
    ("identity key + empty", I_KEY, "", "ExternalKey"),
    ('identity key + "{"', I_KEY, "{", "ExternalKey"),
    ("y=p key + empty", Y_P_KEY, "", "ExternalKey"),
    ("y=p key + junk", Y_P_KEY, "not json at all", "ExternalKey"),
    ("valid key + empty", VALID_KEY, "", "Framing"),
]

failures = []
for name, key, text, expected in probes:
    try:
        verify_history(text, key)
        got = "ACCEPT"
    except Rejection as e:
        got = e.stage
    ok = got == expected
    print(f"{'PASS' if ok else 'FAIL'}  {name:24s} -> {got} (expected {expected})")
    if not ok:
        failures.append(name)

if failures:
    print(f"\nF1 VERIFICATION FAILED: {failures}")
    sys.exit(1)
print("\nF1 VERIFIED: composed ExternalKey gate precedes framing/JSON handling")
