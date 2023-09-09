use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(
    Deserialize,
    attributes(
        big_chunk_version,
        underlying_type,
        padding,
        table,
        table_field,
        normal_chunk
    )
)]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    }: DeriveInput = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Struct(data_struct) => process_data_struct(&data_struct, &ident, &attrs),
        _ => {
            quote!()
        }
    }
    .into()
}

fn process_data_struct(
    data: &syn::DataStruct,
    ident: &syn::Ident,
    _attrs: &Vec<syn::Attribute>,
) -> proc_macro2::TokenStream {
    match &data.fields {
        Fields::Named(raw_fields) => {
            let fields = raw_fields.named.iter().map(|raw_field| {
                let ident = raw_field.ident.as_ref().unwrap();
                let ty = match &raw_field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                let deserialize = quote!(<#ty as Deserialize<V>>::deserialize(ostream)?);
                quote!(#ident: { #deserialize })
            });
            quote! {
                impl<V> Deserialize<V> for #ident where V: FileVersion,
                {
                    type Error = String;

                    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
                    where
                        T: OStream
                    {
                        Ok(Self {#(#fields),*})
                    }
                }
            }
        }
        _ => {
            quote!()
        }
    }
}
