use crate::{
    generic::SimpleHasher,
    william3::{
        basics::{BLOCK_LEN, CHUNK_END, CHUNK_START, IV, KEYED_HASH, PARENT, ROOT},
        portable::hash1,
    },
};

mod basics;
mod portable;

pub const WIDTH: usize = 32;
pub const CHUNK_SIZE: usize = 1024;

pub struct SimpleWilliam3Hasher {
    simple_hasher: SimpleHasher<WIDTH, CHUNK_SIZE, HashChunkInfo, HashInnerInfo>,
}

impl SimpleWilliam3Hasher {
    /// Creates a new WILLIAM3 hasher.
    pub fn new() -> Self {
        Self {
            simple_hasher: SimpleHasher::new(
                hash_chunk,
                hash_inner,
                HashChunkInfo { key: None },
                HashInnerInfo { key: None },
            ),
        }
    }

    /// Creates a new WILLIAM3 hasher for keyed hashing.
    pub fn new_keyed(key: [u32; 8]) -> Self {
        Self {
            simple_hasher: SimpleHasher::new(
                hash_chunk,
                hash_inner,
                HashChunkInfo { key: Some(key) },
                HashInnerInfo { key: Some(key) },
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

struct HashChunkInfo {
    key: Option<[u32; 8]>,
}

fn hash_chunk(chunk: &[u8], is_root: bool, state: &HashChunkInfo, output: &mut [u8; WIDTH]) {
    let mut flags = 0;
    if state.key.is_some() {
        flags |= KEYED_HASH
    }

    let flags_start = CHUNK_START;

    let mut flags_end = CHUNK_END;
    if is_root {
        flags_end |= ROOT;
    }

    hash1(
        chunk,
        state.key.as_ref().unwrap_or(IV),
        0,
        flags,
        flags_start,
        flags_end,
        output,
    );
}

struct HashInnerInfo {
    key: Option<[u32; 8]>,
}

fn hash_inner(
    left_label: &[u8; WIDTH],
    right_label: &[u8; WIDTH],
    length_of_subtree: u64,
    is_root: bool,
    state: &HashInnerInfo,
    output: &mut [u8; WIDTH],
) {
    let mut flags = PARENT;
    if is_root {
        flags |= ROOT
    }

    let flags_start = 0;
    let flags_end = 0;

    let mut message_words = [0; BLOCK_LEN];
    (&mut message_words[..WIDTH]).copy_from_slice(left_label);
    (&mut message_words[WIDTH..]).copy_from_slice(right_label);

    hash1(
        &message_words[..],
        state.key.as_ref().unwrap_or(IV),
        length_of_subtree,
        flags,
        flags_start,
        flags_end,
        output,
    );
}
