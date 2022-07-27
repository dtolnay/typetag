use proc_macro::TokenStream;

use typetag_impl::{Mode, expand};

#[proc_macro_attribute]
pub fn serde(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input, Mode::new(true, true))
}

#[proc_macro_attribute]
pub fn serialize(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input, Mode::new(true, false))
}

#[proc_macro_attribute]
pub fn deserialize(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input, Mode::new(false, true))
}

