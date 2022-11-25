use std::ops::{Index, IndexMut};
use std::convert::{AsRef, AsMut};
use rand::Rng;
use crate::THREAD_COUNT;

#[derive(Default, Debug)]
pub struct Matrix {
    elements: Vec<f32>,
    row_count: usize,
    col_count: usize,
}

impl Matrix {
    pub fn zero(row_count: usize, col_count: usize) -> Self {
        Self {
            elements: vec![0_f32; row_count * col_count],
            row_count,
            col_count,
        }
    }
    
    pub fn random(row_count: usize, col_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut matrix = Self::zero(row_count, col_count);
        
        for i in 0..row_count {
            for j in 0..col_count {
                matrix[(i, j)] = rng.gen();
            }
        }

        matrix
    }
    
    #[inline]
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    #[inline]
    pub fn col_count(&self) -> usize {
        self.col_count
    }
    
    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl AsRef<[f32]> for Matrix {
    fn as_ref(&self) -> &[f32] {
        &self.elements
    }
}

impl AsMut<[f32]> for Matrix {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32] {
        &mut self.elements
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.elements[row * self.col_count + col] 
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[row * self.col_count + col] 
    }
}

pub fn compute_element_row_major(m1: &Matrix, m2: &Matrix, index: usize) -> f32 {
    let i = index / m1.col_count();
    let j = index % m1.col_count();

    let mut value = 0.0;

    for j1 in 0..m1.col_count() {
        for i2 in 0..m2.row_count() {
            value += m1[(i, j1)] * m2[(i2, j)];     
        }
    }
    
    value
}

pub fn compute_element_col_major(m1: &Matrix, m2: &Matrix, index: usize) -> f32 {
    let i = index % m1.col_count();
    let j = index / m1.col_count();

    let mut value = 0.0;

    for j1 in 0..m1.col_count() {
        for i2 in 0..m2.row_count() {
            value += m1[(i, j1)] * m2[(i2, j)];     
        }
    }
    
    value
}

pub fn compute_element_sparse(m1: &Matrix, m2: &Matrix, index: usize, offset: usize) -> f32 {
    let index = (index * THREAD_COUNT + offset) % (m1.col_count() * m2.row_count());
    let i = index % m1.col_count();
    let j = index / m1.col_count();

    let mut value = 0.0;

    for j1 in 0..m1.col_count() {
        for i2 in 0..m2.row_count() {
            value += m1[(i, j1)] * m2[(i2, j)];     
        }
    }
    
    value
}