extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn ensures(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn after_expiry(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn after_expiry_if(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn pure(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_hack]
pub fn invariant(_tokens: TokenStream) -> TokenStream {
    (quote! { () }).into()
}

#[proc_macro_hack]
pub fn thread_ensures (_tokens: TokenStream) -> TokenStream { (quote! { () }).into() }

#[proc_macro_attribute]
pub fn attr_test(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}

#[proc_macro_attribute]
pub fn attr_test1(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    tokens
}
