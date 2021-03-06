use std::collections::HashSet;

use ::quote::quote;
use proc_macro2;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, LitBool, LitInt, Token, parenthesized, parse::{ParseStream, Parser}};
struct LtvFieldInfo {
    ltv_id: u8,
    ident: Option<syn::Ident>,
    ty: syn::Type,
    is_list: bool,
}

#[derive(Debug)]
pub enum ByteOrderOption {
    BE,
    LE,
    None,
}
impl Default for ByteOrderOption {
    fn default() -> Self {
        ByteOrderOption::None
    }
}

#[derive(Debug, Default)]
pub struct LTVObjectAttrabutes {
    pub object_id: Option<u8>,
    pub length_size: Option<u8>,
    pub field_length_size: Option<u8>,
    pub byte_order: ByteOrderOption,
    pub many: bool,
}

impl LTVObjectAttrabutes {
    pub fn parse(input: &DeriveInput) -> Self {
        if let Some(a) = input
            .attrs
            .iter()
            .filter(|a| a.path.is_ident("object"))
            .next()
        {
            let tokens = a.tokens.clone();
            let o = (|input_bracketed: ParseStream<'_>| -> syn::parse::Result<Self> {
                let input;
                parenthesized!(input in input_bracketed);

                let mut ltv_args = LTVObjectAttrabutes::default();

                let mut seen_arguments: HashSet<Ident> = HashSet::new();
                loop {
                    if input.is_empty() {
                        break;
                    }

                    let ident: Ident = input.parse()?;
                    let _eq_token: Token![=] = input.parse()?;

                    if !seen_arguments.insert(ident.clone()) {
                        return Err(syn::parse::Error::new(
                            ident.span(),
                            "argument appears more than once",
                        ));
                    }

                    let ident_str = ident.to_string();
                    match ident_str.as_str() {
                        "id" => {
                            ltv_args.object_id =
                                Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| {
                                    syn::parse::Error::new(
                                        ident.span(),
                                        "unexpected argument value; this should be a u8",
                                    )
                                })?);
                        }
                        "length_size" => {
                            ltv_args.length_size =
                                Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| {
                                    syn::parse::Error::new(
                                        ident.span(),
                                        "unexpected argument value; this should be a usize",
                                    )
                                })?);
                        }
                        "field_length_size" => {
                            ltv_args.field_length_size =
                                Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| {
                                    syn::parse::Error::new(
                                        ident.span(),
                                        "unexpected argument value; this should be a usize",
                                    )
                                })?);
                        }
                        "byte_order" => {
                            match input.parse::<Ident>()?.to_string().to_uppercase().as_str() {
                                "BE" => {
                                    ltv_args.byte_order = ByteOrderOption::BE;
                                }
                                "LE" => {
                                    ltv_args.byte_order = ByteOrderOption::LE;
                                }
                                _ => {
                                    return Err(syn::parse::Error::new(
                                        ident.span(),
                                        "byte_order must be BE or LE",
                                    ))
                                }
                            }
                        },
                        "many" => {
                            ltv_args.many = input.parse::<LitBool>()?.value();
                        }
                        _ => panic!("Invalid argument {}", &ident_str),
                    }

                    if input.is_empty() {
                        break;
                    }
                    let _: Token![,] = input.parse()?;
                }
                Ok(ltv_args)
            })
            .parse2(tokens)
            .unwrap();
            o
        } else {
            Self::default()
        }
    }
}

