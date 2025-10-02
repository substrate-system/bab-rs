pub struct SimpleHasher<const WIDTH: usize, const CHUNK_SIZE: usize> {
    /// The `hash_chunk` spec parameter.
    hash_chunk: fn(&[u8], bool) -> [u8; WIDTH],
    /// The `hash_inner` spec parameter.
    hash_inner: fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool) -> [u8; WIDTH],
    /// How many bytes of input have we processed so far?
    len: u64,
    /// Intuitively, this array stores the label of the rightmost vertex of each tree layer which will never change again. More precisely:
    ///
    /// At index zero, we store the label of the previously processed chunk (when current_chunk_len would reach CHUNK_SIZE, we reset it to zero and update the label at index zero). At index one, we store the root label of the rightmost complete subtree of height two. At index three, we store the root label of the rightmost complete subtree of height three. And so on (index zero does indeed store the root label of the rightmost complete subtree of height one, that's the same as the previous chunk).
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

impl<const WIDTH: usize, const CHUNK_SIZE: usize> SimpleHasher<WIDTH, CHUNK_SIZE> {
    /// Creates a mew bab hasher, using the given `hash_chunk` and `hash_inner` functions.
    pub fn new(
        hash_chunk: fn(&[u8], bool) -> [u8; WIDTH],
        hash_inner: fn(&[u8; WIDTH], &[u8; WIDTH], u64, bool) -> [u8; WIDTH],
    ) -> Self {
        Self {
            hash_chunk,
            hash_inner,
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
            let len_to_complete_current_chunk = CHUNK_SIZE - self.current_chunk_len;
            self.progress_or_complete_current_chunk(&remaining[..len_to_complete_current_chunk]);
            remaining = &remaining[len_to_complete_current_chunk..];
        }
    }

    /// Writes some data into the hasher. The data does not extend across chunk boundaries. It *may* finish the `current_chunk` though.
    fn progress_or_complete_current_chunk(&mut self, bytes: &[u8]) {
        debug_assert!(self.current_chunk_len + bytes.len() <= CHUNK_SIZE);

        if self.current_chunk_len + bytes.len() < CHUNK_SIZE {
            // The data does not complete the chunk, we can simply append it to the current chunk.
            let start = self.current_chunk_len;
            let end = start + bytes.len();
            self.current_chunk[start..end].copy_from_slice(bytes);
        } else {
            // Oh no, real work ahead.

            self.current_chunk_len = 0;

            // Okay, that part was easy.

            // The fun part is updating the `right_frontier`.
            // We always update its index zero, because the chunk we just processed is now the rightmost complete subtree of height one.
            // Then, we check whether we also just completed a subtree of height two. If no, then we are done. If we did, then we update
            // index one, and check whether we also just completed a subtree of height three. And so on, until we reached a height for
            // which we did not just complete a subtree.
            let mut old_label = [0; WIDTH];
            for i in 0..64 {
                let replaced_label = self.update_individual_frontier_index(i, old_label);

                match replaced_label {
                    None => break,
                    Some(replaced) => old_label = replaced,
                }
            }
        }
    }

