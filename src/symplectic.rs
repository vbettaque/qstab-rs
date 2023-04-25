use nalgebra::{Field, Matrix, Dim, Storage, Scalar, U2, U1, U4, U6};
use nalgebra::constraint::{ShapeConstraint, DimEq};

trait Symplectic<T, R: Dim, C: Dim, SB> where SB: Storage<T, R, C> {
    fn symp<R2: Dim, C2: Dim, SB2>(&self, rhs: &Matrix<T, R2, C2, SB2>, conjugate: impl Fn(T) -> T) -> T
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
    fn symp<R2: Dim, C2: Dim, SB2>(&self, rhs: &Matrix<T, R2, C2, SB2>, conjugate: impl Fn(T) -> T) -> T
    where
        SB2: Storage<T, R2, C2>,
        ShapeConstraint: DimEq<R, R2> + DimEq<C, C2>
    {
        assert!(
            self.nrows() % 2 == 0,
            "Symplectic product dimensions mismatch for shape {:?}: number of rows not even.",
            self.shape(),
        );

        assert!(
            self.nrows() == rhs.nrows(),
            "Symplectic product dimensions mismatch for shapes {:?} and {:?}: left rows != right rows.",
            self.shape(),
            rhs.shape(),
        );
    
        assert!(
            self.ncols() == rhs.ncols(),
            "Symplectic product dimensions mismatch for shapes {:?} and {:?}: left cols != right cols.",
            self.shape(),
            rhs.shape(),
        );

        if (R::is::<U2>() || R2::is::<U2>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();
                let b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();

                return a - b;
            }
        }
        if (R::is::<U4>() || R2::is::<U4>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let mut a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();
                let mut b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();
                let c = conjugate(self.get_unchecked((2, 0)).clone())
                    * rhs.get_unchecked((3, 0)).clone();
                let d = conjugate(self.get_unchecked((3, 0)).clone())
                    * rhs.get_unchecked((2, 0)).clone();

                a += c;
                b += d;

                return a - b;
            }
        }
        if (R::is::<U6>() || R2::is::<U6>()) && (C::is::<U1>() || C2::is::<U1>()) {
            unsafe {
                let mut a = conjugate(self.get_unchecked((0, 0)).clone())
                    * rhs.get_unchecked((1, 0)).clone();
                let mut b = conjugate(self.get_unchecked((1, 0)).clone())
                    * rhs.get_unchecked((0, 0)).clone();
                let c = conjugate(self.get_unchecked((2, 0)).clone())
                    * rhs.get_unchecked((3, 0)).clone();
                let d = conjugate(self.get_unchecked((3, 0)).clone())
                    * rhs.get_unchecked((2, 0)).clone();
                let e = conjugate(self.get_unchecked((4, 0)).clone())
                    * rhs.get_unchecked((5, 0)).clone();
                let f = conjugate(self.get_unchecked((5, 0)).clone())
                    * rhs.get_unchecked((4, 0)).clone();

                a += c + e;
                b += d + f;

                return a - b;
            }
        }

        let mut res = T::zero();

        let mut acc0;
        let mut acc1;
        let mut acc2;
        let mut acc3;
        let mut acc4;
        let mut acc5;
        let mut acc6;
        let mut acc7;

        for j in 0..self.ncols() {
            let mut i = 0;

            acc0 = T::zero();
            acc1 = T::zero();
            acc2 = T::zero();
            acc3 = T::zero();
            acc4 = T::zero();
            acc5 = T::zero();
            acc6 = T::zero();
            acc7 = T::zero();

            while self.nrows() - i >= 8 {
                acc0 += unsafe {
                    conjugate(self.get_unchecked((i, j)).clone())
                        * rhs.get_unchecked((i + 1, j)).clone()
                };
                acc1 += unsafe {
                    conjugate(self.get_unchecked((i + 1, j)).clone())
                        * rhs.get_unchecked((i, j)).clone()
                };
                acc2 += unsafe {
                    conjugate(self.get_unchecked((i + 2, j)).clone())
                        * rhs.get_unchecked((i + 3, j)).clone()
                };
                acc3 += unsafe {
                    conjugate(self.get_unchecked((i + 3, j)).clone())
                        * rhs.get_unchecked((i + 2, j)).clone()
                };
                acc4 += unsafe {
                    conjugate(self.get_unchecked((i + 4, j)).clone())
                        * rhs.get_unchecked((i + 5, j)).clone()
                };
                acc5 += unsafe {
                    conjugate(self.get_unchecked((i + 5, j)).clone())
                        * rhs.get_unchecked((i + 4, j)).clone()
                };
                acc6 += unsafe {
                    conjugate(self.get_unchecked((i + 6, j)).clone())
                        * rhs.get_unchecked((i + 7, j)).clone()
                };
                acc7 += unsafe {
                    conjugate(self.get_unchecked((i + 7, j)).clone())
                        * rhs.get_unchecked((i + 6, j)).clone()
                };
                i += 8;
            }

            res += acc0 + acc4;
            res -= acc1 + acc5;
            res += acc2 + acc6;
            res -= acc3 + acc7;

            for k in i..self.nrows() {
                
                res += unsafe {
                    conjugate(self.get_unchecked((k, j)).clone())
                        * rhs.get_unchecked((k, j)).clone()
                }
            }
        }

        res
    }
}