use ark_bls12_381::Fr;
use ark_ff::{Fp, MontBackend};
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial};
use sumcheck::sumcheck::{sum_check,slow_sum_g, n_to_vec, Prover, UniPoly};
fn main() {
let term = SparseTerm::new(vec![(2 as usize,3 as usize),(1 as usize,1 as usize),(0 as usize,1 as usize)]);
let n:usize=12;
let n_vec=n_to_vec(n, 3);

println!("vec_2 {:?}",n_vec[0]);
println!("vec_2 {:?}",n_vec[1]);
println!("vec_2 {:?}",n_vec[2]);
println!("vec_2 {:?}",n_vec[3]);



println!("term is {:?}",term);
    let poly: SparsePolynomial<Fp<MontBackend<ark_bls12_381::FrConfig, 4>, 4>, SparseTerm> = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
let mut pr=Prover::initialize(&poly);
let (mut coeff,mut lett_part)=pr.eval_term(&term,&n_vec);
println!("evaluation of term {:?} at point {:?} is {:?}",term,n_vec,(coeff,lett_part));
pr.r_vec.push(Fr::from(2));
(coeff,lett_part)=pr.eval_term(&term, &n_vec);
println!("evaluation of term {:?} at point {:?} is {:?}",term,n_vec,(coeff,lett_part));

let expected_unipoly= UniPoly::from_coefficients_vec(vec![(0,Fr::from(34)),(1,Fr::from(1))]);
println!("exp_poly {:?}",expected_unipoly);
let g1=pr.gen_unipoly(None);
println!("g0 is {:?}",g1);
//println!("poly is {:?}",poly);
//println!("number of variable of poly is {}",n_var);
let h=slow_sum_g(&poly);
println!("h is {}",h);
sum_check(poly, h);
}
