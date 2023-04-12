extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;

/// This attribute-like macro does nothing. It's a no-op.
#[proc_macro_attribute]
pub fn id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn omit(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn add_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut entry = TokenStream::from(quote! {
        #[rp_pico::entry]
        fn entry() -> ! {
            main();
            loop {}
        }
    });
    entry.extend(item);
    entry
}

// This is what the code looks like in use:
//
// fn main() {
//     trowel::run(DrawFerris { frame: 10 });
// }
// #[trowel::entry]
// fn entry() -> ! {
//     main();
//     loop {}
// }
//
// It'd be nice to do something like this:
// #[trowel::entry]
// fn main() { ... }
//
// And that would do the same thing as above.
