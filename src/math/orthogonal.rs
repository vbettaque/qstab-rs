use nalgebra::{DMatrix, DVector, Dyn, Storage, Vector};
use num_traits::{One, Zero};

use rand::Rng;
use crate::{math::Binary, math::GF2, util::indexed_odd_bit_iter};

/// Returns the order |O(n, GF2)| of the orthogonal group over GF2 for even n.
pub fn group_order(n: usize) -> usize {
    assert!(
        n > 0 && n % 2 == 0,
        "n must be non-zero and even, but was found to be n = {}!",
        n
    );
    let k = n / 2;
    let mut order = usize::pow(2, (k * k) as u32);
    for i in 1..k {
        order *= usize::pow(4, i as u32) - 1;
    }
    order
}

/// Returns the Householder reflection corresponding to the even-parity vector h
/// such that it satisfies v -> v + <v, h> h.
/// Panics if h has odd parity.
fn householder<S>(h: &Vector<GF2, Dyn, S>) -> DMatrix<GF2>
where
    S: Storage<GF2, Dyn>,
{
    assert!(
        h.parity().is_zero(),
        "h must have even parity but does not!"
    );
    let n = h.len();
    DMatrix::<GF2>::identity(n, n) + h * h.transpose()
}

/// Returns a householder reflection or the product of two reflections, such
/// that it maps v1 to v2. Panics if v1.len() != v2.len(), or if a vector 
/// doesn't have odd parity.
fn find_householders<S1, S2>(v1: &Vector<GF2, Dyn, S1>, v2: &Vector<GF2, Dyn, S2>) -> DMatrix<GF2>
where
    S1: Storage<GF2, Dyn>,
    S2: Storage<GF2, Dyn>,
{
    let n = v1.len();

    assert_eq!(n, v2.len(),
        "v1.len() = {} and v2.len() = {} do not match!
            Both vectors must have the same length!",
        n, v2.len()
    );
    assert!(
        v1.parity().is_one() && v2.parity().is_one(), 
        "Required v1 and v2 to have odd parity, but found v1.parity() = {} 
            and v2.parity() = {}!",
        v1.parity(), v2.parity()
    );

    if v1.dot(v2).is_zero() {
        householder(&(v2 - v1))
    } else {
        let h = (v2 - v1).complement();
        householder(&h).complement()
    }
}

/// Returns the (i+1)th orthogonal (n x n) matrix over GF2 according to an
/// internal index of all group elements.
/// Panics if n is zero or odd, or if i >= |O(n, GF2)|.
pub fn indexed_element(n: usize, i: usize) -> DMatrix<GF2> {
    let ord = group_order(n);

    assert!(
        i < ord,
        "i has to be smaller than |O(n, GF2)| = {}, but is i = {}!",
        ord, i
    );

    let mut o = DMatrix::<GF2>::identity(n, n);

    // Number of possible choices for the first and second column vectors
    let p1 = usize::pow(2, (n - 1) as u32);
    let p2 = usize::pow(2, (n - 2) as u32) - 1;

    // Recursion exit at smallest matrix dimension
    if n == 2 {
        return if i.is_zero() { o } else { o.complement() };
    } else {
        let n_rec = n - 2;
        let i_rec = i / (p1 * p2);
        let mut o_rec = o.view_mut((2, 2), (n_rec, n_rec));
        o_rec.copy_from(&indexed_element(n_rec, i_rec));
    }

    // Index of the first and second column vectors
    let i1 = i % p1;
    let i2 = (i / p1) % p2;

    // New first column vector
    let f1 = DVector::<GF2>::from_iterator(n, indexed_odd_bit_iter(n, i1));

    // Intermediate second column vector orthogonal to e1
    let iter = vec![GF2::zero()]
        .into_iter()
        .chain(indexed_odd_bit_iter(n - 1, i2));
    let f2 = DVector::<GF2>::from_iterator(n, iter);

    let t1 = find_householders(&o.column(0), &f1);

    let t2 = if f2[1].is_zero() {
        householder(&(o.column(1) + f2))
    } else {
        let mut h1 = o.column(1).clone_owned();
        let mut h2 = f2.clone_owned();
        let i = f2.iter().skip(2).position(|x| x.is_zero()).unwrap() + 2;
        h1[i] = GF2::one();
        h2[i] = GF2::one();
        householder(&h2) * householder(&h1)
    };
    t1 * t2 * o
}

pub fn sample_element<R: Rng + ?Sized>(n: usize, rng: &mut R) -> DMatrix<GF2> {
    let i = rng.gen_range(0..group_order(n));
    indexed_element(n, i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::*;

    #[test]
    fn test_group_order() {
        assert_eq!(group_order(2), 2);
    }

    #[test]
    fn test_householder() {
        // n = 2
        for i in 0..usize::pow(2, (2 - 1) as u32) {
            for j in 0..usize::pow(2, (2 - 1) as u32) {
                let v1 = Vector2::from_iterator(indexed_odd_bit_iter(2, i));
                let v2 = Vector2::from_iterator(indexed_odd_bit_iter(2, j));
                // let h = get_householder(&v1, &v2);
                // assert_eq!(h * v1, v2);
                // assert_eq!(h.transpose() * h, Matrix2::<GF2>::identity())
            }
        }
        // n = 4
        for i in 0..usize::pow(2, (4 - 1) as u32) {
            for j in 0..usize::pow(2, (4 - 1) as u32) {
                let v1 = Vector4::from_iterator(indexed_odd_bit_iter(4, i));
                let v2 = Vector4::from_iterator(indexed_odd_bit_iter(4, j));
                // let h = get_householder(&v1, &v2);
                // assert_eq!(h * v1, v2);
                // assert_eq!(h.transpose() * h, Matrix4::<GF2>::identity())
            }
        }
        // n = 6
        for i in 0..usize::pow(2, (6 - 1) as u32) {
            for j in 0..usize::pow(2, (6 - 1) as u32) {
                let v1 = Vector6::from_iterator(indexed_odd_bit_iter(6, i));
                let v2 = Vector6::from_iterator(indexed_odd_bit_iter(6, j));
                // let h = get_householder(&v1, &v2);
                // assert_eq!(h * v1, v2);
                // assert_eq!(h.transpose() * h, Matrix6::<GF2>::identity())
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_householder_panic() {}
}
