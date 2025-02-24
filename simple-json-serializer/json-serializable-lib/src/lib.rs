use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};


#[proc_macro_derive(JsonSerializable)]
pub fn auto_impl_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let json_fields = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let field_mappings = fields_named.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_str = field_name.to_string();
                    quote! {
                        format!("\"{}\": {}", #field_str, self.#field_name.to_json())
                    }
                });
                quote! { format!("{{{}}}", vec![#(#field_mappings),*].join(", ")) }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_mappings = fields_unnamed.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! { self.#index.to_json() }
                });
                quote! { format!("[{}]", vec![#(#field_mappings),*].join(", ")) }
            }
            Fields::Unit => quote! { "null".to_string() },
        },
        _ => quote! { "null".to_string() },
    };

    let expanded = quote! {
        impl JsonSerializable for #name {
            fn to_json(&self) -> String {
                #json_fields
            }
        }
    };

    TokenStream::from(expanded)
}