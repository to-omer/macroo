use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    BinOp, Expr, ExprBinary, Item,
};

struct RightFirstAssignVisitor;

impl VisitMut for RightFirstAssignVisitor {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        visit_mut::visit_expr_mut(self, i);
        if let Expr::Binary(binary) = i {
            let ExprBinary {
                attrs,
                left,
                op,
                right,
            } = &binary;
            if attrs.is_empty()
                && matches!(
                    op,
                    BinOp::AddAssign(_)
                        | BinOp::SubAssign(_)
                        | BinOp::MulAssign(_)
                        | BinOp::DivAssign(_)
                        | BinOp::RemAssign(_)
                        | BinOp::BitXorAssign(_)
                        | BinOp::BitAndAssign(_)
                        | BinOp::BitOrAssign(_)
                        | BinOp::ShlAssign(_)
                        | BinOp::ShrAssign(_)
                )
            {
                let e: Expr = parse_quote!({
                    let x = #right;
                    #left #op x;
                });
                *i = e;
            }
        }
    }
}

pub(crate) fn right_first_assign(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as Item);
    RightFirstAssignVisitor.visit_item_mut(&mut item);
    quote!(#item).into()
}
