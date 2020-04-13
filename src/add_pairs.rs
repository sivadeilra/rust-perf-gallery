
pub fn add_pairs_naive_using_input_len(pairs: &[u32], sums: &mut [u32]) {
    for i in 0..pairs.len() / 2 {
        sums[i] = pairs[i * 2] + pairs[i * 2 + 1];
    }
}

pub fn add_pairs_naive_using_output_len(pairs: &[u32], sums: &mut [u32]) {
    for i in 0..sums.len() {
        sums[i] = pairs[i * 2] + pairs[i * 2 + 1];
    }
}

pub fn add_pairs_naive_using_output_len_assert_eq(pairs: &[u32], sums: &mut [u32]) {
    assert_eq!(pairs.len(), sums.len() * 2);
    for i in 0..sums.len() {
        sums[i] = pairs[i * 2] + pairs[i * 2 + 1];
    }
}

pub fn add_pairs_using_chunks(pairs: &[u32], sums: &mut [u32]) {
    let mut i = 0;
    for pair in pairs.chunks(2) {
        sums[i] = pair[0] + pair[1];
        i += 1;
    }
}

pub fn add_pairs_using_chunks_exact(pairs: &[u32], sums: &mut [u32]) {
    for (i, pair) in pairs.chunks(2).enumerate() {
        sums[i] = pair[0] + pair[1];
    }
}

pub fn add_pairs_using_chunks_exact_zipped(pairs: &[u32], sums: &mut [u32]) {
    for (sum, pair) in sums.iter_mut().zip(pairs.chunks_exact(2)) {
        *sum = pair[0] + pair[1];
    }
}

pub fn add_pairs_using_chunks_zipped(pairs: &[u32], sums: &mut [u32]) {
    for (sum, pair) in sums.iter_mut().zip(pairs.chunks(2)) {
        *sum = pair[0] + pair[1];
    }
}

