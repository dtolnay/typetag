use crate::{ImplArgs, Mode};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Error, ItemImpl, Type, TypePath};

pub(crate) fn expand(args: ImplArgs, mut input: ItemImpl, mode: Mode) -> TokenStream {
    if mode.de && !input.generics.params.is_empty() {
        let msg = "deserialization of generic impls is not supported yet; \
                   use #[typetag::serialize] to generate serialization only";
        return Error::new_spanned(input.generics, msg).to_compile_error();
    }

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

    augment_impl(&mut input, &name, mode);

    cfg_if::cfg_if! {
        if #[cfg(feature="runtime")] {
            augment_impl_register(&mut input, &name, mode);

            let expanded = quote! { #input };
        } else {
            let mut expanded = quote! {
                #input
            };

            register_inventory(&mut input, &name, &mut expanded);
        }
    }

    expanded
}

fn augment_impl(input: &mut ItemImpl, name: &TokenStream, mode: Mode) {
    if mode.ser {
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_name(&self) -> &'static str {
                #name
            }
        });
    }

    if mode.de {
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_deserialize(&self) {}
        });
    }
}


#[cfg(feature="runtime")]
fn augment_impl_register(input: &mut ItemImpl, name: &TokenStream, mode: Mode) {
    let object = &input.trait_.as_ref().unwrap().1;
    let this = &input.self_ty;

    if mode.de {
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn register() {
                <dyn #object>::typetag_register(
                    #name,
                    |deserializer| std::result::Result::Ok(
                        std::boxed::Box::new(
                            typetag::erased_serde::deserialize::<#this>(deserializer)?
                        ),
                    ),
                )
            }
        });
    }
}

#[cfg(not(feature="runtime"))]
fn register_inventory(input: &mut ItemImpl, name: &TokenStream, expanded: &mut TokenStream) {
    let object = &input.trait_.as_ref().unwrap().1;
    let this = &input.self_ty;

    expanded.extend(quote! {
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
    });
}

fn type_name(mut ty: &Type) -> Option<String> {
    loop {
        match ty {
            Type::Path(TypePath { qself: None, path }) => {
                return Some(path.segments.last().unwrap().ident.to_string());
            }
            Type::Group(group) => {
                ty = &group.elem;
            }
            _ => return None,
        }
    }
}
