use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Field, GenericArgument,
    GenericParam, Generics, Meta,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = data else {
        unimplemented!()
    };

    let syn::Fields::Named(syn::FieldsNamed { named: fields, .. }) = fields else {
        unimplemented!()
    };

    let generics = add_debug_bounds(generics, &fields);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_impl = fields.iter().map(|field| {
        let ident = field.ident.to_owned();
        let attrs = field.attrs.to_owned();
        let mut custom_format = None;
        for attr in attrs {
            if !attr.path().is_ident("debug") {
                continue;
            }
            if let Meta::NameValue(nv) = attr.meta {
                custom_format = Some(nv.value);
            }
        }
        let format_impl = match custom_format {
            None => quote!("{:?}"),
            Some(expr) => quote!(#expr),
        };
        quote! {
            .field(stringify!(#ident), &format_args!(#format_impl, self.#ident))
        }
    });

    quote! {
        impl #impl_generics std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                f.debug_struct(stringify!(#ident))
                #(#field_impl)*
                .finish()
            }
        }
    }
    .into()
}

fn add_debug_bounds(mut generics: Generics, fields: &Punctuated<Field, Comma>) -> Generics {
    let mut generic_map = HashMap::<syn::Ident, (bool, u8)>::new();

    fields.iter().for_each(|field| {
        if let syn::Type::Path(path) = &field.ty {
            let segment = path.path.segments.last().unwrap();
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                args.args.iter().for_each(|arg| {
                    if let GenericArgument::Type(ty) = arg {
                        let ident = match ty {
                            syn::Type::Path(path) => {
                                path.path.segments.first().unwrap().ident.to_owned()
                            }
                            _ => unimplemented!(),
                        };
                        let val = generic_map.get(&ident);
                        match val {
                            Some((in_phantom, count)) => {
                                let in_phantom =
                                    in_phantom.to_owned() || (segment.ident == "PhantomData");
                                generic_map.insert(ident, (in_phantom, count + 1));
                            }
                            None => {
                                generic_map.insert(ident, (segment.ident == "PhantomData", 1));
                            }
                        }
                    }
                })
            }
        }
    });

    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            let ident = type_param.ident.to_owned();
            if let Some((true, 1)) = generic_map.get(&ident) {
                continue;
            }
            type_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }

    generics
}
