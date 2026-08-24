"""F8-v2 verification: explicit, fail-closed, stateless profile selection.

Covers all 12 properties required by the follow-up remediation, including
the no-ambient-state sequence test (#12).
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import vsig
from vsig import (V_SIG_PROFILE_ID, profile_from_identity, vsig_verify,
                  external_key_admissible)
from magpie_canonical import verify_history, Rejection

failures = []


def check(num, name, cond):
    print(f"{'PASS' if cond else 'FAIL'}  [{num:2d}] {name}")
    if not cond:
        failures.append((num, name))


# 1. exact canonical identity selects
check(1, "exact identity selects",
      profile_from_identity(V_SIG_PROFILE_ID) == V_SIG_PROFILE_ID)

# 2-7. everything else fails closed at selection
for num, bad in [
    (2, "unknown identity"),
    (3, V_SIG_PROFILE_ID.upper()),
    (4, V_SIG_PROFILE_ID + " "),
    (5, V_SIG_PROFILE_ID + "\n"),
    (6, "latest"),
    (7, "magpie-ed25519-canonical-prime-subgroup-v2"),
]:
    check(num, f"fail-closed on {bad if isinstance(bad,str) and len(bad)<40 else bad!r}",
          profile_from_identity(bad) is None)

# probe bytes for gate tests (any bytes; key gate precedes signature work)
KEY = b"\xea\x4a\x6c\x63" + b"\x00" * 28  # arbitrary; verdict irrelevant here

# 8. normative vsig_verify cannot run with an omitted profile
try:
    vsig_verify(KEY, b"\x00" * 64, b"\x00" * 32)
    check(8, "vsig_verify without profile raises", False)
except TypeError:
    check(8, "vsig_verify without profile raises", True)

# 9. normative verify_history cannot run with an omitted profile identity
try:
    verify_history("", "00" * 32)
    check(9, "verify_history without profile_identity raises", False)
except TypeError:
    check(9, "verify_history without profile_identity raises", True)

# 10. explicit correct profile reaches the ordinary V_sig gate
res = vsig_verify(KEY, b"\x00" * 64, b"\x00" * 32, profile=V_SIG_PROFILE_ID)
check(10, "explicit correct profile reaches ordinary gate",
      res["verdict"] == "REJECT")  # KEY here is not admissible; gate ran

# 11. explicit wrong profile cannot fall back to legacy verification
for wrong in ["latest", "magpie-ed25519-canonical-prime-subgroup-v2", "", "x"]:
    try:
        vsig_verify(KEY, b"\x00" * 64, b"\x00" * 32, profile=wrong)
        check(11, f"wrong profile {wrong!r} cannot verify", False)
        break
    except ValueError:
        pass
else:
    check(11, "wrong profile cannot fall back to legacy verification", True)

# 12. no ambient state: a successful earlier selection must NOT enable a
# later profile-less call.
sel = profile_from_identity(V_SIG_PROFILE_ID)   # successful selection
assert sel is not None
try:
    vsig_verify(KEY, b"\x00" * 64, b"\x00" * 32)          # profile-less
    ok12 = False
except TypeError:
    ok12a = True
try:
    verify_history("", "00" * 32)                          # identity-less
    ok12b = False
except TypeError:
    ok12b = True
# also confirm no module state leaked
no_ambient = not hasattr(vsig, "_SELECTED")
check(12, "prior selection creates NO ambient state enabling later "
          "profile-less calls", ok12a and ok12b and no_ambient)

# bonus: pure external-key helper consistency with the relation gate
ok_a = external_key_admissible(b"\x01" + b"\x00" * 31) == (False, "A == I")
ok_b = external_key_admissible(b"\xed" + b"\xff" * 30 + b"\x7f")[0] is False
check(13, "external_key_admissible agrees with relation ExternalKey stage",
      ok_a and ok_b)

if failures:
    print(f"\nF8-v2 FAILED: {failures}")
    sys.exit(1)
print("\nF8-v2 VERIFIED: explicit, fail-closed, stateless profile selection")
