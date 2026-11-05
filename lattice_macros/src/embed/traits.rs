use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn gen_embeded_traits(n : u8) -> TokenStream {
    let p = params_f32(n);
    let d = params_dis(n);
    let ptag = utils::p(n);
    quote! {
        pub trait SDF : Fn(#(#p),*) -> f32 + Send + Sync + 'static {}

        impl <F> SDF for F where F : Fn(#(#p),*) -> f32 + Send + Sync + 'static {}

        pub enum Dis {
            #(#d),*
        }
        pub trait Embeded {
            fn embed( &self, a : &#ptag, b : &#ptag, d : Option<Dis> ) -> #ptag;
        }
    }
}

fn params_f32(k:u8) -> Vec<TokenStream> {
    (0..k).into_iter().map(|_| quote! {f32}).collect()
}

fn params_dis(n:u8) -> Vec<TokenStream> {
    (0..=n).into_iter().map(|k| {
        let tag = utils::s(k);
        let p = params_f32(k);
        quote! { #tag(#(#p),*) }
    }).collect()
}
