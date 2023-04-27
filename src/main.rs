mod util;
mod fields;
mod symplectic;
mod orthogonal;

use fields::GF2;
use nalgebra::*;
use symplectic::Symplectic;

use crate::orthogonal::get_orthogonal_matrix;

fn main() {
    // let v1: Vector6<f64> = Vector6::new(1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into());
    // let v2: Vector6<f64> = Vector6::new(1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into());
    // println!("{}", v1.symp(&v2));
    type D = U6;
    let n = D::dim() as u32;

    get_orthogonal_matrix::<D>(300);
}