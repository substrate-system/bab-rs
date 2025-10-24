#![no_std]

//! An implementation of the [Bab](https://worm-blossom.github.io/bab/) family of hash functions.
//!
//! The crate root exposes the [WILLIAM3](https://worm-blossom.github.io/bab/#instantiations_william) instantiation of Bab, which is a concrete hash function you can use immediately. The [`generic`] module provides parmaterisable implementations of Bab, which you can use to define your own hash functions.
//!
//! ```
//! # #[cfg(feature = "william3")] {
//! use bab_rs::{batch_hash, William3Hasher, William3Digest, WIDTH, Hasher, HasherWrite};
//! let mut hasher = William3Hasher::new();
//! hasher.write(&[0, 1, 2]);
//! hasher.write(&[3, 4]);
//! let incrementally_computed_hash = hasher.finish();
//!
//! let mut batch_digest = William3Digest::default();
//! batch_hash(&[0, 1, 2, 3, 4], &mut batch_digest);
//!
//! assert_eq!(
//!     hasher.finish(),
//!     batch_digest,
//! );
//! # }
//! ```

pub mod generic;

pub use anyhash::{Hasher, HasherWrite};

#[cfg(feature = "william3")]
mod william3;
#[cfg(feature = "william3")]
pub use william3::*;
