use crate::{polynomial::MultiVariatePolynomial, Prover};
use ark_ff::Field;
use ark_ff::PrimeField;

pub struct Verifier<F: PrimeField> {
    polynomial: MultiVariatePolynomial<F>,
    claimed_sum: F,
    current_round: usize,
    values_so_far: Vec<F>,
}

impl<F: PrimeField> Verifier<F> {
    pub fn new(polynomial: MultiVariatePolynomial<F>, claimed_sum: F) -> Self {
        Self {
            polynomial,
            claimed_sum,
            current_round: 0,
            values_so_far: Vec::new(),
        }
    }

    pub fn verify(&mut self, prover: &mut Prover<F>) -> bool {
        let mut current_sum = self.claimed_sum;

        // すべての変数に対してラウンドを実行
        for _i in 0..self.polynomial.num_variables {
            // Proverから単変数多項式を受け取る
            let uni_poly = prover.get_next_polynomial();

            // 評価点0と1での値の合計が前のラウンドの値と一致することを確認
            let sum_at_endpoints = uni_poly.evaluate(F::zero()) + uni_poly.evaluate(F::one());
            if sum_at_endpoints != current_sum {
                return false;
            }

            // ランダムな挑戦点を生成（実際の実装ではより安全な乱数生成が必要）
            let challenge = F::from(2u64); // テスト用の固定値

            // 次のラウンドのための合計値を更新
            current_sum = uni_poly.evaluate(challenge);

            // Proverに挑戦点を送信
            prover.receive_challenge(challenge);
            self.values_so_far.push(challenge);
            self.current_round += 1;
        }

        // 最終的な評価値が正しいことを確認
        let final_eval = self.polynomial.evaluate(&self.values_so_far);
        final_eval == current_sum
    }
}
