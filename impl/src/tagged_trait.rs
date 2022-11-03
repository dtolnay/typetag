use crate::{Mode, TraitArgs};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Error, Ident, ItemTrait, LitStr, Path, TraitBoundModifier, TypeParamBound};

pub(crate) fn expand(args: TraitArgs, mut input: ItemTrait, mode: Mode, crate_path: &Path) -> TokenStream {
    if mode.de && !input.generics.params.is_empty() {
        let msg = "deserialization of generic traits is not supported yet; \
                   use #[typetag::serialize] to generate serialization only";
        return Error::new_spanned(input.generics, msg).to_compile_error();
    }

    augment_trait(&mut input, mode, crate_path);

    let (serialize_impl, deserialize_impl) = match args {
        TraitArgs::External => externally_tagged(&input, crate_path),
        TraitArgs::Internal { tag } => internally_tagged(tag, &input, crate_path),
        TraitArgs::Adjacent { tag, content } => adjacently_tagged(tag, content, &input, crate_path),
    };

    let object = &input.ident;

    let mut expanded = TokenStream::new();

    if mode.ser {
        let mut impl_generics = input.generics.clone();
        impl_generics.params.push(parse_quote!('typetag));
        let (impl_generics, _, _) = impl_generics.split_for_impl();
        let (_, ty_generics, where_clause) = input.generics.split_for_impl();

        expanded.extend(quote! {
            impl #impl_generics #crate_path::__private::serde::Serialize
            for dyn #object #ty_generics + 'typetag #where_clause {
                fn serialize<S>(&self, serializer: S) -> #crate_path::__private::Result<S::Ok, S::Error>
                where
                    S: #crate_path::__private::serde::Serializer,
                {
                    #serialize_impl
                }
            }
        });

        for marker_traits in &[quote!(Send), quote!(Sync), quote!(Send + Sync)] {
            expanded.extend(quote! {
                impl #impl_generics #crate_path::__private::serde::Serialize
                for dyn #object #ty_generics + #marker_traits + 'typetag #where_clause {
                    fn serialize<S>(&self, serializer: S) -> #crate_path::__private::Result<S::Ok, S::Error>
                    where
                        S: #crate_path::__private::serde::Serializer,
                    {
                        #crate_path::__private::serde::Serialize::serialize(self as &dyn #object #ty_generics, serializer)
                    }
                }
            });
        }
    }

    if mode.de {
        let registry = build_registry(&input, crate_path);

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

            impl #crate_path::__private::Strictest for dyn #object {
                type Object = dyn #object + #strictest;
            }

            impl<'de> #crate_path::__private::serde::Deserialize<'de> for #crate_path::__private::Box<dyn #object + #strictest> {
                fn deserialize<D>(deserializer: D) -> #crate_path::__private::Result<Self, D::Error>
                where
                    D: #crate_path::__private::serde::Deserializer<'de>,
                {
                    #deserialize_impl
                }
            }
        });

        for marker_traits in others {
            expanded.extend(quote! {
                impl<'de> #crate_path::__private::serde::Deserialize<'de> for #crate_path::__private::Box<dyn #object + #marker_traits> {
                    fn deserialize<D>(deserializer: D) -> #crate_path::__private::Result<Self, D::Error>
                    where
                        D: #crate_path::__private::serde::Deserializer<'de>,
                    {
                        #crate_path::__private::Result::Ok(
                            <#crate_path::__private::Box<dyn #object + #strictest>
                                as #crate_path::__private::serde::Deserialize<'de>>::deserialize(deserializer)?
                        )
                    }
                }
            });
        }
    }

    wrap_in_dummy_const(input, expanded)
}

