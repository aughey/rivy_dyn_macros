use dyn_macros_lib::process_function;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse item as a function
    let ast = syn::parse::<syn::ItemFn>(item).unwrap();

    process_function(ast, attr.into()).unwrap().into()
}

#[proc_macro_attribute]
pub fn stream_type(_attr: TokenStream,item: TokenStream) -> TokenStream {
    // parse item as a function
    // let ast = syn::parse::<syn::Struc>(item).unwrap();

    // process_function(ast, attr.into()).unwrap().into()
    item
}

#[proc_macro_attribute]
pub fn iterator_type(_attr: TokenStream,item: TokenStream) -> TokenStream {
    // parse item as a function
    // let ast = syn::parse::<syn::Struc>(item).unwrap();

    // process_function(ast, attr.into()).unwrap().into()
    item
}
