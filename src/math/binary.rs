use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, Matrix, OMatrix, Storage};
use num_traits::{One, Zero};

use crate::math::GF2;

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
        self.fold(GF2::zero(), |acc, x| acc + x)
    }

    fn complement(&self) -> Self::Output {
        self.add_scalar(GF2::one())
    }
}
