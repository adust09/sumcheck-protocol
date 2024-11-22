use crate::polynomial::MultiVariatePolynomial;
use crate::polynomial::UnivariatePolynomial;
use ark_ff::PrimeField;

pub struct Prover<F: PrimeField> {
    polynomial: MultiVariatePolynomial<F>,
    current_round: usize,
    values_so_far: Vec<F>,
}

impl<F: PrimeField> Prover<F> {
    pub fn new(polynomial: MultiVariatePolynomial<F>) -> Self {
        Self {
            polynomial,
            current_round: 0,
            values_so_far: Vec::new(),
        }
    }

    pub fn get_next_polynomial(&self) -> UnivariatePolynomial<F> {
        self.polynomial
            .get_univariate_at_round(self.current_round, &self.values_so_far)
    }

    pub fn receive_challenge(&mut self, challenge: F) {
        self.values_so_far.push(challenge);
        self.current_round += 1;
    }
}
