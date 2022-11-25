use crate::matrix::{self, Matrix};

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }    
    
    pub fn run(&self, m1: &Matrix, m2: &Matrix) {
        for i in 0..(m1.row_count() * m2.col_count()) {
           criterion::black_box(matrix::compute_element_col_major(m1, m2, i));
        }
    }
}
