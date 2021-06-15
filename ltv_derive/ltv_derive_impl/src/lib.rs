mod object;
mod sets;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Ltv, attributes(ltv_field, ltv))]
pub fn derive_ltv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    object::impl_ltv(input).into()
}

#[proc_macro_derive(LtvSet, attributes(ltv))]
pub fn derive_ltv_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sets::impl_ltv_set(input).into()
}
