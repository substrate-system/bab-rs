use core::cmp::min;
use std::println;

use crate::{CHUNK_SIZE, WIDTH};

use super::{HashChunk, HashInner};

pub struct SimpleHasher<
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    HashChunkContext,
    HashInnerContext,
> {
    /// The `hash_chunk` spec parameter.
    hash_chunk: HashChunk<WIDTH, HashChunkContext>,
    /// The `hash_inner` spec parameter.
    hash_inner: HashInner<WIDTH, HashInnerContext>,
    hash_chunk_state: HashChunkContext,
    hash_inner_state: HashInnerContext,
    /// How many bytes of input have we processed so far?
    len: u64,
    /// Intuitively, this array stores the label of the rightmost vertex of each tree layer which will never change again. More precisely:
    ///
    /// At index zero, we store the label of the previously processed chunk (when current_chunk_len would reach CHUNK_SIZE, we reset it to zero and update the label at index zero). At index one, we store the root label of the rightmost complete subtree on 2^1 leaves. At index three, we store the root label of the rightmost complete subtree on 2^2 leaves. And so on (index zero does indeed store the root label of the rightmost complete subtree on 2^0, this is the same as the previous chunk).
    ///
    /// These label computations always assume that the `is_root` flag is false, since this array is primarily used for containing temporary data for internal computations, not for actual digest computation.
    right_frontier: [[u8; WIDTH]; 64],
    /// The label of the largest complete subtree we built so far, with is_root set to true in its computations.
    /// We store this because we need to return it when asked for a digest after having ingested exactly the chunks for a complete tree and not a single byte more.
    complete_root_label: [u8; WIDTH],
    /// The incomplete chunk of data we are currently receiving. Once we complete the chunk, we update the `right_frontier` and can forget (i.e., later overwrite) the actual chunk data.
    current_chunk: [u8; CHUNK_SIZE],
    /// How many bytes of the `current_chunk` we have already received. Reset to zero once we complete a chunk.
    current_chunk_len: usize,
}

