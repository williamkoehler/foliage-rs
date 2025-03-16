use syn::{GenericArgument, Ident, Path, PathSegment, Type, TypePath};

pub enum ResultTypes {
    ResultType(Box<Type>, Box<Type>),
    NonResultType(Box<Type>),
}

pub fn extract_result_types(ty: &Type) -> ResultTypes {
    if let Type::Path(return_type_path) = ty {
        let last_segement = return_type_path.path.segments.last().unwrap();
        if last_segement.ident == "Result" {
            if let syn::PathArguments::AngleBracketed(args) = &last_segement.arguments {
                // Extract the Ok types
                let ok_argument = args.args.first().unwrap();
                let ok_ty = if let GenericArgument::Type(ok_ty) = ok_argument {
                    ok_ty
                } else {
                    panic!("Expected type ok argument")
                };

                // Extract the Err types
                let err_argument = args.args.last().unwrap();
                let err_ty = if let GenericArgument::Type(err_ty) = err_argument {
                    err_ty
                } else {
                    panic!("Expected type err argument")
                };

                return ResultTypes::ResultType(Box::new(ok_ty.clone()), Box::new(err_ty.clone()));
            }
        }
    }

    ResultTypes::NonResultType(Box::new(ty.clone()))
}

pub fn ident_to_type(ident: &Ident) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None, // No global path (::MyStruct)
            segments: vec![PathSegment {
                ident: ident.clone(),
                arguments: syn::PathArguments::None, // No generic arguments
            }]
            .into_iter()
            .collect(),
        },
    })
}
