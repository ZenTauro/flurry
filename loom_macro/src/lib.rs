use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro_attribute]
pub fn wrapper(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let ts = input.clone();
    let ast = parse_macro_input!(ts);

    if cfg!(all(loom, test)) {
        let ts = quote! {
            loom::model(|| {
                #ast
            })
        };
        ts.into()
    } else {
        input
    }
}
