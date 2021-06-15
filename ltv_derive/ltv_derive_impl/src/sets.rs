use ::quote::quote;
use proc_macro2;
use syn::{Attribute, Data, DataEnum, DeriveInput, Fields, LitInt};

struct LtvSetInfo {
    pub obj_id: usize,
}

pub fn impl_ltv_set(input: DeriveInput) -> proc_macro2::TokenStream {
    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("this derive macro only works on enums"),
    };

    let struct_name = format!("{}", input.ident);

    let ltv_objs: Vec<LtvSetInfo> = variants
        .into_iter()
        .map(|v| {
            let full_name = format!("{}::{}", &struct_name, v.ident);

            let obj_id: usize = {
                let lit: LitInt = v
                    .attrs
                    .iter()
                    .filter(|a| a.path.is_ident("ltv"))
                    .next()
                    .expect(&format!("{} does not have ltv(n) attribute", &full_name))
                    .parse_args()
                    .expect(&format!(
                        "{} has invalid ltv object id. Must be a number",
                        &full_name
                    ));

                lit.base10_parse()
                    .expect(&format!("{} has invalid ltv object id.", &full_name))
            };

            let fields = v.fields;
            LtvSetInfo { obj_id }
        })
        .collect();

    Default::default()
}
