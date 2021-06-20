use super::object::{ByteOrderOption, LTVObjectAttrabutes};

use ::quote::quote;
use proc_macro2::{self, Ident};
use syn::{Data, DataEnum, DeriveInput, Field};

#[derive(Clone)]
struct LtvCollectionInfo {
    pub enum_field: Ident,
    pub inner_data: Field,
}

pub fn impl_ltv_collection(input: DeriveInput) -> proc_macro2::TokenStream {
    let attrs = LTVObjectAttrabutes::parse(&input);
    let enum_ident = input.ident;

    let variants: Vec<LtvCollectionInfo> = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("this derive macro only works on enums"),
    }
    .into_iter()
    .map(|v| {
        let inner_data = v
            .fields
            .into_iter()
            .next()
            .expect("Requires enums with LTVItems as fields.  `MyEnum::Object1(MyLtvItem)");
        LtvCollectionInfo {
            enum_field: v.ident,
            inner_data,
        }
    })
    .collect();

    let byte_order = match attrs.byte_order {
        ByteOrderOption::BE => quote! { {::ltv::ByteOrder::BE} },
        ByteOrderOption::LE => quote! { {::ltv::ByteOrder::LE} },
        ByteOrderOption::None => quote! { T },
    };

    let byte_order_impl = match attrs.byte_order {
        ByteOrderOption::BE => quote! { impl LTVItem<{::ltv::ByteOrder::BE}> },
        ByteOrderOption::LE => quote! {impl LTVItem<{::ltv::ByteOrder::LE}> },
        ByteOrderOption::None => quote! {impl<const ED: ::ltv::ByteOrder> LTVItem<ED> },
    };
    let len_size = attrs.length_size.unwrap_or(1) as usize;

    let from_ltv_fn = {
        let object_match_branches = variants.iter().map(|info| {
            let inner_ltv = &info.inner_data;
            let branch_name = &info.enum_field;
            let branch_err_name = format!("{}::{}",enum_ident, branch_name);
            quote! {
                <#inner_ltv as ::ltv::LTVObject<#len_size>>::OBJECT_ID =>
                    Ok(Self::#branch_name(
                        <#inner_ltv as ::ltv::LTVItem<#byte_order>>::from_ltv(
                            <#inner_ltv as ::ltv::LTVObject<#len_size>>::OBJECT_ID, data
                        ).map_err(|e| ::ltv::LTVError::InnerParseError(
                            e.into(),
                            format!(#branch_err_name)
                        ))?
                    )
                )
            }
        });

        quote! {
            fn from_ltv(field_id:u8, data: &[u8]) -> ::ltv::LTVResult<Self> {
                match field_id {
                    #(#object_match_branches),*
                    ,_ => Err(
                            ::ltv::LTVError::NotFound(field_id)
                    )
                }
            }
        }
    };

    let to_ltv_fn = {
        let object_match_branches = variants.iter().map(|info| {
            let branch_name = &info.enum_field;
            let inner_ltv = &info.inner_data;
            quote! {
                Self::#branch_name(v) => <#inner_ltv as LTVItem<#byte_order>>::to_ltv(v)
            }
        });

        quote! {
            fn to_ltv(&self) -> Vec<u8>{
                match self {
                    #(#object_match_branches),*
                }
            }
        }
    };

    let to_ltv_object_branches = {
        let object_match_branches = variants.iter().map(|info| {
            let branch_name = &info.enum_field;
            let inner_ltv = &info.inner_data;
            quote! {
                Self::#branch_name(v) => <#inner_ltv as LTVObjectConvertable<#byte_order, #len_size>>::to_ltv_object(v)
            }
        });

        quote! {
            match self {
                #(#object_match_branches),*
            }
        }
    };

    let e = quote! {
        #[automatically_derived]
        #byte_order_impl for #enum_ident {
            #from_ltv_fn
            #to_ltv_fn
        }

        impl <'a> LTVObjectConvertable<'a, #byte_order, #len_size> for #enum_ident {
            fn from_ltv_object(data: &'a [u8]) -> LTVResult<Self> {
                let (_, obj_id, data) = ::ltv::LTVReader::<'a, #byte_order, #len_size>::parse_ltv(data)?;
                Ok(<Self as LTVItem<#byte_order>>::from_ltv(obj_id, data)?)
            }

            fn to_ltv_object(&self) -> Vec<u8> {
                #to_ltv_object_branches
            }
        }
    };
    /*
    use std::fs;
    fs::write(
        format!("target/object_impl_{}.rs", &enum_ident),
        e.to_string(),
    )
    .ok();
    */
    e
}
