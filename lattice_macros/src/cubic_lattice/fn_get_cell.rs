use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn impl_fn_get_cell(n:u8, k:u8) -> TokenStream {
    let ctag = utils::c(k);
    let p = get_u(n);
    if k > 0 && k < n {
        quote! {
            pub fn get_cell( &self ) -> #ctag {
                let s = self.get_s();
                let e = self.get_e();
                #ctag::new( #((#p) as i32),* , e )
            }
        }
    } else {
        quote! {
            pub fn get_cell( &self ) -> #ctag {
                let s = self.get_s();
                #ctag::new( #((#p) as i32),* )
            }
        }
    }
}

fn get_u(n:u8) -> Vec<TokenStream> {
    (0..n as usize).into_iter().map(|i| {
        let s : Vec<_> = (0..i).into_iter().map(|j| {
            quote! { s.0[#j]}
        }).collect();
        if i == 0 {
            quote! { self.id % s.0[#i] }
        } else {
            let denominator = utils::join_op(s, |acc,x| quote! {#acc * #x});
            quote! { ( self.id / (#denominator) ) % s.0[#i] }
        }
    }
    ).collect()
}
