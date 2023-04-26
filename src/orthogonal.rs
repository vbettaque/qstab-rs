use std::ops::BitOr;

use nalgebra::{DimName, allocator::Allocator, DefaultAllocator, OMatrix};

use crate::fields::GF2;

fn group_order(n: u32) -> usize {
    assert!(n % 2 == 0);
    
    let k: u32 = n / 2;
    let base: usize = 2;

    let mut order = base.pow(k.pow(2));
    for i in 1..k {
        order *= base.pow(2*i) - 1;
    }
    order
}

pub fn get_orthogonal_matrix<D: DimName>(i: usize) -> OMatrix::<GF2, D, D>
where 
    DefaultAllocator: Allocator<GF2, D, D>
{
    let n = D::dim() as u32;
    let ord = group_order(n);
    let base: usize = 2;

    assert!(i < ord);

    let mut o: OMatrix<GF2, D, D> = OMatrix::<GF2, D, D>::zeros();

    let s: usize = base.pow(n);
    let mut k = i % s;
    k = if k.count_ones() % 2 == 0 {k | s} else {k};

    let v = (0..n).map(|n| (k >> n) & 1);


    
    let b1 = o.column_mut(0);
    let b2 = o.column_mut(1);
    
    return o
}