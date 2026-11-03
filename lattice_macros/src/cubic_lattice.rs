mod types;
mod impl_lattice;

use proc_macro2::TokenStream;
use quote::quote;

use self::types::gen_lattice;
pub fn gen_lattice_types(n : u8) -> TokenStream {
    let t : Vec<TokenStream> = (0..=n).into_iter().map(|k| gen_lattice(n, k)).collect();
    quote! {
        #(#t)*
    }
}

use self::impl_lattice::impl_lattice;
pub fn gen_lattice_impls(n : u8) -> TokenStream {
    let t : Vec<TokenStream> = (0..=n).into_iter().map(|k| impl_lattice(n, k)).collect();
    quote! {
        #(#t)*
    }
}
