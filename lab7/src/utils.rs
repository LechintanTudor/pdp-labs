use std::iter;

pub fn generate_polynomials(len: usize) -> (Vec<i64>, Vec<i64>) {
    let p1 = iter::repeat(1_i64).take(len).collect::<Vec<_>>();
    let p2 = iter::repeat(1_i64).take(len).collect::<Vec<_>>();
    (p1, p2)
}
