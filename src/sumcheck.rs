use ark_ff::Field;
use ark_bls12_381::Fr;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{DenseMVPolynomial, Polynomial};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_std::cfg_into_iter;
use rand::Rng;

pub type ScalarField=Fr;
pub type MultyPoly= SparsePolynomial<ScalarField, SparseTerm>; //This is for the original polinomial g
pub type UniPoly = UniSparsePolynomial<ScalarField>; // This is for the polinomial gj generated in the j-th round

#[derive(Debug, Clone)]
pub struct Prover{
    pub g: MultyPoly,//The original polinomial
    pub r_vec: Vec<ScalarField>, //This is a growing vector; it grows within rounds
}
impl Prover{
    pub fn initialize(g: &MultyPoly)->Self{
        Prover { g: g.clone(), r_vec: vec![] }}
    
    pub fn gen_unipoly(&mut self,r:Option<ScalarField>)->UniPoly{
/*This function takes as input the random value (generated by the verifier at the j-th round) and it outputs
the univariant polinomial g_j(x_j) */
if r.is_some() {
    self.r_vec.push(r.unwrap());
  }
  let v = self.g.num_vars() - self.r_vec.len();
  (0..(2u32.pow(v as u32 - 1))).fold(
    UniPoly::from_coefficients_vec(vec![(0, 0u32.into())]),
    |sum, n| sum + self.evaluate_gj(n_to_vec(n as usize,v)),
  )
}
fn evaluate_gj(&self, points: Vec<ScalarField>)->UniPoly{
    cfg_into_iter!(self.g.terms()).fold(
        UniPoly::from_coefficients_vec(vec![]),
        |sum,(coeff,term)| {
            let (coeff_eval, letteral_part)=self.eval_term(term, &points);
            let curr = match letteral_part {
                None=>UniPoly::from_coefficients_vec(vec![(0,coeff*&coeff_eval)]),
                _=>UniPoly::from_coefficients_vec(vec![(letteral_part.unwrap().degree(),coeff*&coeff_eval)])
                
            };
            sum+curr
        })
}
fn eval_term(&self, term:&SparseTerm,points: &Vec<ScalarField>)->(ScalarField,Option<SparseTerm>){
    /*this funtion outputs the evaluation of the letteral part of the monomial (term) 
    by substituting values in points*/
    let mut fixed_term:Option<SparseTerm>=None; //initialize fixed_term to none
    let coeff:ScalarField=cfg_into_iter!(term).fold(1u32.into(),|product,(var,exp)| match *var {
        /*here j is an alias for *var */
        j if j==self.r_vec.len()=>{
                //println!("j is equal to r_vec.len() that is {}",j);
                fixed_term=Some(SparseTerm::new(vec![(j,*exp)]));
                product }
        j if j<self.r_vec.len()=>{ 
                //println!("j is les than r_vec.len () that is {}",j);
                self.r_vec[j].pow(&[*exp as u64])*product}
    _=> points[*var-self.r_vec.len()].pow(&[*exp as u64])*product});
    

    (coeff,fixed_term)
}
}

/*The verifier is represented by the following */
pub struct Verifier{
    pub g: MultyPoly,//The original polinomial
    pub check_value:ScalarField
}
impl Verifier {
    pub fn initialize(g:&MultyPoly,v:ScalarField)->Self{
        Verifier { g: g.clone(), check_value: v }
    }
    pub fn get_random(&self)->Option<ScalarField> {
        let mut rng = rand::thread_rng();
        let r: ScalarField = rng.gen();
        Some(r)
    }
    pub fn degrees_look_up(&self)->Vec<usize>{
        let mut lookup:Vec<usize>=vec![0;self.g.num_vars()];
        cfg_into_iter!(self.g.terms()).for_each(|(_,term)| {
            cfg_into_iter!(term).for_each(|(var,exp)|{
                if *exp>lookup[*var]{lookup[*var]=*exp}
        });
        });
        lookup
    }    
}
pub fn n_to_vec(i:usize,v:usize)->Vec<ScalarField>{
    format!("{:0>width$}", format!("{:b}", i), width = v)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()}

pub fn slow_sum_g(g:&MultyPoly)->ScalarField {
    let v=g.num_vars();
    let n=2u32.pow(v as u32);
        (0..n).map(|x| g.evaluate(&n_to_vec(x as usize, v))).sum()
    }
pub fn sum_check(g:MultyPoly,v:ScalarField)->bool{
    let mut pr=Prover::initialize(&g);
    let vr=Verifier::initialize(&g, v);
    /*This is the starting round. The prover calculate g_0 */
    let mut g_i=pr.gen_unipoly(None); /*Prover sends g_0 to Verifier */
    let expected_value= g_i.evaluate(&ScalarField::from(0))+g_i.evaluate(&ScalarField::from(1));
    let degrees=vr.degrees_look_up();
    assert_eq!(expected_value,v);
    assert!(g_i.degree() <= degrees[0]);
    println!("Round 0 ok!");
    let mut r: Option<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>>;/*Verifier gets a random element in the field and sends it to prover */
    let mut expected_value:ScalarField;/*Verifier calculate g_i(r); this is the next value to be checked */
    let mut v:ScalarField;
    for i in 1..g.num_vars{
        r= vr.get_random();
        expected_value=g_i.evaluate(&ScalarField::from(r.unwrap()));
        g_i=pr.gen_unipoly(r);
        v=g_i.evaluate(&ScalarField::from(0))+g_i.evaluate(&ScalarField::from(1));
        assert_eq!(expected_value,v);
        assert!(g_i.degree()<=degrees[i]);
        println!("Round {} ok!",i);
        }
    r=vr.get_random();
    pr.r_vec.push(r.unwrap());
    expected_value=g_i.evaluate(&ScalarField::from(r.unwrap()));
    v=pr.g.evaluate(&pr.r_vec);
    assert_eq!(expected_value,v);
    println!("Final round ok!");
    true

}