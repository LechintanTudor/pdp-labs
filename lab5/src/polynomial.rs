use std::fmt;

#[derive(Clone, Debug)]
pub struct Polynomial {
    coefficients: Vec<i64>,
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut term_iter = self
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, &coefficient)| coefficient != 0)
            .map(|(i, coefficient)| (coefficient, i));

        if let Some((coefficient, power)) = term_iter.next() {
            write!(f, "{coefficient}x^{power}")?;

            while let Some((coefficient, power)) = term_iter.next() {
                write!(f, " + {coefficient}x^{power}")?;
            }
        }

        Ok(())
    }
}

impl Polynomial {
    pub fn from_coefficients(mut coefficients: Vec<i64>) -> Self {
        while let Some(&coefficient) = coefficients.last() {
            if coefficient == 0 {
                coefficients.pop();
            } else {
                break;
            }
        }

        if coefficients.is_empty() {
            coefficients.push(0);
        }

        Self { coefficients }
    }

    #[inline]
    pub fn coefficients(&self) -> &[i64] {
        self.coefficients.as_slice()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    #[inline]
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}
