mod v0;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Storage)]
pub fn derive_storage_param(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);
    let struct_name = &input.ident;

    println!("333333333333333333333333333 {}", struct_name);

    TokenStream::from(quote! {
        impl #struct_name {
            fn as_default() -> #struct_name {
                let a = #struct_name {#(
                    #field_name : #field_type::new(stringify!(#field_name).to_string()),
                )*};

                println!("a: {:?}", a);

                return a;
            }
        }
    })
}
