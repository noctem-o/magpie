"""Independent V_sig checker — Hermes parallel track, A-021 spike.

Implements ADR-0010 V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"
from scratch. No host crypto library used for verdicts.

Gates (in order):
  ExternalKey stage: 32-byte A_bytes, canonical decode, A != I, [L]A == I
  Signature stage:   64 bytes, canonical R decode, [L]R == I (R = I ok),
                     0 <= S < L (no reduction), uncofactored equation.
"""
import hashlib

p = 2**255 - 19
L = 2**252 + 27742317777372353535851937790883648493
d = (-121665 * pow(121666, p - 2, p)) % p

# Identity is a concrete point, never None (representation-normalized).
IDENTITY = (0, 1)

# RFC 8032 base point: By = 4*inv(5), Bx even root.
_By = (4 * pow(5, p - 2, p)) % p


def _recover_x(y, sign):
    """Recover x for curve point with ordinate y and parity sign. None if none."""
    xx = ((y * y - 1) * pow(d * y * y + 1, p - 2, p)) % p
    x = pow(xx, (p + 3) // 8, p)
    if (x * x - xx) % p != 0:
        x = (x * pow(2, (p - 1) // 4, p)) % p
        if (x * x - xx) % p != 0:
            return None
    if x == 0 and sign == 1:
        return None  # x=0 requires sign bit zero (canonical rule)
    if (x & 1) != sign:
        x = p - x
    return x


def _decode(b):
    """Canonical compressed Edwards25519 decode. None on any failure."""
    if not isinstance(b, (bytes, bytearray)) or len(b) != 32:
        return None
    y = int.from_bytes(b, "little") & ((1 << 255) - 1)
    sign = b[31] >> 7
    if y >= p:  # no reduction modulo p
        return None
    x = _recover_x(y, sign)
    if x is None:
        return None
    if _encode((x, y)) != bytes(b):  # byte-identical re-encode
        return None
    return (x, y)


def _encode(pt):
    x, y = pt
    return ((y | ((x & 1) << 255))).to_bytes(32, "little")


def _add(pt1, pt2):
    """Complete Edwards addition (handles identity as (0,1))."""
    x1, y1 = pt1
    x2, y2 = pt2
    x3 = (x1 * y2 + x2 * y1) * pow(1 + d * x1 * x2 * y1 * y2, p - 2, p) % p
    y3 = (y1 * y2 + x1 * x2) * pow(1 - d * x1 * x2 * y1 * y2, p - 2, p) % p
    return (x3, y3)


_B = None


def base_point():
    global _B
    if _B is None:
        bx = _recover_x(_By, 0)
        assert bx is not None and bx & 1 == 0
        _B = (bx, _By)
    return _B


def _mul(n, pt):
    if n < 0:
        raise ValueError("negative scalar")
    result = IDENTITY
    addend = pt
    while n:
        if n & 1:
            result = _add(result, addend)
        addend = _add(addend, addend)
        n >>= 1
    return result


# ---------------------------------------------------------------- V_sig core

def canonical_decode_ok(b32):
    return _decode(b32) is not None


def vsig_verify(A_bytes, signature, M):
    """Return dict(verdict='ACCEPT'|'REJECT', stage, detail). Pure V_sig."""
    if not isinstance(A_bytes, (bytes, bytearray)) or len(A_bytes) != 32:
        return {"verdict": "REJECT", "stage": "ExternalKey", "detail": "key not 32 bytes"}
    A = _decode(A_bytes)
    if A is None:
        return {"verdict": "REJECT", "stage": "ExternalKey", "detail": "non-canonical / undecodable key"}
    if A == IDENTITY:
        return {"verdict": "REJECT", "stage": "ExternalKey", "detail": "A == I"}
    if _mul(L, A) != IDENTITY:
        return {"verdict": "REJECT", "stage": "ExternalKey", "detail": "[L]A != I"}
    if not isinstance(signature, (bytes, bytearray)) or len(signature) != 64:
        return {"verdict": "REJECT", "stage": "Signature", "detail": "signature not 64 bytes"}
    R_bytes = bytes(signature[:32])
    R = _decode(R_bytes)
    if R is None:
        return {"verdict": "REJECT", "stage": "Signature", "detail": "non-canonical / undecodable R"}
    if _mul(L, R) != IDENTITY:
        return {"verdict": "REJECT", "stage": "Signature", "detail": "[L]R != I"}
    S = int.from_bytes(bytes(signature[32:]), "little")
    if not (0 <= S < L):
        return {"verdict": "REJECT", "stage": "Signature", "detail": "S out of range (not canonical)"}
    k = int.from_bytes(hashlib.sha512(R_bytes + bytes(A_bytes) + M).digest(), "little") % L
    lhs = _mul(S, base_point())
    rhs = _add(R, _mul(k, A))
    if lhs != rhs:
        return {"verdict": "REJECT", "stage": "Signature", "detail": "equation false"}
    return {"verdict": "ACCEPT", "stage": None, "detail": "ok"}
