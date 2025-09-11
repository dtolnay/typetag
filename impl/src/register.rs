use proc_macro2::TokenStream;
use syn::Error;

use crate::{tagged_impl::type_name, RegisterArgs};
use quote::quote;

pub(crate) fn expand(args: RegisterArgs) -> TokenStream {
    let trait_ty = args.trait_ty;
    let impl_ty = args.impl_ty;
    let name = match args.name {
        Some(name) => quote!(#name),
        None => match type_name(&impl_ty) {
            Some(name) => quote!(#name),
            None => {
                let msg = "use #typetag::register(..., name = \"...\")] to specify a unique name";
                return Error::new_spanned(&impl_ty, msg).to_compile_error();
            }
        },
    };

    let mut expanded = TokenStream::new();

    if args.mode.ser {
        expanded.extend(quote! {
            impl typetag::TypetagName for #impl_ty {
                fn typetag_name() -> &'static str {
                    #name
                }
            }
        });
    }

    if args.mode.de {
        expanded.extend(quote! {
            typetag::__private::inventory::submit! {
                <dyn #trait_ty>::typetag_register(
                    #name,
                    (|deserializer| typetag::__private::Result::Ok(
                        typetag::__private::Box::new(
                            typetag::__private::erased_serde::deserialize::<#impl_ty>(deserializer)?
                        ),
                    )) as typetag::__private::DeserializeFn<dyn #trait_ty>,
                )
            }
        });
    }

    expanded
}
