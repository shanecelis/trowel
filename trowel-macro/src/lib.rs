extern crate proc_macro;
use proc_macro::{TokenStream};

/// This attribute-like macro does nothing. It's a no-op.
#[proc_macro_attribute]
pub fn id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
