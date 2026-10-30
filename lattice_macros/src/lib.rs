mod utils;
mod types;
mod cubic_lattice;
mod combinatorics;
mod impl_cells;

use proc_macro::TokenStream;
use syn::{parse_macro_input,LitInt};

#[proc_macro]
pub fn define_types( input : TokenStream ) -> TokenStream {
    let n : u8 = parse_macro_input!(input as LitInt)
        .base10_parse()
        .expect("dimension must be an u8");
    let mut out = proc_macro2::TokenStream::new();
    out.extend(types::gen_u_n_types(n));
    out.extend(types::gen_e_enums(n));
    out.extend(types::gen_c_types(n));
    out.extend(cubic_lattice::gen_lattice_types(n));
    out.into()
}

#[proc_macro]
pub fn implement_cells( input : TokenStream ) -> TokenStream {
    let n : u8 = parse_macro_input!(input as LitInt)
        .base10_parse()
        .expect("dimension must be an u8");
    let mut out = proc_macro2::TokenStream::new();
    for k in 0..=n {
        out.extend(impl_cells::impl_c_k(n, k));
    }
    out.into()
}

