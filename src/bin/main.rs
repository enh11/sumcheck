use ark_ff::{Fp, MontBackend};
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial};
use sumcheck::sumcheck::{ScalarField,sum_check,slow_sum_g};
fn main() {
    let poly: SparsePolynomial<Fp<MontBackend<ark_bls12_381::FrConfig, 4>, 4>, SparseTerm> = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (ScalarField::from(2), SparseTerm::new(vec![(0, 3)])),
            (ScalarField::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (ScalarField::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
//println!("poly is {:?}",poly);
//println!("number of variable of poly is {}",n_var);
let h=slow_sum_g(&poly);
sum_check(poly, h);
}
