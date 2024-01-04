use ark_std::{One,Zero};
use ark_bls12_381::Fr;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::polynomial::{DenseMVPolynomial, Polynomial};
use ark_poly::univariate::SparsePolynomial as UniSparsePolynomial;
use crate::prover::Prover;
use crate::verifier::Verifier;

pub type MultyPoly= SparsePolynomial<Fr, SparseTerm>; //This is for the original polinomial g
pub type UniPoly = UniSparsePolynomial<Fr>; // This is for the polinomial gj generated in the j-th round

/*The verifier is represented by the following */
pub fn n_to_vec(i:usize,v:usize)->Vec<Fr>{
    format!("{:0>width$}", format!("{:b}", i), width = v)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()}

pub fn slow_sum(g:&MultyPoly)->Fr {
    let v=g.num_vars();
    (0..1<<v).map(|x| g.evaluate(&n_to_vec(x as usize, v))).sum()
    }

pub fn sum_check(g:MultyPoly,v:Fr)->bool{
    let mut pr=Prover::initialize(&g);
    let mut vr=Verifier::initialize(&g, v);
    /*This is the starting round. The prover calculate g_0 */
    let mut g_i:UniPoly=pr.gen_unipoly(None); /*Prover sends g_0 to Verifier */
    
    /*Verifier checks g0(0)+g1(1)=v and checks the degrees. 
    If it is all right then "Round 0 is ok" and the iteration starts.
    Verifier generate a random r in Fr, computes g0(r)+g1(r) and sends r to the Prover
    The prover generate the univariate g1(x1) and sends it back to the Verifier. Verifier checks g1(0)+g1(1)=?g0(r) */
    let expected_value= g_i.evaluate(&Fr::zero())+g_i.evaluate(&Fr::one());  
    let degrees=vr.degrees_look_up();
    assert_eq!(expected_value,v);
    assert!(g_i.degree() <= degrees[0]);
    println!("Round 0 is ok!");

    let mut r: Option<Fr>;
    let mut expected_value:Fr;

    for i in 1..g.num_vars{
        r= vr.get_random();//verifier generates a random r in the field and sends it to the prover
        expected_value=vr.next_expected_value(&g_i,r.unwrap());// verifier computes the expected value for the next round.
        
        g_i=pr.gen_unipoly(r);//Prover gets r and generates a univariate polinomial and sends it to the verifier.
        vr.check(&g_i,expected_value,&i);// verifiers check value and degree
        println!("Round {} is ok!",i);
        }
    //Last round with call to the oracle
    
    r=vr.get_random();
    pr.r_vec.push(r.unwrap());
    expected_value=vr.next_expected_value(&g_i,r.unwrap());// verifier computes the expected value for the last check.

    assert_eq!(vr.r_vec,pr.r_vec);
    
    vr.call_to_the_oracle(expected_value);
    println!("Final check with call to the oracle is ok!");
    true
}
