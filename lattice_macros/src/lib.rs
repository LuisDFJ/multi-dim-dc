mod types;
mod combinatorics;

use proc_macro::TokenStream;
use syn::{parse_macro_input,LitInt};

#[proc_macro]
pub fn define_u_n_types( input : TokenStream ) -> TokenStream {
    let n : u8 = parse_macro_input!(input as LitInt)
        .base10_parse()
        .expect("dimension must be an u8");
    types::gen_u_n_types(n).into()
}

