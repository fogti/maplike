// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Assign)]
pub fn derive_assign(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_assign(input).unwrap_or_else(|err| err.to_compile_error().into())
}

#[proc_macro_derive(Container)]
pub fn derive_container(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_container(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_assign(input: DeriveInput) -> syn::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;

    let output = quote! {
        impl #impl_generics ::maplike::ops::Assign for #name #ty_generics
        #where_clause
        {
            fn assign(&mut self, value: Self) {
                *self = value;
            }
        }
    };
    Ok(output.into())
}

fn expand_container(input: DeriveInput) -> syn::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;

    let output = quote! {
        impl #impl_generics ::maplike::containers::Container for #name #ty_generics
        #where_clause
        {
            type Value = Self;
        }
    };
    Ok(output.into())
}
