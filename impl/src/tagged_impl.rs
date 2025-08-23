use crate::{ImplArgs, Mode};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, token::Where, Error, ItemImpl, Type, TypePath, WhereClause
};

pub(crate) fn expand(args: ImplArgs, mut input: ItemImpl, mode: Mode) -> TokenStream {
    // if mode.de && !input.generics.params.is_empty() {
    //     let msg = "deserialization of generic impls is not supported yet; \
    //                use #[typetag::serialize] to generate serialization only";
    //     return Error::new_spanned(input.generics, msg).to_compile_error();
    // }

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

    let object = &input.trait_.as_ref().unwrap().1;
    let this = &input.self_ty;

    let mut expanded = quote! {
        #input
    };

    if mode.de && input.generics.params.is_empty() {
        expanded.extend(quote! {
            typetag::__private::inventory::submit! {
                <dyn #object>::typetag_register(
                    #name,
                    (|deserializer| typetag::__private::Result::Ok(
                        typetag::__private::Box::new(
                            typetag::__private::erased_serde::deserialize::<#this>(deserializer)?
                        ),
                    )) as typetag::__private::DeserializeFn<<dyn #object as typetag::__private::Strictest>::Object>,
                )
            }
        });
    }

    expanded
}

fn augment_impl(input: &mut ItemImpl, name: &TokenStream, mode: Mode) {
    if mode.ser && !input.generics.params.is_empty() {
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_name(&self) -> &'static str {
                <Self as typetag::TypetagName>::typetag_name()
            }
        });

        input
            .generics
            .where_clause
            .get_or_insert_with(|| WhereClause {
                where_token: Where::default(),
                predicates: Punctuated::new(),
            })
            .predicates
            .push(parse_quote! {
                Self: typetag::TypetagName
            });
    } else if mode.ser {
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

fn type_name(mut ty: &Type) -> Option<String> {
    loop {
        match ty {
            Type::Path(TypePath { qself: None, path }) => {
                let segment = path.segments.last().unwrap();
                let ident = segment.ident.to_string();
                return Some(match &segment.arguments {
                    syn::PathArguments::None => ident,
                    syn::PathArguments::Parenthesized(_) => ident,
                    syn::PathArguments::AngleBracketed(args) => {
                        let mut name = ident;
                        for t in args.to_token_stream() {
                            name += &t.to_string();
                        }
                        name
                    }
                });
            }
            Type::Group(group) => {
                ty = &group.elem;
            }
            _ => return None,
        }
    }
}
