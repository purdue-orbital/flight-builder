#![no_std]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::parse_macro_input;

#[proc_macro_derive(States)]
pub fn create_states_for_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl States for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Event)]
pub fn create_event_for_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Event for #name {}
    };

    TokenStream::from(expanded)
}
