use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use rand::{thread_rng, Rng};
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    BinOp, Expr, ExprBinary, Ident, Item,
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
                let mut rng = thread_rng();
                let name = format!(
                    "__macroo_right_first_assign_ident_{:018}",
                    rng.gen_range(0..10u64.pow(18))
                );
                let ident = Ident::new(&name, Span::call_site());
                let e: Expr = parse_quote!({
                    let #ident = #right;
                    #left #op #ident;
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
