use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Error, ImplItem, ItemImpl, ItemTrait, LitStr, Token, TraitItem, Visibility};

mod kw {
    syn::custom_keyword!(tag);
    syn::custom_keyword!(content);
    syn::custom_keyword!(default_variant);
    syn::custom_keyword!(deny_unknown_fields);
    syn::custom_keyword!(name);
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
                    let type_token = assoc.type_token;
                    let semi_token = assoc.semi_token;
                    let span = quote!(#type_token #semi_token);
                    let msg = "typetag trait with associated type is not supported yet";
                    return Err(Error::new_spanned(span, msg));
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
                } else if let ImplItem::Type(assoc) = assoc {
                    let type_token = assoc.type_token;
                    let semi_token = assoc.semi_token;
                    let span = quote!(#type_token #semi_token);
                    let msg = "typetag trait with associated type is not supported yet";
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
            return Ok(TraitArgs::Internal {
                tag,
                default_variant: Some(default_variant),
            });
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
