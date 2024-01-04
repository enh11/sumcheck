pub mod sumcheck;
pub mod prover;
pub mod verifier;
#[cfg(test)]
mod tests {
    use ark_bls12_381::Fr;
    use ark_poly::{multivariate::{SparsePolynomial, SparseTerm, Term}, DenseMVPolynomial};
    use crate::sumcheck::{UniPoly, sum_check, slow_sum};
    use crate::prover::Prover;
    #[test]
    fn test_slow_sum() {

        let poly: SparsePolynomial<Fr, SparseTerm> = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
                (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        let h = slow_sum(&poly);
        assert_eq!(h, 12.into());
    }
    #[test]
    fn test_gen_unipoly() {
        let poly: SparsePolynomial<Fr, SparseTerm> = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
                (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        let mut pr=Prover::initialize(&poly);
        let expected_unipoly= UniPoly::from_coefficients_vec(vec![(0,Fr::from(34)),(1,Fr::from(1))]);
        let g1=pr.gen_unipoly(Some(Fr::from(2)));
        assert_eq!(g1,expected_unipoly);
    }
#[test]
fn test_sum_check(){
    let v=Fr::from(12);
    let g: SparsePolynomial<Fr, SparseTerm> = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
    assert!(sum_check(g, v));
}
}
