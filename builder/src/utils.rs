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
