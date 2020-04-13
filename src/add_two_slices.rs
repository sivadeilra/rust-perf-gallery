
pub fn add_two_slices_naive(a: &[u32], b: &[u32], sum: &mut [u32]) {
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

pub fn add_two_slices_assert_eq(a: &[u32], b: &[u32], sum: &mut [u32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), sum.len());
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

pub fn add_two_slices_assert2(a: &[u32], b: &[u32], sum: &mut [u32]) {
    assert!(a.len() == b.len() && a.len() == sum.len());
    for i in 0..a.len() {
        sum[i] = a[i] + b[i];
    }
}

pub fn add_two_slices_zip(a: &[u32], b: &[u32], sum: &mut [u32]) {
    for (sum_item, (a_item, b_item)) in sum.iter_mut().zip(a.iter().zip(b.iter())) {
        *sum_item = *a_item + *b_item;
    }
}

