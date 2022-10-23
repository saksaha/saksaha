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
    let field_type = fields.iter().map(|field| {
        let a = &field.ty;
        println!("aaaaaaaaaaaaa, {:?}", a);
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
                    #field_name : ::#field_type::new(stringify!(#field_name).to_string()),
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
