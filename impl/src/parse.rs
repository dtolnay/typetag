use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Error, ItemImpl, ItemTrait, LitStr, Token, Visibility};

mod kw {
    syn::custom_keyword!(tag);
    syn::custom_keyword!(content);
    syn::custom_keyword!(name);
}

pub enum TraitArgs {
    External,
    Internal { tag: LitStr },
    Adjacent { tag: LitStr, content: LitStr },
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
// #[typetag::serde(tag = "type", content = "content")]
impl Parse for TraitArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(TraitArgs::External);
        }
        input.parse::<kw::tag>()?;
        input.parse::<Token![=]>()?;
        let tag: LitStr = input.parse()?;
        if input.is_empty() {
            return Ok(TraitArgs::Internal { tag });
        }
        input.parse::<Token![,]>()?;
        input.parse::<kw::content>()?;
        input.parse::<Token![=]>()?;
        let content: LitStr = input.parse()?;
        Ok(TraitArgs::Adjacent { tag, content })
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
            Some(name)
        };
        Ok(ImplArgs { name })
    }
}
