use crate::Polynomial;

pub const POLYNOMIAL_LENGTH: usize = 1000;

pub fn generate_polynomials() -> (Polynomial, Polynomial) {
    let p1 = (1..=POLYNOMIAL_LENGTH).map(|c| c as i64).collect::<Vec<_>>();
    let p2 = (1..=POLYNOMIAL_LENGTH).rev().map(|c| c as i64).collect::<Vec<_>>();
    (Polynomial::from_coefficients(p1), Polynomial::from_coefficients(p2))
}
