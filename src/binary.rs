use nalgebra::{Storage, Dim, DefaultAllocator, allocator::Allocator, Matrix, OMatrix};
use num_traits::{One, Zero};

use crate::fields::GF2;

pub trait Binary<T: One + Zero> {
    type Output;

    fn parity(&self) -> T;

    fn complement(&self) -> Self::Output;
}

impl<R: Dim, C: Dim, S> Binary<GF2> for Matrix<GF2, R, C, S>
where
    S: Storage<GF2, R, C>,
    DefaultAllocator: Allocator<GF2, R, C>,
{
    type Output = OMatrix<GF2, R, C>;

    fn parity(&self) -> GF2 {
        return self.fold(GF2::zero(), |acc, x| acc + x)
    }

    fn complement(&self) -> Self::Output {
         self.add_scalar(GF2::one()).into()
    }
}