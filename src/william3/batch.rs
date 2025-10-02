use crate::{CHUNK_SIZE, HashChunkContext, HashInnerContext, WIDTH, hash_chunk, hash_inner};

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
