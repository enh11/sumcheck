use ark_bls12_381::Fr;
use ark_ff::{Fp, MontBackend};
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial};
use sumcheck::sumcheck::{sum_check,slow_sum};
fn main() {
    let poly: SparsePolynomial<Fp<MontBackend<ark_bls12_381::FrConfig, 4>, 4>, SparseTerm> = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
let h=slow_sum(&poly);
println!("Sumcheck protocol with polynomial {:?}.\n Value to be tested is h = {}",poly,h);
sum_check(poly, h);
}
