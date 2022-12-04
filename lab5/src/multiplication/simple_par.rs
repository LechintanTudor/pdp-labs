use rayon::prelude::*;

pub fn multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    let r_len = p1.len() + p2.len() - 1;
    let mut r = Vec::<i64>::new();

    (0..r_len)
        .into_par_iter()
        .map(|r_index| {
            let p1_index_range = if r_index < p1.len() {
                0..(r_index + 1)
            } else {
                (r_index - p2.len() + 1)..p1.len()
            };

            p1_index_range
                .map(|p1_index| {
                    let c1 = p1[p1_index];
                    let c2 = p2[r_index - p1_index];
                    c1 * c2
                })
                .sum::<i64>()
        })
        .collect_into_vec(&mut r);

    r
}
