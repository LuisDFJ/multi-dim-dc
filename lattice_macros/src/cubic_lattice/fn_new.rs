use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn impl_fn_new(n:u8, k:u8) -> TokenStream {
    let ntag = utils::n(n);
    let n_mid = n_mid(n, k);
    if k > 0 && k < n {
        let n_max = n_max(n);
        quote! {
            pub fn new( id : u32, n : #ntag, stride : u32 ) -> Self {
                let n_mid = (#(#n_mid),*);
                let n_max = #(#n_max)+*;
                //Self { id, n, stride, n_max, n_mid }
                todo!()
            }
        }
    } else {
        let n_max = n_mid[0].clone();
        quote! {
            pub fn new( id : u32, n : #ntag, stride : u32 ) -> Self {
                let n_max = #n_max;
                Self { id, n, stride, n_max }
            }
        }
    }
}

use crate::combinatorics::Lattice;
fn n_mid(n:u8, k:u8) -> Vec<TokenStream> {
    Lattice::new(n).k_combinations(k)
        .iter().map(|s| {
            let p : Vec<TokenStream> = (0..n).into_iter().map(|i| {
                let l = i as usize;
                if s.contains(&i) {
                    quote! { (n.0[#l] - 1) }
                } else {
                    quote! { n.0[#l] }
                }
            }).collect();
            utils::join_op(p, |accum, x| quote! {#accum * #x})
        }).collect()
}

fn n_max(n:u8) -> Vec<TokenStream> {
    (0..n as usize).into_iter().map(|i|{
        let idx = syn::Index::from(i);
        quote! {n_mid.#idx}
    }).collect()
}
