use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = data else {
        unimplemented!()
    };

    let syn::Fields::Named(syn::FieldsNamed { named, .. }) = fields else {
        unimplemented!()
    };

    let builder_ident = format_ident!("{ident}Builder");

    let field_names = named.iter().map(|field| {
        let nident = field.ident.to_owned();
        quote! { #nident }
    });

    let option_fields = named.iter().map(|field| {
        let nident = field.ident.to_owned();
        let nty = field.ty.to_owned();
        quote! { #nident: Option<#nty> }
    });

    let builder_fn_impl = named.iter().map(|field| {
        let nident = field.ident.to_owned();
        let nty = field.ty.to_owned();
        quote! {
            fn #nident(&mut self, #nident: #nty) -> &mut Self {
                self.#nident = Some(#nident);
                self
            }
        }
    });

    let unwrap_impl = named.iter().map(|field| {
        let fident = field.ident.to_owned();
        quote! {
            // TODO: Error handling
            #fident: self.#fident.clone().unwrap()
        }
    });

    let tokens = quote! {
        pub struct #builder_ident {
            #(#option_fields),*
        }

        impl #builder_ident{
            #(#builder_fn_impl)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_names: None),*
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
