mod fields;
mod symplectic;
mod orthogonal;

use fields::GF2;
use nalgebra::*;
use symplectic::Symplectic;

use crate::orthogonal::get_bit_vector;

fn main() {
    // let v1: Vector6<f64> = Vector6::new(1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into());
    // let v2: Vector6<f64> = Vector6::new(1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into());
    // println!("{}", v1.symp(&v2));
    let n: u32 = 4;
    let base: usize = 2;

    let s: usize = base.pow(n - 1);

    for i in 0..s {
        let mut k = i;
        k = if k.count_ones() % 2 == 0 {k | s} else {k};
        print!("{}", get_bit_vector::<U4>(k))
    }
}