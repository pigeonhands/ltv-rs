use ::quote::quote;
use proc_macro2;
use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, LitInt};

struct LtvFieldInfo {
    ltv_id: usize,
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub fn impl_ltv(input: DeriveInput) -> proc_macro2::TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let struct_name = format!("{}", input.ident);

    let ltv_fields: Vec<LtvFieldInfo> = fields
        .into_iter()
        .map(|f| {
            let ident_name = match &f.ident {
                Some(i) => i.to_string(),
                None => String::from("<No Name>"),
            };
            let full_name = format!("{}::{}", &struct_name, ident_name);

            let ltv_id: usize = {
                let ltv_id_lit: LitInt = f
                    .attrs
                    .into_iter()
                    .filter(|e| e.path.is_ident("ltv_field"))
                    .next()
                    .expect(&format!("{} does not have ltv_field", &full_name))
                    .parse_args()
                    .expect(&format!(
                        "{} has invalid field id. Must be a number",
                        &full_name
                    ));
                ltv_id_lit
                    .base10_parse()
                    .expect(&format!("{} has invalid field id.", &full_name))
            };

            LtvFieldInfo {
                ltv_id,
                ident: f.ident,
                ty: f.ty,
            }
        })
        .collect();

    let byte_order = {
        let bo_lit: Option<Attribute> = input
            .attrs
            .into_iter()
            .filter(|e| e.path.is_ident("ltv"))
            .next();
        match bo_lit {
            Some(Attribute { tokens,..  }) => quote! { ::ltv::#tokens},
            None => quote! { ::ltv::DEFAULT_ED },
        }
    };

    let from_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(|LtvFieldInfo { ident, ty, ltv_id }| {
            quote! {
                #ident: reader.get_item::<#ty>(#ltv_id)?
            }
        });

        quote! {
            fn from_ltv(field_id:usize, data: &[u8]) -> ::ltv::LTVResult<Self> {
                use ::ltv::LTVReader;
                let reader = LTVReader::<{#byte_order}, 1>::new(&data);
                Ok(
                    Self{
                        #(#ltv_fields),*
                    }
                )
            }
        }
    };

    let to_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(|LtvFieldInfo { ident, ltv_id,.. }| {
            quote! {
                buffer.write_ltv(#ltv_id, &self.#ident).ok();
            }
        });

        quote! {
            fn to_ltv(&self) -> Vec<u8>{
                let mut buffer = LTVWriter::<_, {#byte_order}>::new(Vec::new());
                #(#ltv_fields)*
                buffer.into_inner()
            }
        }
    };

    let st_name = input.ident;

    quote! {
        #[automatically_derived]
        impl LTVItem<{#byte_order}> for #st_name {
            type Item = Self;

            #from_ltv_fn
            #to_ltv_fn
        }

        #[automatically_derived]
        impl LTVObject<'_, {#byte_order}, 1> for #st_name{}
    }
}
