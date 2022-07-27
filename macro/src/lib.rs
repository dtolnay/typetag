use proc_macro::TokenStream;

use typetag_impl::{Mode, expand, get_crate_path};

#[proc_macro_attribute]
pub fn serde(args: TokenStream, input: TokenStream) -> TokenStream {
    let crate_path = get_crate_path("typetag");
    expand(args, input, Mode::new(true, true), &crate_path)
}

#[proc_macro_attribute]
pub fn serialize(args: TokenStream, input: TokenStream) -> TokenStream {
    let crate_path = get_crate_path("typetag");
    expand(args, input, Mode::new(true, false), &crate_path)
}

#[proc_macro_attribute]
pub fn deserialize(args: TokenStream, input: TokenStream) -> TokenStream {
    let crate_path = get_crate_path("typetag");
    expand(args, input, Mode::new(false, true), &crate_path)
}
