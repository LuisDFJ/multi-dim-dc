use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn gen_d_p_types(n:u8) -> TokenStream {
    let dtag = utils::d(n);
    let ptag = utils::p(n);
    let l = n as usize;

    let p = utils::params("x", n);
    let g = p.iter().map(|x| quote! {#x.0 >= #x.1}).collect();
    let guard = utils::join_op(g, |acc,x| quote! { #acc || #x });
    let derives = utils::derives_float();
    quote! {
        #derives
        pub struct #dtag ( pub [(f32,f32);#l] );
        #derives
        pub struct #ptag ( pub [f32;#l] );

        impl #dtag {
            pub fn new( #(#p : (f32,f32)),* ) -> Self {
                if #guard {
                    panic!("Dimensions mismatch: xi.0 must be less than xi.1")
                }
                Self( [#(#p),*] )
            }
        }

        impl #ptag {
            pub fn new( #(#p : f32),* ) -> Self {
                Self( [#(#p),*] )
            }
        }
    }
}
