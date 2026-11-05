use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn gen_space(n : u8) -> TokenStream {
    let ptag = utils::p(n);
    let ntag = utils::n(n);
    let dtag = utils::d(n);

    let params_a : Vec<_> = (0..n as usize).into_iter().map(|i|{
        quote! { (d.0[#i].1 - d.0[#i].0) / n.0[#i] as f32 }
    }).collect();
    let idx : Vec<_> = (0..n as usize).collect();

    quote! {
        pub struct Space<F : SDF> {
            a : #ptag,
            b : #ptag,
            f : F,
        }


        impl <F : SDF> Space<F> {
            pub fn new( n : #ntag, d : #dtag, f : F ) -> Self {
                let a = #ptag::new( #(#params_a),*  );
                let b = #ptag::new( #(d.0[#idx].0),*  );
                Self { a,b,f }
            }
            pub fn eval<C : Embeded>(&self, c : &C, d : Option<Dis> ) -> f32 {
                let x = c.embed(&self.a, &self.b, d);
                (self.f)( #(x.0[#idx]),* )
            }
        }
    }
}
