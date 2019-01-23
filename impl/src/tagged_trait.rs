use crate::TraitArgs;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, ItemTrait, LitStr};

pub fn expand(args: TraitArgs, mut input: ItemTrait) -> TokenStream {
    augment_trait(&mut input);

    let registry = build_registry(&input);

    let (serialize_impl, deserialize_impl) = match args {
        TraitArgs::External => externally_tagged(&input),
        TraitArgs::Internal { tag } => internally_tagged(tag, &input),
        TraitArgs::Adjacent { tag, content } => adjacently_tagged(tag, content, &input),
    };

    let object = &input.ident;

    let expanded = quote! {
        #registry

        impl<'a> typetag::serde::Serialize for dyn #object + 'a {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: typetag::serde::Serializer,
            {
                #serialize_impl
            }
        }

        impl<'de> typetag::serde::Deserialize<'de> for std::boxed::Box<dyn #object> {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: typetag::serde::Deserializer<'de>,
            {
                #deserialize_impl
            }
        }
    };

    wrap_in_dummy_const(input, expanded)
}

fn augment_trait(input: &mut ItemTrait) {
    input
        .supertraits
        .push(parse_quote!(typetag::erased_serde::Serialize));

    input.items.push(parse_quote! {
        #[doc(hidden)]
        fn typetag_name(&self) -> &'static str;
    });
}

fn build_registry(input: &ItemTrait) -> TokenStream {
    let object = &input.ident;

    quote! {
        type TypetagFn = typetag::DeserializeFn<dyn #object>;

        pub struct TypetagRegistration {
            name: &'static str,
            deserializer: TypetagFn,
        }

        typetag::inventory::collect!(TypetagRegistration);

        impl dyn #object {
            pub fn typetag_register(name: &'static str, deserializer: TypetagFn) -> TypetagRegistration {
                TypetagRegistration { name, deserializer }
            }
        }

        typetag::lazy_static::lazy_static! {
            static ref TYPETAG: typetag::Registry<dyn #object> = {
                let mut map = std::collections::BTreeMap::new();
                let mut names = std::vec::Vec::new();
                for registered in typetag::inventory::iter::<TypetagRegistration> {
                    map.insert(registered.name, registered.deserializer);
                    names.push(registered.name);
                }
                names.sort_unstable();
                typetag::Registry { map, names }
            };
        }
    }
}

fn externally_tagged(input: &ItemTrait) -> (TokenStream, TokenStream) {
    let trait_object = input.ident.to_string();

    let serialize_impl = quote! {
        let name = Self::typetag_name(self);
        typetag::externally::serialize(serializer, name, self)
    };

    let deserialize_impl = quote! {
        typetag::externally::deserialize(deserializer, #trait_object, &TYPETAG)
    };

    (serialize_impl, deserialize_impl)
}

fn internally_tagged(tag: LitStr, input: &ItemTrait) -> (TokenStream, TokenStream) {
    let trait_object = input.ident.to_string();

    let serialize_impl = quote! {
        let name = Self::typetag_name(self);
        typetag::internally::serialize(serializer, #tag, name, self)
    };

    let deserialize_impl = quote! {
        typetag::internally::deserialize(deserializer, #trait_object, #tag, &TYPETAG)
    };

    (serialize_impl, deserialize_impl)
}

fn adjacently_tagged(
    tag: LitStr,
    content: LitStr,
    input: &ItemTrait,
) -> (TokenStream, TokenStream) {
    let trait_object = input.ident.to_string();

    let serialize_impl = quote! {
        let name = Self::typetag_name(self);
        typetag::adjacently::serialize(serializer, #trait_object, #tag, name, #content, self)
    };

    let deserialize_impl = quote! {
        typetag::adjacently::deserialize(deserializer, #trait_object, &[#tag, #content], &TYPETAG)
    };

    (serialize_impl, deserialize_impl)
}

fn wrap_in_dummy_const(input: ItemTrait, expanded: TokenStream) -> TokenStream {
    let dummy_const_name = format!("_{}_registry", input.ident);
    let dummy_const = Ident::new(&dummy_const_name, Span::call_site());

    quote! {
        #input

        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            #expanded
        };
    }
}
