"""F8 verification: explicit fail-closed profile selection (Luna's finding 8)."""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from vsig import (V_SIG_PROFILE_ID, profile_from_identity, require_profile,
                  vsig_verify)

failures = []


def check(name, cond):
    print(f"{'PASS' if cond else 'FAIL'}  {name}")
    if not cond:
        failures.append(name)


# exact match selects
check("exact identity selects", profile_from_identity(V_SIG_PROFILE_ID) == V_SIG_PROFILE_ID)
# everything else fails closed
for bad in ["", "latest", V_SIG_PROFILE_ID.upper(), V_SIG_PROFILE_ID + " ",
            V_SIG_PROFILE_ID + "\n", "magpie-ed25519-canonical-prime-subgroup-v2"]:
    check(f"fail-closed on {bad!r}", profile_from_identity(bad) is None)
# vsig_verify rejects an unknown profile argument
try:
    vsig_verify(b"\x00" * 32, b"\x00" * 64, b"x", profile="bogus")
    check("vsig_verify rejects unknown profile", False)
except ValueError:
    check("vsig_verify rejects unknown profile", True)
check("vsig_verify accepts canonical profile",
      vsig_verify(b"\x00" * 32, b"\x00" * 64, b"x",
                  profile=V_SIG_PROFILE_ID)["verdict"] == "REJECT")

if failures:
    print(f"\nF8 FAILED: {failures}")
    sys.exit(1)
print("\nF8 VERIFIED: explicit, fail-closed profile-selection seam present")
