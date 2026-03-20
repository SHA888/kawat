use sha1_smol::Sha1;

/// Generate a SHA-1 content fingerprint, Base64-encoded.
pub fn content_fingerprint(content: &str) -> String {
    let digest = Sha1::from(content).digest();
    // Use URL-safe Base64 without padding (same as trafilatura)
    base64_encode(&digest.bytes())
}

fn base64_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    // Simple Base64 implementation matching Python's b64encode
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        let _ = write!(out, "{}", CHARS[((triple >> 18) & 0x3F) as usize] as char);
        let _ = write!(out, "{}", CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            let _ = write!(out, "{}", CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            let _ = write!(out, "{}", CHARS[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}
