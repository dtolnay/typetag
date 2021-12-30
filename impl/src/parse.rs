use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Error, ExprPath, ItemImpl, ItemTrait, LitInt, LitStr, Token, Visibility};

mod kw {
    syn::custom_keyword!(tag);
    syn::custom_keyword!(content);
    syn::custom_keyword!(name);
    syn::custom_keyword!(compare);
    syn::custom_keyword!(priority);
    syn::custom_keyword!(default);
}

pub enum TraitArgs {
    External,
    Internal { tag: LitStr },
    Adjacent { tag: LitStr, content: LitStr },
}

pub enum Priority {
    Defined(LitInt),
    Undefined,
    Default,
}

impl Priority {
    pub fn is_default(&self) -> bool {
        match self {
            Priority::Default => true,
            _ => false,
        }
    }
}

pub struct ImplArgs {
    pub name: Option<LitStr>,
    pub comparison_function: Option<ExprPath>,
    pub priority: Priority,
}

pub enum Input {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

fn parse_argument<T: Parse, U: Parse>(input: &ParseStream) -> Result<U> {
    input.parse::<T>()?;
    input.parse::<Token![=]>()?;
    let result = input.parse()?;
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(result)
}

fn parse_flag<T: Parse>(input: &ParseStream) -> Result<()> {
    input.parse::<T>()?;
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(())
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
        let mut tag = None;
        let mut content = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::tag) {
                tag = Some(parse_argument::<kw::tag, _>(&input)?);
            } else if lookahead.peek(kw::content) {
                content = Some(parse_argument::<kw::content, _>(&input)?);
            }
        }
        match (tag, content) {
            (None, None) => Ok(TraitArgs::External),
            (Some(tag), None) => Ok(TraitArgs::Internal { tag }),
            (Some(tag), Some(content)) => Ok(TraitArgs::Adjacent { tag, content }),
            (None, Some(content)) => Err(Error::new(
                content.span(),
                "Adjacently tagged enumerations must have a tag defined.",
            )),
        }
    }
}

// #[typetag::serde]
// #[typetag::serde(name = "Tag", comparison_function = fn(&str)->bool, priority)]
// #[typetag::serde(name = "Tag", default)]
impl Parse for ImplArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut impl_args = ImplArgs {
            name: None,
            comparison_function: None,
            priority: Priority::Undefined,
        };
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::name) {
                impl_args.name = Some(parse_argument::<kw::name, _>(&input)?);
            } else if lookahead.peek(kw::compare) {
                impl_args.comparison_function = Some(parse_argument::<kw::compare, _>(&input)?);
            } else if lookahead.peek(kw::priority) {
                impl_args.priority = Priority::Defined(parse_argument::<kw::priority, _>(&input)?);
            } else if lookahead.peek(kw::default) {
                parse_flag::<kw::default>(&input)?;
                impl_args.priority = Priority::Default;
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(impl_args)
    }
}
