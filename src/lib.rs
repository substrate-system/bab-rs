#![no_std]

// #[cfg(feature = "std")]
// extern crate std;

pub mod generic;

#[cfg(feature = "william3")]
mod william3;
#[cfg(feature = "william3")]
pub use william3::*;