    /// Checks whether the chunk we just completed finished a complete subtree of height `frontier_arr_index + 1`.
    /// If it did not, does nothing and returns `None`. If it did, updates the label stored
    /// in `self.right_frontier[frontier_arr_index]`, and returns the old label at that position (i.e., the label it just
    /// overwrote, because we need that label one final time to compute the label of the next-higher completed subtree, if there is one).
    ///
    /// For `frontier_arr_index == 0`, the `old_label_of_previous_frontier_arr_index` can be anything and will be ignored.
    fn update_individual_frontier_index(
        &mut self,
        frontier_arr_index: usize,
        old_label_of_previous_frontier_arr_index: [u8; WIDTH],
    ) -> Option<[u8; WIDTH]> {
        // How do we determine whether we just completed a subtree?
        // First, we need to know the how-manyethst subtree we just completed.
        let chunk_index = self.number_of_completed_chunks() - 1;
        // We subtract one, because zero-indexing the count of chunks makes things really nice:

        // Knowing our `chunk_index`, did we just complete a tree?
        // For `frontier_arr_index == 0`, this is always the case.
        // For `frontier_arr_index == 1`, this is the case iff the least-significant binary digit of `chunk_index` is a one.
        // For `frontier_arr_index == 2`, this is the case iff the two least-significant binary digits of `chunk_index` are ones.
        // For `frontier_arr_index == 3`, this is the case iff the three least-significant binary digits of `chunk_index` are ones.
        // And so on.
        // So neat! Binary trees, amiright?
        // (To see this for yourself, take a piece of paper, put down the three-digit binary numbers
        // 000, 001, 010, 011, 100, 101, 110, 111), and then draw a complete binary tree with those as the leaves.
        //
        // In other words, we need to test whether the number of trailing ones in the binary representation of `chunk_index`
        // is at least `frontier_arr_index`. And rust happens to have a function for counting trailing ones.

        let should_do_stuff = (chunk_index.trailing_ones() as usize) >= frontier_arr_index;

        // If we did not complete a subtree, we simply signal so, and do not need to update anything.
        if !should_do_stuff {
            return None;
        } else {
            // Okay, we need to actually do stuff. I.e., update `self.right_frontier[frontier_arr_index]`.

            // First, cache the old label, because we need to return that.
            let old_label = self.right_frontier[frontier_arr_index];

            // We don't compute real digests, only labels for internal processing. The `self.finish()` method will
            // take care of recomputing labels with `is_root = true` when necessary.
            let is_root = false;

            // But, for the roots of coplete trees that cover *all* chunks we had so far, we do store the
            // label computed with is_root = true specifically.
            let store_root_label_so_far = (chunk_index + 1).is_power_of_two();

            if frontier_arr_index == 0 {
                // If `frontier_arr_index == 0`, we need to compute the label of a leaf. Easy.
                let label = (self.hash_chunk)(&self.current_chunk, is_root);
                if store_root_label_so_far {
                    self.complete_root_label =
                        (self.hash_chunk)(&self.current_chunk, store_root_label_so_far);
                }

                self.right_frontier[frontier_arr_index] = label;
            } else {
                // Else, we need to compute an inner label.
                // We can compute it from the `old_label_of_previous_frontier_arr_index` and
                // the new label of the previous frontier arr index - which is stimply stored in the `right_frontier` array,
                // courtesy of the prior invocation of this method.

                let label = (self.hash_inner)(
                    &old_label_of_previous_frontier_arr_index,
                    &self.right_frontier[frontier_arr_index - 1],
                    self.len,
                    is_root,
                );
                if store_root_label_so_far {
                    self.complete_root_label = (self.hash_inner)(
                        &old_label_of_previous_frontier_arr_index,
                        &self.right_frontier[frontier_arr_index - 1],
                        self.len,
                        store_root_label_so_far,
                    );
                }

                self.right_frontier[frontier_arr_index] = label;
            }

            // And we are done. Yay!
            return Some(old_label);
        }
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

        // But first, a simple special case.
        if self.len == 0 {
            // The hash of the empty string is hard-defined to be all-zero-bytes.
            return [0; WIDTH];
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

                // We store these parent labels in an accumulator `acc`, as we iterate. We further store the value `k` such that `k + 1` is the height of the previously processed subtree.
                // The initial values for these depend on whether we have a partial chunk or not.
                let (mut acc, starting_k) = if self.current_chunk_len > 0 {
                    // We have a partial chunk. Its label becomes the first accumulated value, and its `k` is always zero (because the partial chunk forms a complete subtree of height one).
                    (
                        // is_root is always false here; if it was true, then chunk_count would have
                        // been 1, i.e., a power of two, and we would not be in this branch.
                        (self.hash_chunk)(&self.current_chunk[..self.current_chunk_len], false),
                        0,
                    )
                } else {
                    // If we do not have a partial chunk, we need to find the least k such that
                    // the tree contains a complete subtree of height `k + 1`.
                    // Then we initialise the accumulator with the precomputed label for that subtree.
                    let mut acc = [0; WIDTH];
                    let mut least_relevant_k = 0;
                    for k in 0..64u32 {
                        if is_bit_set(chunk_count, k) {
                            // We found the starting point. Note that if this was the *only* one bit, chunk_count
                            // would be a power of two, and we would not be in this branch in the first place.
                            acc = self.right_frontier[k as usize];
                            least_relevant_k = k;
                            break;
                        }
                    }
                    (acc, least_relevant_k)
                };

                // Now we can build up the accumulator by repeatedly computing the parent label of
                // the next complete subtree and the previous accumulator.
                // When we reached the final subtree, we need to set `is_root` to true in the label computation.
                // To check for that, we use that the floored base-two logarithm of `chunk_size` is equal to
                // the height of its greatest complete subtree.
                for k in starting_k..64 {
                    if is_bit_set(chunk_count, k) {
                        let is_greatest_subtree = chunk_count.ilog2() == k;

                        acc = (self.hash_inner)(
                            &self.right_frontier[k as usize],
                            &acc,
                            self.len,
                            is_greatest_subtree,
                        );

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
