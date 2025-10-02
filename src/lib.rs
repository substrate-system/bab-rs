#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod generic;
mod william3;
pub use william3::*;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
