use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct StructAttrs {
    table: Option<TableAttr>,
}

impl StructAttrs {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self {
            table: TableAttr::parse(attrs),
        }
    }
}

struct TableAttr {
    typecode: Option<syn::Type>,
}

impl TableAttr {
    fn parse(attrs: &Vec<syn::Attribute>) -> Option<Self> {
        match attrs.iter().find(|attr| attr.path.is_ident("table")) {
            Some(attr) => {
                if attr.tokens.is_empty() {
                    Some(TableAttr { typecode: None })
                } else {
                    Some(TableAttr {
                        typecode: Some(attr.parse_args::<syn::Type>().unwrap()),
                    })
                }
            }
            None => None,
        }
    }
}

struct FieldAttrs {
    typecode: Option<syn::Type>,
}

impl FieldAttrs {
    fn parse(field: &syn::Field) -> Self {
        Self {
            typecode: Self::parse_typecode(&field.attrs),
        }
    }

    fn parse_typecode(attrs: &Vec<syn::Attribute>) -> Option<syn::Type> {
        match attrs.iter().find(|attr| attr.path.is_ident("field")) {
            Some(attr) => Some(attr.parse_args::<syn::Type>().unwrap()),
            None => None,
        }
    }
}

#[proc_macro_derive(Deserialize, attributes(table, field))]
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
    attrs: &Vec<syn::Attribute>,
) -> TokenStream2 {
    let struct_attrs = StructAttrs::parse(&attrs);
    match struct_attrs.table {
        Some(table_attr) => generate_table_deserialize(data, ident, &table_attr),
        None => generate_struct_deserialize(data, ident),
    }
}

fn generate_field_deserializes(fields: &syn::Fields) -> Vec<TokenStream2> {
    match fields {
        Fields::Named(raw_fields) => raw_fields
            .named
            .iter()
            .map(|raw_field| {
                let ident = raw_field.ident.as_ref().unwrap();
                let ty = match &raw_field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                let deserialize = quote!(<#ty as Deserialize<V>>::deserialize(ostream)?);
                quote!(#ident: { #deserialize })
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_struct_deserialize(data: &syn::DataStruct, ident: &syn::Ident) -> TokenStream2 {
    let field_deserializes = generate_field_deserializes(&data.fields);
    quote! {
        impl<V> Deserialize<V> for #ident
        where
            V: FileVersion,
        {
            type Error = String;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream
            {
                Ok(Self {#(#field_deserializes),*})
            }
        }
    }
}

fn generate_table_field_deserializes(fields: &syn::Fields) -> Vec<TokenStream2> {
    match fields {
        Fields::Named(raw_fields) => raw_fields
            .named
            .iter()
            .map(|raw_field| {
                let attrs = FieldAttrs::parse(raw_field);
                let typecode = attrs.typecode.unwrap();
                let ident = raw_field.ident.as_ref().unwrap();
                let ty = match &raw_field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                let deserialize = quote!(<#ty as Deserialize<V>>::deserialize(&mut chunk)?);
                quote!(
                    typecode::#typecode => {
                        table.#ident = #deserialize;
                    }
                )
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_table_deserialize(
    data: &syn::DataStruct,
    ident: &syn::Ident,
    table_attr: &TableAttr,
) -> TokenStream2 {
    let body = match table_attr.typecode.as_ref() {
        Some(typecode) => generate_body_deserialize_for_table_with_typecode(data, typecode),
        None => generate_body_deserialize_for_table_without_typecode(data),
    };
    quote! {
        impl<V> Deserialize<V> for #ident
        where
            V: FileVersion,
            chunk::Begin: Deserialize<V>,
            String: From<<chunk::Begin as Deserialize<V>>::Error>,
        {
            type Error = String;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream
            {
                #body
            }
        }
    }
}

fn generate_body_deserialize_for_table_without_typecode(data: &syn::DataStruct) -> TokenStream2 {
    let field_deserializes = generate_table_field_deserializes(&data.fields);
    quote! {
        let mut table = Self::default();
        loop {
            let begin = <chunk::Begin as Deserialize<V>>::deserialize(ostream)?;
            let mut chunk = ostream.ochunk(Some(begin.length));
            match begin.typecode {
                #(#field_deserializes)*
                _ => {
                    break;
                }
            }
            chunk.seek(SeekFrom::End(0)).unwrap();
        }
        Ok(table)
    }
}

fn generate_body_deserialize_for_table_with_typecode(
    data: &syn::DataStruct,
    typecode: &syn::Type,
) -> TokenStream2 {
    let field_deserializes = generate_table_field_deserializes(&data.fields);
    quote! {
        let mut table = Self::default();
        let begin = <chunk::Begin as Deserialize<V>>::deserialize(ostream)?;
        let mut properties_chunk = ostream.ochunk(Some(begin.length));
        if typecode::#typecode == begin.typecode {
            loop {
                let begin = <chunk::Begin as Deserialize<V>>::deserialize(&mut properties_chunk)?;
                let mut chunk = properties_chunk.ochunk(Some(begin.length));
                match begin.typecode {
                    #(#field_deserializes)*
                    typecode::ENDOFFILE => {
                        properties_chunk = chunk.into_inner();
                        break;
                    }
                    _ => {
                    }
                }
                chunk.seek(SeekFrom::End(0)).unwrap();
                properties_chunk = chunk.into_inner();
            }
        }
        properties_chunk.seek(SeekFrom::End(0)).unwrap();
        Ok(table)
    }
}