impl<const WIDTH: usize, const CHUNK_SIZE: usize, HashChunkContext, HashInnerContext>
    SimpleHasher<WIDTH, CHUNK_SIZE, HashChunkContext, HashInnerContext>
{
    /// Creates a mew bab hasher, using the given `hash_chunk` and `hash_inner` functions.
    pub fn new(
        hash_chunk: fn(&[u8], bool, &HashChunkContext, &mut [u8; WIDTH]),
        hash_inner: fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool, &HashInnerContext, &mut [u8; WIDTH]),
        hash_chunk_context: HashChunkContext,
        hash_inner_context: HashInnerContext,
    ) -> Self {
        Self {
            hash_chunk,
            hash_inner,
            hash_chunk_state: hash_chunk_context,
            hash_inner_state: hash_inner_context,
            len: 0,
            right_frontier: [[0; WIDTH]; 64],
            complete_root_label: [0; WIDTH],
            current_chunk: [0; CHUNK_SIZE],
            current_chunk_len: 0,
        }
    }

    /// Writes some data into the given Hasher.
    pub fn write(&mut self, bytes: &[u8]) {
        // The logic for updating our state when adding new bytes is pretty simple while stying within the same `current_chunk`, and then we need to do some extra work once we finish the current chunk.
        // To not have to handle too many cases (e.g. an input whose length is seven times the chunk length), we split up the input bytes into slices which do not extend across chunk boundaries, and feed those successively to [`self.progress_or_complete_current_chunk`].
        let mut remaining = bytes;

        while remaining.len() > 0 {
            let len_to_complete_current_chunk =
                min(remaining.len(), CHUNK_SIZE - self.current_chunk_len);
            self.progress_or_complete_current_chunk(&remaining[..len_to_complete_current_chunk]);
            remaining = &remaining[len_to_complete_current_chunk..];
        }
    }

    /// Writes some data into the hasher. The data does not extend across chunk boundaries. It *may* finish the `current_chunk` though.
    fn progress_or_complete_current_chunk(&mut self, bytes: &[u8]) {
        debug_assert!(self.current_chunk_len + bytes.len() <= CHUNK_SIZE);

        self.len += bytes.len() as u64;

        let start = self.current_chunk_len;
        let end = start + bytes.len();
        self.current_chunk[start..end].copy_from_slice(bytes);
        self.current_chunk_len += bytes.len();

        if self.current_chunk_len == CHUNK_SIZE {
            // Oh no, a chunk was completed; real work lies ahead.
            debug_assert!(
                self.number_of_completed_chunks() > 0,
                "self.len {:?}, self.current_chunk_len {:?}, bytes.len {:?}",
                self.len,
                self.current_chunk_len,
                bytes.len()
            );

            let chunk_count = self.number_of_completed_chunks(); // Includes the chunk we just completed.

            // First, we update the `right_frontier`.
            // We always update its index zero, because the chunk we just processed is now the rightmost complete subtree on 2^0 leaves.
            // Then, we check whether we also just completed a subtree on 2^1. If no, then we are done. If we did, then we update
            // index one, and check whether we also just completed a subtree on 2^2. And so on, until we reached a height for
            // which we did not just complete a subtree.

            // The number of subtrees we completed is always one plus the number of trailing zeros in the binary representation of `chunk_count`.
            let num_completed_subtrees = (chunk_count.trailing_zeros() + 1) as usize;

            let mut left_label = [0; WIDTH];
            for exponent in 0..num_completed_subtrees {
                self.update_individual_frontier_index(exponent, &mut left_label);
            }

            // Finally, reset the chunk len.
            self.current_chunk_len = 0;
        }
    }

    /// Updates `self.right_frontier[exponent]` with the label of the complete subtree over the `2^exponent` most recent chunks, with is_root = false. For `exponent == 0`, this is a simple chunk label computation. Otherwise, the inner label is computed from two child labels. The right child label can be simply looked up as `self.right_frontier[exponent - 1]`. The left label is passed to this function via the `left_label` argument. It replaces the contents of `left_label` with the value at `self.right_frontier[exponent]` (before overwriting it). This allows us to pass it as the next right label to the next call of this function with `exponent + 1` (*if* we make such a call at all).
    ///
    /// For `exponent == 0`, the `left_label` argument will be ignored.
    fn update_individual_frontier_index(&mut self, exponent: usize, left_label: &mut [u8; WIDTH]) {
        // First, cache the old value at `self.right_frontier[exponent]`, because we need to write that to `left_label` before returning.
        let next_left_label = self.right_frontier[exponent];

        // If we are updating a label which happens to cover the *full* input so far, we also compute the label with `is_root = true` and
        // buffer it explicitly.
        let update_root_label = self.number_of_completed_chunks() == 1 << exponent;

        if exponent == 0 {
            // If `exponent == 0`, we need to compute the label of a leaf. Easy.
            (self.hash_chunk)(
                &self.current_chunk[..self.current_chunk_len],
                false,
                &self.hash_chunk_state,
                &mut self.right_frontier[exponent],
            );

            if update_root_label {
                (self.hash_chunk)(
                    &self.current_chunk[..self.current_chunk_len],
                    true,
                    &self.hash_chunk_state,
                    &mut self.complete_root_label,
                );
            }
        } else {
            // Else, we need to compute an inner label.
            // We can compute it from the `left_label` and the new label of the most recent complete
            // tree on `exponent - 1` leaves - which is already stored in the `right_frontier` array,
            // courtesy of the prior invocation of this method.

            // Since we are working with completed chunks only, the length of the tree we are labelling is
            // the CHUNK_SIZE times the number of its leaves.
            let tree_len = (CHUNK_SIZE as u64) * (1 << exponent);

            let mut new_label = [0; WIDTH];
            (self.hash_inner)(
                &left_label,
                &self.right_frontier[exponent - 1],
                tree_len,
                false,
                &self.hash_inner_state,
                &mut new_label,
            );
            self.right_frontier[exponent] = new_label;

            if update_root_label {
                (self.hash_inner)(
                    &left_label,
                    &self.right_frontier[exponent - 1],
                    tree_len,
                    true,
                    &self.hash_inner_state,
                    &mut self.complete_root_label,
                );
            }
        }

        // And we are done. Yay!
        // To finish, overwrite the `left_label` with what will be the left label in the call to this method for `exponent + 1`.
        *left_label = next_left_label;
    }

    /// Returns the number of chunks we have fully processed already.
    fn number_of_completed_chunks(&self) -> u64 {
        self.len / (CHUNK_SIZE as u64)
    }

    /// Returns the digest for the values written so far.
    ///
    /// Despite its name, the method does not reset the hasher’s internal state. Additional writes will continue from the current value. If you need to start a fresh hash value, you will have to create a new hasher.
    pub fn finish(&self) -> [u8; WIDTH] {
        // So. Here we need to combine the information in `self.right_frontier` with the data
        // of the chunk we are currently processing, in order to obtain a proper digest.

        if self.len <= (CHUNK_SIZE as u64) {
            // We only have a single chunk. Simply call `hash_chunk` with `is_root = true` and call it a day.
            let mut digest = [0; WIDTH];
            (self.hash_chunk)(
                &self.current_chunk[..self.len as usize],
                true,
                &self.hash_chunk_state,
                &mut digest,
            );
            println!("single chunk self.len {:?}", self.len);
            return digest;
        } else {
            // Okay, real work ahead. We have a root label of a Merkle tree to compute!

            // We need to know how many leaves the tree will have.
            // If `self.current_chunk_len == 0`, then we have no partial chunk, else, we have an extra chunk beyond the already-completed ones.
            let chunk_count =
                self.number_of_completed_chunks() + if self.current_chunk_len == 0 { 0 } else { 1 };

            if self.current_chunk_len == 0 && chunk_count.is_power_of_two() {
                // In the special case that the number of chunks we processed is a power of two and there is no incomplete chunk,
                // we have already precomputed the root label, and stored it in `self.complete_root_label`.
                return self.complete_root_label;
            } else {
                // Otherwise, we need to compute the root label, using the precomputed labels of the complete
                // subtrees (each conveniently computed with `is_root = false`) for a tree of `chunk_count` leaves.

                // For which heights do we need to incorporate the subtree labels?
                // The definition of the unique tree shape for each `chunk_count` implies that there is a complete
                // subtree of height `k + 1` iff the `k`-th-least-significant bit of `chunk_count` is a one.
                // Handwavily explained, this is the case because the decomposition into complete subtrees amounts to
                // expressing `chunk_count` as a sum of strictly decreasing powers of two, which is exactly
                // what a binary representation of a number also does.

                // Hence, we can iterate through the complete subtree sizes that must occur in the tree, in ascending
                // order, and successively compute the parent label of the parent nodes joining the rightmost and
                // second-to-rightmost root respectively.

                // We store these parent labels in an accumulator `acc`, as we iterate. We further store the value `k` such that `k` is the height of the previously processed subtree, and the total number of bytes summarised in the previously processed subtree.
                // The initial values for these depend on whether we have a partial chunk or not.
                let mut acc = [0; WIDTH];
                let (starting_k, mut len) = if self.current_chunk_len > 0 {
                    // We have a partial chunk. Its label becomes the first accumulated value, and its `k` is always zero (because the partial chunk forms a complete subtree of height one).
                    // is_root is always false here; if it was true, then chunk_count would have
                    // been 1, i.e., a power of two, and we would not be in this branch.
                    (self.hash_chunk)(
                        &self.current_chunk[..self.current_chunk_len],
                        false,
                        &self.hash_chunk_state,
                        &mut acc,
                    );

                    (1, self.current_chunk_len as u64)
                } else {
                    // If we do not have a partial chunk, we need to find the least k such that
                    // the tree contains a complete subtree of height `k`. This happens to equal
                    // The number of trailing zeroes in the binary representation of the number of leaves.
                    let least_relevant_k = chunk_count.trailing_zeros();
                    // Then we initialise the accumulator with the precomputed label for that subtree.
                    // Note that there must be at least one other one bit, since otherwise chunk_count
                    // would be a power of two, and we would not be in this branch in the first place.
                    acc = self.right_frontier[least_relevant_k as usize];

                    (
                        least_relevant_k,
                        (CHUNK_SIZE as u64) * (1 << least_relevant_k),
                    )
                };

                println!(
                    "simple starting k {:?}, len {:?}, initial acc {:?}, chunk_count {chunk_count}",
                    starting_k, len, acc
                );

                // Now we can build up the accumulator by repeatedly computing the parent label of
                // the next complete subtree and the previous accumulator.
                // When we reached the final subtree, we need to set `is_root` to true in the label computation.
                // To check for that, we use that the floored base-two logarithm of `chunk_size` is equal to
                // the height of its greatest complete subtree.
                for k in starting_k..64 {
                    if is_bit_set(chunk_count, k - 1 as u32) {
                        let is_greatest_subtree = chunk_count.ilog2() == k;

                        // The total length of bytes we are summarising in this tree node is the sum of the bytes
                        // in the left tree (easy to compute, since it consists of full chunks only) and the right tree
                        // (which we already know from the previous iteration).
                        len = (CHUNK_SIZE as u64) * ((1 << k) - 1) + len;

                        println!(
                            "simple loop k {k} is_greatest_subtree {is_greatest_subtree} len {len}"
                        );
                        println!(
                            "simple loop precomputed right_frontier[k - 1] {:?}",
                            self.right_frontier[(k - 1) as usize]
                        );

                        let mut next_acc = [0; WIDTH];
                        (self.hash_inner)(
                            &self.right_frontier[(k - 1) as usize],
                            &acc,
                            len,
                            is_greatest_subtree,
                            &self.hash_inner_state,
                            &mut next_acc,
                        );
                        acc = next_acc;

                        if is_greatest_subtree {
                            return acc;
                        }
                    }
                }

                // We can never leave the above for loop, because we return after having found the greatest useful `k`.
                unreachable!();
            }
        }
    }
}

/// Checks whether the k-th-least-significant bit is set to one in `num`.
/// `k` starts at zero for the elast significant bit.
fn is_bit_set(num: u64, k: u32) -> bool {
    ((1 << k) & num) > 0
}
