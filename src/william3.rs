use crate::generic::SimpleHasher;

pub static WIDTH: usize = 32;
pub static CHUNK_SIZE: usize = 1024;

pub struct SimpleWilliam3Hasher(
    SimpleHasher<
        WIDTH,
        CHUNK_SIZE,
        fn(&[u8], bool) -> [u8; WIDTH],
        fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool) -> [u8; WIDTH],
    >,
);

impl SimpleWilliam3Hasher {
    /// Creates a mew WILLIAM3 hasher.
    pub fn new() -> Self {
        Self(SimpleHasher::new(hash_chunk, hash_inner))
    }

    /// Writes some data into the given Hasher.
    pub fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    /// Returns the digest for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
    pub fn finish(&self) -> [u8; WIDTH] {
        self.0.finish()
    }
}

fn hash_chunk(chunk: &[u8], is_root: bool) -> [u8; WIDTH] {
    todo!()
}

fn hash_inner(
    left_label: &[u8; WIDTH],
    right_label: &[u8; WIDTH],
    length_of_subtree: u64,
    is_root: bool,
) -> [u8; WIDTH] {
    todo!()
}

// The input chaining value, h0 ... h7 (256 bits).
// • The message block, m0 ... m15 (512 bits).
// • A 64-bit counter, t = t0, t1, with t0 the lower order word and t1 the higher order word.
// • The number of input bytes in the block, b (32 bits).
// • A set of domain separation bit flags, d (32 bits)

fn compression_function(
    input_chaining_value: [u32; 8],
    message_block: [u32; 16],
    counter: [u32; 2],
    input_len_in_bytes: u32,
    domain_separation_flags: u32,
) -> [u8; WIDTH] {
    todo!()
}
