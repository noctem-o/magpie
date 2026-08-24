pub(super) fn decode_lower_hex<const LENGTH: usize>(text: &str) -> Option<[u8; LENGTH]> {
    if text.len() != LENGTH * 2 {
        return None;
    }

    let mut decoded = [0_u8; LENGTH];
    for (index, pair) in text.as_bytes().chunks_exact(2).enumerate() {
        let high = lower_hex_nibble(pair[0])?;
        let low = lower_hex_nibble(pair[1])?;
        decoded[index] = (high << 4) | low;
    }
    Some(decoded)
}

fn lower_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
