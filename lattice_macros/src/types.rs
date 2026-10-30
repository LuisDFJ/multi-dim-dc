use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

fn derives() -> TokenStream {
    //quote! { #[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)] }
    quote! {}
}

pub fn gen_u_n_types( n: u8 ) -> TokenStream {
    let utag = utils::u(n);
    let ntag = utils::n(n);
    let names = utils::params("x", n);

    let len = n as usize;
    let derives = derives();
    quote! {
        #derives
        pub struct #utag(pub [i32;#len]);

        #derives
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

pub fn gen_e_enums( n: u8 ) -> TokenStream {
    let mut out = TokenStream::new();
    for i in 1..n {
        out.extend(gen_e(n, i));
    }
    out
}

use crate::combinatorics::Lattice;
fn gen_e( n: u8, k:u8 ) -> TokenStream {
    let tag = utils::e(k);
    let mut params : Vec<TokenStream> = Vec::new();
    let mut cases : Vec<TokenStream> = Vec::new();
    for (i,s) in Lattice::new(n).k_combinations(k).into_iter().enumerate() {
        params.push( s.tokenize(false) );
        let case = s.tokenize(true);
        let i = i as u8;
        cases.push(quote! { #i => #case, });
    }
    let derives = derives();
    quote! {
        #derives
        pub enum #tag {
            #(#params),*
        }

        impl #tag {
            pub fn from( i : u8 ) -> Self {
                match i {
                    #(#cases)*
                    _ => panic!("Invalid Enum Convserion!"),
                }
            }
        }
    }
}

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
    let derives = derives();
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


