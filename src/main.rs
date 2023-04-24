mod galois;

use galois::GF2;
use num_traits::{One, Zero};
use nalgebra::{Vector2, DMatrix};

fn main() {
    let one: GF2 = 1.into();
    println!("{}", one);
}