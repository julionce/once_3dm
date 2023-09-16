use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct StructAttrs {
    table: TableAttr,
    chunk_version: ChunkVersion,
    from_chunk_version: Option<FromChunkVersion>,
}

impl StructAttrs {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self {
            table: TableAttr::parse(attrs),
            chunk_version: ChunkVersion::parse(attrs),
            from_chunk_version: FromChunkVersion::parse(attrs),
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

struct FromChunkVersion {
    major: u8,
    minor: u8,
}

impl FromChunkVersion {
    fn parse(attrs: &Vec<syn::Attribute>) -> Option<Self> {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("from_chunk_version"))
        {
            Some(attr) => match attr.parse_args::<syn::ExprTuple>() {
                Ok(tuple) => {
                    if tuple.elems.len() == 2 {
                        let version = tuple
                            .elems
                            .iter()
                            .map(|expr| match expr {
                                syn::Expr::Lit(lit) => match &lit.lit {
                                    syn::Lit::Int(int) => int.base10_parse::<u8>().unwrap(),
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            })
                            .collect::<Vec<u8>>();
                        Some(Self {
                            major: version[0],
                            minor: version[1],
                        })
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            },
            None => None,
        }
    }
}

struct TableAttr(bool);

impl TableAttr {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self(
            attrs
                .iter()
                .find(|attr| attr.path.is_ident("table"))
                .is_some(),
        )
    }
}

struct FieldAttrs {
    typecode: Option<syn::Type>,
    padding: Option<syn::Type>,
}

impl FieldAttrs {
    fn parse(field: &syn::Field) -> Self {
        Self {
            typecode: Self::parse_typecode(&field.attrs),
            padding: Self::parse_padding(&field.attrs),
        }
    }

    fn parse_typecode(attrs: &Vec<syn::Attribute>) -> Option<syn::Type> {
        match attrs.iter().find(|attr| attr.path.is_ident("field")) {
            Some(attr) => Some(attr.parse_args::<syn::Type>().unwrap()),
            None => None,
        }
    }

