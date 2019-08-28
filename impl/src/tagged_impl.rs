use crate::{ImplArgs, Mode};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Error, Ident, ItemImpl, Type, TypePath};

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

    let object = &input.trait_.as_ref().unwrap().1;
    let this = &input.self_ty;
    let trait_name = &object
        .segments
        .last()
        .expect("Expected path to have last segment")
        .ident;
    let type_name = match &*input.self_ty {
        Type::Path(TypePath { path, .. }) => path
            .segments
            .last()
            .expect("Expected path to have last segment")
            .ident
            .to_string(),
        Type::Never(..) => String::from("NEVER"),
        _ => panic!("Unsupported type. TODO: Better error message."),
    };
    let registration_static = {
        let mut registration_static = format!("{}_{}_REGISTRATION", type_name, trait_name);
        registration_static.make_ascii_uppercase();
        registration_static
    };
    let registration_static = &Ident::new(&registration_static, Span::call_site());

    let mut expanded = quote! {
        #input
    };

    if mode.de {
        let mut typetag_registration_static = format!("{}_TYPETAG_REGISTRATIONS", trait_name);
        typetag_registration_static.make_ascii_uppercase();
        let typetag_registration_static =
            &Ident::new(&typetag_registration_static, Span::call_site());

        let dummy_const_name = format!("_{}_registry", trait_name);
        let dummy_const = Ident::new(&dummy_const_name, Span::call_site());

        expanded.extend(quote! {
            #[typetag::linkme::distributed_slice(#dummy_const::#typetag_registration_static)]
            static #registration_static: fn() -> #dummy_const::TypetagRegistration = {
                || <dyn #object>::typetag_register(
                    #name,
                    |deserializer| std::result::Result::Ok(
                        std::boxed::Box::new(
                            typetag::erased_serde::deserialize::<#this>(deserializer)?
                        ),
                    ),
                )
            };
        });
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

fn type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            Some(path.segments.last().unwrap().ident.to_string())
        }
        _ => None,
    }
}
