#![recursion_limit = "256"]

extern crate proc_macro;

mod parse;
mod tagged_impl;
mod tagged_trait;

use crate::parse::{ImplArgs, Input, TraitArgs};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn serde(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    TokenStream::from(match input {
        Input::Trait(input) => {
            let args = parse_macro_input!(args as TraitArgs);
            tagged_trait::expand(args, input)
        }
        Input::Impl(input) => {
            let args = parse_macro_input!(args as ImplArgs);
            tagged_impl::expand(args, input)
        }
    })
}
