use proc_macro2::TokenStream;
use quote::quote;

use crate::utils;

pub fn impl_embeded_k(n:u8, k:u8) -> TokenStream {
    let ctag = utils::c(k);
    let ptag = utils::p(n);
    if k == 0 {
        quote! {
            impl Embeded for #ctag {
                fn embed( &self, a : &#ptag, b : &#ptag, _ : Option<Dis> ) -> #ptag {
                    #ptag::new(
                        0.0,0.0,0.0
                    )
                }
            }
        }
    } else {
        quote! {
            impl Embeded for #ctag {
                fn embed( &self, a : &#ptag, b : &#ptag, d : Option<Dis> ) -> #ptag {
                    #ptag::new(
                        0.0,0.0,0.0
                    )
                }
            }
        }
    }
}
