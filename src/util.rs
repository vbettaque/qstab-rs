use crate::math::GF2;

/// Returns the n first bits in the binary representation of k from lowest (2^0)
/// to highest (2^(n-1)) as a vector over GF2.
pub fn bit_iter(n: usize, k: usize) -> impl Iterator<Item = GF2> {
    (0..n).map(move |i| GF2::from((k >> i) & 1))
}

/// Returns the (i+1)th odd-parity vector of length n according to an internal
/// index of all possibilities. Panics if k >= 2^(n-1).
pub fn indexed_odd_bit_iter(n: usize, i: usize) -> impl Iterator<Item = GF2> {
    assert!(
        i < usize::pow(2, (n - 1) as u32),
        "Parity vector index i = {} out of bounds for D = {}.
        i must be smaller than 2^(D-1)",
        i, n
    );
    let k = i << 1 | (if i.count_ones() % 2 == 0 { 1 } else { 0 });
    bit_iter(n, k)
}
