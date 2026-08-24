"""Self-falsification A-F required by the follow-up remediation."""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import vsig
from vsig import V_SIG_PROFILE_ID, vsig_verify, profile_from_identity
from magpie_canonical import verify_history

results = []


def check(tag, name, cond):
    results.append((tag, name, cond))
    print(f"{'PASS' if cond else 'FAIL'}  [{tag}] {name}")


# A. Can vsig_verify still produce a normative verdict with no profile? NO.
try:
    r = vsig_verify(b"\x00" * 32, b"\x00" * 64, b"\x00" * 32)
    check("A", "vsig_verify without profile produces no verdict", False)
except TypeError:
    check("A", "vsig_verify without profile produces no verdict", True)

# B. Can verify_history produce a normative verdict with no explicit identity? NO.
try:
    verify_history("", "00" * 32)
    check("B", "verify_history without identity produces no verdict", False)
except TypeError:
    check("B", "verify_history without identity produces no verdict", True)

# C. Can an earlier successful selection enable a later profile-less call? NO.
sel = profile_from_identity(V_SIG_PROFILE_ID)
assert sel == V_SIG_PROFILE_ID
try:
    vsig_verify(b"\x00" * 32, b"\x00" * 64, b"\x00" * 32)
    check("C", "prior selection does not enable later profile-less call", False)
except TypeError:
    check("C", "prior selection does not enable later profile-less call", True)

# D. Can an unknown profile fall through into ordinary verification? NO.
fell_through = False
for wrong in ["latest", "", V_SIG_PROFILE_ID.upper(),
              "magpie-ed25519-canonical-prime-subgroup-v2", V_SIG_PROFILE_ID + " "]:
    try:
        vsig_verify(b"\x00" * 32, b"\x00" * 64, b"\x00" * 32, profile=wrong)
        fell_through = True   # any success (or verdict return) = fall-through
        break
    except ValueError:
        continue              # correct: rejected before any verification
check("D", "unknown profile never falls through to ordinary verification",
      not fell_through)

# E. Did F8-v2 regress F1 ExternalKey-before-Framing precedence? NO.
from magpie_canonical import Rejection
import json
I_KEY = "01" + "00" * 31
Y_P_KEY = "ed" + "ff" * 30 + "7f"
golden_path = os.path.join(os.path.dirname(HERE), "crates", "magpie-log",
                           "testdata", "golden-v1.jsonl")
with open(golden_path, encoding="utf-8") as f:
    VALID_KEY = json.loads(f.readline())["core"]["payload"]["verifying_key"]

probes = [
    ("identity+empty", I_KEY, "", "ExternalKey"),
    ("identity+{", I_KEY, "{", "ExternalKey"),
    ("y=p+empty", Y_P_KEY, "", "ExternalKey"),
    ("valid+empty", VALID_KEY, "", "Framing"),
]
e_ok = True
for name, key, text, expected in probes:
    try:
        verify_history(text, key, profile_identity=V_SIG_PROFILE_ID)
        got = "ACCEPT"
    except Rejection as e:
        got = e.stage
    if got != expected:
        e_ok = False
        print(f"      precedence regression: {name} -> {got}, want {expected}")
check("E", "F1 precedence intact after F8-v2 (4/4 probes)", e_ok)

# F. No claim stronger than executable evidence — assert required boundary
# language is present and no overclaim strings remain in the report.
report = open(os.path.join(HERE, "JOINT_REPORT_HERMES.md"), encoding="utf-8").read()
# Markdown line-wrapping can split phrases across lines; compare on
# whitespace-normalized text.
import re
flat = re.sub(r"\s+", " ", report)
required = [
    "directly executed relation subset",
    "NOT a full portable verifier conformer",
]
bad_claims = [s for s in ["matches the portable verifier input language",
                          "all official RFC vectors",
                          "full corpus conformer"] if s in flat]
f_ok = all(s in flat for s in required) and not bad_claims
check("F", "report keeps claims at executed-evidence strength", f_ok)
if not f_ok:
    if bad_claims:
        print(f"      overclaim strings found: {bad_claims}")
    missing = [s for s in required if s not in report]
    if missing:
        print(f"      missing required scope strings: {missing}")

sys.exit(0 if all(c for _, _, c in results) else 1)
