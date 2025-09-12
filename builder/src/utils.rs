use quote::format_ident;

pub fn unwrap_option(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path { segments, .. },
    }) = ty
    else {
        return None;
    };
    let Some(segment) = segments.first() else {
        return None;
    };
    if segment.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) =
        &segment.arguments
    else {
        return None;
    };
    let Some(syn::GenericArgument::Type(gty)) = args.first() else {
        return None;
    };
    Some(gty.to_owned())
}

pub fn unwrap_vector(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path { segments, .. },
    }) = ty
    else {
        return None;
    };
    let Some(segment) = segments.first() else {
        return None;
    };
    if segment.ident != "Vec" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) =
        &segment.arguments
    else {
        return None;
    };
    let Some(syn::GenericArgument::Type(gty)) = args.first() else {
        return None;
    };
    Some(gty.to_owned())
}

pub fn unwrap_vec_key(attrs: &Vec<syn::Attribute>) -> Option<syn::Ident> {
    let mut each_key: Option<syn::Ident> = None;

    for attr in attrs {
        if !attr.path().is_ident("builder") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("each") {
                return Err(meta.error("unmatched meta ident"));
            }
            let key = meta.value()?.parse::<syn::LitStr>()?;
            each_key = Some(format_ident!("{}", key.value()));
            Ok(())
        });
    }

    each_key
}
