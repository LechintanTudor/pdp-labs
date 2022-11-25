use crate::matrix::{Matrix, self};
use crate::THREAD_COUNT;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct Benchmark(ThreadPool);

impl Benchmark {
    pub fn new() -> Self {
        Self(ThreadPoolBuilder::new().num_threads(THREAD_COUNT).build().unwrap())
    }    
    
    pub fn run(&self, m1: &Matrix, m2: &Matrix) {
        let total_elements = m1.row_count() * m2.col_count();
        let elements_per_thread = total_elements / THREAD_COUNT;
        let remaining_elements = total_elements - (elements_per_thread * THREAD_COUNT);

        self.0.scope(|scope| {
            for thread_index in 0..(THREAD_COUNT - 1) {
                scope.spawn(move |_| {
                    let start = thread_index * elements_per_thread;
                    let end = start + elements_per_thread;
                    
                    for i in start..end {
                        criterion::black_box(matrix::compute_element_sparse(m1, m2, i, thread_index));
                    }
                });
            }
            
            scope.spawn(|_| {
                let start = (THREAD_COUNT - 1) * elements_per_thread;
                let end = start + elements_per_thread + remaining_elements;

                for i in start..end {
                    criterion::black_box(matrix::compute_element_sparse(m1, m2, i, THREAD_COUNT - 1));
                }
            });
        })
    }
}
