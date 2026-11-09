//! Real procedural macros used by the selected rule examples.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, parse_macro_input};

#[proc_macro_derive(Greet)]
pub fn derive_greet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::checked_macros::Greet for #name #ty_generics #where_clause {
            fn greet(&self) -> String {
                ::checked_macros::__private::format_greeting(stringify!(#name))
            }
        }
    }
    .into()
}

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_hello_impl(&input).into()
}

fn generate_hello_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::checked_macros::Hello for #name #ty_generics #where_clause {
            fn hello(&self) -> &'static str { stringify!(#name) }
        }
    }
}

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    match syn::parse::<DeriveInput>(input).and_then(|input| derive_my_trait_inner(&input)) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn derive_my_trait_inner(input: &DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                &input.ident,
                "MyTrait can only be derived on structs, not enums",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                &input.ident,
                "MyTrait can only be derived on structs, not unions",
            ));
        }
    };
    let first = fields
        .iter()
        .next()
        .ok_or_else(|| Error::new_spanned(&input.ident, "MyTrait requires at least one field"))?;
    let field_name = match fields {
        Fields::Named(_) => first
            .ident
            .as_ref()
            .ok_or_else(|| Error::new_spanned(first, "MyTrait requires named fields"))?,
        _ => return Err(Error::new_spanned(first, "MyTrait requires named fields")),
    };
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::checked_macros::MyTrait for #name #ty_generics #where_clause {
            fn first_field_name() -> &'static str { stringify!(#field_name) }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_hello_preserves_type_parameters_and_where_clause() {
        let input: DeriveInput =
            syn::parse_quote! { struct Borrowed<'a, T: Clone> where T: 'a { value: &'a T } };
        let generated = generate_hello_impl(&input);
        let parsed: syn::ItemImpl =
            syn::parse2(generated).expect("generated impl must be valid Rust");
        assert_eq!(parsed.generics.params.len(), 2);
        assert!(parsed.generics.where_clause.is_some());
    }
}
