pub fn multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    if p1.len() <= 32 {
        return simple_multiply(p1, p2);
    }

    let half_len = p1.len() / 2;
    let (p1_low, p1_high) = p1.split_at(half_len);
    let (p2_low, p2_high) = p2.split_at(half_len);

    let p1_sum = p1_low.iter().zip(p1_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();
    let p2_sum = p2_low.iter().zip(p2_high.iter()).map(|(c1, c2)| c1 + c2).collect::<Vec<_>>();

    let low_product = simple_multiply(p1_low, p2_low);
    let high_product = simple_multiply(p1_high, p2_high);
    let sum_product = simple_multiply(&p1_sum, &p2_sum);

    let middle_product: Vec<i64> = sum_product
        .iter()
        .zip(low_product.iter())
        .zip(high_product.iter())
        .map(|((s, l), h)| s - l - h)
        .collect();

    let mut result = vec![0; p1.len() + p2.len() - 1];

    for i in 0..(p1.len() - 1) {
        result[i] += low_product[i];
        result[i + half_len] += middle_product[i];
        result[i + p1.len()] += high_product[i];
    }

    result
}

fn simple_multiply(p1: &[i64], p2: &[i64]) -> Vec<i64> {
    let mut result = vec![0_i64; p1.len() + p2.len() - 1];

    for (i, c1) in p1.iter().enumerate() {
        for (j, c2) in p2.iter().enumerate() {
            result[i + j] += c1 * c2;
        }
    }

    result
}
