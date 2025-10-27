//! WebAssembly bindings for the Bab hash function library.

use wasm_bindgen::prelude::*;

#[cfg(feature = "william3")]
use crate::{Hasher, HasherWrite, WIDTH, William3Digest, William3Hasher, batch_hash};

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Hash data using WILLIAM3 in a single batch operation.
/// Returns the hash as a hex string.
#[cfg(feature = "william3")]
#[wasm_bindgen]
pub fn william3_hash(data: &[u8]) -> String {
    let mut digest = William3Digest::default();
    batch_hash(data, &mut digest);
    hex_encode(digest.as_bytes())
}

/// Hash data using WILLIAM3 in a single batch operation.
/// Returns the hash as a byte array.
#[cfg(feature = "william3")]
#[wasm_bindgen]
pub fn william3_hash_bytes(data: &[u8]) -> Vec<u8> {
    let mut digest = William3Digest::default();
    batch_hash(data, &mut digest);
    digest.into_bytes().to_vec()
}

/// A WebAssembly-compatible wrapper for William3Hasher that allows incremental hashing.
#[cfg(feature = "william3")]
#[wasm_bindgen]
pub struct William3HasherWasm {
    hasher: William3Hasher,
}

#[cfg(feature = "william3")]
#[wasm_bindgen]
impl William3HasherWasm {
    /// Create a new WILLIAM3 hasher instance.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            hasher: William3Hasher::new(),
        }
    }

    /// Write data to the hasher incrementally.
    pub fn write(&mut self, data: &[u8]) {
        self.hasher.write(data);
    }

    /// Finalize the hash and return it as a hex string.
    /// This consumes the hasher.
    pub fn finish_hex(self) -> String {
        let digest = self.hasher.finish();
        hex_encode(digest.as_bytes())
    }

    /// Finalize the hash and return it as a byte array.
    /// This consumes the hasher.
    pub fn finish_bytes(self) -> Vec<u8> {
        let digest = self.hasher.finish();
        digest.into_bytes().to_vec()
    }
}

/// Get the digest width (size in bytes) for WILLIAM3.
#[cfg(feature = "william3")]
#[wasm_bindgen]
pub fn william3_width() -> usize {
    WIDTH
}

// Helper function to convert bytes to hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_william3_hash() {
        let data = b"hello world";
        let hash = william3_hash(data);
        assert_eq!(hash.len(), WIDTH * 2); // hex encoding doubles the length
    }

    #[wasm_bindgen_test]
    fn test_william3_hasher_incremental() {
        let mut hasher = William3HasherWasm::new();
        hasher.write(b"hello ");
        hasher.write(b"world");
        let hash = hasher.finish_hex();
        assert_eq!(hash.len(), WIDTH * 2);
    }

    #[wasm_bindgen_test]
    fn test_william3_hasher_matches_batch() {
        let data = b"test data";

        let mut hasher = William3HasherWasm::new();
        hasher.write(data);
        let incremental_hash = hasher.finish_hex();

        let batch_hash = william3_hash(data);

        assert_eq!(incremental_hash, batch_hash);
    }
}
