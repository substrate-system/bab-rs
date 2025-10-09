mod basics;
mod portable;

mod hasher;
pub use hasher::Hasher;

mod batch;
pub use batch::{batch_hash, batch_hash_keyed};

use crate::william3::{
    basics::{BLOCK_LEN, CHUNK_END, CHUNK_START, IV, KEYED_HASH, PARENT, ROOT},
    portable::hash1,
};

/// The number of bytes in a WILLIAM3 digest.
pub const WIDTH: usize = 32;
pub(crate) const CHUNK_SIZE: usize = 1024;

pub(crate) struct HashChunkContext {
    key: Option<[u32; 8]>,
}

impl HashChunkContext {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn new_keyed(key: [u32; 8]) -> Self {
        Self { key: Some(key) }
    }
}

pub(crate) fn hash_chunk(
    chunk: &[u8],
    is_root: bool,
    state: &HashChunkContext,
    output: &mut [u8; WIDTH],
) {
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

pub(crate) struct HashInnerContext {
    key: Option<[u32; 8]>,
}

impl HashInnerContext {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn new_keyed(key: [u32; 8]) -> Self {
        Self { key: Some(key) }
    }
}

pub(crate) fn hash_inner(
    left_label: &[u8; WIDTH],
    right_label: &[u8; WIDTH],
    length_of_subtree: u64,
    is_root: bool,
    state: &HashInnerContext,
    output: &mut [u8; WIDTH],
) {
    let mut flags = PARENT;
    if is_root {
        flags |= ROOT
    }

    let flags_start = 0;
    let flags_end = 0;

    let mut message_words = [0; BLOCK_LEN];
    message_words[..WIDTH].copy_from_slice(left_label);
    message_words[WIDTH..].copy_from_slice(right_label);

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
