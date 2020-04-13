//! Add each pair of elements in two slices, writing the results to a third slice.

/// Naive implementation. Results in 3 branches, 2 panic BBs.
pub fn add_two_slices_naive(a: &[u32], b: &[u32], sum: &mut [u32]) {
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

/// Native implementation, using output-dependent looping rather than input-dependent.
/// Results are nearly the same as `add_two_slices_naive`: 3 branches, 2 panic BBs.
pub fn add_two_slices_naive_output_dependent(a: &[u32], b: &[u32], sum: &mut [u32]) {
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

/// Slightly smarter implementation.  `assert_eq` allows the compiler to elide all of the bounds
/// checks within the loop; the bounds checks are hoisted above the loop.
/// 
/// `assert_eq` generates quite verbose code; the panic blocks are longer than the main body.
pub fn add_two_slices_assert_eq(a: &[u32], b: &[u32], sum: &mut [u32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), sum.len());
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

/// Slightly smarter implementation. `assert` allows the compiler to elide all bounds checks within
/// the loop. A single `assert` checks all conditions, and using `assert` rather than `assert_eq`
/// generates more compact code.
pub fn add_two_slices_assert2(a: &[u32], b: &[u32], sum: &mut [u32]) {
    assert!(a.len() == b.len() && a.len() == sum.len());
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

/// Idiomatic Rust implemntation, using `zip` and iterators. All bounds checks are eliminated.
/// 
/// The loop optimizer generates SIMD vector code for this, unrolling 8 elements per loop iteration.
/// This is better code than most humans could write by hand.
/// 
/// TODO: Why does the loop vectorizer work on this loop, but not on `add_two_slices_assert2`?
/// In both cases, the compiler should have the same information.
pub fn add_two_slices_zip(a: &[u32], b: &[u32], sum: &mut [u32]) {
    for (sum_item, (a_item, b_item)) in sum.iter_mut().zip(a.iter().zip(b.iter())) {
        *sum_item = *a_item + *b_item;
    }
}

