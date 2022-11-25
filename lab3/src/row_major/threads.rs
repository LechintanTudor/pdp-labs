use crate::matrix::{Matrix, self};
use crate::THREAD_COUNT;

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }    
    
    pub fn run(&self, m1: &Matrix, m2: &Matrix) {
        let total_elements = m1.row_count() * m2.col_count();
        let elements_per_thread = total_elements / THREAD_COUNT;
        let remaining_elements = total_elements - (elements_per_thread * THREAD_COUNT);

        std::thread::scope(|scope| {
            for i in 0..(THREAD_COUNT - 1) {
                scope.spawn(move || {
                    let start = i * elements_per_thread;
                    let end = start + elements_per_thread;
                    
                    for i in start..end {
                        criterion::black_box(matrix::compute_element_row_major(m1, m2, i));
                    }
                });
            }
            
            scope.spawn(|| {
                let start = (THREAD_COUNT - 1) * elements_per_thread;
                let end = start + elements_per_thread + remaining_elements;

                for i in start..end {
                    criterion::black_box(matrix::compute_element_row_major(m1, m2, i));
                }
            });
        });
    }
}
