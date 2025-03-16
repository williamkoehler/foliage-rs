#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate proc_macro;
use convert_case::{Case, Casing};
use core::panic;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Item, Type};
mod helper {
    use syn::{GenericArgument, Ident, Path, PathSegment, Type, TypePath};
    pub enum ResultTypes {
        ResultType(Box<Type>, Box<Type>),
        NonResultType(Box<Type>),
    }
    pub fn extract_result_types(ty: &Type) -> ResultTypes {
        if let Type::Path(return_type_path) = ty {
            let last_segement = return_type_path.path.segments.last().unwrap();
            if last_segement.ident == "Result" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segement
                    .arguments
                {
                    let ok_argument = args.args.first().unwrap();
                    let ok_ty = if let GenericArgument::Type(ok_ty) = ok_argument {
                        ok_ty
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("Expected type ok argument"),
                            );
                        }
                    };
                    let err_argument = args.args.last().unwrap();
                    let err_ty = if let GenericArgument::Type(err_ty) = err_argument {
                        err_ty
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("Expected type err argument"),
                            );
                        }
                    };
                    return ResultTypes::ResultType(
                        Box::new(ok_ty.clone()),
                        Box::new(err_ty.clone()),
                    );
                }
            }
        }
        ResultTypes::NonResultType(Box::new(ty.clone()))
    }
    pub fn ident_to_type(ident: &Ident) -> Type {
        Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            PathSegment {
                                ident: ident.clone(),
                                arguments: syn::PathArguments::None,
                            },
                        ]),
                    )
                    .into_iter()
                    .collect(),
            },
        })
    }
}
#[darling(default)]
struct Options {
    request: Option<Type>,
    response: Option<Type>,
    error: Option<Type>,
    my_service: bool,
    other_service: bool,
    peer: bool,
}
#[automatically_derived]
impl ::core::default::Default for Options {
    #[inline]
    fn default() -> Options {
        Options {
            request: ::core::default::Default::default(),
            response: ::core::default::Default::default(),
            error: ::core::default::Default::default(),
            my_service: ::core::default::Default::default(),
            other_service: ::core::default::Default::default(),
            peer: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(clippy::manual_unwrap_or_default)]
impl ::darling::FromMeta for Options {
    fn from_list(__items: &[::darling::export::NestedMeta]) -> ::darling::Result<Self> {
        let mut request: (bool, ::darling::export::Option<Option<Type>>) = (false, None);
        let mut response: (bool, ::darling::export::Option<Option<Type>>) = (
            false,
            None,
        );
        let mut error: (bool, ::darling::export::Option<Option<Type>>) = (false, None);
        let mut my_service: (bool, ::darling::export::Option<bool>) = (false, None);
        let mut other_service: (bool, ::darling::export::Option<bool>) = (false, None);
        let mut peer: (bool, ::darling::export::Option<bool>) = (false, None);
        let mut __errors = ::darling::Error::accumulator();
        for __item in __items {
            match *__item {
                ::darling::export::NestedMeta::Meta(ref __inner) => {
                    let __name = ::darling::util::path_to_string(__inner.path());
                    match __name.as_str() {
                        "request" => {
                            if !request.0 {
                                request = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("request")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("request")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        "response" => {
                            if !response.0 {
                                response = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("response")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("response")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        "error" => {
                            if !error.0 {
                                error = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("error")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("error")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        "my_service" => {
                            if !my_service.0 {
                                my_service = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("my_service")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("my_service")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        "other_service" => {
                            if !other_service.0 {
                                other_service = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("other_service")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("other_service")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        "peer" => {
                            if !peer.0 {
                                peer = (
                                    true,
                                    __errors
                                        .handle(
                                            ::darling::FromMeta::from_meta(__inner)
                                                .map_err(|e| e.with_span(&__inner).at("peer")),
                                        ),
                                );
                            } else {
                                __errors
                                    .push(
                                        ::darling::Error::duplicate_field("peer")
                                            .with_span(&__inner),
                                    );
                            }
                        }
                        __other => {
                            __errors
                                .push(
                                    ::darling::Error::unknown_field_with_alts(
                                            __other,
                                            &[
                                                "request",
                                                "response",
                                                "error",
                                                "my_service",
                                                "other_service",
                                                "peer",
                                            ],
                                        )
                                        .with_span(__inner),
                                );
                        }
                    }
                }
                ::darling::export::NestedMeta::Lit(ref __inner) => {
                    __errors
                        .push(
                            ::darling::Error::unsupported_format("literal")
                                .with_span(__inner),
                        );
                }
            }
        }
        __errors.finish()?;
        let __default: Self = ::darling::export::Default::default();
        ::darling::export::Ok(Self {
            request: if let Some(__val) = request.1 { __val } else { __default.request },
            response: if let Some(__val) = response.1 {
                __val
            } else {
                __default.response
            },
            error: if let Some(__val) = error.1 { __val } else { __default.error },
            my_service: if let Some(__val) = my_service.1 {
                __val
            } else {
                __default.my_service
            },
            other_service: if let Some(__val) = other_service.1 {
                __val
            } else {
                __default.other_service
            },
            peer: if let Some(__val) = peer.1 { __val } else { __default.peer },
        })
    }
}
#[proc_macro_attribute]
pub fn service_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = darling::ast::NestedMeta::parse_meta_list(attr.into()).unwrap();
    let options = Options::from_list(&attr_args).unwrap();
    let input = match ::syn::parse::<Item>(item) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let (require_impl, type_ident, signatures) = match &input {
        Item::Impl(input) => {
            let type_ident = if let Type::Path(type_path) = &*input.self_ty {
                type_path.path.segments.last().unwrap().ident.clone()
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("unsupported type for impl block"),
                    );
                };
            };
            let signatures: Vec<_> = input
                .items
                .iter()
                .filter_map(|item| match item {
                    syn::ImplItem::Fn(function) => Some(function.sig.clone()),
                    _ => None,
                })
                .collect();
            (false, type_ident, signatures)
        }
        Item::Trait(input) => {
            let type_ident = input.ident.clone();
            let signatures: Vec<_> = input
                .items
                .iter()
                .filter_map(|item| match item {
                    syn::TraitItem::Fn(function) => Some(function.sig.clone()),
                    _ => None,
                })
                .collect();
            (true, type_ident, signatures)
        }
        _ => {
            ::core::panicking::panic_fmt(format_args!("unsupported item"));
        }
    };
    let mut request_variants = Vec::new();
    let mut response_variants = Vec::new();
    let mut error_variants = Vec::new();
    let mut my_rpcs = Vec::new();
    let mut peer_rpcs = Vec::new();
    for signature in &signatures {
        let function_name = &signature.ident;
        let variant_name = match ::quote::__private::IdentFragmentAdapter(
            &function_name.to_string().to_case(Case::UpperCamel),
        ) {
            arg => {
                ::quote::__private::mk_ident(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", arg));
                        res
                    }),
                    ::quote::__private::Option::None.or(arg.span()),
                )
            }
        };
        let input_types: Vec<Box<Type>> = signature
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
            .map(|(i, _)| match ::quote::__private::IdentFragmentAdapter(&i) {
                arg => {
                    ::quote::__private::mk_ident(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("param{0}", arg),
                            );
                            res
                        }),
                        ::quote::__private::Option::None.or(arg.span()),
                    )
                }
            })
            .collect();
        let (ok_output_type, err_output_type) = match &signature.output {
            syn::ReturnType::Type(_, return_type) => {
                match helper::extract_result_types(return_type) {
                    helper::ResultTypes::ResultType(ok_type, err_type) => {
                        (ok_type, err_type)
                    }
                    helper::ResultTypes::NonResultType(_) => {
                        ::core::panicking::panic_fmt(
                            format_args!("unsupported function return type"),
                        );
                    }
                }
            }
            syn::ReturnType::Default => {
                ::core::panicking::panic_fmt(
                    format_args!("unsupported function return type"),
                );
            }
        };
        request_variants
            .push({
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut input_types, i) = input_types.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let input_types = match input_types.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&input_types, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            });
        response_variants
            .push({
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&ok_output_type, &mut _s);
                        _s
                    },
                );
                _s
            });
        error_variants
            .push({
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&err_output_type, &mut _s);
                        _s
                    },
                );
                _s
            });
        if options.my_service {
            my_rpcs
                .push({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Request");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            {
                                use ::quote::__private::ext::*;
                                let mut _i = 0usize;
                                let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                #[allow(unused_mut)]
                                let (mut input_idents, i) = input_idents.quote_into_iter();
                                let has_iter = has_iter | i;
                                let _: ::quote::__private::HasIterator = has_iter;
                                while true {
                                    let input_idents = match input_idents.next() {
                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                        None => break,
                                    };
                                    if _i > 0 {
                                        ::quote::__private::push_comma(&mut _s);
                                    }
                                    _i += 1;
                                    ::quote::ToTokens::to_tokens(&input_idents, &mut _s);
                                }
                            }
                            _s
                        },
                    );
                    ::quote::__private::push_fat_arrow(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "match");
                            ::quote::__private::push_ident(&mut _s, "self");
                            ::quote::__private::push_dot(&mut _s);
                            ::quote::ToTokens::to_tokens(&function_name, &mut _s);
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    {
                                        use ::quote::__private::ext::*;
                                        let mut _i = 0usize;
                                        let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                        #[allow(unused_mut)]
                                        let (mut input_idents, i) = input_idents.quote_into_iter();
                                        let has_iter = has_iter | i;
                                        let _: ::quote::__private::HasIterator = has_iter;
                                        while true {
                                            let input_idents = match input_idents.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::quote::ToTokens::to_tokens(&input_idents, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "ok");
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "Self");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "Response");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "ok");
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "err");
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "Self");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "Error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "err");
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    _s
                });
        }
        if options.peer {
            peer_rpcs
                .push({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "pub");
                    ::quote::__private::push_ident(&mut _s, "async");
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ::quote::ToTokens::to_tokens(&function_name, &mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "self");
                            ::quote::__private::push_comma(&mut _s);
                            {
                                use ::quote::__private::ext::*;
                                let mut _i = 0usize;
                                let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                #[allow(unused_mut)]
                                let (mut input_idents, i) = input_idents.quote_into_iter();
                                let has_iter = has_iter | i;
                                #[allow(unused_mut)]
                                let (mut input_types, i) = input_types.quote_into_iter();
                                let has_iter = has_iter | i;
                                let _: ::quote::__private::HasIterator = has_iter;
                                while true {
                                    let input_idents = match input_idents.next() {
                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                        None => break,
                                    };
                                    let input_types = match input_types.next() {
                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                        None => break,
                                    };
                                    if _i > 0 {
                                        ::quote::__private::push_comma(&mut _s);
                                    }
                                    _i += 1;
                                    ::quote::ToTokens::to_tokens(&input_idents, &mut _s);
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&input_types, &mut _s);
                                }
                            }
                            _s
                        },
                    );
                    ::quote::__private::push_rarrow(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "error");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "peer");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "ResultRpc");
                    ::quote::__private::push_lt(&mut _s);
                    ::quote::ToTokens::to_tokens(&ok_output_type, &mut _s);
                    ::quote::__private::push_comma(&mut _s);
                    ::quote::ToTokens::to_tokens(&err_output_type, &mut _s);
                    ::quote::__private::push_gt(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "match");
                            ::quote::__private::push_ident(&mut _s, "self");
                            ::quote::__private::push_dot(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "peer");
                            ::quote::__private::push_dot(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "rpc");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "Self");
                                    ::quote::__private::push_colon2(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Request");
                                    ::quote::__private::push_colon2(&mut _s);
                                    ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            {
                                                use ::quote::__private::ext::*;
                                                let mut _i = 0usize;
                                                let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                                #[allow(unused_mut)]
                                                let (mut input_idents, i) = input_idents.quote_into_iter();
                                                let has_iter = has_iter | i;
                                                let _: ::quote::__private::HasIterator = has_iter;
                                                while true {
                                                    let input_idents = match input_idents.next() {
                                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                                        None => break,
                                                    };
                                                    if _i > 0 {
                                                        ::quote::__private::push_comma(&mut _s);
                                                    }
                                                    _i += 1;
                                                    ::quote::ToTokens::to_tokens(&input_idents, &mut _s);
                                                }
                                            }
                                            _s
                                        },
                                    );
                                    _s
                                },
                            );
                            ::quote::__private::push_dot(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "await");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "Self");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "Response");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "response");
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "response");
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_underscore(&mut _s);
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "peer");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "ErrorRpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "InternalError");
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Brace,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "message");
                                                    ::quote::__private::push_colon(&mut _s);
                                                    ::quote::__private::parse(
                                                        &mut _s,
                                                        "\"unexpected response type\"",
                                                    );
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "peer");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "ErrorRpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "RpcError");
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "Self");
                                                    ::quote::__private::push_colon2(&mut _s);
                                                    ::quote::__private::push_ident(&mut _s, "Error");
                                                    ::quote::__private::push_colon2(&mut _s);
                                                    ::quote::ToTokens::to_tokens(&variant_name, &mut _s);
                                                    ::quote::__private::push_group(
                                                        &mut _s,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            ::quote::__private::push_ident(&mut _s, "err");
                                                            _s
                                                        },
                                                    );
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "peer");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "ErrorRpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "RpcError");
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "err");
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "peer");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "ErrorRpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "RpcError");
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_underscore(&mut _s);
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "error");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "peer");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "ErrorRpc");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "InternalError");
                                            ::quote::__private::push_group(
                                                &mut _s,
                                                ::quote::__private::Delimiter::Brace,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::__private::push_ident(&mut _s, "message");
                                                    ::quote::__private::push_colon(&mut _s);
                                                    ::quote::__private::parse(
                                                        &mut _s,
                                                        "\"unexpected error type\"",
                                                    );
                                                    _s
                                                },
                                            );
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "err");
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_fat_arrow(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "Err");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "err");
                                            _s
                                        },
                                    );
                                    ::quote::__private::push_comma(&mut _s);
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    _s
                });
        }
    }
    let (request_type, request_enum) = if let Some(request_type) = options.request {
        (request_type, ::quote::__private::TokenStream::new())
    } else {
        let enum_ident = match ::quote::__private::IdentFragmentAdapter(&type_ident) {
            arg => {
                ::quote::__private::mk_ident(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}Request", arg));
                        res
                    }),
                    ::quote::__private::Option::None.or(arg.span()),
                )
            }
        };
        (
            helper::ident_to_type(&enum_ident),
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "Debug");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Serialize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Deserialize");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::ToTokens::to_tokens(&enum_ident, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut request_variants, i) = request_variants
                                .quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let request_variants = match request_variants.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&request_variants, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            },
        )
    };
    let (response_type, response_enum) = if let Some(response_type) = options.response {
        (response_type, ::quote::__private::TokenStream::new())
    } else {
        let enum_ident = match ::quote::__private::IdentFragmentAdapter(&type_ident) {
            arg => {
                ::quote::__private::mk_ident(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}Response", arg));
                        res
                    }),
                    ::quote::__private::Option::None.or(arg.span()),
                )
            }
        };
        (
            helper::ident_to_type(&enum_ident),
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "Debug");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Serialize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Deserialize");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::ToTokens::to_tokens(&enum_ident, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut response_variants, i) = response_variants
                                .quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let response_variants = match response_variants.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&response_variants, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            },
        )
    };
    let (error_type, error_enum) = if let Some(error_type) = options.error {
        (error_type, ::quote::__private::TokenStream::new())
    } else {
        let enum_ident = match ::quote::__private::IdentFragmentAdapter(&type_ident) {
            arg => {
                ::quote::__private::mk_ident(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}Error", arg));
                        res
                    }),
                    ::quote::__private::Option::None.or(arg.span()),
                )
            }
        };
        (
            helper::ident_to_type(&enum_ident),
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_pound(&mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "derive");
                        ::quote::__private::push_group(
                            &mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "Debug");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Serialize");
                                ::quote::__private::push_comma(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "serde");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Deserialize");
                                _s
                            },
                        );
                        _s
                    },
                );
                ::quote::__private::push_ident(&mut _s, "pub");
                ::quote::__private::push_ident(&mut _s, "enum");
                ::quote::ToTokens::to_tokens(&enum_ident, &mut _s);
                ::quote::__private::push_group(
                    &mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let mut _i = 0usize;
                            let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut error_variants, i) = error_variants
                                .quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let error_variants = match error_variants.next() {
                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                    None => break,
                                };
                                if _i > 0 {
                                    ::quote::__private::push_comma(&mut _s);
                                }
                                _i += 1;
                                ::quote::ToTokens::to_tokens(&error_variants, &mut _s);
                            }
                        }
                        _s
                    },
                );
                _s
            },
        )
    };
    let my_service = if options.my_service {
        let mut type_ident = type_ident.clone();
        let service_impl = if require_impl {
            type_ident = match ::quote::__private::IdentFragmentAdapter(&type_ident) {
                arg => {
                    ::quote::__private::mk_ident(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Other{0}", arg),
                            );
                            res
                        }),
                        ::quote::__private::Option::None.or(arg.span()),
                    )
                }
            };
            Some({
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "struct");
                ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
                ::quote::__private::push_semi(&mut _s);
                _s
            })
        } else {
            None
        };
        Some({
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::ToTokens::to_tokens(&service_impl, &mut _s);
            ::quote::__private::push_ident(&mut _s, "impl");
            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
            ::quote::__private::push_colon2(&mut _s);
            ::quote::__private::push_ident(&mut _s, "MyService");
            ::quote::__private::push_ident(&mut _s, "for");
            ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Request");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&request_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Response");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&response_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Error");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&error_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "async");
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ::quote::__private::push_ident(&mut _s, "on_rpc");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_and(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "self");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "tag");
                            ::quote::__private::push_colon(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Tag");
                            ::quote::__private::push_comma(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "request");
                            ::quote::__private::push_colon(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Request");
                            _s
                        },
                    );
                    ::quote::__private::push_rarrow(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Result");
                    ::quote::__private::push_lt(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Response");
                    ::quote::__private::push_comma(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Error");
                    ::quote::__private::push_gt(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "match");
                            ::quote::__private::push_ident(&mut _s, "request");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    {
                                        use ::quote::__private::ext::*;
                                        let mut _i = 0usize;
                                        let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                                        #[allow(unused_mut)]
                                        let (mut my_rpcs, i) = my_rpcs.quote_into_iter();
                                        let has_iter = has_iter | i;
                                        let _: ::quote::__private::HasIterator = has_iter;
                                        while true {
                                            let my_rpcs = match my_rpcs.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::quote::ToTokens::to_tokens(&my_rpcs, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    _s
                },
            );
            _s
        })
    } else {
        None
    };
    let other_service = if options.other_service {
        let mut type_ident = type_ident.clone();
        let service_impl = if require_impl {
            type_ident = match ::quote::__private::IdentFragmentAdapter(&type_ident) {
                arg => {
                    ::quote::__private::mk_ident(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Other{0}", arg),
                            );
                            res
                        }),
                        ::quote::__private::Option::None.or(arg.span()),
                    )
                }
            };
            Some({
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "struct");
                ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
                ::quote::__private::push_semi(&mut _s);
                _s
            })
        } else {
            None
        };
        Some({
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::ToTokens::to_tokens(&service_impl, &mut _s);
            ::quote::__private::push_ident(&mut _s, "impl");
            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
            ::quote::__private::push_colon2(&mut _s);
            ::quote::__private::push_ident(&mut _s, "OtherService");
            ::quote::__private::push_ident(&mut _s, "for");
            ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Request");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&request_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Response");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&response_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "type");
                    ::quote::__private::push_ident(&mut _s, "Error");
                    ::quote::__private::push_eq(&mut _s);
                    ::quote::ToTokens::to_tokens(&error_type, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    _s
                },
            );
            _s
        })
    } else {
        None
    };
    let peer = if options.peer {
        Some({
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_ident(&mut _s, "pub");
            ::quote::__private::push_ident(&mut _s, "struct");
            ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "peer");
                    ::quote::__private::push_colon(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Peer");
                    ::quote::__private::push_comma(&mut _s);
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "impl");
            ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    {
                        use ::quote::__private::ext::*;
                        let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                        #[allow(unused_mut)]
                        let (mut peer_rpcs, i) = peer_rpcs.quote_into_iter();
                        let has_iter = has_iter | i;
                        let _: ::quote::__private::HasIterator = has_iter;
                        while true {
                            let peer_rpcs = match peer_rpcs.next() {
                                Some(_x) => ::quote::__private::RepInterp(_x),
                                None => break,
                            };
                            ::quote::ToTokens::to_tokens(&peer_rpcs, &mut _s);
                        }
                    }
                    _s
                },
            );
            ::quote::__private::push_ident(&mut _s, "impl");
            ::quote::__private::push_ident(&mut _s, "From");
            ::quote::__private::push_lt(&mut _s);
            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
            ::quote::__private::push_colon2(&mut _s);
            ::quote::__private::push_ident(&mut _s, "Peer");
            ::quote::__private::push_gt(&mut _s);
            ::quote::__private::push_ident(&mut _s, "for");
            ::quote::ToTokens::to_tokens(&type_ident, &mut _s);
            ::quote::__private::push_group(
                &mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ::quote::__private::push_ident(&mut _s, "from");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "peer");
                            ::quote::__private::push_colon(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "foliage_rpc");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "Peer");
                            _s
                        },
                    );
                    ::quote::__private::push_rarrow(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "peer");
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    _s
                },
            );
            _s
        })
    } else {
        None
    };
    TokenStream::from({
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::ToTokens::to_tokens(&request_enum, &mut _s);
        ::quote::ToTokens::to_tokens(&response_enum, &mut _s);
        ::quote::ToTokens::to_tokens(&error_enum, &mut _s);
        ::quote::ToTokens::to_tokens(&my_service, &mut _s);
        ::quote::ToTokens::to_tokens(&other_service, &mut _s);
        ::quote::ToTokens::to_tokens(&peer, &mut _s);
        ::quote::ToTokens::to_tokens(&input, &mut _s);
        _s
    })
}
const _: () = {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    #[used]
    #[allow(deprecated)]
    static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::attr("service_impl", service_impl),
    ];
};
