use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

use super::fn_new::impl_fn_new;
use super::fn_get_s::impl_fn_get_s;
use super::fn_get_cell::impl_fn_get_cell;

pub fn impl_lattice_k(n : u8, k : u8) -> TokenStream {
    let tag = utils::lattice(k);
    let ctag = utils::c(k);
    let fn_new = impl_fn_new(n, k);
    let fn_get_s = impl_fn_get_s(n, k);
    let fn_get_cell = impl_fn_get_cell(n, k);
    quote! {
        impl #tag {
            #fn_new
            #fn_get_s
            #fn_get_cell
        }

        impl Iterator for #tag {
            type Item = #ctag;
            fn next( &mut self ) -> Option<Self::Item> {
                if self.id >= self.n_max { return None }
                let res = Some( self.get_cell() );
                self.id += self.stride;
                return res;
            }
        }
    }
}





