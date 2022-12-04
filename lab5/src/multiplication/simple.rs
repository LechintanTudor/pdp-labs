use crate::multiplication::common;
use crate::Polynomial;
use criterion::black_box;

pub struct Benchmark {
    p1: Polynomial,
    p2: Polynomial,
    r_len: usize,
    r: Vec<i64>,
}

impl Benchmark {
    pub fn new() -> Self {
        let (p1, p2) = common::generate_polynomials();
        let r_len = (p1.len() + p2.len()).checked_sub(1).unwrap_or(1);

        Self { p1, p2, r_len, r: Vec::new() }
    }

    pub fn run(&mut self) {
        let r_iter = (0..self.r_len).map(|r_degree| {
            let p1_degree_range = if r_degree <= self.p1.degree() {
                0..=r_degree
            } else {
                (r_degree - self.p2.degree())..=self.p1.degree()
            };

            p1_degree_range
                .map(|p1_degree| {
                    let p2_degree = r_degree - p1_degree;
                    let c1 = self.p1.coefficients()[p1_degree];
                    let c2 = self.p2.coefficients().get(p2_degree).copied().unwrap_or(0);
                    c1 * c2
                })
                .sum::<i64>()
        });

        self.r.clear();
        self.r.extend(r_iter);

        black_box(self.r.as_slice());
    }
}
