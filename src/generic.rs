//! A generic implementation of the [Bab](https://worm-blossom.github.io/bab/) family of hash functions.

/// An instantiation of the `hash_chunk` spec parameter, with immutable access to a value of type `HashChunkContext`.
/// The computed tree node label must be written into the final argument.
pub type HashChunk<const WIDTH: usize, HashChunkContext> =
    fn(&[u8], bool, &HashChunkContext, &mut [u8; WIDTH]);

/// An instantiation of the `hash_inner` spec parameter, with immutable access to a value of type `HashInnerContext`.
/// The computed tree node label must be written into the final argument.
pub type HashInner<const WIDTH: usize, HashInnerContext> =
    fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool, &HashInnerContext, &mut [u8; WIDTH]);

mod hasher;
pub use hasher::Hasher;

mod batch;
pub use batch::batch_hash;
