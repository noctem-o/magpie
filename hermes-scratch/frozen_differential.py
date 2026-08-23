"""Frozen-corpus differential run — Hermes.

Runs key frozen corpus cases through MY Python V_sig relation and prints a
per-case verdict matrix suitable for direct comparison with Qwen's Rust
harness output. Uses exact frozen bytes; nothing synthetic. Cases with no
frozen counterpart are listed as such rather than substituted.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from vsig import vsig_verify

WT = os.path.dirname(HERE)
CASES_DIR = os.path.join(WT, "fixtures", "verifier-language-v1", "cases")

# case_id -> expected (verdict, stage) per the frozen manifest expectations.
# These mirror the manifest's expectation records; any divergence between my
# verdict and this oracle is reported as DIVERGENCE, not silently passed.
# golden-v1-rec0 / k13-a21-identity-key are key-only (no records in the
# frozen file): the relation cannot execute them, so they are listed as
# NO-RECORDS rather than counted as PASS or substituted synthetically.
FROZEN_EXPECT = {
    "golden-v1-rec0": ("NO-RECORDS", None),
    # identity-R family
    "a21-d2-identity-r-equation-true": ("ACCEPT", None),
    "a21-r-identity-equation-false": ("REJECT", "Signature"),
    # small-order keys
    "a21-s4-identity-key-s-zero": ("REJECT", "ExternalKey"),
    "a21-s5-identity-key-s-l-minus-one": ("REJECT", "ExternalKey"),
    "k13-a21-identity-key": ("NO-RECORDS", None),
    # R families
    "a21-r-nonidentity-small-order": ("REJECT", "Signature"),
    "a21-r-x-zero-sign-one": ("REJECT", "Signature"),
    "a21-r-y-equals-p": ("REJECT", "Signature"),
    "a21-r-nondecompress": ("REJECT", "Signature"),
    "a21-r-order-four-cofactor": ("REJECT", "Signature"),
    "a21-r-mixed-torsion-cofactor": ("REJECT", "Signature"),
}

def load_case(cid):
    p = os.path.join(CASES_DIR, cid + ".jsonl")
    if not os.path.exists(p):
        return None
    return [json.loads(ln) for ln in open(p, encoding="utf-8") if ln.strip()]

def run_case(cid):
    recs = load_case(cid)
    if not recs:
        # k13-a21-identity-key and golden-v1-rec0 are key-only/empty cases in
        # the frozen corpus: the identity-key gate is input-language-level
        # (hex parse / structural) and the case file carries no records.
        # Report as such rather than silently passing or substituting.
        return ("NO-RECORDS", None, "frozen case carries no records (key-only/empty)")
    core = recs[0]["core"]
    key_hex = core["payload"]["verifying_key"]
    A = bytes.fromhex(key_hex)
    for i, rec in enumerate(recs):
        M = b"magpie-sig-v1" + bytes.fromhex(rec["hash"])
        res = vsig_verify(A, bytes.fromhex(rec["signature"]), M)
        if res["verdict"] == "REJECT":
            stage = "ExternalKey" if res["stage"] == "ExternalKey" else "Signature"
            # For ExternalKey-class: fires on record 0 by construction (the key
            # is fixed for the whole history).
            return ("REJECT", stage, f"record {i}: {res['detail']}")
    return ("ACCEPT", None, f"{len(recs)} record(s) verified")

def main():
    rows, divergences = [], []
    executed = 0
    for cid, (exp_v, exp_s) in FROZEN_EXPECT.items():
        v, s, detail = run_case(cid)
        ok = v == exp_v and (v != "REJECT" or s == exp_s)
        rows.append((cid, exp_v, exp_s or "-", v, s or "-", ok))
        if v in ("ACCEPT", "REJECT"):
            executed += 1
        if not ok:
            divergences.append((cid, exp_v, exp_s, v, s))
    w = max(len(r[0]) for r in rows) + 2
    print(f"{'case':<{w}}{'expect':<22}{'actual':<22}ok")
    for cid, ev, es, av, as_, ok in rows:
        e = ev + (f"({es})" if es else "")
        a = av + (f"({as_})" if as_ else "")
        print(f"{cid:<{w}}{e:<22}{a:<22}{'PASS' if ok else 'DIVERGENCE'}")
    print()
    if divergences:
        print(f"FROZEN DIFFERENTIAL: {len(divergences)} divergence(s)")
        sys.exit(1)
    print(f"FROZEN DIFFERENTIAL: {executed} frozen cases executed + "
          f"{len(rows) - executed} key-only listed as NO-RECORDS; "
          "all match manifest-derived expectations (V_sig relation only)")

if __name__ == "__main__":
    main()
