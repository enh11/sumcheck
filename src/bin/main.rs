use ark_bls12_381::Fr;
use ark_ff::{Fp, MontBackend};
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial};
use sumcheck::sumcheck::{sum_check,slow_sum_g, n_to_vec};
fn main() {
    let num:usize=8;
    let vec=n_to_vec(8, 8);
    println!("{} to vec is {:?}",num,vec);
    let poly: SparsePolynomial<Fp<MontBackend<ark_bls12_381::FrConfig, 4>, 4>, SparseTerm> = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
//println!("poly is {:?}",poly);
//println!("number of variable of poly is {}",n_var);
let h=slow_sum_g(&poly);
println!("h is {}",h);
sum_check(poly, h);
}
