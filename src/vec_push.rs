// 2 panic BBs (one for allocation failure, one for capacity overflow)
// Also note that much of the alloc() code gets inlined into this, so the generated code for
// push_simple() is actually quite long.
//
// Overall, the generated code does not look very good, due to excessive inlining.
pub fn push_simple(v: &mut Vec<u32>, x: u32) {
    v.push(x);
}

// Vec::push() contains a capacity check. However, if you do your own capacity check before calling
// Vec::push(), then the compiler can elide the second capacity check.
//
// The generated code for push_with_capacity_check() is much shorter than that for
// push_simple(). This is because the allocation path is removed (it's unreachable), and so it
// does not get inlined.
//
// 0 panic BBs
pub fn push_with_capacity_check(v: &mut Vec<u32>, x: u32) -> bool {
    if v.len() < v.capacity() {
        v.push(x);
        true
    } else {
        false
    }
}
