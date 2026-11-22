//! Synthetic corpus helpers. Fixtures MUST NOT contain real PII.

#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};

/// Marker that corpus generation is available.
pub fn corpus_policy() -> &'static str {
    "synthetic-only; no real PII"
}

/// SHA-256 hex of bytes (immutability checks).
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Minimal one-page PDF containing the given ASCII text (Helvetica).
/// Suitable for synthetic PAN samples — not a production generator.
pub fn minimal_text_pdf(text: &str) -> Vec<u8> {
    let escaped = text.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)");
    let stream = format!("BT /F1 24 Tf 72 720 Td ({escaped}) Tj ET");
    let stream_len = stream.len();

    let objects = [
        "1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n".to_string(),
        "2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj\n".to_string(),
        "3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>endobj\n".to_string(),
        format!("4 0 obj<< /Length {stream_len} >>stream\n{stream}\nendstream\nendobj\n"),
        "5 0 obj<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>endobj\n".to_string(),
    ];

    let mut body: Vec<u8> = b"%PDF-1.4\n%".to_vec();
    body.extend_from_slice(&[0xE2, 0xE3, 0xCF, 0xD3, b'\n']);
    let mut offsets = vec![0usize];
    for obj in &objects {
        offsets.push(body.len());
        body.extend_from_slice(obj.as_bytes());
    }
    let xref_pos = body.len();
    let mut trailer = format!("xref\n0 {}\n", objects.len() + 1);
    trailer.push_str("0000000000 65535 f \n");
    for off in offsets.iter().skip(1) {
        trailer.push_str(&format!("{off:010} 00000 n \n"));
    }
    trailer.push_str(&format!(
        "trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_pos}\n%%EOF\n",
        objects.len() + 1
    ));
    body.extend_from_slice(trailer.as_bytes());
    body
}

/// Synthetic PAN sample PDF bytes + expected PAN token.
pub fn text_pan_sample() -> (Vec<u8>, &'static str) {
    let pan = "AAAPA1234A";
    let pdf = minimal_text_pdf(&format!("Client PAN {pan} confidential"));
    (pdf, pan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pan_sample_is_pdf() {
        let (bytes, pan) = text_pan_sample();
        assert!(bytes.starts_with(b"%PDF"));
        assert_eq!(pan, "AAAPA1234A");
        assert_eq!(sha256_hex(&bytes).len(), 64);
    }
}
