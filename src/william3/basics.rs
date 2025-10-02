///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// All of the below is copied and slightly adapted from https://github.com/BLAKE3-team/BLAKE3/blob/master/src/lib.rs //
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use core::fmt;

use arrayvec::ArrayString;

/// The number of bytes in a [`Hash`], 32.
pub const OUT_LEN: usize = 32;

/// The number of bytes in a key, 32.
pub const KEY_LEN: usize = 32;

/// The number of bytes in a block, 64.
///
/// You don't usually need to think about this number. One case where it matters is calling
/// [`OutputReader::fill`] in a loop, where using a `buf` argument that's a multiple of `BLOCK_LEN`
/// avoids repeating work.
pub const BLOCK_LEN: usize = 64;

/// The number of bytes in a chunk, 1024.
///
/// You don't usually need to think about this number, but it often comes up in benchmarks, because
/// the maximum degree of parallelism used by the implementation equals the number of chunks.
pub const CHUNK_LEN: usize = 1024;

const MAX_DEPTH: usize = 54; // 2^54 * CHUNK_LEN = 2^64

// While iterating the compression function within a chunk, the CV is
// represented as words, to avoid doing two extra endianness conversions for
// each compression in the portable implementation. But the hash_many interface
// needs to hash both input bytes and parent nodes, so its better for its
// output CVs to be represented as bytes.
pub(crate) type CVWords = [u32; 8];
pub(crate) type CVBytes = [u8; 32]; // little-endian

// These are different for WILLIAM3 than for BLAKE3!
pub(crate) const IV: &CVWords = &[
    0xc88f633b, 0x4168fbf2, 0x6ba32583, 0xb0ff1847, 0xac57e47d, 0xa8931330, 0x796a4645, 0x6b28a3ee,
];

