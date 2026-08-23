"""Hostile check of Qwen's Rust y_below_p predicate (PR #134) — Hermes.

Rust predicate under test:
    !((enc[31] & 0x7f) == 0x7f && (1..31).all(|i| enc[i] == 0xff) && enc[0] >= 0xed)

Claim: rejects exactly y >= p where y = low 255 bits LE, p = 2^255 - 19.
"""
p = 2**255 - 19


def rust_rejects(b: bytes) -> bool:
    if len(b) != 32:
        raise ValueError
    return (
        (b[31] & 0x7F) == 0x7F
        and all(b[i] == 0xFF for i in range(1, 31))
        and b[0] >= 0xED
    )


def y_of(b: bytes) -> int:
    return int.from_bytes(b, "little") & ((1 << 255) - 1)


mismatches = []

# 1. The exact boundary region: high bytes all set, sweep the low byte.
for lo in range(0xE8, 0xF4):
    b = bytearray(32)
    b[0] = lo
    for i in range(1, 31):
        b[i] = 0xFF
    b[31] = 0x7F
    b = bytes(b)
    if (y_of(b) >= p) != rust_rejects(b):
        mismatches.append(("boundary-low", b.hex()))

# 2. y = p-1 .. p+3 exactly (LE encodings).
for delta in range(-2, 5):
    val = p + delta
    if val < 0:
        continue
    b = bytearray((val).to_bytes(32, "little"))
    b[31] &= 0x7F  # clear sign bit; y itself unchanged
    # re-set sign bit variant too
    for sign in (False, True):
        bb = bytearray(b)
        if sign:
            bb[31] |= 0x80
        bb = bytes(bb)
        if (y_of(bb) >= p) != rust_rejects(bb):
            mismatches.append((f"delta{delta}sign{int(sign)}", bb.hex()))

# 3. y = 0xffed (the documented false-reject trap): low two bytes FF ED.
b = bytearray(32)
b[0], b[1] = 0xED, 0xFF
b[31] = 0x00
b = bytes(b)
print("y=0xffed: y>=p?", y_of(b) >= p, "| rust rejects?", rust_rejects(b))
assert not rust_rejects(b), "false reject of valid y!"

# 4. Random sampling both sides of the threshold with random other bytes.
import random

random.seed(1337)
for _ in range(200000):
    b = bytearray(random.randbytes(32))
    # half the time force the top pattern so we sample near-threshold values
    if _ % 2 == 0:
        b[31] = 0x7F | (b[31] & 0x80)
        for i in range(1, 31):
            b[i] = 0xFF if random.random() < 0.9 else b[i]
    bb = bytes(b)
    if (y_of(bb) >= p) != rust_rejects(bb):
        mismatches.append(("random", bb.hex()))
        if len(mismatches) > 5:
            break

print("total mismatches:", len(mismatches))
for tag, hx in mismatches[:6]:
    print(" ", tag, hx)
if not mismatches:
    print("PREDICATE EXACT: rejects exactly y >= p over all tested inputs")
