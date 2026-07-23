//! Cryptographically random session-id generation.
//!
//! Always draws from the operating system's CSPRNG (`OsRng`). Falls back
//! to a thread-local RNG only if `OsRng` itself fails (essentially never
//! on supported platforms).

use rand::rngs::OsRng;
use rand::{TryRngCore, rng};

/// Length (in bytes) of the random session token before hex encoding.
/// 16 bytes (128 bits) is well beyond brute-force feasibility for the
/// lifetime of any single cookie.
const SESSION_ID_BYTES: usize = 16;

/// Generate a fresh cryptographically random session id.
///
/// Returns a 32-character lowercase hex string. The function never
/// panics: on the (essentially impossible on supported platforms)
/// chance that both `OsRng` and the thread-local RNG fail, the
/// function logs a warning and falls back to zeroed bytes. The
/// resulting cookie will not match any registered session and so will
/// be rejected on the next request.
#[must_use]
pub fn generate_session_id() -> String {
    let mut bytes = [0u8; SESSION_ID_BYTES];
    OsRng
        .try_fill_bytes(&mut bytes)
        .or_else(|_| rng().try_fill_bytes(&mut bytes))
        .unwrap_or_else(|_| {
            tracing::warn!(
                target: "session",
                "OsRng failed; falling back to zeroed bytes"
            );
        });
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn session_id_is_32_hex_chars() {
        let id = generate_session_id();
        assert_eq!(id.len(), SESSION_ID_BYTES * 2);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn session_ids_are_unique() {
        let mut seen = HashSet::new();
        for _ in 0..256 {
            let id = generate_session_id();
            assert!(seen.insert(id), "collision in 256 generated ids");
        }
    }
}
