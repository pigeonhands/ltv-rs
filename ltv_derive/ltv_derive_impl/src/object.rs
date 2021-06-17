use std::collections::HashSet;

use ::quote::quote;
use proc_macro2;
use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput,AttributeArgs, Fields, LitInt, LitStr, NestedMeta};

struct LtvFieldInfo {
    ltv_id: usize,
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(Debug)]
enum ByteOrderOption {
    BE,
    LE
}
impl Default for ByteOrderOption{
    fn default() -> Self{
        ByteOrderOption::BE
    }
}

#[derive(Debug, Default)]
struct LTVObjectAttrabutes {
    pub object_id: Option<u8>,
    pub length_size: Option<u8>,
    pub field_length_size: Option<u8>,
    pub byte_order: ByteOrderOption,
}

impl LTVObjectAttrabutes {
    pub fn parse(input: &DeriveInput) -> Self {
        let mut attrs = Self::default();

        for m in input.attrs
        .iter()
        .map(|a| a.parse_meta().expect("Failed to parse meta")){
            match m {
                syn::Meta::NameValue(nv) => {
                    if nv.path.is_ident("object_id"){
                        attrs.object_id = match &nv.lit {
                            syn::Lit::Int(i) => Some(i.base10_parse().expect("object_id myst be u8")),
                            _ => panic!("Bad object_id"),
                        }
                    }
                    if nv.path.is_ident("length_size"){
                        attrs.length_size = match &nv.lit {
                            syn::Lit::Int(i) => Some(i.base10_parse().expect("length_size myst be u8")),
                            _ => panic!("Bad length_size"),
                        }
                    }
                   
                },
                syn::Meta::Path(p) => {
                    if p.is_ident("be"){
                        attrs.byte_order = ByteOrderOption::BE;
                    }
                    if p.is_ident("le"){
                        attrs.byte_order = ByteOrderOption::LE;
                    }
                },
                _ => {},
            }
           
        }
    
       attrs
    }
}

pub fn impl_ltv(input: DeriveInput) -> proc_macro2::TokenStream {
    let attrs = LTVObjectAttrabutes::parse(&input);

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let struct_name = format!("{}", input.ident);

   
    
    //let list: syn::MetaList = first_attr.parse_args().expect("Not meta.");


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


/*
    let s = fields_named.attrs
        .into_iter().map(|e| format!("{}", e.tokens))
        .collect::<Vec<String>>()
        .join(", ");
    panic!("{}", s); 


    let s = input
        .attrs
        .into_iter().map(|e| format!("{}", e.parse_args::<syn::FieldsNamed>()))
        .collect::<Vec<String>>()
        .join(", ");
    panic!("{}", s); 
     */


 
    let byte_order = match attrs.byte_order{
        ByteOrderOption::BE => quote! { ::ltv::ByteOrder::BE },
        ByteOrderOption::LE => quote! { ::ltv::ByteOrder::LE },
    };

    let field_length_size = attrs.field_length_size.unwrap_or(1) as usize;

    let from_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(|LtvFieldInfo { ident, ty, ltv_id }| {
            quote! {
                #ident: reader.get_item::<#ty>(#ltv_id)?
            }
        });

        
        quote! {
            fn from_ltv(field_id:usize, data: &[u8]) -> ::ltv::LTVResult<Self> {
                use ::ltv::LTVReader;
                let reader = LTVReader::<{#byte_order}, #field_length_size>::new(&data);
                Ok(
                    Self{
                        #(#ltv_fields),*
                    }
                )
            }
        }
    };

    let to_ltv_fn = {
        let ltv_fields = ltv_fields.iter().map(|LtvFieldInfo { ident, ltv_id, .. }| {
            quote! {
                buffer.write_ltv(#ltv_id, &self.#ident).ok();
            }
        });

        quote! {
            fn to_ltv(&self) -> Vec<u8>{
                let mut buffer = LTVWriter::<_, {#byte_order}, #field_length_size>::new(Vec::new());
                #(#ltv_fields)*
                buffer.into_inner()
            }
        }
    };

    let st_name = input.ident;

    let obj_impl = {
        if let Some(obj_id) = attrs.object_id {
            let len_size = attrs.length_size.unwrap_or(1) as usize;
            Some(
                quote! {
                    #[automatically_derived]
                    impl LTVObject<'_, {#byte_order}, #len_size, #obj_id> for #st_name{}
                }
            )
        }else{
            None
        }
    };
    

    quote! {
        #[automatically_derived]
        impl LTVItem<{#byte_order}> for #st_name {
            type Item = Self;

            #from_ltv_fn
            #to_ltv_fn
        }

        #obj_impl
    }
}
/*

use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    Data, DataStruct, DeriveInput, Fields, Ident, LitInt, Token,
};
impl LTVObjectAttrabutes {
    pub fn parse(input: &DeriveInput) -> Self {
        if let Some(a) = input.attrs.iter().filter(|a| a.path.is_ident("ltv")).next() {
            let tokens = a.tokens.clone();

            let o = (|input: ParseStream<'_>| -> syn::parse::Result<Self> {
                panic!("{}", input);
                let mut ltv_args = LTVObjectAttrabutes::default();

                let seen_arguments : HashSet<Ident> = HashSet::new();
                loop{
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
                            ltv_args.object_id = Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| syn::parse::Error::new(
                                ident.span(),
                                "unexpected argument value; this should be a u8",
                            ))?);
                        },
                        "length_size" => {
                            ltv_args.length_size = Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| syn::parse::Error::new(
                                ident.span(),
                                "unexpected argument value; this should be a usize",
                            ))?);
                        },"field_length_size" => {
                            ltv_args.field_length_size = Some(input.parse::<LitInt>()?.base10_parse().map_err(|_| syn::parse::Error::new(
                                ident.span(),
                                "unexpected argument value; this should be a usize",
                            ))?);
                        }, "byte_order" => {
                            match input.parse::<Ident>()?.to_string().to_uppercase().as_str() {
                                "BE" => {
                                    ltv_args.byte_order = ByteOrderOption::BE;
                                }, "LE" => {
                                    ltv_args.byte_order = ByteOrderOption::LE;
                                },
                                _ => return Err(syn::parse::Error::new(
                                    ident.span(),
                                    "byte_order must be BE or LE",
                                ))
                            }
                        },
                        _ => panic!("Invalid argument {}", &ident_str),
                    }
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
*/