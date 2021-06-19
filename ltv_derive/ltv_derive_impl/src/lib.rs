#![feature(const_generics)]
mod collection;
mod object;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Ltv, attributes(ltv_field, ltv_field_list, object))]
pub fn derive_ltv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    object::impl_ltv(input).into()
}

#[proc_macro_derive(LtvCollection, attributes(object))]
pub fn derive_ltv_set(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input_tokens as DeriveInput);
    collection::impl_ltv_collection(input).into()
}
