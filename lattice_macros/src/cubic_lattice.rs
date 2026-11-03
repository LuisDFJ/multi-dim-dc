mod types;
mod impl_lattice;
mod fn_new;
mod fn_get_s;
mod fn_get_cell;

use proc_macro2::TokenStream;
use quote::quote;

use self::types::gen_lattice;
pub fn gen_lattice_types(n : u8) -> TokenStream {
    let t : Vec<TokenStream> = (0..=n).into_iter().map(|k| gen_lattice(n, k)).collect();
    quote! {
        #(#t)*
    }
}

pub use self::impl_lattice::impl_lattice_k;
