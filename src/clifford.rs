use nalgebra::DMatrix;
use rand::{Rng, prelude::Distribution, distributions::Standard};

use crate::math::{orthogonal, GF2};

pub enum Clifford {
    Majorana { n_majorana: usize, matrix: DMatrix<GF2> },
    Pauli { n_pauli: usize, matrix: DMatrix<GF2> },
}

impl Clifford {
    
    pub fn identity_majorana(n_majorana: usize) -> Clifford {
        assert!(n_majorana > 0 && n_majorana % 2 == 0);
        Clifford::Majorana { 
            n_majorana,
            matrix: DMatrix::<GF2>::identity(n_majorana, n_majorana),
        }
    }

    pub fn identity_pauli(n_pauli: usize) -> Clifford {
        assert!(n_pauli > 0);
        Clifford::Pauli { 
            n_pauli,
            matrix: DMatrix::<GF2>::identity(2 * n_pauli, 2 * n_pauli),
        }
    }

    pub fn sample_pauli<R: Rng + ?Sized>(n_pauli: usize, rng: &mut R) -> Clifford {
        todo!()
    }

    pub fn sample_parity<R: Rng + ?Sized>(n_majorana: usize, rng: &mut R) -> Clifford {
        todo!()
    }

}