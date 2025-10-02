use crate::generic::SimpleHasher;

pub static WIDTH: usize = 32;
pub static CHUNK_SIZE: usize = 1024;

pub struct SimpleWilliam3Hasher(
    SimpleHasher<
        WIDTH,
        CHUNK_SIZE,
        fn(&[u8], bool) -> [u8; WIDTH],
        fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool) -> [u8; WIDTH],
    >,
);

impl SimpleWilliam3Hasher {
    /// Creates a mew WILLIAM3 hasher.
    pub fn new() -> Self {
        Self(SimpleHasher::new(hash_chunk, hash_inner))
    }

    /// Writes some data into the given Hasher.
    pub fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    /// Returns the digest for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
    pub fn finish(&self) -> [u8; WIDTH] {
        self.0.finish()
    }
}

fn hash_chunk(chunk: &[u8], is_root: bool) -> [u8; WIDTH] {
    todo!()
}

fn hash_inner(
    left_label: &[u8; WIDTH],
    right_label: &[u8; WIDTH],
    length_of_subtree: u64,
    is_root: bool,
) -> [u8; WIDTH] {
    todo!()
}

// These are different for WILLIAM3 than for BLAKE3.
static IV: [u32; 8] = [
    0xc88f633b, 0x4168fbf2, 0x6ba32583, 0xb0ff1847, 0xac57e47d, 0xa8931330, 0x796a4645, 0x6b28a3ee,
];

fn compression_function(
    h: [u32; 8],  // input chaining value
    m: [u32; 16], // message blocks
    // Careful: works differently in WILLIAM3 than in BLAKE3.
    // Also careful: little-endian!
    t: [u32; 2], // counter (leaf size sum)
    b: u32,      // number of input bytes in the block
    d: u32,      // domain-separation flags
) -> [u8; WIDTH] {
    // Initial internal state.
    let mut v: [u32; 16] = [
        h[0], h[1], h[2], h[3], //
        h[4], h[5], h[6], h[7], //
        IV[0], IV[1], IV[2], IV[3], //
        t[0], t[1], b, d,
    ];

    // Do a silly dance to keep rust happy. We cannot directly pass mutable references to multiple
    // state words into the `G` function, unless we use `split_at_mut` (or do unsafe stuff).
    let rest = &mut v[..];
    let (v0, rest) = rest.split_at_mut(1);
    let (v1, rest) = rest.split_at_mut(1);
    let (v2, rest) = rest.split_at_mut(1);
    let (v3, rest) = rest.split_at_mut(1);
    let (v4, rest) = rest.split_at_mut(1);
    let (v5, rest) = rest.split_at_mut(1);
    let (v6, rest) = rest.split_at_mut(1);
    let (v7, rest) = rest.split_at_mut(1);
    let (v8, rest) = rest.split_at_mut(1);
    let (v9, rest) = rest.split_at_mut(1);
    let (v10, rest) = rest.split_at_mut(1);
    let (v11, rest) = rest.split_at_mut(1);
    let (v12, rest) = rest.split_at_mut(1);
    let (v13, rest) = rest.split_at_mut(1);
    let (v14, rest) = rest.split_at_mut(1);
    let (v15, _rest) = rest.split_at_mut(1);

    let v0 = v0.get_mut(0).unwrap();
    let v1 = v1.get_mut(0).unwrap();
    let v2 = v2.get_mut(0).unwrap();
    let v3 = v3.get_mut(0).unwrap();
    let v4 = v4.get_mut(0).unwrap();
    let v5 = v5.get_mut(0).unwrap();
    let v6 = v6.get_mut(0).unwrap();
    let v7 = v7.get_mut(0).unwrap();
    let v8 = v8.get_mut(0).unwrap();
    let v9 = v9.get_mut(0).unwrap();
    let v10 = v10.get_mut(0).unwrap();
    let v11 = v11.get_mut(0).unwrap();
    let v12 = v12.get_mut(0).unwrap();
    let v13 = v13.get_mut(0).unwrap();
    let v14 = v14.get_mut(0).unwrap();
    let v15 = v15.get_mut(0).unwrap();

    // Run seven rounds of keyed permutations.
    for i in 0..7 {
        // First apply `G` to each column.
        G(v0, v4, v8, v12, m[0], m[1]);
        G(v1, v5, v9, v13, m[2], m[3]);
        G(v2, v6, v10, v14, m[4], m[5]);
        G(v3, v7, v11, v15, m[6], m[7]);

        // Then apply `G` to each diagonal.
        G(v0, v5, v10, v15, m[8], m[9]);
        G(v1, v6, v11, v12, m[10], m[11]);
        G(v2, v7, v8, v13, m[12], m[13]);
        G(v3, v4, v9, v14, m[14], m[15]);

        // After each but the final round, permute the message words.
        if i != 6 {
            let tmp_m = m;
            m[0] = tmp_m[2];
            todo!()
        }
    }

    // Return the proper output.
    todo!()
}

#[allow(non_snake_case)]
fn G(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32, m_2i_plus_0: u32, m_2i_plus_1: u32) {
    todo!()
}
