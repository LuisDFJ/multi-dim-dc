use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_u_n_types( n: u8 ) -> TokenStream {
    let utag = quote::format_ident!( "U{}", n );
    let ntag = quote::format_ident!( "N{}", n );
    let len = n as usize;

    let names: Vec<_> = (0..n).map(|i| format_ident!("x{}", i)).collect();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct #utag(pub [i32;#len]);

        #[derive(Debug, Clone, Copy)]
        pub struct #ntag(pub [u32;#len]);

        impl #utag {
            pub fn new( #(#names : i32),* ) -> Self {
                Self([ #(#names),* ])
            }
        }

        impl #ntag {
            pub fn new( #(#names : u32),* ) -> Self {
                Self([ #(#names),* ])
            }
        }
    }
}

pub fn gen_e_enums( _: u8 ) -> TokenStream {
    quote! {

    }
}

