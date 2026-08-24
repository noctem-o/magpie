import ast
for f in ["calibrate.py", "adversarial_witness.py", "probe_mixed_torsion.py",
          "probe_s_zero.py", "check_qwen_constants.py", "run_vectors.py",
          "frozen_differential.py"]:
    ast.parse(open(f, encoding="utf-8").read())
    print(f, "parses OK")
