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
    // Consider making this read the function name it is prefixed to. Going
    // against that, however, what value is there if it's not main? That breaks
    // the expectations that it'll be the entry point for both sprig and pc
    // simulator.
    //
    // https://stackoverflow.com/questions/56718336/rust-function-name-caller-or-any-other-context-inside-macro
    let parseable = item.clone();
    let input = syn::parse_macro_input!(parseable as syn::ItemFn);
    let fn_name = input.sig.ident;
    let mut entry = TokenStream::from(quote! {
        #[rp_pico::entry]
        fn entry() -> ! {
            #fn_name();
            loop {}
        }
    });
    // We can avoid an extend by doing #&item above.
    entry.extend(item);
    entry
}

// This is what the code looks like in use:
//
// fn main() { ... }
//
// #[rp_pico::entry]
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
