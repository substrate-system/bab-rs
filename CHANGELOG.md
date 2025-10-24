## 0.4.2

Derive `core::hash::Hash` on `BabDigest` and `William3Digest`.

## 0.4.1

Add the `dev` feature. When it is enabled, `BabDigest` and `William3Digest`
implement the `Arbitrary` trait.

# 0.4.0

Remove the `impl<const WIDTH: usize> From<BabDigest<WIDTH>> for [u8; WIDTH]` and
`impl From<William3Digest> for [u8; WIDTH]` impls, to make it less likely that
users accidentally sidestep constant-time-equality checks and/or zero-on-drop.
Replaces these with non-trait-backed `into_bytes`, `as_bytes`, and
`as_mut_bytes` methods.

# 0.3.0

Introduce proper wrapper types for digests: `BabDigest` and `William3Digest`.
Implement constant-time equality comparisons and zero-on-drop on them. Thank you
@Miaourt for implementing these!

# 0.2.0

- Rename `generic::Hasher` to `generic::BabHasher`, and `william3::Hasher` to
  `william3::William3Hasher`.
- Implement the traits of the [`anyhash`](https://crates.io/crates/anyhash)
  crates, and remove the old non-trait methods of the same names.

# 0.1.0

Initial release.
