use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn gen_lattice(n : u8, k : u8) -> TokenStream {
    let tag = utils::lattice(k);
    let ntag = utils::n(n);
    if k > 0 && k < n {
        let p : Vec<TokenStream> = (0..n).into_iter().map(|_| quote! {u32}).collect();
        quote! {
            pub struct #tag {
                id: u32,
                n : #ntag,
                stride: u32,
                n_max: u32,
                n_mid: (#(#p),*),
            }
        }
    } else {
        quote! {
            pub struct #tag {
                id: u32,
                n : #ntag,
                stride: u32,
                n_max: u32,
            }
        }
    }
}
