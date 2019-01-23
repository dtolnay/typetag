use crate::ImplArgs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Error, ItemImpl, Type, TypePath};

pub fn expand(args: ImplArgs, mut input: ItemImpl) -> TokenStream {
    let object = &input.trait_.as_ref().unwrap().1;
    let this = &input.self_ty;

    let name = match args.name {
        Some(name) => quote!(#name),
        None => match type_name(&input.self_ty) {
            Some(name) => quote!(#name),
            None => {
                let msg = "use #[typetag::serde(name = \"...\")] to specify a unique name";
                return Error::new_spanned(&input.self_ty, msg).to_compile_error();
            }
        },
    };

    input.items.push(parse_quote! {
        fn typetag_name(&self) -> &'static str {
            #name
        }
    });

    quote! {
        #input

        typetag::inventory::submit! {
            #![crate = typetag]
            <dyn #object>::typetag_register(
                #name,
                |deserializer| std::result::Result::Ok(
                    std::boxed::Box::new(
                        typetag::erased_serde::deserialize::<#this>(deserializer)?
                    ),
                ),
            )
        }
    }
}

fn type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            Some(path.segments.last().unwrap().into_value().ident.to_string())
        }
        _ => None,
    }
}