    fn parse_padding(attrs: &Vec<syn::Attribute>) -> Option<syn::Type> {
        match attrs.iter().find(|attr| attr.path.is_ident("padding")) {
            Some(attr) => Some(attr.parse_args::<syn::Type>().unwrap()),
            None => None,
        }
    }
}

#[proc_macro_derive(
    Deserialize,
    attributes(table, field, chunk_version, from_chunk_version, padding)
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
    attrs: &Vec<syn::Attribute>,
) -> TokenStream2 {
    let struct_attrs = StructAttrs::parse(&attrs);
    generate_deserialize(data, ident, &struct_attrs)
}

fn generate_deserialize(
    data: &syn::DataStruct,
    ident: &syn::Ident,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let header = generate_header_deserialize(data, ident, struct_attrs);
    let body = generate_body_deserialize(data, struct_attrs);
    quote! {
        #header
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

fn generate_header_deserialize(
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

fn generate_body_deserialize(data: &syn::DataStruct, struct_attrs: &StructAttrs) -> TokenStream2 {
    match struct_attrs.table.0 {
        true => generate_table_body_deserialize(data, struct_attrs),
        false => generate_struct_body_deserialize(&data.fields, struct_attrs),
    }
}

fn generate_struct_body_deserialize(
    fields: &syn::Fields,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let version_deserialize = generate_version_deserialize(struct_attrs);
    let field_deserializes = generate_field_deserializes(&fields);
    match &struct_attrs.from_chunk_version {
        Some(version) => {
            let major_version = version.major;
            let minor_version = version.minor;
            quote! {
                #version_deserialize
                if version.major() >= #major_version && version.minor() >= #minor_version {
                    Ok(Self {#(#field_deserializes),*})
                } else {
                    Ok(Self::default())
                }
            }
        }
        None => quote! {
            #version_deserialize
            Ok(Self {#(#field_deserializes),*})
        },
    }
}

fn generate_table_body_deserialize(
    data: &syn::DataStruct,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let version_deserialize = generate_version_deserialize(struct_attrs);
    let table_body_loop = generate_table_body_loop(data);
    match &struct_attrs.from_chunk_version {
        Some(version) => {
            let major_version = version.major;
            let minor_version = version.minor;
            quote! {
                #version_deserialize
                if version.major() >= #major_version && version.minor() >= #minor_version {
                    let mut table = Self::default();
                    #table_body_loop
                    Ok(table)
                } else {
                    Ok(Self::default())
                }
            }
        }
        None => quote! {
            let mut table = Self::default();
            #version_deserialize
            #table_body_loop
            Ok(table)
        },
    }
}

fn generate_version_deserialize(struct_attrs: &StructAttrs) -> TokenStream2 {
    match struct_attrs.chunk_version {
        ChunkVersion::Big => {
            quote!(let version = <chunk::BigVersion as Deserialize<V>>::deserialize(ostream)?;)
        }
        ChunkVersion::Short => {
            quote!(let version = <chunk::ShortVersion as Deserialize<V>>::deserialize(ostream)?;)
        }
        ChunkVersion::None => quote!(),
    }
}

//TODO: take common factor between this function and generate_table_padding_deserialize
fn generate_struct_padding_deserialize(field_attrs: &FieldAttrs) -> TokenStream2 {
    match &field_attrs.padding.as_ref() {
        Some(ty) => quote!(<#ty as Deserialize<V>>::deserialize(ostream)?;),
        None => quote!(),
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
                let attrs = FieldAttrs::parse(raw_field);
                let padding_deserialize = generate_struct_padding_deserialize(&attrs);
                quote!(#ident: { #padding_deserialize #deserialize })
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
    let chunk_begin_trait_bounds = generate_chunk_begin_trait_bounds(struct_attrs);
    let chunk_version_trait_bounds = match struct_attrs.chunk_version {
        ChunkVersion::Big => quote! {
            chunk::BigVersion: Deserialize<V>,
            String: From<<chunk::BigVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::Short => quote! {
            chunk::ShortVersion: Deserialize<V>,
            String: From<<chunk::ShortVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::None => quote! {},
    };
    quote! {
        #chunk_version_trait_bounds
        #chunk_begin_trait_bounds
    }
}

fn generate_chunk_begin_trait_bounds(struct_attrs: &StructAttrs) -> TokenStream2 {
    match struct_attrs.table.0 {
        true => quote! {
            chunk::Begin: Deserialize<V>,
            String: From<<chunk::Begin as Deserialize<V>>::Error>,
        },
        false => quote!(),
    }
}

fn generate_table_padding_deserialize(field_attrs: &FieldAttrs) -> TokenStream2 {
    match &field_attrs.padding.as_ref() {
        Some(ty) => quote!(<#ty as Deserialize<V>>::deserialize(&mut chunk)?;),
        None => quote!(),
    }
}

fn generate_table_field_deserializes(fields: &syn::Fields) -> Vec<TokenStream2> {
    match fields {
        Fields::Named(raw_fields) => raw_fields
            .named
            .iter()
            .map(|raw_field| {
                let attrs = FieldAttrs::parse(raw_field);
                let typecode = attrs.typecode.as_ref().unwrap();
                let ident = raw_field.ident.as_ref().unwrap();
                let ty = match &raw_field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                let deserialize = quote!(<#ty as Deserialize<V>>::deserialize(&mut chunk)?);
                let padding_deserialize = generate_table_padding_deserialize(&attrs);
                quote!(
                    typecode::#typecode => {
                        #padding_deserialize
                        table.#ident = #deserialize;
                    }
                )
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_table_body_loop(data: &syn::DataStruct) -> TokenStream2 {
    let field_deserializes = generate_table_field_deserializes(&data.fields);
    quote! {
        loop {
            let begin = <chunk::Begin as Deserialize<V>>::deserialize(ostream)?;
            let mut chunk = ostream.ochunk(Some(begin.length));
            match begin.typecode {
                #(#field_deserializes)*
                typecode::ENDOFTABLE | typecode::ENDOFFILE => {
                    chunk.seek(SeekFrom::End(0)).unwrap();
                    break;
                }
                _ => {
                }
            }
            chunk.seek(SeekFrom::End(0)).unwrap();
        }
    }
}
