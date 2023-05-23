use nalgebra::DVector;

use crate::{math::GF2, math::Binary};

pub struct MajoranaString {
    length: usize,
    vector: DVector<GF2>,
}

impl MajoranaString {
    pub fn identity(length: usize) -> MajoranaString {
        assert!(
            length > 0 && length % 2 == 0,
            "Input length must be non-zero and even!"
        );
        MajoranaString {length, vector: DVector::zeros(length)}
    }
    
    pub fn length(&self) -> usize { self.length }

    pub fn weight(&self) -> usize {
        self.vector.fold(0, |acc, x| acc + usize::from(x))
    }
}

impl Binary<GF2> for MajoranaString {
    type Output = Self;

    fn parity(&self) -> GF2 { self.vector.parity() }

    fn complement(&self) -> Self::Output {
        MajoranaString { length: self.length, vector: self.vector.complement() }
    }
}