fn impl_ltv_named(
    input: &DeriveInput,
    fields_named: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let fields = fields_named.named.clone();

    let struct_name = format!("{}", input.ident);
    let attrs = LTVObjectAttrabutes::parse(&input);

    let ltv_fields: Vec<LtvFieldInfo> = fields
        .into_iter()
        .map(|f| {
            let ident_name = match &f.ident {
                Some(i) => i.to_string(),
                None => String::from("<No Name>"),
            };
            let full_name = format!("{}::{}", &struct_name, ident_name);

            let (is_list, ltv_id): (bool, u8) = {
                let ltv_id_attr = f
                    .attrs
                    .into_iter()
                    .filter(|e| e.path.is_ident("ltv_field") || e.path.is_ident("ltv_field_list"))
                    .next()
                    .expect(&format!(
                        "{} does not have ltv_field or ltv_field_list",
                        &full_name
                    ));

                let lit_id_lit_args: LitInt = ltv_id_attr.parse_args().expect(&format!(
                    "{} has invalid field id. Must be a number",
                    &full_name
                ));
                (
                    ltv_id_attr.path.is_ident("ltv_field_list"),
                    lit_id_lit_args
                        .base10_parse()
                        .expect(&format!("{} has invalid field id.", &full_name)),
                )
            };

            LtvFieldInfo {
                ltv_id,
                ident: f.ident,
                ty: f.ty,
                is_list: is_list || attrs.many,
            }
        })
        .collect();
    let st_name = &input.ident;

    let byte_order = match attrs.byte_order {
        ByteOrderOption::BE => quote! { {::ltv::ByteOrder::BE} },
        ByteOrderOption::LE => quote! { {::ltv::ByteOrder::LE} },
        ByteOrderOption::None => quote! { ED },
    };

    let byte_order_impl = match attrs.byte_order {
        ByteOrderOption::BE => quote! { impl },
        ByteOrderOption::LE => quote! { impl },
        ByteOrderOption::None => quote! {impl<const ED: ::ltv::ByteOrder> },
    };

    let field_length_size = attrs.field_length_size.unwrap_or(1) as usize;

    let from_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(|LtvFieldInfo { ident, ty, ltv_id, is_list }| {
            let branch_err_name = format!("{}::{:?}", st_name, match ident{
                Some(e) => format!("{}", e),
                None => "<Unknown>".into()
            });
            if *is_list {
                quote! {
                    #ident: reader.get_many::<<#ty as LTVItemMany<#byte_order>>::Item, _>(#ltv_id).map_err(|e| ::ltv::LTVError::InnerParseError(
                        e.into(),
                        format!(#branch_err_name)
                    ))?
                }
            }else{
                quote! {
                    #ident: reader.get_item::<#ty>(#ltv_id).map_err(|e| ::ltv::LTVError::InnerParseError(
                        e.into(),
                        format!(#branch_err_name)
                    ))?
                }
            }
        });

        quote! {
            fn from_ltv(field_id: u8, data: &[u8]) -> ::ltv::LTVResult<Self> {
                let reader = ::ltv::LTVReader::<#byte_order, #field_length_size>::new(&data);
                Ok(
                    Self{
                        #(#ltv_fields),*
                    }
                )
            }
        }
    };

    let to_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(
            |LtvFieldInfo {
                 ident,
                 ty,
                 ltv_id,
                 is_list,
             }| {
                if *is_list {
                    quote! {
                        for o in <#ty as LTVItemMany<#byte_order>>::get_items(&self.#ident){
                            buffer.write_ltv(#ltv_id, o).ok();
                        }
                    }
                } else {
                    quote! {
                        buffer.write_ltv(#ltv_id, &self.#ident).ok();
                    }
                }
            },
        );

        quote! {
            fn to_ltv(&self) -> Vec<u8>{
                let mut buffer = LTVWriter::<_, #byte_order, #field_length_size>::new(Vec::new());
                #(#ltv_fields)*
                buffer.into_inner()
            }
        }
    };
    let len_size = attrs.length_size.unwrap_or(1) as usize;


    let obj_impl = {
        if let Some(obj_id) = attrs.object_id {
            Some(quote! {
                #[automatically_derived]
                impl LTVObject<#len_size> for #st_name{
                    const OBJECT_ID: u8 = #obj_id;
                }
            })
        } else {
            None
        }
    };

    let e = quote! {
        #[automatically_derived]
        #byte_order_impl LTVItem<#byte_order> for #st_name {
            #from_ltv_fn
            #to_ltv_fn
        }

        #obj_impl
    };
    /*
    use std::fs;
    fs::write(
        format!("target/object_impl_{}.rs", &struct_name),
        e.to_string(),
    )
    .ok();
    */
    e
}

