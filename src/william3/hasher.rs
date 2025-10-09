use crate::{
    CHUNK_SIZE, HashChunkContext, HashInnerContext, WIDTH, generic::Hasher as GenericHasher,
    hash_chunk, hash_inner,
};

/// A stateful hasher for incrementally computing WILLIAM3 digests.
///
/// ```
/// # #[cfg(feature = "william3")] {
/// use bab_rs::{Hasher, batch_hash, WIDTH};
/// let mut hasher = Hasher::new();
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
pub struct Hasher {
    hasher: GenericHasher<WIDTH, CHUNK_SIZE, HashChunkContext, HashInnerContext>,
}

impl Hasher {
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

    /// Writes some data into the given Hasher.
    pub fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }

    /// Returns the digest for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
    pub fn finish(&self) -> [u8; WIDTH] {
        self.hasher.finish()
    }
}

impl Default for Hasher {
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

        let mut hasher = Hasher::new();
        hasher.write(&data[..len]);
        let digest_hasher = hasher.finish();

        assert_eq!(digest_hasher, digest_batch);
    }
}
