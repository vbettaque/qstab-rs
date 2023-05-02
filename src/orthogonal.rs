use nalgebra::{DimName, allocator::Allocator, DefaultAllocator, OMatrix, 
    OVector, Vector, Storage, U1};
use num_traits::{Zero, One};

use crate::{fields::GF2, util::get_odd_bit_iter, binary::Binary};

/// Returns the order |O(n, GF2)| of the orthogonal group over GF2 for even n.
fn get_group_order(n: usize) -> usize {
    assert!(n % 2 == 0);
    
    let k = n / 2;
    let mut order = usize::pow(2, (k*k) as u32);
    for i in 1..k {
        order *= usize::pow(4, n as u32) - 1;
    }
    order
}

fn get_householder<D, S>(v1: &Vector<GF2, D, S>, v2: &Vector<GF2, D, S>)
    -> OMatrix<GF2, D, D>
where
    D: DimName,
    S: Storage<GF2, D>,
    DefaultAllocator:  Allocator<GF2, D> + Allocator<GF2, U1, D> 
        + Allocator<GF2, D, D>,
{
    assert!(v1.parity().is_one() && v2.parity().is_one());

    if v1.dot(&v2).is_zero() {
        let h = v2 - v1;
        OMatrix::<GF2, D, D>::identity() + &h * h.transpose()
    } else {
        let h = (v2 - v1).complement();
        OMatrix::<GF2, D, D>::identity().complement() + &h * h.transpose()
    }
}

pub fn get_orthogonal_matrix<D>(i: usize) -> OMatrix::<GF2, D, D>
where
    D: DimName,
    DefaultAllocator: Allocator<GF2, D, D> + Allocator<GF2, D>,
{
    let n = D::dim();
    let ord = get_group_order(n);

    assert!(i < ord);

    let s1 = usize::pow(2, (n - 1) as u32);
    let s2 = usize::pow(2, (n - 2) as u32) - 1;

    let b1 = i % s1;
    let b2 = if s2 > 0 {(i / s1) % s2} else {0};

    let iter1 = get_odd_bit_iter(n, b1);
    let v1 = OVector::<GF2, D>::from_iterator(iter1);

    let iter2 = vec![GF2::zero()].into_iter().chain(get_odd_bit_iter(n-1, b2));
    let v2 = OVector::<GF2, D>::from_iterator(iter2);

    let o: OMatrix<GF2, D, D> = OMatrix::<GF2, D, D>::identity();

    return o
}

#[cfg(test)]
mod tests {
    use nalgebra::*;
    use super::*;

    #[test]
    fn test_householder() {
        // n = 2
        for i in 0..usize::pow(2, (2 - 1) as u32) {
            for j in 0..usize::pow(2, (2 - 1) as u32) {
                let v1 = Vector2::from_iterator(get_odd_bit_iter(2, i));
                let v2 = Vector2::from_iterator(get_odd_bit_iter(2, j));
                let h = get_householder(&v1, &v2);
                assert_eq!(h * v1, v2);
                assert_eq!(h.transpose() * h, Matrix2::<GF2>::identity())
            }
        }
        // n = 4
        for i in 0..usize::pow(2, (4 - 1) as u32) {
            for j in 0..usize::pow(2, (4 - 1) as u32) {
                let v1 = Vector4::from_iterator(get_odd_bit_iter(4, i));
                let v2 = Vector4::from_iterator(get_odd_bit_iter(4, j));
                let h = get_householder(&v1, &v2);
                assert_eq!(h * v1, v2);
                assert_eq!(h.transpose() * h, Matrix4::<GF2>::identity())
            }
        }
        // n = 6
        for i in 0..usize::pow(2, (6 - 1) as u32) {
            for j in 0..usize::pow(2, (6 - 1) as u32) {
                let v1 = Vector6::from_iterator(get_odd_bit_iter(6, i));
                let v2 = Vector6::from_iterator(get_odd_bit_iter(6, j));
                let h = get_householder(&v1, &v2);
                assert_eq!(h * v1, v2);
                assert_eq!(h.transpose() * h, Matrix6::<GF2>::identity())
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_householder_panic() {

    }
}