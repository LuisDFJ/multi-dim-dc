use proc_macro2::TokenStream;
use quote::quote;
use crate::utils;

pub fn impl_lattice(n : u8, k : u8) -> TokenStream {
    let tag = utils::lattice(k);
    let fn_new = impl_fn_new(n, k);
    let fn_get_s = impl_fn_get_s(n, k);
    let fn_get_cell = impl_fn_get_cell(n, k);
    quote! {
        impl #tag {
            #fn_new
            #fn_get_s
            #fn_get_cell
        }
    }
}

fn impl_fn_new(n:u8, k:u8) -> TokenStream {
    let ntag = utils::n(n);
    let n_mid = n_mid(n, k);
    if k > 0 && k < n {
        let n_max = n_max(n);
        quote! {
            pub fn new( id : u32, n : #ntag, stride : u32 ) -> Self {
                let n_mid = (#(#n_mid),*);
                let n_max = #(#n_max)+*;
                //Self { id, n, stride, n_max, n_mid }
                todo!()
            }
        }
    } else {
        let n_max = n_mid[0].clone();
        quote! {
            pub fn new( id : u32, n : #ntag, stride : u32 ) -> Self {
                let n_max = #n_max;
                Self { id, n, stride, n_max }
            }
        }
    }
}

fn impl_fn_get_s(n:u8, k:u8) -> TokenStream {
    let ntag = utils::n(n);
    let etag = utils::e(k);
    let i_cases = i_cases(n, k);
    if k > 0 && k < n {
        let e_cases = e_cases(n, k);
        quote! {
            fn get_e(&self) -> #etag {
                match self.id {
                    #(#e_cases),*,
                    _ => panic!("Id is out of bound!"),
                }
            }

            fn get_s( &self ) -> #ntag {
                let mut s = self.n;
                match self.get_e() {
                    #(#i_cases),*
                }
                s
            }
        }
    } else {
        let i_case = i_case(n,k);
        quote! {
            fn get_s( &self ) -> #ntag {
                #i_case
            }
        }
    }
}

fn impl_fn_get_cell(n:u8, k:u8) -> TokenStream {
    let ctag = utils::c(k);
    let p : Vec<TokenStream> = Vec::new();
    if k > 0 && k < n {
        quote! {
            pub fn get_cell( &self ) -> #ctag {
                let s = self.get_s();
                let e = self.get_e();
                #ctag::new( #(#p),* , e )
                todo!()
            }
        }
    } else {
        quote! {
            pub fn get_cell( &self ) -> #ctag {
                let s = self.get_s();
                #ctag::new( #(#p),* )
                todo!()
            }
        }
    }
}

use crate::combinatorics::Lattice;
fn n_mid(n:u8, k:u8) -> Vec<TokenStream> {
    Lattice::new(n).k_combinations(k)
        .iter().map(|s| {
            let p : Vec<TokenStream> = (0..n).into_iter().map(|i| {
                let l = i as usize;
                if s.contains(&i) {
                    quote! { (n.0[#l] - 1) }
                } else {
                    quote! { n.0[#l] }
                }
            }).collect();
            utils::join_op(p, |accum, x| quote! {#accum * #x})
        }).collect()
}

fn n_max(n:u8) -> Vec<TokenStream> {
    (0..n as usize).into_iter().map(|i|{
        let idx = syn::Index::from(i);
        quote! {n_mid.#idx}
    }).collect()
}

fn e_cases(n:u8, k:u8) -> Vec<TokenStream> {
    let etag = utils::e(k);
    (0..n).into_iter().map(|i| {
        let j : Vec<_> = (0..=i as usize).into_iter().map(|i| syn::Index::from(i)).collect();
        quote! { i if i < #(self.n_mid.#j)+* => #etag::from(#i)  }
    }).collect()
}

fn i_cases(n:u8, k:u8) -> Vec<TokenStream> {
    Lattice::new(n).k_combinations(k)
        .iter().map(|s| {
            let itag = s.tokenize(true);
            let p : Vec<TokenStream> = (0..n).into_iter()
                .filter(|i| s.contains(i)).map(|i| {
                let i = i as usize;
                quote! { s.0[#i] -= 1 }
            }).collect();
            quote! { #itag => { #(#p;)* } }
        }).collect()
}

fn i_case(n:u8, k:u8) -> TokenStream {
    match Lattice::new(n).k_combinations(k).iter().next() {
        Some(s) => {
            let p : Vec<TokenStream> = (0..n).into_iter()
                .filter(|i| s.contains(i)).map(|i| {
                    let i = i as usize;
                    quote! { s.0[#i] -= 1 }
                }).collect();
            if p.len() == 0 {
                quote! { self.n }

            } else {
                quote! {
                    let mut s = self.n;
                    #(#p;)*
                    s
                }
            }
        },
        None => quote! {},
    }
}
