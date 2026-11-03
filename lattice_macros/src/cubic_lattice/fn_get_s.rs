use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn impl_fn_get_s(n:u8, k:u8) -> TokenStream {
    let ntag = utils::n(n);
    let etag = utils::e(k);
    let i_cases = i_cases(n, k);
    if k > 0 && k < n {
        let e_cases = e_cases(n, k);
        quote! {
            fn get_e(&self) -> #etag {
                match self.id {
                    #(#e_cases),*,
                    _ => panic!("Id is out of bound!"),
                }
            }

            fn get_s( &self ) -> #ntag {
                let mut s = self.n;
                match self.get_e() {
                    #(#i_cases),*
                }
                s
            }
        }
    } else {
        let i_case = i_case(n,k);
        quote! {
            fn get_s( &self ) -> #ntag {
                #i_case
            }
        }
    }
}

fn e_cases(n:u8, k:u8) -> Vec<TokenStream> {
    let etag = utils::e(k);
    (0..n).into_iter().map(|i| {
        let j : Vec<_> = (0..=i as usize).into_iter().map(|i| syn::Index::from(i)).collect();
        quote! { i if i < #(self.n_mid.#j)+* => #etag::from(#i)  }
    }).collect()
}

use crate::combinatorics::Lattice;
fn i_cases(n:u8, k:u8) -> Vec<TokenStream> {
    Lattice::new(n).k_combinations(k)
        .iter().map(|s| {
            let itag = s.tokenize(true);
            let p : Vec<TokenStream> = (0..n).into_iter()
                .filter(|i| s.contains(i)).map(|i| {
                let i = i as usize;
                quote! { s.0[#i] -= 1 }
            }).collect();
            quote! { #itag => { #(#p;)* } }
        }).collect()
}

fn i_case(n:u8, k:u8) -> TokenStream {
    match Lattice::new(n).k_combinations(k).iter().next() {
        Some(s) => {
            let p : Vec<TokenStream> = (0..n).into_iter()
                .filter(|i| s.contains(i)).map(|i| {
                    let i = i as usize;
                    quote! { s.0[#i] -= 1 }
                }).collect();
            if p.len() == 0 {
                quote! { self.n }

            } else {
                quote! {
                    let mut s = self.n;
                    #(#p;)*
                    s
                }
            }
        },
        None => quote! {},
    }
}
