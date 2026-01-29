use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, GenericParam, Generics, Meta};

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

    let generics = add_debug_bounds(generics);
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

fn add_debug_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }
    generics
}
