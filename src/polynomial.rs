use ark_ff::Field;
use ark_ff::PrimeField;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MultiVariatePolynomial<F: PrimeField> {
    pub coefficients: HashMap<Vec<u64>, F>,
    pub num_variables: usize,
}

impl<F: PrimeField> MultiVariatePolynomial<F> {
    pub fn new(coefficients: HashMap<Vec<u64>, F>, num_variables: usize) -> Self {
        Self {
            coefficients,
            num_variables,
        }
    }

    pub fn evaluate(&self, point: &[F]) -> F {
        let mut result = F::zero();
        for (degrees, coeff) in self.coefficients.iter() {
            let mut term = *coeff;
            for (var, deg) in degrees.iter().enumerate() {
                term *= point[var].pow(&[*deg]);
            }
            result += term;
        }
        result
    }

    pub fn get_univariate_at_round(
        &self,
        i: usize,
        previous_values: &[F],
    ) -> UnivariatePolynomial<F> {
        let mut uni_coeffs = HashMap::new();

        for (degrees, coeff) in self.coefficients.iter() {
            let mut term_coeff = *coeff;

            for (j, val) in previous_values.iter().enumerate() {
                if j != i {
                    term_coeff *= val.pow(&[degrees[j]]);
                }
            }

            let degree = degrees[i];
            *uni_coeffs.entry(degree).or_insert(F::zero()) += term_coeff;
        }

        UnivariatePolynomial {
            coefficients: uni_coeffs,
        }
    }
}

#[derive(Clone)]
pub struct UnivariatePolynomial<F: Field> {
    pub coefficients: HashMap<u64, F>,
}

impl<F: Field> UnivariatePolynomial<F> {
    pub fn evaluate(&self, point: F) -> F {
        let mut result = F::zero();
        for (deg, coeff) in self.coefficients.iter() {
            result += *coeff * point.pow(&[*deg]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Prover;
    use crate::Verifier;

    #[test]
    fn test_sumcheck_protocol() {
        // 簡単な2変数多項式 f(x,y) = x + 2y を作成
        let mut coeffs = HashMap::new();
        coeffs.insert(vec![1, 0], F::from(1)); // x項
        coeffs.insert(vec![0, 1], F::from(2)); // y項

        let polynomial = MultiVariatePolynomial::new(coeffs, 2);

        // 正しい合計値を計算 (f(0,0) + f(0,1) + f(1,0) + f(1,1))
        let true_sum = F::from(6);

        let mut prover = Prover::new(polynomial.clone());
        let mut verifier = Verifier::new(polynomial, true_sum);

        assert!(verifier.verify(&mut prover));
    }
}

// ark:mleで書き直す
