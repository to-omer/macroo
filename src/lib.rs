use proc_macro::TokenStream;

mod default_int_suffix;
mod right_first_assign;

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
    default_int_suffix::default_int_suffix(attr, item)
}

/// replace binary assignment with evaluate right expression first
///
/// # Example
/// ```
/// #[derive(Copy, Clone, Debug, PartialEq)]
/// struct I(i32);
///
/// impl std::ops::AddAssign for I {
///     fn add_assign(&mut self, rhs: Self) {
///         self.0 += rhs.0;
///     }
/// }
///
/// #[macroo::right_first_assign]
/// fn main() {
///     let mut a = vec![I(1)];
///     a[0] += a[0];
///     assert_eq!(a[0], I(2));
/// }
/// ```
#[proc_macro_attribute]
pub fn right_first_assign(attr: TokenStream, item: TokenStream) -> TokenStream {
    right_first_assign::right_first_assign(attr, item)
}
