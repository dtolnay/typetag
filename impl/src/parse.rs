use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    Attribute, Error, Generics, ImplItem, ItemImpl, ItemTrait, LitStr, Token, TraitItem, Type,
    TypeParamBound, Visibility, WherePredicate,
};

use crate::Mode;

mod kw {
    syn::custom_keyword!(tag);
    syn::custom_keyword!(content);
    syn::custom_keyword!(default_variant);
    syn::custom_keyword!(deny_unknown_fields);
    syn::custom_keyword!(name);
    syn::custom_keyword!(mode);
    syn::custom_keyword!(serialize);
    syn::custom_keyword!(deserialize);
}

pub enum TraitArgs {
    External,
    Internal {
        tag: LitStr,
        default_variant: Option<LitStr>,
    },
    Adjacent {
        tag: LitStr,
        content: LitStr,
        default_variant: Option<LitStr>,
        deny_unknown_fields: bool,
    },
}

pub struct ImplArgs {
    pub name: Option<LitStr>,
}

pub struct RegisterArgs {
    pub trait_ty: Type,
    pub impl_ty: Type,
    pub name: Option<LitStr>,
    pub mode: Mode,
}

pub enum Input {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;

        let ahead = input.fork();
        ahead.parse::<Visibility>()?;
        ahead.parse::<Option<Token![unsafe]>>()?;

        if ahead.peek(Token![trait]) {
            let mut item: ItemTrait = input.parse()?;
            for assoc in &item.items {
                if let TraitItem::Const(assoc) = assoc {
                    let const_token = assoc.const_token;
                    let semi_token = assoc.semi_token;
                    let span = quote!(#const_token #semi_token);
                    let msg = "typetag trait with associated const is not supported yet";
                    return Err(Error::new_spanned(span, msg));
                } else if let TraitItem::Type(assoc) = assoc {
                    if !is_self_sized(&assoc.generics) {
                        let type_token = assoc.type_token;
                        let semi_token = assoc.semi_token;
                        let span = quote!(#type_token #semi_token);
                        let msg = "typetag trait with associated type is not supported yet";
                        return Err(Error::new_spanned(span, msg));
                    }
                }
            }
            attrs.extend(item.attrs);
            item.attrs = attrs;
            Ok(Input::Trait(item))
        } else if ahead.peek(Token![impl]) {
            let mut item: ItemImpl = input.parse()?;
            if item.trait_.is_none() {
                let impl_token = item.impl_token;
                let ty = item.self_ty;
                let span = quote!(#impl_token #ty);
                let msg = "expected impl Trait for Type";
                return Err(Error::new_spanned(span, msg));
            }
            for assoc in &item.items {
                if let ImplItem::Const(assoc) = assoc {
                    let const_token = assoc.const_token;
                    let semi_token = assoc.semi_token;
                    let span = quote!(#const_token #semi_token);
                    let msg = "typetag trait with associated const is not supported yet";
                    return Err(Error::new_spanned(span, msg));
                }
            }
            attrs.extend(item.attrs);
            item.attrs = attrs;
            Ok(Input::Impl(item))
        } else {
            Err(input.error("expected trait or impl block"))
        }
    }
}

// #[typetag::serde]
// #[typetag::serde(tag = "type")]
// #[typetag::serde(tag = "type", default_variant = "default")]
// #[typetag::serde(tag = "type", content = "content")]
// #[typetag::serde(tag = "type", content = "content", deny_unknown_fields)]
// #[typetag::serde(tag = "type", content = "content", default_variant = "default")]
impl Parse for TraitArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(TraitArgs::External);
        }

        input.parse::<kw::tag>()?;
        input.parse::<Token![=]>()?;
        let tag: LitStr = input.parse()?;
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
        if input.is_empty() {
            return Ok(TraitArgs::Internal {
                tag,
                default_variant: None,
            });
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(kw::content) {
            input.parse::<kw::content>()?;
            input.parse::<Token![=]>()?;
            let content: LitStr = input.parse()?;
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            let mut default_variant = None;
            let mut deny_unknown_fields = false;
            while !input.is_empty() {
                let lookahead = input.lookahead1();
                if default_variant.is_none() && lookahead.peek(kw::default_variant) {
                    input.parse::<kw::default_variant>()?;
                    input.parse::<Token![=]>()?;
                    default_variant = Some(input.parse()?);
                } else if !deny_unknown_fields && lookahead.peek(kw::deny_unknown_fields) {
                    input.parse::<kw::deny_unknown_fields>()?;
                    deny_unknown_fields = true;
                } else {
                    return Err(lookahead.error());
                }
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
            }

            Ok(TraitArgs::Adjacent {
                tag,
                content,
                default_variant,
                deny_unknown_fields,
            })
        } else if lookahead.peek(kw::default_variant) {
            input.parse::<kw::default_variant>()?;
            input.parse::<Token![=]>()?;
            let default_variant: LitStr = input.parse()?;
            input.parse::<Option<Token![,]>>()?;
            Ok(TraitArgs::Internal {
                tag,
                default_variant: Some(default_variant),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

// #[typetag::serde]
// #[typetag::serde(name = "Tag")]
impl Parse for ImplArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = if input.is_empty() {
            None
        } else {
            input.parse::<kw::name>()?;
            input.parse::<Token![=]>()?;
            let name: LitStr = input.parse()?;
            input.parse::<Option<Token![,]>>()?;
            Some(name)
        };
        Ok(ImplArgs { name })
    }
}

// #typetag::register(Trait, Concrete)
// #typetag::register(Trait, Concrete, name = "Tag", mode = serde|serialize|deserialize)
impl Parse for RegisterArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_ty = input.parse()?;
        input.parse::<Token![,]>()?;

        let impl_ty = input.parse()?;
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        let mut name = None;
        let mut mode: Option<Mode> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if name.is_none() && lookahead.peek(kw::name) {
                input.parse::<kw::name>()?;
                input.parse::<Token![=]>()?;
                name = Some(input.parse()?);
            } else if mode.is_none() && lookahead.peek(kw::mode) {
                input.parse::<kw::deny_unknown_fields>()?;
                input.parse::<Token![=]>()?;
                let lookahead = input.lookahead1();

                if lookahead.peek(kw::serialize) {
                    input.parse::<kw::serialize>()?;
                    mode = Some(Mode {
                        ser: true,
                        de: false,
                    })
                } else if lookahead.peek(kw::deserialize) {
                    input.parse::<kw::deserialize>()?;
                    mode = Some(Mode {
                        ser: false,
                        de: true,
                    })
                } else {
                    return Err(lookahead.error());
                }
            } else {
                return Err(lookahead.error());
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(RegisterArgs {
            trait_ty,
            impl_ty,
            name,
            mode: mode.unwrap_or(Mode {
                ser: true,
                de: true,
            }),
        })
    }
}

fn is_self_sized(generics: &Generics) -> bool {
    if let Some(where_clause) = &generics.where_clause {
        for predicate in &where_clause.predicates {
            if let WherePredicate::Type(pred_type) = predicate {
                if let Type::Path(type_path) = &pred_type.bounded_ty {
                    if type_path.path.is_ident("Self") {
                        for bound in &pred_type.bounds {
                            if let TypeParamBound::Trait(trait_bound) = bound {
                                if trait_bound.path.is_ident("Sized") {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