pub(crate) const MSG_SCHEDULE: [[usize; 16]; 7] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
    [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
    [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
    [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
    [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
    [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
];

// These are the internal flags that we use to domain separate root/non-root,
// chunk/parent, and chunk beginning/middle/end. These get set at the high end
// of the block flags word in the compression function, so their values start
// high and go down.
pub(crate) const CHUNK_START: u8 = 1 << 0;
pub(crate) const CHUNK_END: u8 = 1 << 1;
pub(crate) const PARENT: u8 = 1 << 2;
pub(crate) const ROOT: u8 = 1 << 3;
pub(crate) const KEYED_HASH: u8 = 1 << 4;

#[inline]
pub(crate) fn counter_low(counter: u64) -> u32 {
    counter as u32
}

#[inline]
pub(crate) fn counter_high(counter: u64) -> u32 {
    (counter >> 32) as u32
}

// /// An output of the default size, 32 bytes, which provides constant-time
// /// equality checking.
// ///
// /// `Hash` implements [`From`] and [`Into`] for `[u8; 32]`, and it provides
// /// [`from_bytes`] and [`as_bytes`] for explicit conversions between itself and
// /// `[u8; 32]`. However, byte arrays and slices don't provide constant-time
// /// equality checking, which is often a security requirement in software that
// /// handles private data. `Hash` doesn't implement [`Deref`] or [`AsRef`], to
// /// avoid situations where a type conversion happens implicitly and the
// /// constant-time property is accidentally lost.
// ///
// /// `Hash` provides the [`to_hex`] and [`from_hex`] methods for converting to
// /// and from hexadecimal. It also implements [`Display`] and [`FromStr`].
// ///
// /// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
// /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
// /// [`as_bytes`]: #method.as_bytes
// /// [`from_bytes`]: #method.from_bytes
// /// [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
// /// [`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
// /// [`to_hex`]: #method.to_hex
// /// [`from_hex`]: #method.from_hex
// /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
// /// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
// #[derive(Clone, Copy, Hash, Eq)]
// pub struct Hash([u8; OUT_LEN]);

// impl Hash {
//     /// The raw bytes of the `Hash`. Note that byte arrays don't provide
//     /// constant-time equality checking, so if  you need to compare hashes,
//     /// prefer the `Hash` type.
//     #[inline]
//     pub const fn as_bytes(&self) -> &[u8; OUT_LEN] {
//         &self.0
//     }

//     /// Create a `Hash` from its raw bytes representation.
//     pub const fn from_bytes(bytes: [u8; OUT_LEN]) -> Self {
//         Self(bytes)
//     }

//     /// Create a `Hash` from its raw bytes representation as a slice.
//     ///
//     /// Returns an error if the slice is not exactly 32 bytes long.
//     pub fn from_slice(bytes: &[u8]) -> Result<Self, core::array::TryFromSliceError> {
//         Ok(Self::from_bytes(bytes.try_into()?))
//     }

//     /// Encode a `Hash` in lowercase hexadecimal.
//     ///
//     /// The returned [`ArrayString`] is a fixed size and doesn't allocate memory
//     /// on the heap. Note that [`ArrayString`] doesn't provide constant-time
//     /// equality checking, so if you need to compare hashes, prefer the `Hash`
//     /// type.
//     pub fn to_hex(&self) -> ArrayString<{ 2 * OUT_LEN }> {
//         let mut s = ArrayString::new();
//         let table = b"0123456789abcdef";
//         for &b in self.0.iter() {
//             s.push(table[(b >> 4) as usize] as char);
//             s.push(table[(b & 0xf) as usize] as char);
//         }
//         s
//     }

//     /// Decode a `Hash` from hexadecimal. Both uppercase and lowercase ASCII
//     /// bytes are supported.
//     ///
//     /// Any byte outside the ranges `'0'...'9'`, `'a'...'f'`, and `'A'...'F'`
//     /// results in an error. An input length other than 64 also results in an
//     /// error.
//     ///
//     /// Note that `Hash` also implements `FromStr`, so `Hash::from_hex("...")`
//     /// is equivalent to `"...".parse()`.
//     pub fn from_hex(hex: impl AsRef<[u8]>) -> Result<Self, HexError> {
//         fn hex_val(byte: u8) -> Result<u8, HexError> {
//             match byte {
//                 b'A'..=b'F' => Ok(byte - b'A' + 10),
//                 b'a'..=b'f' => Ok(byte - b'a' + 10),
//                 b'0'..=b'9' => Ok(byte - b'0'),
//                 _ => Err(HexError(HexErrorInner::InvalidByte(byte))),
//             }
//         }
//         let hex_bytes: &[u8] = hex.as_ref();
//         if hex_bytes.len() != OUT_LEN * 2 {
//             return Err(HexError(HexErrorInner::InvalidLen(hex_bytes.len())));
//         }
//         let mut hash_bytes: [u8; OUT_LEN] = [0; OUT_LEN];
//         for i in 0..OUT_LEN {
//             hash_bytes[i] = 16 * hex_val(hex_bytes[2 * i])? + hex_val(hex_bytes[2 * i + 1])?;
//         }
//         Ok(Hash::from(hash_bytes))
//     }
// }

// impl From<[u8; OUT_LEN]> for Hash {
//     #[inline]
//     fn from(bytes: [u8; OUT_LEN]) -> Self {
//         Self::from_bytes(bytes)
//     }
// }

// impl From<Hash> for [u8; OUT_LEN] {
//     #[inline]
//     fn from(hash: Hash) -> Self {
//         hash.0
//     }
// }

// impl core::str::FromStr for Hash {
//     type Err = HexError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Hash::from_hex(s)
//     }
// }

// /// This implementation is constant-time.
// impl PartialEq for Hash {
//     #[inline]
//     fn eq(&self, other: &Hash) -> bool {
//         constant_time_eq::constant_time_eq_32(&self.0, &other.0)
//     }
// }

// /// This implementation is constant-time.
// impl PartialEq<[u8; OUT_LEN]> for Hash {
//     #[inline]
//     fn eq(&self, other: &[u8; OUT_LEN]) -> bool {
//         constant_time_eq::constant_time_eq_32(&self.0, other)
//     }
// }

// /// This implementation is constant-time if the target is 32 bytes long.
// impl PartialEq<[u8]> for Hash {
//     #[inline]
//     fn eq(&self, other: &[u8]) -> bool {
//         constant_time_eq::constant_time_eq(&self.0, other)
//     }
// }

// impl fmt::Display for Hash {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Formatting field as `&str` to reduce code size since the `Debug`
//         // dynamic dispatch table for `&str` is likely needed elsewhere already,
//         // but that for `ArrayString<[u8; 64]>` is not.
//         let hex = self.to_hex();
//         let hex: &str = hex.as_str();

//         f.write_str(hex)
//     }
// }

// impl fmt::Debug for Hash {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Formatting field as `&str` to reduce code size since the `Debug`
//         // dynamic dispatch table for `&str` is likely needed elsewhere already,
//         // but that for `ArrayString<[u8; 64]>` is not.
//         let hex = self.to_hex();
//         let hex: &str = hex.as_str();

//         f.debug_tuple("Hash").field(&hex).finish()
//     }
// }

// /// The error type for [`Hash::from_hex`].
// ///
// /// The `.to_string()` representation of this error currently distinguishes between bad length
// /// errors and bad character errors. This is to help with logging and debugging, but it isn't a
// /// stable API detail, and it may change at any time.
// #[derive(Clone, Debug)]
// pub struct HexError(HexErrorInner);

// #[derive(Clone, Debug)]
// enum HexErrorInner {
//     InvalidByte(u8),
//     InvalidLen(usize),
// }

// impl fmt::Display for HexError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self.0 {
//             HexErrorInner::InvalidByte(byte) => {
//                 if byte < 128 {
//                     write!(f, "invalid hex character: {:?}", byte as char)
//                 } else {
//                     write!(f, "invalid hex character: 0x{:x}", byte)
//                 }
//             }
//             HexErrorInner::InvalidLen(len) => {
//                 write!(f, "expected 64 hex bytes, received {}", len)
//             }
//         }
//     }
// }

// #[cfg(feature = "std")]
// impl std::error::Error for HexError {}
