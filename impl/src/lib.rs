#![recursion_limit = "256"]
#![allow(clippy::needless_pass_by_value, clippy::single_match_else)]

extern crate proc_macro;

mod parse;
mod tagged_impl;
mod tagged_trait;

use crate::parse::{ImplArgs, Input, TraitArgs};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[derive(Copy, Clone)]
pub struct Mode {
    ser: bool,
    de: bool,
}

impl Mode {
    pub fn new(ser: bool, de: bool) -> Self {
        Mode { ser, de }
    }
}

pub fn expand(args: TokenStream, input: TokenStream, mode: Mode) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    TokenStream::from(match input {
        Input::Trait(input) => {
            let args = parse_macro_input!(args as TraitArgs);
            tagged_trait::expand(args, input, mode)
        }
        Input::Impl(input) => {
            let args = parse_macro_input!(args as ImplArgs);
            tagged_impl::expand(args, input, mode)
        }
    })
}