fn impl_ltv_unnamed(
    input: &DeriveInput,
    fields_unnamed: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let fields = fields_unnamed.unnamed.clone();
    let attrs = LTVObjectAttrabutes::parse(&input);
    //let struct_name = format!("{}", input.ident);

    let field = {
        let mut field_iter = fields.iter();
        let single_item = field_iter
            .next()
            .expect("Unnamed struct must have a inner type.");
        if let Some(_) = field_iter.next() {
            panic!("Unnamed struct must only have a single inner type.");
        }
        single_item
    };

    let struct_ident = &input.ident;
    let struct_ident_str = format!("{}", &input.ident);

    let byte_order = match attrs.byte_order {
        ByteOrderOption::BE => quote! { {::ltv::ByteOrder::BE} },
        ByteOrderOption::LE => quote! { {::ltv::ByteOrder::LE} },
        ByteOrderOption::None => quote! { ED },
    };

    let byte_order_impl = match attrs.byte_order {
        ByteOrderOption::BE => quote! { impl },
        ByteOrderOption::LE => quote! { impl },
        ByteOrderOption::None => quote! {impl<const ED: ::ltv::ByteOrder> },
    };

    let field_length_size = attrs.field_length_size.unwrap_or(1) as usize;

    let obj_impl = {
        if let Some(obj_id) = attrs.object_id {
            Some(quote! {
                #[automatically_derived]
                impl LTVObject<#field_length_size> for #struct_ident{
                    const OBJECT_ID: u8 = #obj_id;
                }
            })
        } else {
            None
        }
    };
   
    if attrs.many {
        let ltv_id = attrs.object_id.expect("Must have object ID with many");
       
        let e = quote! {
            #[automatically_derived]
            #byte_order_impl LTVItem<#byte_order> for #struct_ident {
                fn to_ltv(&self) -> Vec<u8>{
                    let mut buffer = LTVWriter::<_, #byte_order, #field_length_size>::new(Vec::new());
                    for o in <#field as LTVItemMany<#byte_order>>::get_items(&self.0){
                        buffer.write_ltv(#ltv_id, o).ok();
                    }
                    buffer.into_inner()
                }
    
                fn from_ltv(field_id: u8, data: &[u8]) -> ::ltv::LTVResult<Self> {
                    let reader = ::ltv::LTVReader::<#byte_order, #field_length_size>::new(&data);

                    Ok(Self(reader.get_many::<<#field as LTVItemMany<#byte_order>>::Item, _>(field_id).map_err(|e| ::ltv::LTVError::InnerParseError(
                        e.into(),
                        String::from(#struct_ident_str)
                    ))?))
                }
            }
            #obj_impl
        };
            /*
            use std::fs;
            fs::write(
                format!("target/object_impl_unnamed_{}.rs", &struct_ident),
                e.to_string(),
            )
            .ok();
            */
            e
    }else{
        let struct_ident = &input.ident;
        let e = quote! {
            #[automatically_derived]
            #byte_order_impl LTVItem<#byte_order> for #struct_ident {
                fn to_ltv(&self) -> Vec<u8>{
                    <#field as LTVItem<#byte_order>>::to_ltv(&self.0)
                }
    
                fn from_ltv(field_id: u8, data: &[u8]) -> ::ltv::LTVResult<Self> {
                    Ok(Self(<#field as LTVItem<#byte_order>>::from_ltv(field_id, data)?))
                }
            }
            #obj_impl
        };
            /*
            use std::fs;
            fs::write(
                format!("target/object_impl_unnamed_{}.rs", &struct_name),
                e.to_string(),
            )
            .ok();'
            */
            e
    }
 
}

pub fn impl_ltv(input: DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => impl_ltv_named(&input, fields),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => impl_ltv_unnamed(&input, fields),
        _ => panic!("this derive macro only works on structs"),
    }
}
/*

*/
