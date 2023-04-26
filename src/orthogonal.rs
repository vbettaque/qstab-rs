use nalgebra::SquareMatrix;

pub fn even_group_order(n: u32) -> usize {
    assert_eq!(n % 2, 0);
    
    let k: u32 = n / 2;
    let base: usize = 2;

    let mut order = base.pow(k.pow(2));
    for i in 1..k {
        order *= base.pow(2*i) - 1;
    }
    order
}

// pub fn get_orthogonal_matrix() -> SquareMatrix<GF2, Dyn, S>