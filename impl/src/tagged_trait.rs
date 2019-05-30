use crate::{Mode, TraitArgs};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Error, Ident, ItemTrait, LitStr, TraitBoundModifier, TypeParamBound};

pub(crate) fn expand(args: TraitArgs, mut input: ItemTrait, mode: Mode) -> TokenStream {
    if mode.de && !input.generics.params.is_empty() {
        let msg = "deserialization of generic traits is not supported yet; \
                   use #[typetag::serialize] to generate serialization only";
        return Error::new_spanned(input.generics, msg).to_compile_error();
    }

    augment_trait(&mut input, mode);

    let (serialize_impl, deserialize_impl) = match args {
        TraitArgs::External => externally_tagged(&input),
        TraitArgs::Internal { tag } => internally_tagged(tag, &input),
        TraitArgs::Adjacent { tag, content } => adjacently_tagged(tag, content, &input),
    };

    let object = &input.ident;

    let mut expanded = TokenStream::new();

    if mode.ser {
        let mut impl_generics = input.generics.clone();
        impl_generics.params.push(parse_quote!('typetag));
        let (impl_generics, _, _) = impl_generics.split_for_impl();
        let (_, ty_generics, where_clause) = input.generics.split_for_impl();

        expanded.extend(quote! {
            impl #impl_generics typetag::serde::Serialize
            for dyn #object #ty_generics + 'typetag #where_clause {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: typetag::serde::Serializer,
                {
                    #serialize_impl
                }
            }
        });

        for marker_traits in &[quote!(Send), quote!(Sync), quote!(Send + Sync)] {
            expanded.extend(quote! {
                impl #impl_generics typetag::serde::Serialize
                for dyn #object #ty_generics + #marker_traits + 'typetag #where_clause {
                    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: typetag::serde::Serializer,
                    {
                        typetag::serde::Serialize::serialize(self as &dyn #object #ty_generics, serializer)
                    }
                }
            });
        }
    }

    if mode.de {
        let registry = build_registry(&input);

        let is_send = has_supertrait(&input, "Send");
        let is_sync = has_supertrait(&input, "Sync");
        let (strictest, others) = match (is_send, is_sync) {
            (false, false) => (quote!(), vec![]),
            (true, false) => (quote!(Send), vec![quote!()]),
            (false, true) => (quote!(Sync), vec![quote!()]),
            (true, true) => (
                quote!(Send + Sync),
                vec![quote!(), quote!(Send), quote!(Sync)],
            ),
        };

        expanded.extend(quote! {
            #registry

            impl typetag::Strictest for dyn #object {
                type Object = dyn #object + #strictest;
            }

            impl<'de> typetag::serde::Deserialize<'de> for std::boxed::Box<dyn #object + #strictest> {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: typetag::serde::Deserializer<'de>,
                {
                    #deserialize_impl
                }
            }
        });

        for marker_traits in others {
            expanded.extend(quote! {
                impl<'de> typetag::serde::Deserialize<'de> for std::boxed::Box<dyn #object + #marker_traits> {
                    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where
                        D: typetag::serde::Deserializer<'de>,
                    {
                        std::result::Result::Ok(
                            <std::boxed::Box<dyn #object + #strictest>
                                as typetag::serde::Deserialize<'de>>::deserialize(deserializer)?
                        )
                    }
                }
            });
        }
    }

    wrap_in_dummy_const(input, expanded)
}

fn augment_trait(input: &mut ItemTrait, mode: Mode) {
    if mode.ser {
        input.supertraits.push(parse_quote!(typetag::Serialize));

        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_name(&self) -> &'static str;
        });
    }

    if mode.de {
        input.supertraits.push(parse_quote!(typetag::Deserialize));

        // Only to catch missing typetag attribute on impl blocks. Not called.
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_deserialize(&self);
        });
    }
}

fn build_registry(input: &ItemTrait) -> TokenStream {
    let object = &input.ident;

    quote! {
        type TypetagStrictest = <dyn #object as typetag::Strictest>::Object;
        type TypetagFn = typetag::DeserializeFn<TypetagStrictest>;

        pub struct TypetagRegistration {
            name: &'static str,
            deserializer: TypetagFn,
        }

        typetag::inventory::collect!(TypetagRegistration);

        impl dyn #object {
            #[doc(hidden)]
            pub fn typetag_register(name: &'static str, deserializer: TypetagFn) -> TypetagRegistration {
                TypetagRegistration { name, deserializer }
            }
        }

        typetag::lazy_static::lazy_static! {
            static ref TYPETAG: typetag::Registry<TypetagStrictest> = {
                let mut map = std::collections::BTreeMap::new();
                let mut names = std::vec::Vec::new();
                for registered in typetag::inventory::iter::<TypetagRegistration> {
                    match map.entry(registered.name) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(std::option::Option::Some(registered.deserializer));
                        }
                        std::collections::btree_map::Entry::Occupied(mut entry) => {
                            entry.insert(std::option::Option::None);
                        }
                    }
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

fn has_supertrait(input: &ItemTrait, find: &str) -> bool {
    for supertrait in &input.supertraits {
        if let TypeParamBound::Trait(trait_bound) = supertrait {
            if let TraitBoundModifier::None = trait_bound.modifier {
                if trait_bound.path.is_ident(find) {
                    return true;
                }
            }
        }
    }
    false
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
