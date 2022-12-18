#![allow(dead_code)]

mod karatsuba;
mod mpi_state;
mod simple;
mod utils;

fn main() {
    karatsuba::multiply_distributed();
}
