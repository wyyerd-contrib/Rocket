#![feature(proc_macro)]
#![recursion_limit="256"]

extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

mod spanned;

#[cfg(feature = "database_attribute")]
mod database;

#[allow(dead_code)]
use proc_macro::TokenStream;

#[cfg(feature = "database_attribute")]
#[proc_macro_attribute]
pub fn database(attr: TokenStream, input: TokenStream) -> TokenStream {
    ::database::database_attr(attr, input).unwrap_or_else(|diag| {
        diag.emit();
        TokenStream::new()
    })
}
