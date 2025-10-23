use anyhash::{Hasher, HasherWrite};

use crate::{
    CHUNK_SIZE, HashChunkContext, HashInnerContext, WIDTH, generic::BabHasher as GenericHasher,
    hash_chunk, hash_inner,
};

/// A stateful hasher for incrementally computing WILLIAM3 digests.
///
/// Use the [`anyhash::Hasher`] and [`anyhash::HasherWrite`] traits to compute digests. This crate reexports them at the root for convenience.
///
/// ```
/// # #[cfg(feature = "william3")] {
/// use bab_rs::{William3Hasher, batch_hash, WIDTH, Hasher, HasherWrite};
/// let mut hasher = William3Hasher::new();
/// hasher.write(&[0, 1, 2]);
/// hasher.write(&[3, 4]);
/// let digest1 = hasher.finish();
///
/// let mut batch_digest1 = [0; WIDTH];
/// batch_hash(&[0, 1, 2, 3, 4], &mut batch_digest1);
///
/// assert_eq!(digest1, batch_digest1);
///
/// // You can continue using the hasher after calling `finish`.
/// hasher.write(&[5, 6]);
///
/// let mut batch_digest2 = [0; WIDTH];
/// batch_hash(&[0, 1, 2, 3, 4, 5, 6], &mut batch_digest2);
///
/// assert_eq!(
///     hasher.finish(),
///     batch_digest2,
/// );
/// # }
/// ```
pub struct William3Hasher {
    hasher: GenericHasher<WIDTH, CHUNK_SIZE, HashChunkContext, HashInnerContext>,
}

impl William3Hasher {
    /// Creates a new WILLIAM3 hasher.
    pub fn new() -> Self {
        Self {
            hasher: GenericHasher::new(
                hash_chunk,
                hash_inner,
                HashChunkContext::new(),
                HashInnerContext::new(),
            ),
        }
    }

    /// Creates a new WILLIAM3 hasher for keyed hashing.
    pub fn new_keyed(key: [u32; 8]) -> Self {
        Self {
            hasher: GenericHasher::new(
                hash_chunk,
                hash_inner,
                HashChunkContext::new_keyed(key),
                HashInnerContext::new_keyed(key),
            ),
        }
    }
}

impl HasherWrite for William3Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

impl Hasher<[u8; WIDTH]> for William3Hasher {
    fn finish(&self) -> [u8; WIDTH] {
        self.hasher.finish()
    }
}

impl Default for William3Hasher {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_hasher() {
    let data = [17u8; CHUNK_SIZE * 16];

    for len in 0..data.len() {
        let mut digest_batch = [0; WIDTH];
        crate::batch_hash(&data[..len], &mut digest_batch);

        let mut hasher = William3Hasher::new();
        hasher.write(&data[..len]);
        let digest_hasher = hasher.finish();

        assert_eq!(digest_hasher, digest_batch);
    }
}
