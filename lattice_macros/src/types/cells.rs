use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn gen_c_types( n: u8 ) -> TokenStream {
    let mut out = TokenStream::new();
    for k in 0..=n {
        out.extend(gen_c_k(n, k));
    }
    out
}

fn gen_c_k( n: u8, k: u8 ) -> TokenStream {
    let tag = utils::c(k);
    let utag = utils::u(n);
    let etag = utils::e(k);
    let new_params = utils::params("u", n);
    let derives = utils::derives();
    if k > 0 && k < n {
        quote! {
            #derives
            pub struct #tag {
                pub u : #utag,
                pub e : #etag,
            }

            impl #tag {
                pub fn new( #(#new_params : i32),*, e : #etag ) -> Self {
                    Self{ u : #utag([ #(#new_params),* ]), e }
                }
            }
        }
    } else {
        quote! {
            #derives
            pub struct #tag {
                pub u : #utag,
            }

            impl #tag {
                pub fn new( #(#new_params : i32),* ) -> Self {
                    Self{ u : #utag([ #(#new_params),* ]) }
                }
            }
        }
    }
}
