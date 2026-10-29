use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::*;

impl Cell {
    pub fn new( n: u8, k: u8, eps: &Mask, s_x: &Set, x: &Set ) -> Self {
        Self { n, k, mask:eps.clone(), offset:s_x.clone(), face:x.clone() }
    }

    pub fn tokenize(&self) -> TokenStream {
        let tag = format_ident!("Cell{}", self.k);
        let args : Vec<TokenStream> = (0..self.n).into_iter().map(|i| {
            let d = if self.mask[i as usize] { " + " } else { "" };
            quote! {
                format_ident!("u{i}{d}")
            }
        }).collect();
        quote! {
            #tag::new(#(#args),*)
        }
    }
}
