use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::traits::IsIdentity;

use curve25519_dalek::Scalar;

fn probe(name: &str, bytes: [u8; 32]) {
    let c = CompressedEdwardsY(bytes);
    match c.decompress() {
        Some(pt) => println!("{name}: DECOMPRESSED -> {:02x?}", pt.compress().to_bytes()),
        None => println!("{name}: NONE"),
    }
}

fn hex32(h: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap(); }
    out
}

fn main() {
    probe("y=p (k9)", hex32("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"));
    probe("y=p+1 (k10)", hex32("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"));
    probe("identity canonical", hex32("0100000000000000000000000000000000000000000000000000000000000000"));
    probe("identity noncanonical sign", hex32("0100000000000000000000000000000000000000000000000000000000000080"));
    let bpb = hex32("5866666666666666666666666666666666666666666666666666666666666666");
    probe("basepoint", bpb);
    let bpt = CompressedEdwardsY(bpb).decompress().unwrap();
    let l = Scalar::from_bytes_mod_order(hex32("edd3f55c1a631258d69cf7a2def9de1400000000000000000000000000000010"));
    let lbp: EdwardsPoint = bpt * l;
    println!("[L]B is_identity: {}", lbp.is_identity());
    let p2 = hex32("5c4d1a631258d69c8a69b5316948114a3a3f97366663e8698a0808314b50479e");
    if let Some(pt) = CompressedEdwardsY(p2).decompress() {
        let lpt: EdwardsPoint = pt * l;
        println!("[L](0,121666) is_identity: {}", lpt.is_identity());
    }
    // mixed torsion: B + T4
    let t4 = hex32("0200000000000000000000000000000000000000000000000000000000000000");
    if let Some(pt4) = CompressedEdwardsY(t4).decompress() {
        let mixed: EdwardsPoint = bpt + pt4;
        let lm: EdwardsPoint = mixed * l;
        println!("[L](B+T4) is_identity: {}", lm.is_identity());
        println!("(B+T4) compressed: {:02x?}", mixed.compress().to_bytes());
    }
    // S check helper: L as LE bytes
    let l_bytes = [
        0x9ad9d4ed92d16e32u64, 0x0b8568366cd44bc8u64, 0u64, 0x1u64,
    ];
    println!("L low u64 = 0x{:x}", l_bytes[0]);
}
