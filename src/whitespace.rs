mod ast;
mod parser;
mod tokens;

use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote_spanned,
    spanned::Spanned,
    token::Paren,
    Attribute, Block, Ident, Token,
};

#[derive(Debug, Clone)]
struct WhitespaceFn {
    attrs: Vec<Attribute>,
    fn_token: Token![fn],
    ident: Ident,
    paren_token: Paren,
    block: Block,
}

impl Parse for WhitespaceFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let fn_token = input.parse()?;
        let ident = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        if !content.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "whitespace function should not have arguments",
            ));
        }
        let block = input.parse()?;
        Ok(Self {
            attrs,
            fn_token,
            ident,
            paren_token,
            block,
        })
    }
}

impl ToTokens for WhitespaceFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.paren_token.surround(tokens, |_tokens| {});
        self.block.to_tokens(tokens);
    }
}

pub(crate) fn whitespace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item.clone().into_iter();
    let mut item = parse_macro_input!(item as WhitespaceFn);
    let source = item.block.span().source_text().unwrap_or_default();
    let source = source.trim_start_matches('{').trim_end_matches('}');
    let mut parser = parser::Parser::new(source);
    let ast = parser.parse::<ast::Ast>().unwrap();
    item.block = parse_quote_spanned! {item.block.span()=>
        { #ast }
    };
    quote!(#item).into()
}
