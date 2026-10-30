use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::*;

impl Cell {
    pub fn new( n: u8, k: u8, eps: &Mask, s_x: &Set, x: &Set ) -> Self {
        Self { n, k, mask:eps.clone(), offset:s_x.clone(), face:x.clone() }
    }
    pub fn tokenize(&self) -> TokenStream {
        let tag = format_ident!("C{}", self.k);
        let mut args : Vec<TokenStream> = (0..self.n).into_iter().map(|i| {
            let arg = format_ident!("u");
            let n = i as usize;
            quote! {self.#arg[#n]}
        }).collect();
        for (i,m) in self.mask.iter().enumerate() {
            if *m {
                let offset = self.offset[i] as usize;
                let u = args[offset].clone();
                args[offset] = quote!{ #u + 1 }
            }
        }
        if self.face.len() > 0 {
            let id = self.face.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("");
            let face = format_ident!("E{}", id);
            args.push(quote! { #face });
        }
        quote! {
            #tag::new(#(#args),*)
        }
    }
}
