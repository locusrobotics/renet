use chacha20poly1305::aead::{rand_core::RngCore, OsRng};

/// Generate a buffer with random bytes using randomness from the operating system.
///
/// The implementation is provided by the `getrandom` crate. Refer to
/// `getrandom` documentation for details.
pub fn generate_random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0; N];
    OsRng.fill_bytes(&mut bytes);
    bytes
}