fn augment_trait(input: &mut ItemTrait, mode: Mode, crate_path: &Path) {
    if mode.ser {
        input.supertraits.push(parse_quote!(#crate_path::Serialize));

        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_name(&self) -> &'static str;
        });
    }

    if mode.de {
        input.supertraits.push(parse_quote!(#crate_path::Deserialize));

        // Only to catch missing typetag attribute on impl blocks. Not called.
        input.items.push(parse_quote! {
            #[doc(hidden)]
            fn typetag_deserialize(&self);
        });
    }
}

fn build_registry(input: &ItemTrait, crate_path: &Path) -> TokenStream {
    let vis = &input.vis;
    let object = &input.ident;

    quote! {
        type TypetagStrictest = <dyn #object as #crate_path::__private::Strictest>::Object;
        type TypetagFn = #crate_path::__private::DeserializeFn<TypetagStrictest>;

        #vis struct TypetagRegistration<T> {
            name: &'static str,
            deserializer: T,
        }

        #crate_path::__private::inventory::collect!(TypetagRegistration<TypetagFn>);

        impl dyn #object {
            #[doc(hidden)]
            #vis const fn typetag_register<T>(name: &'static str, deserializer: T) -> TypetagRegistration<T> {
                TypetagRegistration { name, deserializer }
            }
        }
    }
}

fn static_registry(crate_path: &Path) -> TokenStream {
    quote! {
        static TYPETAG: #crate_path::__private::once_cell::race::OnceBox<#crate_path::__private::Registry<TypetagStrictest>> = #crate_path::__private::once_cell::race::OnceBox::new();
        let registry = TYPETAG.get_or_init(|| {
            let mut map = #crate_path::__private::BTreeMap::new();
            let mut names = #crate_path::__private::Vec::new();
            for registered in #crate_path::__private::inventory::iter::<TypetagRegistration<TypetagFn>> {
                match map.entry(registered.name) {
                    #crate_path::__private::btree_map::Entry::Vacant(entry) => {
                        entry.insert(#crate_path::__private::Option::Some(registered.deserializer));
                    }
                    #crate_path::__private::btree_map::Entry::Occupied(mut entry) => {
                        entry.insert(#crate_path::__private::Option::None);
                    }
                }
                names.push(registered.name);
            }
            names.sort_unstable();
            #crate_path::__private::Box::new(#crate_path::__private::Registry { map, names })
        });
    }
}

fn externally_tagged(input: &ItemTrait, crate_path: &Path) -> (TokenStream, TokenStream) {
    let object = &input.ident;
    let object_name = object.to_string();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let static_registry = static_registry(crate_path);

    let serialize_impl = quote! {
        let name = <Self as #object #ty_generics>::typetag_name(self);
        #crate_path::__private::externally::serialize(serializer, name, self)
    };

    let deserialize_impl = quote! {
        #static_registry
        #crate_path::__private::externally::deserialize(deserializer, #object_name, registry)
    };

    (serialize_impl, deserialize_impl)
}

fn internally_tagged(tag: LitStr, input: &ItemTrait, crate_path: &Path) -> (TokenStream, TokenStream) {
    let object = &input.ident;
    let object_name = object.to_string();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let static_registry = static_registry(crate_path);

    let serialize_impl = quote! {
        let name = <Self as #object #ty_generics>::typetag_name(self);
        #crate_path::__private::internally::serialize(serializer, #tag, name, self)
    };

    let deserialize_impl = quote! {
        #static_registry
        #crate_path::__private::internally::deserialize(deserializer, #object_name, #tag, registry)
    };

    (serialize_impl, deserialize_impl)
}

fn adjacently_tagged(
    tag: LitStr,
    content: LitStr,
    input: &ItemTrait,
    crate_path: &Path,
) -> (TokenStream, TokenStream) {
    let object = &input.ident;
    let object_name = object.to_string();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let static_registry = static_registry(crate_path);

    let serialize_impl = quote! {
        let name = <Self as #object #ty_generics>::typetag_name(self);
        #crate_path::__private::adjacently::serialize(serializer, #object_name, #tag, name, #content, self)
    };

    let deserialize_impl = quote! {
        #static_registry
        #crate_path::__private::adjacently::deserialize(deserializer, #object_name, &[#tag, #content], registry)
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
