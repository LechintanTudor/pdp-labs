pub fn multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    let r_len = p1.len() + p2.len() - 1;

    (0..r_len)
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
        .collect::<Vec<_>>()
}
