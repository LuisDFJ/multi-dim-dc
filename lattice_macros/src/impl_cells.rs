use proc_macro2::TokenStream;
use quote::quote;
use crate::{combinatorics::Lattice, utils};

pub fn impl_c_k( n: u8, k: u8 ) -> TokenStream {
    let tag = utils::c(k);
    let fns = gen_fn_get(n,k);
    quote! {
        impl #tag {
            #(#fns)*
        }
    }
}

fn gen_fn_get( n: u8, k: u8 ) -> Vec<TokenStream> {
    let mut out = Vec::new();
    for m in 0..=n {
        if m != k {
            out.push( gen_fn_k_m(n, k, m) );
        }
    }
    out
}

fn gen_fn_k_m( n: u8, k: u8, m: u8 ) -> TokenStream {
    let tag = utils::get_c(m);
    let l = Lattice::adj_size(n, k, m);
    let impl_match = gen_match(n, k, m);
    quote! {
        pub fn #tag( &self ) -> [Self;#l] {
            #impl_match
        }
    }
}

type Case = (Option<TokenStream>, Vec<TokenStream>);
fn gen_match( n: u8, k: u8, m: u8 ) -> TokenStream {
    let cases : Vec<TokenStream> = gen_cases_k_m(n, k, m)
        .into_iter()
        .map(|c| parse_case(c))
        .collect();
    if cases.len() == 1 {
        quote! { #(#cases)* }
    } else {
        quote! {
            match self.e {
                #(#cases),*
            }
        }
    }
}

fn parse_case( (tag, cells): Case ) -> TokenStream {
    if let Some(tag) = tag {
        quote! { #tag => [ #(#cells),* ] }
    } else {
        quote! { [ #(#cells),* ] }
    }
}

fn gen_cases_k_m( n: u8, k: u8, m: u8 ) -> Vec<Case> {
    let lattice = Lattice::new(n);
    lattice.k_combinations(k).into_iter()
        .map(|s| {
            let cells = lattice.adj_cells(m, &s)
                .into_iter().map(|c| c.tokenize()).collect();
            let case = if k > 0 && k < n {
                Some(s.tokenize(true))
            } else { None };
            (case, cells)
        }).collect()
}
