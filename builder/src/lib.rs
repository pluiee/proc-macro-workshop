mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = data else {
        unimplemented!()
    };

    let syn::Fields::Named(syn::FieldsNamed { named, .. }) = fields else {
        unimplemented!()
    };

    let builder_ident = format_ident!("{ident}Builder");

    let option_fields = named.iter().map(|field| {
        let fident = &field.ident;
        let fty = &field.ty;
        if utils::unwrap_option(fty).is_some() {
            return quote! { #fident: #fty };
        }
        quote! { #fident: Option<#fty> }
    });

    let builder_fn_impl = named.iter().map(|field| {
        let fident = field.ident.to_owned();
        let fty = field.ty.to_owned();

        if let Some(ufty) = utils::unwrap_option(&fty) {
            return quote! {
                fn #fident(&mut self, #fident: #ufty) -> &mut Self {
                    self.#fident = Some(#fident);
                    self
                }
            };
        }

        quote! {
            fn #fident(&mut self, #fident: #fty) -> &mut Self {
                self.#fident = Some(#fident);
                self
            }
        }
    });

    let unwrap_impl = named.iter().map(|field| {
        let fty = &field.ty;
        let fident = field.ident.to_owned();

        if utils::unwrap_option(fty).is_some() {
            return quote! {
                #fident: self.#fident.to_owned()
            }
        }

        quote! {
            #fident: self.#fident.take().ok_or_else(|| format!("missing field: {}", stringify!(#fident)))?
        }
    });

    let tokens = quote! {
        #[derive(Default)]
        pub struct #builder_ident {
            #(#option_fields),*
        }

        impl #builder_ident{
            #(#builder_fn_impl)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    ..Default::default()
                }
            }
        }

        impl #builder_ident {
            pub fn build(&mut self) -> Result<#ident, String> {
                Ok(#ident {
                    #(#unwrap_impl),*
                })
            }
        }
    };

    TokenStream::from(tokens)
}
