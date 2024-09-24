extern crate proc_macro;

#[rustversion::any(before(1.43.0), all(nightly, before(2020-02-27)))]
#[proc_macro_attribute]
pub fn compat_expect(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // using #[allow(...)] does not work before 1.43.0 due to a bug
    item
}

#[rustversion::all(
    any(since(1.43.0), all(nightly, since(2020-02-27))),
    any(before(1.81), all(nightly, before(2024-06-27))))]
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
        #[allow(#attr)]
        #item
    };
    expanded.into()
}

#[rustversion::any(since(1.81), all(nightly, since(2024-06-27)))]
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
