use dyn_macros_lib::process_function;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse item as a function
    let ast = syn::parse::<syn::Item>(item).unwrap();

    process_function(ast, attr.into()).unwrap().into()
}
