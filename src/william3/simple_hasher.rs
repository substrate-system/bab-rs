use crate::{
    CHUNK_SIZE, HashChunkContext, HashInnerContext, WIDTH, generic::SimpleHasher as GenericHasher,
    hash_chunk, hash_inner,
};

pub struct SimpleHasher {
    simple_hasher: GenericHasher<WIDTH, CHUNK_SIZE, HashChunkContext, HashInnerContext>,
}

impl SimpleHasher {
    /// Creates a new WILLIAM3 hasher.
    pub fn new() -> Self {
        Self {
            simple_hasher: GenericHasher::new(
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
            simple_hasher: GenericHasher::new(
                hash_chunk,
                hash_inner,
                HashChunkContext::new_keyed(key),
                HashInnerContext::new_keyed(key),
            ),
        }
    }

    /// Writes some data into the given Hasher.
    pub fn write(&mut self, bytes: &[u8]) {
        self.simple_hasher.write(bytes)
    }

    /// Returns the digest for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
    pub fn finish(&self) -> [u8; WIDTH] {
        self.simple_hasher.finish()
    }
}
