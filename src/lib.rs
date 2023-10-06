use std::{fmt::Display, mem::take, str::FromStr};

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Literal, TokenTree};
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, Item, LitInt};

enum IntSuffix {
    I8,
    U8,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Isize,
    Usize,
}

impl Display for IntSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntSuffix::I8 => f.write_str("i8"),
            IntSuffix::U8 => f.write_str("u8"),
            IntSuffix::I32 => f.write_str("i32"),
            IntSuffix::U32 => f.write_str("u32"),
            IntSuffix::I64 => f.write_str("i64"),
            IntSuffix::U64 => f.write_str("u64"),
            IntSuffix::I128 => f.write_str("i128"),
            IntSuffix::U128 => f.write_str("u128"),
            IntSuffix::Isize => f.write_str("isize"),
            IntSuffix::Usize => f.write_str("usize"),
        }
    }
}

impl FromStr for IntSuffix {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i8" => IntSuffix::I8,
            "u8" => IntSuffix::U8,
            "i32" => IntSuffix::I32,
            "u32" => IntSuffix::U32,
            "i64" => IntSuffix::I64,
            "u64" => IntSuffix::U64,
            "i128" => IntSuffix::I128,
            "u128" => IntSuffix::U128,
            "isize" => IntSuffix::Isize,
            "usize" => IntSuffix::Usize,
            _ => Err("expected integer type")?,
        })
    }
}

struct DefaultIntSuffixVisitor {
    suffix: IntSuffix,
}

impl DefaultIntSuffixVisitor {
    fn convert_literal(&self, lit: &mut Literal) {
        let s = lit.to_string();
        let mut b = s.as_bytes();
        match (
            b.get(0).cloned().unwrap_or_default(),
            b.get(1).cloned().unwrap_or_default(),
        ) {
            (b'0', b'x') => b = &b[2..],
            (b'0', b'o') => b = &b[2..],
            (b'0', b'b') => b = &b[2..],
            (b'0'..=b'9', _) => {}
            _ => return,
        }
        if b.iter().all(|c| matches!(c, b'0'..=b'9' | b'_')) {
            if let Ok(lit_suffixed) = Literal::from_str(&format!("{}_{}", s, self.suffix)) {
                *lit = lit_suffixed;
            }
        }
    }
    fn convert_token_stream(&self, t: &mut proc_macro2::TokenStream) {
        let mut prev = false;
        *t = take(t)
            .into_iter()
            .map(|tt| match tt {
                TokenTree::Group(group) => {
                    let mut stream = group.stream();
                    self.convert_token_stream(&mut stream);
                    TokenTree::Group(Group::new(group.delimiter(), stream))
                }
                TokenTree::Literal(mut lit) => {
                    if !prev {
                        self.convert_literal(&mut lit);
                    }
                    prev = false;
                    TokenTree::Literal(lit)
                }
                TokenTree::Punct(p) if p.as_char() == '.' => {
                    prev = true;
                    TokenTree::Punct(p)
                }
                _ => {
                    prev = false;
                    tt
                }
            })
            .collect()
    }
}

impl VisitMut for DefaultIntSuffixVisitor {
    fn visit_macro_mut(&mut self, i: &mut syn::Macro) {
        self.convert_token_stream(&mut i.tokens);
    }
    fn visit_lit_mut(&mut self, i: &mut syn::Lit) {
        match i {
            syn::Lit::Verbatim(lit) => self.convert_literal(lit),
            syn::Lit::Int(i) => self.visit_lit_int_mut(i),
            _ => {}
        }
    }
    fn visit_lit_int_mut(&mut self, i: &mut syn::LitInt) {
        if i.suffix().is_empty() {
            let repr = format!("{}_{}", i.base10_digits(), self.suffix);
            let e = LitInt::new(&repr, i.span());
            *i = e;
        }
    }
}

/// add suffix to integet literal without suffix
///
/// # Example
/// ```
/// #[macroo::default_int_suffix(u64)]
/// fn main() {
///     assert_eq!((!0).to_string(), "18446744073709551615");
/// }
/// ```
#[proc_macro_attribute]
pub fn default_int_suffix(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as Ident);
    let suffix = match ident.to_string().parse() {
        Ok(suffix) => suffix,
        Err(err) => return syn::Error::new(ident.span(), err).to_compile_error().into(),
    };
    let mut item = parse_macro_input!(item as Item);
    let mut v = DefaultIntSuffixVisitor { suffix };
    v.visit_item_mut(&mut item);
    quote!(#item).into()
}
