use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, LitInt, parse_macro_input};
use ::quote::{quote};
use proc_macro2::{self};


fn generate_from_ltv(fields: &[(usize, Option<syn::Ident>, syn::Type)]) -> proc_macro2::TokenStream {

    let ltv_fields = fields.iter().map(|(ltv_id, ident, ty)| {
        quote! {
            #ident: reader.get_item::<#ty>(#ltv_id)?
        }
    });
   
    quote! {
        fn from_ltv(field_id:usize, data: &[u8]) -> ::ltv::LTVResult<Self> {
            use ::ltv::{ed, LTVReader};
            let reader = LTVReader::<ed::BE, 1>::new(&data);
            Ok(
                Self{
                    #(#ltv_fields),*
                }
            )
        }
    }
}

fn generate_to_ltv(fields: &[(usize, Option<syn::Ident>, syn::Type)]) -> proc_macro2::TokenStream {
    let ltv_fields = fields.iter().map(|(ltv_id, ident, _ty)| {
        quote! {
            buffer.write_ltv(#ltv_id, &self.#ident).ok();
        }
    });

    quote! {
        fn to_ltv(&self) -> Vec<u8>{
            let mut buffer = Vec::new();
            #(#ltv_fields)*
            buffer
        }
    }
}

fn impl_ltv(input: DeriveInput) -> proc_macro2::TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let struct_name = format!("{}", input.ident);


    let ltv_fields : Vec<(usize, Option<syn::Ident>, syn::Type)> = fields.into_iter().map(|f| {
        let ident_name = match &f.ident{
            Some(i) => i.to_string(),
            None => String::from("<No Name>")
        };
        let full_name = format!("{}::{}", &struct_name,  ident_name);

        let ltv_id : usize = {
            let ltv_id_lit : LitInt = f.attrs
            .into_iter()
            .filter(|e| e.path.is_ident("ltv_field"))
            .next().expect(&format!("{} does not have ltv_field", &full_name))
            .parse_args()
            .expect(&format!("{} has invalid field id. Must be a number", &full_name));
            ltv_id_lit.base10_parse().expect(&format!("{} has invalid field id.", &full_name))
        };
        (ltv_id, f.ident, f.ty)
    }).collect();

    let from_ltv_fn = generate_from_ltv(&ltv_fields);
    let to_ltv_fn = generate_to_ltv(&ltv_fields);

    let st_name = input.ident;

    let byte_order = {
        let bo_lit : Option<Attribute> = input.attrs
        .into_iter()
        .filter(|e| e.path.is_ident("ltv"))
        .next();

        match bo_lit {
            Some(e) => quote!{ #e.tokens},
            None  =>  quote!{ ::ltv::DefaultED }
        }
    };
    quote! {
        // It's a good practice to use this attribute on macro-generated impl blocks.
        #[automatically_derived]
        impl LTVItem<'_, #byte_order> for #st_name {
            type Item = Self;

            #from_ltv_fn
            #to_ltv_fn
        }

        #[automatically_derived]
        impl LTVObject<'_, #byte_order, 1> for #st_name{}
    }
}

#[proc_macro_derive(Ltv, attributes(ltv_field, ltv))]
pub fn derive_helper_attr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_ltv(input).into()
}

