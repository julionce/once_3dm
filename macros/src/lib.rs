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
    underlying_type: Option<syn::Type>,
}

impl FieldAttrs {
    fn parse(field: &syn::Field) -> Self {
        Self {
            typecode: Self::parse_typecode(&field.attrs),
            padding: Self::parse_padding(&field.attrs),
            underlying_type: Self::parse_underlying_type(&field.attrs),
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

    fn parse_underlying_type(attrs: &Vec<syn::Attribute>) -> Option<syn::Type> {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("underlying_type"))
        {
            Some(attr) => Some(attr.parse_args::<syn::Type>().unwrap()),
            None => None,
        }
    }
}

#[proc_macro_derive(
    Deserialize,
    attributes(
        table,
        field,
        chunk_version,
        from_chunk_version,
        padding,
        underlying_type
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
            type Error = ErrorStack;

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
    let chunk_trait_bounds = generate_chunk_trait_bounds_deserialize(struct_attrs);
    let type_trait_bounds = generate_type_trait_bounds_deserialize(&data.fields);
    quote! {
        impl<V> Deserialize<V> for #ident
        where
            V: FileVersion,
            #chunk_trait_bounds
            #(#type_trait_bounds)*
    }
}

fn generate_chunk_trait_bounds_deserialize(struct_attrs: &StructAttrs) -> TokenStream2 {
    let chunk_begin_trait_bounds = match struct_attrs.table.0 {
        true => quote! {
            chunk::Begin: Deserialize<V>,
            ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
        },
        false => quote!(),
    };
    let chunk_version_trait_bounds = match struct_attrs.chunk_version {
        ChunkVersion::Big => quote! {
            chunk::BigVersion: Deserialize<V>,
            ErrorStack: From<<chunk::BigVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::Short => quote! {
            chunk::ShortVersion: Deserialize<V>,
            ErrorStack: From<<chunk::ShortVersion as Deserialize<V>>::Error>,
        },
        ChunkVersion::None => quote! {},
    };
    quote! {
        #chunk_version_trait_bounds
        #chunk_begin_trait_bounds
    }
}

fn generate_type_trait_bounds_deserialize(fields: &syn::Fields) -> Vec<TokenStream2> {
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
                    ErrorStack: From<<#ty as Deserialize<V>>::Error>,
                }
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_body_deserialize(data: &syn::DataStruct, struct_attrs: &StructAttrs) -> TokenStream2 {
    let version_deserialize = generate_version_deserialize(struct_attrs);
    let body_core = generate_body_core_deserialize(data, struct_attrs);
    match &struct_attrs.from_chunk_version {
        Some(version) => {
            let major_version = version.major;
            let minor_version = version.minor;
            quote! {
                #version_deserialize
                if version.major() >= #major_version && version.minor() >= #minor_version {
                    #body_core
                } else {
                    Ok(Self::default())
                }
            }
        }
        None => quote! {
            #version_deserialize
            #body_core
        },
    }
}

fn generate_body_core_deserialize(
    data: &syn::DataStruct,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    let field_deserializes = generate_field_deserializes(&data.fields, struct_attrs);
    match struct_attrs.table.0 {
        true => {
            quote! {
                let mut table = Self::default();
                loop {
                    let begin = match <chunk::Begin as Deserialize<V>>::deserialize(ostream) {
                        Ok(ok) => ok,
                        Err(e) => {
                            let mut stack: ErrorStack = From::from(e);
                            stack.push_frame("begin", "chunk::Begin");
                            return Err(stack);
                        }
                    };
                    let mut chunk = ostream.ochunk(Some(begin.length));
                    match begin.typecode {
                        #(#field_deserializes)*
                        typecode::ENDOFTABLE | typecode::ENDOFFILE => {
                            //TODO: remove unwrap
                            chunk.seek(SeekFrom::End(0)).unwrap();
                            break;
                        }
                        _ => {
                            //TODO: remove unwrap
                            chunk.seek(SeekFrom::End(0)).unwrap();
                        }
                    }
                }
                Ok(table)
            }
        }
        false => {
            quote! {
                Ok(Self {#(#field_deserializes),*})
            }
        }
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

fn generate_padding_deserialize(
    field_attrs: &FieldAttrs,
    struct_attrs: &StructAttrs,
) -> TokenStream2 {
    match &field_attrs.padding.as_ref() {
        Some(ty) => match struct_attrs.table.0 {
            true => quote!(<#ty as Deserialize<V>>::deserialize(&mut chunk)?;),
            false => quote!(<#ty as Deserialize<V>>::deserialize(ostream)?;),
        },
        None => quote!(),
    }
}

fn generate_field_deserializes(
    fields: &syn::Fields,
    struct_attrs: &StructAttrs,
) -> Vec<TokenStream2> {
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
                let ident_str = ident.to_string();
                let ty_str = ty.to_string();
                let attrs = FieldAttrs::parse(raw_field);
                let padding_deserialize = generate_padding_deserialize(&attrs, struct_attrs);
                match struct_attrs.table.0 {
                    true => {
                        let deserialize = match attrs.underlying_type {
                            Some(underlying_ty) => quote! {
                                match <#underlying_ty as Deserialize<V>>::deserialize(&mut chunk) {
                                    Ok(ok) => ok.into(),
                                    Err(mut e) => {
                                        let mut stack: ErrorStack = From::from(e);
                                        stack.push_frame(#ident_str, #ty_str);
                                        return Err(stack);
                                    }
                                }
                            },
                            None => quote! {
                                match <#ty as Deserialize<V>>::deserialize(&mut chunk) {
                                    Ok(ok) => ok,
                                    Err(mut e) => {
                                        let mut stack: ErrorStack = From::from(e);
                                        stack.push_frame(#ident_str, #ty_str);
                                        return Err(stack);
                                    }
                                }
                            },
                        };
                        let typecode = attrs.typecode.as_ref().unwrap();
                        quote!(
                            typecode::#typecode => {
                                #padding_deserialize
                                table.#ident = #deserialize;
                                match chunk.seek(SeekFrom::End(0)) {
                                    Ok(v) => {
                                        if v != begin.length {
                                            let mut stack = ErrorStack::new(Error::Simple(ErrorKind::InvalidChunkSize));
                                            stack.push_frame(#ident_str, #ty_str);
                                            return Err(stack);
                                        }
                                    },
                                    Err(e) => {
                                        let mut stack = ErrorStack::new(Error::IoError(e));
                                        stack.push_frame(#ident_str, #ty_str);
                                        return Err(stack);
                                    }
                                };
                            }
                        )
                    }
                    false => {
                        let deserialize = match attrs.underlying_type {
                            Some(underlying_ty) => quote! {
                                match <#underlying_ty as Deserialize<V>>::deserialize(ostream) {
                                    Ok(ok) => ok.into(),
                                    Err(e) => {
                                        let mut stack: ErrorStack = From::from(e);
                                        stack.push_frame(#ident_str, #ty_str);
                                        return Err(stack);
                                    }
                                }
                            },
                            None => quote! {
                                match <#ty as Deserialize<V>>::deserialize(ostream) {
                                    Ok(ok) => ok,
                                    Err(e) => {
                                        let mut stack: ErrorStack = From::from(e);
                                        stack.push_frame(#ident_str, #ty_str);
                                        return Err(stack);
                                    }
                                }
                            },
                        };
                        quote!(#ident: { #padding_deserialize #deserialize })
                    }
                }
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}
