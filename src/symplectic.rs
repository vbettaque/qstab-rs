use nalgebra::{Field, Matrix, Dim, Storage, Scalar};
use nalgebra::constraint::{ShapeConstraint, DimEq};

pub trait Symplectic<T, R: Dim, C: Dim, SB> where SB: Storage<T, R, C> {
    fn symp<R2: Dim, C2: Dim, SB2>(&self, rhs: &Matrix<T, R2, C2, SB2>) -> T
    where
        T: Scalar + Field,
        SB2: Storage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>;
}

impl<T, R: Dim, C: Dim, SB> Symplectic<T, R, C, SB> for Matrix<T, R, C, SB>
where
    T: Scalar + Field,
    SB: Storage<T, R, C>,
{
    #[inline(always)]
    fn symp<R2: Dim, C2: Dim, SB2>(&self, rhs: &Matrix<T, R2, C2, SB2>) -> T
    where
        SB2: Storage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>
    {
        assert_eq!(
            self.nrows() % 2, 0,
            "Symplectic product dimensions mismatch for shape {:?}:
                number of rows not even.",
            self.shape(),
        );

        assert_eq!(
            self.nrows(), rhs.nrows(),
            "Symplectic product dimensions mismatch for shapes {:?} and {:?}:
                left rows != right rows.",
            self.shape(),
            rhs.shape(),
        );
    
        assert_eq!(
            self.ncols(), rhs.ncols(),
            "Symplectic product dimensions mismatch for shapes {:?} and {:?}:
                left cols != right cols.",
            self.shape(),
            rhs.shape(),
        );

        let size = self.len() / 2;
        let a1= self.rows_with_step(0, size, 1);
        let a2 = self.rows_with_step(1, size, 1);
        let b1= rhs.rows_with_step(0, size, 1);
        let b2= rhs.rows_with_step(1, size, 1);

        a1.dot(&b2) - a2.dot(&b1)
    }
}