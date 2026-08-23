"""Frozen-fixture witness consumption for run_vectors.py — Hermes.

Replaces synthetic constants with bytes read from the frozen corpus where a
corpus case exists, and labels every remaining synthetic case SYNTHETIC.

Frozen sources consumed:
  fixtures/verifier-language-v1/cases/*.jsonl  (frozen exact-byte cases)
  crates/magpie-log/testdata/golden-v1.jsonl   (frozen golden chain)

This module exposes loaders; run_vectors.py wires them in. Kept separate so
the main harness diff stays small and reviewable.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
WT = os.path.dirname(os.path.dirname(HERE))  # repo root (hermes-scratch/../..)
# NOTE: hermes-scratch sits at <repo>/hermes-scratch, so repo root is dirname(HERE).
WT = os.path.dirname(HERE)

CASES_DIR = os.path.join(WT, "fixtures", "verifier-language-v1", "cases")
GOLDEN = os.path.join(WT, "crates", "magpie-log", "testdata", "golden-v1.jsonl")


def load_frozen_case(case_id):
    """Return the frozen case record dict or None if absent."""
    path = os.path.join(CASES_DIR, case_id + ".jsonl")
    if not os.path.exists(path):
        return None
    recs = [json.loads(ln) for ln in open(path, encoding="utf-8") if ln.strip()]
    return {"id": case_id, "records": recs, "frozen": True}


def frozen_key_and_first_sig(case_id):
    """Extract (external_key_hex, first_signature_hex) from a frozen case."""
    c = load_frozen_case(case_id)
    if not c:
        return None, None
    rec0 = c["records"][0]
    core = rec0["core"]
    key_hex = core["payload"]["verifying_key"]
    return key_hex, rec0.get("signature")


def golden_first_event():
    """Return (core_dict, hash_hex, signature_hex) of frozen golden rec0."""
    with open(GOLDEN, encoding="utf-8") as f:
        ev = json.loads(f.readline())
    return ev["core"], ev["hash"], ev["signature"]


if __name__ == "__main__":
    # sanity: list which adversarial families have frozen counterparts
    wanted = [
        "a21-d2-identity-r-equation-true",
        "a21-r-identity-equation-false",
        "a21-r-nonidentity-small-order",
        "a21-s4-identity-key-s-zero",
        "a21-s5-identity-key-s-l-minus-one",
        "k13-a21-identity-key",
    ]
    for w in wanted:
        print(f"{w}: {'FROZEN' if load_frozen_case(w) else 'absent'}")
    core, h, sig = golden_first_event()
    print("golden rec0 hash:", h[:16], "… sig:", sig[:16], "…")
