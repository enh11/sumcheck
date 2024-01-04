use ark_std::cfg_into_iter;
use ark_bls12_381::Fr;
use ark_std::{Zero,One};
use ark_poly::Polynomial;
use ark_poly::polynomial::DenseMVPolynomial;
use rand::Rng;
use crate::sumcheck::{UniPoly,MultyPoly};


pub struct Verifier{
    pub g: MultyPoly,//The original polinomial
    pub check_value:Fr,
    pub r_vec:Vec<Fr>
}
impl Verifier {
    pub fn initialize(g:&MultyPoly,v:Fr)->Self{
        Verifier { g: g.clone(), check_value: v,r_vec: vec![] }
    }
    pub fn get_random(&mut self)->Option<Fr> {
        let mut rng = rand::thread_rng();
        let r: Fr = rng.gen();
        self.r_vec.push(r);
        Some(r)
    }
    pub fn next_expected_value(&self,g:&UniPoly,r:Fr)->Fr{
        g.evaluate(&r)
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
    pub fn check(&self, g_i:&UniPoly,expected_value:Fr,round:&usize){
        let val:Fr=g_i.evaluate(&Fr::zero())+g_i.evaluate(&Fr::one());
        let degrees=self.degrees_look_up();
        assert_eq!(expected_value,val);
        assert!(g_i.degree()<=degrees[*round]);
    }   
    pub fn call_to_the_oracle(&self,expected_value:Fr){
        assert_eq!(self.g.evaluate(&self.r_vec),expected_value);
    }
}

