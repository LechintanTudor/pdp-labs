pub fn generate_polynomials(len: usize) -> (Vec<i64>, Vec<i64>) {
    let p1 = (1..=(len as i64)).collect::<Vec<_>>();
    let p2 = (1..=(len as i64)).rev().collect::<Vec<_>>();
    (p1, p2)
}
