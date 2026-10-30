use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::utils;

use super::Set;

impl Set {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn push(&mut self, v : u8) { self.0.push(v); }
    pub fn pop(&mut self) -> Option<u8> { self.0.pop() }
    pub fn append(&mut self, s : &mut Self) { self.0.append(&mut s.0); }
    pub fn contains(&self, v : &u8) -> bool { self.0.contains(v) }

    pub fn tokenize(&self, complete: bool) -> TokenStream {
        let id = self.0.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("");
        let face = utils::i(&id);
        let element = utils::e(self.0.len() as u8);
        if complete {
            quote! { #element::#face }
        } else {
            quote! { #face }
        }
    }
}

use std::ops::{Index,IndexMut};

impl Index<usize> for Set {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Set {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <'a> IntoIterator for &'a Set {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }

}

impl <'a> IntoIterator for &'a mut Set {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for Set {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
