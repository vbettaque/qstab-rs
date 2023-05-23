
mod math;
mod util;
mod clifford;
mod majorana;

use std::collections::HashSet;

use math::GF2;
use math::orthogonal::*;
use nalgebra::*;
use num_traits::{One, Zero};

fn main() {
    let n = 8;

    let mut matrices = HashSet::new();

    let ord = group_order(n);

    let v = vector![
        GF2::one(),
        GF2::one(),
        GF2::zero(),
        GF2::zero(),
        GF2::zero(),
        GF2::zero(),
        GF2::zero(),
        GF2::zero()
    ];

    let mut w_total = 0;

    println!("Total: n = {}", ord);
    for i in 0..ord {
        let o = indexed_element(n, i);
        if i % 10000 == 0 {
            println!("{:.2}% checked", 100. * (i as f64) / (ord as f64));
        }
        assert_eq!(o.transpose() * &o, DMatrix::<GF2>::identity(n, n), "bloop");
        assert!(matrices.insert(o.clone()), "double: {}", o);
        let w = (&o * v).map(|x| if x.is_one() { 1 } else { 0 }).sum();
        w_total += w;
        println!("{}", w_total as f64 / (i + 1) as f64)
    }
    println!("{:.2}", ((w_total as f64) / (ord as f64)));
}
