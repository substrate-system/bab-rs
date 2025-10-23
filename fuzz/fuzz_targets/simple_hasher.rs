#![no_main]
use libfuzzer_sys::fuzz_target;

use bab_rs::{Hasher, HasherWrite, WIDTH, William3Hasher, batch_hash, batch_hash_keyed};

fuzz_target!(|data: (Vec<u8>, Option<[u32; 8]>, Vec<usize>)| {
    let (input_bytes, key, mut write_sizes) = data;
    if !write_sizes.iter().any(|size| *size > 0) {
        write_sizes.push(173);
    }

    let mut digest_batch = [0; WIDTH];

    match key {
        Some(key) => batch_hash_keyed(&input_bytes[..], key, &mut digest_batch),
        None => batch_hash(&input_bytes[..], &mut digest_batch),
    }

    let mut hasher = match key {
        Some(key) => William3Hasher::new_keyed(key),
        None => William3Hasher::new(),
    };

    let mut write_size_index = 0;
    let mut written_so_far = 0;
    loop {
        if written_so_far == input_bytes.len() {
            break;
        }

        let slice_len = core::cmp::min(
            write_sizes[write_size_index],
            input_bytes.len() - written_so_far,
        );

        hasher.write(&input_bytes[written_so_far..written_so_far + slice_len]);
        written_so_far += slice_len;

        write_size_index += 1;
        if write_size_index == write_sizes.len() {
            write_size_index = 0;
        }
    }

    let digest_hasher = hasher.finish();

    assert_eq!(digest_hasher, digest_batch);
});
