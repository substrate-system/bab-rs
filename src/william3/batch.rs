use crate::{CHUNK_SIZE, HashChunkContext, HashInnerContext, WIDTH, hash_chunk, hash_inner};

/// Computes the WILLIAM3 digest of the given input bytes, and writes it into `out`.
///
/// This is the simplemost hashing API; it requires the full string to be available at once. See the [`Hasher`](crate::Hasher) API for incremental hashing.
pub fn batch_hash(bytes: &[u8], out: &mut [u8; WIDTH]) {
    let (hash_chunk_context, hash_inner_context) =
        (HashChunkContext::new(), HashInnerContext::new());

    crate::generic::batch_hash::<WIDTH, CHUNK_SIZE, _, _>(
        hash_chunk,
        hash_inner,
        &hash_chunk_context,
        &hash_inner_context,
        bytes,
        out,
    );
}

/// Computes the keyed WILLIAM3 digest of the given input bytes for a given `key`, and writes the digest into `out`.
///
/// This is the simplemost hashing API; it requires the full string to be available at once. See the [`Hasher`](crate::Hasher) API for incremental hashing.
pub fn batch_hash_keyed(bytes: &[u8], key: [u32; 8], out: &mut [u8; WIDTH]) {
    let (hash_chunk_context, hash_inner_context) = (
        HashChunkContext::new_keyed(key),
        HashInnerContext::new_keyed(key),
    );

    crate::generic::batch_hash::<WIDTH, CHUNK_SIZE, _, _>(
        hash_chunk,
        hash_inner,
        &hash_chunk_context,
        &hash_inner_context,
        bytes,
        out,
    );
}
