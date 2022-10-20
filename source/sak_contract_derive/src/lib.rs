use proc_macro::TokenStream;

mod v0;

#[proc_macro_derive(Storage)]
pub fn derive_storage_param(input: TokenStream) -> TokenStream {
    v0::_derive_storage_param(input)
}
