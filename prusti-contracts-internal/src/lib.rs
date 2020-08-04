extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_attribute]
pub fn requires(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::requires(attr.into(), tokens.into()).into()
}

#[proc_macro_attribute]
pub fn ensures(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::ensures(attr.into(), tokens.into()).into()
}

#[proc_macro_attribute]
pub fn after_expiry(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::after_expiry(attr.into(), tokens.into()).into()
}

#[proc_macro_attribute]
pub fn after_expiry_if(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::after_expiry_if(attr.into(), tokens.into()).into()
}

#[proc_macro_attribute]
pub fn pure(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::pure(attr.into(), tokens.into()).into()
}

#[proc_macro_hack]
pub fn invariant(tokens: TokenStream) -> TokenStream {
    prusti_specs::invariant(tokens.into()).into()
}

#[proc_macro_hack]
pub fn thread_ensures(tokens: TokenStream) -> TokenStream {
    prusti_specs::thread_ensures(tokens.into()).into()
}

#[proc_macro_attribute]
pub fn attr_test(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::attr_test(attr.into(), tokens.into()).into()
}

#[proc_macro_attribute]
pub fn attr_test1(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    prusti_specs::attr_test(attr.into(), tokens.into()).into()
}