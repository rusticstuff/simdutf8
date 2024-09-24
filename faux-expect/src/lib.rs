extern crate proc_macro;

#[rustversion::not(any(since(2024-06-27), since(1.81)))]
#[proc_macro_attribute]
pub fn compat_expect(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[rustversion::any(since(2024-06-27), since(1.81))]
#[proc_macro_attribute]
pub fn compat_expect(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let expanded = quote! {
        #[expect(#attr)]
        #item
    };
    expanded.into()
}
