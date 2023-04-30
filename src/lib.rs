use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(BirdBinding)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, .. } = parse_macro_input!(input as DeriveInput);
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(named),
        ..
    }) = data
    {
        named
    } else {
        panic!("Not supported")
    };
    let binding_ident: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let binding_names: Vec<_> = binding_ident
        .iter()
        .map(|ident| ident.to_string())
        .collect();
    let output = quote! {
        impl bird_binding::Bindings for #ident {
             fn binding_names() -> Vec<String>{
                 vec![#(#binding_names.to_string(), )*]
             }

             fn bindings(&self) -> Vec<bird_binding::Binding>{
                 vec![#(self.#binding_ident.clone(), )*]
             }
        }
    };

    output.into()
}

#[proc_macro_derive(BindFoos)]
pub fn foo(input: TokenStream) -> TokenStream {
    let DeriveInput { data, .. } = parse_macro_input!(input as DeriveInput);
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(named),
        ..
    }) = data
    {
        named
    } else {
        panic!("Not supported")
    };
    let binding_ident: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let binding_names: Vec<_> = binding_ident
        .iter()
        .map(|ident| ident.to_string())
        .collect();
    let output = quote! {
        #(pub fn #binding_ident(input: bevy::prelude::Res<bird_binding::UserInput>) -> bool{
           input.check(#binding_names)
        }) *

    };

    output.into()
}
