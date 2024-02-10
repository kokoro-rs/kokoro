use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

mod util;

use util::*;

fn sort(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let vis = input.vis;
    let name = input.ident;
    let generics = input.generics;
    let expanded: proc_macro2::TokenStream = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let mut named_fields: Vec<_> = fields.named.iter().collect();
            named_fields.sort_by_key(|f| hash(&f.ident.clone().unwrap().to_string()));
            quote! {
                #vis struct #name #generics {
                    #(#named_fields),*
                }
            }
        }
        Data::Struct(_) => panic!("Expected named fields"),
        Data::Enum(data) => {
            let mut variants: Vec<_> = data.variants.iter().collect();
            variants.sort_by_key(|v| hash(&v.ident.to_string()));
            quote! {
                #vis enum #name #generics {
                    #(#variants),*
                }
            }
        }
        _ => panic!("Expected a struct or enum"),
    };
    TokenStream::from(expanded)
}

/// Automatic impl Event trait
#[proc_macro_derive(Event)]
pub fn event_id(input: TokenStream) -> TokenStream {
    let pkg_name = get_pkg_name();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let type_string = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let type_str_list: Vec<String> = fields
                    .named
                    .iter()
                    .map(|f| {
                        format!(
                            "{}:{}",
                            f.ident.clone().unwrap().to_string(),
                            f.ty.to_token_stream().to_string()
                        )
                    })
                    .collect();
                format!(
                    "{pkg_name}@struct~{}{{{}}}",
                    name.to_string(),
                    type_str_list.join(";")
                )
            }
            Fields::Unit => {
                format!("{pkg_name}@struct~{}", name.to_string())
            }
            Fields::Unnamed(fields) => {
                let type_str_list: Vec<String> = fields
                    .unnamed
                    .iter()
                    .map(|f| f.ty.to_token_stream().to_string())
                    .collect();
                format!(
                    "{pkg_name}@struct~{}({})",
                    name.to_string(),
                    type_str_list.join(",")
                )
            }
        },
        Data::Enum(data) => {
            let type_str_list: Vec<String> = data
                .variants
                .iter()
                .map(|v| {
                    format!(
                        "{}{}{}",
                        v.ident.to_string(),
                        v.fields.to_token_stream().to_string(),
                        if let Some((_, y)) = &v.discriminant {
                            format!("={}", y.to_token_stream().to_string())
                        } else {
                            "".to_string()
                        }
                    )
                })
                .collect();
            format!(
                "{pkg_name}@enum~{}{{{}}}",
                name.to_string(),
                type_str_list.join(",")
            )
        }
        _ => panic!("Expected a struct or enum"),
    };
    let hash = hash(&type_string);
    let expanded = quote! {
        #[doc = #type_string]
        impl kokoro::event::Event for #name {
            fn event_id(&self) -> &'static kokoro::event::EventId where Self: Sized {
                &kokoro::event::EventId(#hash)
            }
        }
        #[doc = stringify!(#hash)]
        impl kokoro::event::EventID for #name {
            const _EVENT_ID: &'static kokoro::event::EventId = &kokoro::event::EventId(#hash);
        }
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}
    };
    TokenStream::from(expanded)
}

/// Fields are sorted and automatically impl Event
#[proc_macro_attribute]
pub fn stable_sorted_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let sort = sort(attr, item);
    let stable = event_id(sort.clone());
    let sort = proc_macro2::TokenStream::from(sort);
    let stable = proc_macro2::TokenStream::from(stable);
    let expanded = quote! {
        #sort
        #stable
    };
    TokenStream::from(expanded)
}
