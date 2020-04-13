/// The first 256 prime numbers.
pub static PRIMES_256: [u32; 256] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
    1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279,
    1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409,
    1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499,
    1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613,
    1619,
];

/// The first 16 primes
pub static PRIMES_16: [u32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

/// The first 7 primes
pub static PRIMES_7: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];

/// Naive implementation: bounds check + panic BB
pub fn lookup_16_naive(i: usize) -> u32 {
    PRIMES_16[i]
}

pub fn lookup_16_assert_bound(i: usize) -> u32 {
    assert!(i < 16);
    PRIMES_16[i]
}

/// Slightly smarter: The mask operator allows the compiler to understand that the range of the
/// array index is constrained to 0..16, and so the bounds check is eliminated.
pub fn lookup_16_masked(i: usize) -> u32 {
    PRIMES_16[i & 0xf]
}

/// Slightly smarter: The modulus operator allows the compiler to understand that the range of the
/// array index is constrained to 0..16, and so the bounds check is eliminated.
pub fn lookup_16_modulus(i: usize) -> u32 {
    PRIMES_16[i % 16]
}

pub fn lookup_256_naive(i: usize) -> u32 {
    PRIMES_256[i]
}

/// Compiler understands that the range of `i` cannot exceed the size of the array, and so no
/// bounds check is generated.
pub fn lookup_256_u8(i: u8) -> u32 {
    PRIMES_256[usize::from(i)]
}

pub fn lookup_256_masked(i: usize) -> u32 {
    PRIMES_256[i & 0xff]
}

pub fn lookup_256_modulus(i: usize) -> u32 {
    PRIMES_256[i % 256]
}

pub fn lookup_7_naive(i: usize) -> u32 {
    PRIMES_7[i]
}

/// Modulus operator allows compiler to eliminate bounds check. However, because the divisor is not
/// a power of w, it is a bit more costly. For certain constant divisors, Rust generates a sequence
/// of lower-cost ops (subtract, shift, add).
/// 
/// Whether the modulus or the bounds check is more costly will be usage-dependent.
pub fn lookup_7_modulus(i: usize) -> u32 {
    PRIMES_7[i % 7]
}

// These next samples are about double indirection through tables. A dynamic index `i` is used to
// look up a value in a static table, and the result of that lookup is used to index another table.
// This is common in C code, such as image decoders.
//
// The ultimate goal is to have a compiler that can observe that the range of the values on the
// first table lies within the index range of the second table. However, Rust does not currently
// do this.

// These indices are meaningless, but they are all legal indices into PRIMES_7.
pub static INDIRECT_LOOKUP_TABLE: [usize; 8] = [1, 3, 5, 1, 2, 6, 2, 4];

pub fn lookup_7_indirect_naive(i: usize) -> u32 {
    let j = INDIRECT_LOOKUP_TABLE[i];
    PRIMES_7[j]
}

pub fn lookup_7_indirect_mask_naive(i: usize) -> u32 {
    let j = INDIRECT_LOOKUP_TABLE[i & 7];
    PRIMES_7[j]
}
