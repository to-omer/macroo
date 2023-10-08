use proc_macro::TokenStream;

mod default_int_suffix;

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
