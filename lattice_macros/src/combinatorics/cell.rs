use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

use super::*;

impl Cell {
    pub fn new( n: u8, k: u8, m: u8, eps: &Mask, s_x: &Set, x: &Set ) -> Self {
        Self { n, k, m, mask:eps.clone(), offset:s_x.clone(), face:x.clone() }
    }
    pub fn tokenize(self) -> TokenStream {
        let tag = utils::c(self.k);
        let mut params = utils::params_idx("u", self.n);
        for (i,flag) in self.mask.into_iter().enumerate() {
            if flag {
                let offset = self.offset[i] as usize;
                let u = params[offset].clone();
                //params[offset] = quote!{ #u + 1 }
                if self.k < self.m {
                    params[offset] = quote!{ #u - 1 }
                } else {
                    params[offset] = quote!{ #u + 1 }
                }
            }
        }
        let mut params = quote! { #(self.#params),* };
        if self.face.len() > 0 && self.face.len() < self.n as usize {
            params.extend(self.face.tokenize(true));
        }
        quote! {
            #tag::new(#params)
        }
    }
}


