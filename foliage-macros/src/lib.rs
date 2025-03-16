extern crate proc_macro;
use convert_case::{Case, Casing};
use core::panic;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemTrait, TraitItem, Type};

mod helper;
mod options;

fn parse_item<FunctionItemCallback>(
    item: &ItemTrait,
    mut function_item_callback: FunctionItemCallback,
) where
    FunctionItemCallback:
        FnMut(&Ident, &Ident, bool, &Vec<Box<Type>>, &Vec<Ident>, &Box<Type>, &Box<Type>),
{
    for item in &item.items {
        if let TraitItem::Fn(function) = item {
            let function_name = &function.sig.ident;
            let variant_name =
                format_ident!("{}", function_name.to_string().to_case(Case::UpperCamel));

            let is_async = function.sig.asyncness.is_some();

            // Parse parameter types / input types
            let input_types: Vec<Box<Type>> = function
                .sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let syn::FnArg::Typed(pat_type) = arg {
                        Some(pat_type.ty.clone())
                    } else {
                        None
                    }
                })
                .collect();
            let input_idents: Vec<Ident> = input_types
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("param{}", i))
                .collect();

            // Parse return type / output type
            let (ok_output_type, err_output_type) = match &function.sig.output {
                syn::ReturnType::Type(_, return_type) => {
                    match helper::extract_result_types(return_type) {
                        helper::ResultTypes::ResultType(ok_type, err_type) => (ok_type, err_type),
                        helper::ResultTypes::NonResultType(_) => {
                            panic!("unsupported function return type")
                        }
                    }
                }
                syn::ReturnType::Default => panic!("unsupported function return type"),
            };

            // Ensure function does not define any body
            if function.default.is_some() {
                panic!(
                    "function '{}' cannot define a default body",
                    function.sig.ident.to_string()
                )
            }

            function_item_callback(
                &function_name,
                &variant_name,
                is_async,
                &input_types,
                &input_idents,
                &ok_output_type,
                &err_output_type,
            );
        }
    }
}

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(attr.into()).unwrap();
    let _ = options::ServiceOptions::from_list(&attr_args).unwrap();

    let item = parse_macro_input!(item as ItemTrait);

    let mut request_variants = Vec::new();
    let mut response_variants = Vec::new();
    let mut error_variants = Vec::new();

    let mut my_rpcs = Vec::new();
    let mut peer_rpcs = Vec::new();

    let type_ident = item.ident.clone();

    // Generate identifiers
    let request_type = helper::ident_to_type(&format_ident!("{}Request", type_ident));
    let response_type = helper::ident_to_type(&format_ident!("{}Response", type_ident));
    let error_type = helper::ident_to_type(&format_ident!("{}Error", type_ident));

    let my_service_ident = format_ident!("My{}", type_ident);
    let other_service_ident = format_ident!("Other{}", type_ident);
    let peer_ident = format_ident!("{}Peer", type_ident);

    parse_item(
        &item,
        |function_name,
         variant_name,
         is_async: bool,
         input_types,
         input_idents,
         output_ok_type,
         output_err_type| {
            request_variants.push(quote! { #variant_name(#(#input_types),*) });
            response_variants.push(quote! { #variant_name(#output_ok_type) });
            error_variants.push(quote! { #variant_name(#output_err_type) });

            // My service match arms
            {
                let postfix = if is_async {
                    Some(quote! {.await})
                } else {
                    None
                };

                my_rpcs.push(quote! {
                    #request_type::#variant_name(#(#input_idents),*) => {
                        match self.0.#function_name(#(#input_idents),*)#postfix {
                            Ok(ok) => Ok(#response_type::#variant_name(ok)),
                            Err(err) => Err(#error_type::#variant_name(err))
                        }
                    }
                });
            }

            // Peer functions
            {
                peer_rpcs.push(quote! {
                    pub async fn #function_name(&mut self, #(#input_idents: #input_types),*) -> foliage_rpc::error::peer::ResultRpc<#output_ok_type, #output_err_type> {
                        match self.peer.rpc(#request_type::#variant_name(#(#input_idents),*)).await {
                            Ok(#response_type::#variant_name(response)) => Ok(response),
                            Ok(_) => Err(foliage_rpc::error::peer::ErrorRpc::InternalError { message: "unexpected response type" }),
                            Err(foliage_rpc::error::peer::ErrorRpc::RpcError(#error_type::#variant_name(err))) => Err(foliage_rpc::error::peer::ErrorRpc::RpcError(err)),
                            Err(foliage_rpc::error::peer::ErrorRpc::RpcError(_)) => Err(foliage_rpc::error::peer::ErrorRpc::InternalError { message: "unexpected error type" }),
                            Err(err) => Err(foliage_rpc::error::peer::error_rpc_match(err, |_| "internal error".to_string())),
                        }
                    }
                });
            }
        },
    );

    // Generate request enum
    let request_enum = quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum #request_type {
            #(#request_variants),*
        }
    };

    // Generate response enum
    let response_enum = quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum #response_type {
            #(#response_variants),*
        }
    };

    // Generate error enum
    let error_enum = quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum #error_type {
            #(#error_variants),*
        }
    };

    // My service
    let my_service = {
        quote! {
            pub struct #my_service_ident<T: #type_ident + Send + Sync>(T);

            impl<T: #type_ident + Send + Sync> #my_service_ident<T> {
                pub fn new(t: T) -> Self {
                    Self(t)
                }
            }

            impl<T: #type_ident + Send + Sync> foliage_rpc::MyService for #my_service_ident<T> {
                type Request = #request_type;
                type Response = #response_type;
                type Error = #error_type;

                async fn on_rpc(&self, request: #request_type) -> Result<#response_type, #error_type> {
                    match request {
                        #(#my_rpcs),*
                    }
                }
            }
        }
    };

    // Other service
    let other_service = quote! {
            pub struct #other_service_ident;

            impl foliage_rpc::OtherService for #other_service_ident {
                type Request = #request_type;
                type Response = #response_type;
                type Error = #error_type;
            }

    };

    // Peer
    let peer = quote! {
        pub struct #peer_ident {
            peer: foliage_rpc::Peer<#other_service_ident>,
        }

        impl #peer_ident {
            #(#peer_rpcs)*
        }

        impl From<foliage_rpc::Peer<#other_service_ident>> for #peer_ident {
            fn from(peer: foliage_rpc::Peer<#other_service_ident>) -> Self {
                Self { peer }
            }
        }
    };

    TokenStream::from(quote! {
        #request_enum
        #response_enum
        #error_enum

        #my_service
        #other_service
        #peer

        #item
    })
}
