use proc_macro2::TokenStream;
use quote::{IdentFragment, format_ident, quote};

fn tag<T: IdentFragment>(id: &str, i: T) -> TokenStream {
    let tag = format_ident!("{}{}", id, i);
    quote! {#tag}
}

fn tag_idx(id: &str, i: usize) -> TokenStream {
    let tag = format_ident!("{}", id);
    quote! {#tag[#i]}
}

pub fn u(n:u8) -> TokenStream { tag("U", n) }

pub fn n(n:u8) -> TokenStream { tag("N", n) }

pub fn c(k:u8) -> TokenStream { tag("C", k) }

pub fn e(k:u8) -> TokenStream { tag("E", k) }

pub fn i(id: &String) -> TokenStream { tag("I", id) }

pub fn get_c(m:u8) -> TokenStream { tag("get_c", m) }

pub fn params( id: &str, n: u8 ) -> Vec<TokenStream> {
    (0..n).into_iter().map( |i| tag(&id, i) ).collect()
}

pub fn params_idx( id: &str, n: u8 ) -> Vec<TokenStream> {
    (0..n).into_iter().map( |i| tag_idx(&id, i as usize) ).collect()
}
