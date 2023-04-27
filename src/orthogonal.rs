use nalgebra::{DimName, allocator::Allocator, DefaultAllocator, OMatrix, OVector};
use num_traits::Zero;

use crate::{fields::GF2, util::get_odd_bit_iter};

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


    println!("{}, {}", v1, v2);
    // let b1 = get_odd_bit_vector::<D>(i % s);

    let o: OMatrix<GF2, D, D> = OMatrix::<GF2, D, D>::identity();

    return o
}