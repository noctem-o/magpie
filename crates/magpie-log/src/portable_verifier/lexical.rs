pub(super) fn decode_lower_hex<const LENGTH: usize>(text: &str) -> Option<[u8; LENGTH]> {
    if text.len() != LENGTH * 2 {
        return None;
    }

    let encoded = text.as_bytes();
    let mut decoded = [0_u8; LENGTH];
    for (index, decoded_byte) in decoded.iter_mut().enumerate() {
        let encoded_index = index * 2;
        let high = lower_hex_nibble(encoded[encoded_index])?;
        let low = lower_hex_nibble(encoded[encoded_index + 1])?;
        *decoded_byte = (high << 4) | low;
    }
    Some(decoded)
}

pub(super) fn decode_u64_token(token: &str) -> Option<u64> {
    let bytes = token.as_bytes();
    let lexical_form_is_valid = match bytes {
        [b'0'] => true,
        [b'1'..=b'9', rest @ ..] => rest.iter().all(u8::is_ascii_digit),
        _ => false,
    };
    lexical_form_is_valid
        .then(|| token.parse::<u64>().ok())
        .flatten()
}

fn lower_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
