use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct StructAttrs {
    table: Option<TableAttr>,
    chunk_version: ChunkVersion,
}

impl StructAttrs {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self {
            table: TableAttr::parse(attrs),
            chunk_version: ChunkVersion::parse(attrs),
        }
    }
}

enum ChunkVersion {
    Short,
    Big,
    None,
}

impl ChunkVersion {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("chunk_version"))
        {
            Some(attr) => match attr.parse_args::<syn::Path>() {
                Ok(expr) => {
                    if expr.is_ident("short") {
                        Self::Short
                    } else if expr.is_ident("big") {
                        Self::Big
                    } else if expr.is_ident("none") {
                        Self::None
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            },
            None => Self::None,
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

#[proc_macro_derive(Deserialize, attributes(table, field, chunk_version))]
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
        Some(_) => generate_table_deserialize(data, ident, &struct_attrs),
        None => generate_struct_deserialize(data, ident, &struct_attrs),
    }
}

fn generate_version_deserialize(struct_attrs: &StructAttrs) -> TokenStream2 {
    match struct_attrs.chunk_version {
        ChunkVersion::Big => {
            quote!(let _version = <chunk::BigVersion as Deserialize<V>>::deserialize(ostream)?;)
        }
        ChunkVersion::Short => {
            quote!(let _version = <chunk::ShortVersion as Deserialize<V>>::deserialize(ostream)?;)
        }
        ChunkVersion::None => quote!(),
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

fn generate_impl_deserialize_trait_bounds(fields: &syn::Fields) -> Vec<TokenStream2> {
    match fields {
        Fields::Named(raw_fields) => raw_fields
            .named
            .iter()
            .map(|raw_field| {
                let ty = match &raw_field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                quote! {
                    #ty: Deserialize<V>,
                    String: From<<#ty as Deserialize<V>>::Error>,
                }
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_impl_deserialize_chunk_trait_bounds(struct_attrs: &StructAttrs) -> TokenStream2 {
    match struct_attrs.chunk_version {
        ChunkVersion::Big => quote! {
            chunk::BigVersion: Deserialize<V>,
            String: From<<chunk::BigVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::Short => quote! {
            chunk::ShortVersion: Deserialize<V>,
            String: From<<chunk::ShortVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::None => quote!(),
    }
}

fn generate_impl_deserialize_header(
    data: &syn::DataStruct,
    ident: &syn::Ident,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let impl_deserialize_chunk_trait_bounds =
        generate_impl_deserialize_chunk_trait_bounds(struct_attrs);
    let impl_deserialize_trait_bounds = generate_impl_deserialize_trait_bounds(&data.fields);
    quote! {
        impl<V> Deserialize<V> for #ident
        where
            V: FileVersion,
            #impl_deserialize_chunk_trait_bounds
            #(#impl_deserialize_trait_bounds)*
    }
}

fn generate_struct_deserialize(
    data: &syn::DataStruct,
    ident: &syn::Ident,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let version_deserialize = generate_version_deserialize(struct_attrs);
    let field_deserializes = generate_field_deserializes(&data.fields);
    let impl_deserialize_header = generate_impl_deserialize_header(data, ident, struct_attrs);
    quote! {
        #impl_deserialize_header
        {
            type Error = String;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream
            {
                #version_deserialize
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
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let version_deserialize = generate_version_deserialize(struct_attrs);
    let body = match struct_attrs.table.as_ref().unwrap().typecode.as_ref() {
        Some(typecode) => generate_body_deserialize_for_table_with_typecode(data, typecode),
        None => generate_body_deserialize_for_table_without_typecode(data),
    };
    let impl_deserialize_header = generate_impl_deserialize_header(data, ident, struct_attrs);
    quote! {
        #impl_deserialize_header
            chunk::Begin: Deserialize<V>,
            String: From<<chunk::Begin as Deserialize<V>>::Error>,
        {
            type Error = String;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream
            {
                #version_deserialize
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
                    typecode::ENDOFTABLE => {
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
