use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

pub(crate) fn _derive_mrs_store(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_name2 = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| {
        let a = &field.ty;
        println!("field_type, {:?}", a);
        a
    });

    let struct_name = &input.ident;

    TokenStream::from(quote! {
        type _MRS = #struct_name;

        fn make_mrs_storage_param() -> #struct_name {
            #struct_name::new_as_contract_param()
        }

        impl #struct_name {
            fn new_as_contract_param() -> #struct_name {
                let a = #struct_name {#(
                    // #field_name: <sak_contract_std::parse_generics!(#field_type)>::default(),
                    #field_name: <sak_contract_std::parse_generics!(#field_type)>::new(stringify!(#field_name).to_string()),
                    // #field_name : sak_contract_std::temp!(#field_type::new(stringify!(#field_name).to_string())),
                )*};

                println!("a: {:?}", a);

                unsafe {
                    HOST__log(1, 2);
                }

                return a;
            }

            pub fn receipt(&self) -> std::collections::HashMap<String, Vec<u8>> {
                println!("receipt!!!");

                let mut map = std::collections::HashMap::new();

                #(
                    map.extend(self.#field_name2.receipt());
                )*

                map
            }
        }
    })
}

pub(crate) fn _derive_ctr_state_store(input: TokenStream) -> TokenStream {
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

    TokenStream::from(quote! {
        fn make_mrs_storage_param() -> #struct_name {
            #struct_name::new_as_contract_param()
        }

        impl #struct_name {
            fn new_as_contract_param() -> #struct_name {
                let a = #struct_name {#(
                    #field_name : #field_type::new(stringify!(#field_name).to_string()),
                )*};

                println!("a: {:?}", a);

                unsafe {
                    HOST__log(1, 2);
                }

                return a;
            }
        }
    })
}
