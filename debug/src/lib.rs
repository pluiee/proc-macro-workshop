use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = data else {
        unimplemented!()
    };

    let syn::Fields::Named(syn::FieldsNamed { named: fields, .. }) = fields else {
        unimplemented!()
    };

    let field_impl = fields.iter().map(|field| field.ident.to_owned());

    quote! {
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                f.debug_struct(stringify!(#ident))
                #(.field(stringify!(#field_impl), &self.#field_impl))* 
                .finish()
            }
        }
    }
    .into()
}
