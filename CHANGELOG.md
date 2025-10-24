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
