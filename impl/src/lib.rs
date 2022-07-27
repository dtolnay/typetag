#![recursion_limit = "256"]
#![allow(clippy::needless_pass_by_value, clippy::single_match_else)]

extern crate proc_macro;

mod parse;
mod tagged_impl;
mod tagged_trait;

use crate::parse::{ImplArgs, Input, TraitArgs};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Path};

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

pub fn expand(args: TokenStream, input: TokenStream, mode: Mode, crate_path: &Path) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    TokenStream::from(match input {
        Input::Trait(input) => {
            let args = parse_macro_input!(args as TraitArgs);
            tagged_trait::expand(args, input, mode, crate_path)
        }
        Input::Impl(input) => {
            let args = parse_macro_input!(args as ImplArgs);
            tagged_impl::expand(args, input, mode, crate_path)
        }
    })
}

pub fn get_crate_path(package_name: &str) -> Path {
    let found_crate = proc_macro_crate::crate_name(package_name)
        .unwrap_or_else(|e| panic!("{} is present in `Cargo.toml`: {:?}", package_name, e));

    match found_crate {
        proc_macro_crate::FoundCrate::Itself => {
            parse_str(package_name).expect("Unable to parse package_name")
        }
        proc_macro_crate::FoundCrate::Name(name) => {
            parse_str(&name).expect("Unable to parse crate name")
        }
    }
}

fn parse_str<T: syn::parse::Parse>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    Ok(syn::parse(path.parse::<TokenStream>()?)?)
}
