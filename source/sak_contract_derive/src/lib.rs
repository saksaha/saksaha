use proc_macro::TokenStream;

mod v0;

#[proc_macro_derive(MRSStore)]
pub fn derive_mrs_store(input: TokenStream) -> TokenStream {
    v0::_derive_mrs_store(input)
}

#[proc_macro_derive(CtrStateStore)]
pub fn derive_ctr_state_store(input: TokenStream) -> TokenStream {
    v0::_derive_ctr_state_store(input)
}
