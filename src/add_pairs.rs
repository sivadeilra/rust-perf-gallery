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

pub fn add_pairs_mut_naive(s: &mut [u32]) {
    let n = s.len() / 2;
    for i in 0..n {
        s[i] = s[i * 2] + s[i * 2 + 1];
    }
}

// 4 inner branches
pub fn add_pairs_mut_naive_lteq(s: &mut [u32]) {
    let mut i = 0;
    while i + 2 <= s.len() {
        s[i / 2] = s[i] + s[i + 1];
        i += 2;
    }
}

// 3 inner branches
pub fn add_pairs_mut_naive2_plusone_lt(s: &mut [u32]) {
    let mut i = 0;
    while i + 1 < s.len() {
        s[i / 2] = s[i] + s[i + 1];
        i += 2;
    }
}

// 3 inner branches
pub fn add_pairs_mut_sliced(s: &mut [u32]) {
    let mut i = 2;
    while i <= s.len() {
        let p = &s[i - 2..i];
        s[(i - 2) / 2] = p[0] + p[1];
        i += 2;
    }
}

// 4 inner branches
pub fn add_pairs_mut_naive3(s: &mut [u32]) {
    let mut i = 2;
    while i <= s.len() {
        s[(i - 2) / 2] = s[i - 2] + s[i - 1];
        i += 2;
    }
}

// 3 inner branches
pub fn sum_pairs_product_naive(s: &mut [u32]) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i + 2 <= s.len() {
        sum += s[i] * s[i + 1];
        i += 2;
    }
    sum
}

// ugly output
pub fn sum_pairs_product_iter(s: &mut [u32]) -> u32 {
    let mut sum = 0;
    for i in (0..s.len()).step_by(2) {
        sum += s[i] * s[i + 1];
    }
    sum
}

// bad: inner loop has panic check on pair[1], no loop unrolling
pub fn sum_pairs_product_chunks(s: &mut [u32]) -> u32 {
    s.chunks(2).map(|pair| pair[0] * pair[1]).sum()
}

// very good; loop unrolled x4
pub fn sum_pairs_product_chunks_exact(s: &mut [u32]) -> u32 {
    s.chunks_exact(2).map(|pair| pair[0] * pair[1]).sum()
}
