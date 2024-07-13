use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct StructAttrs {
    table: TableAttr,
    with_chunk_version: WithChunkVersion,
    if_version: IfVersion,
}

impl StructAttrs {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self {
            table: TableAttr::parse(attrs),
            with_chunk_version: WithChunkVersion::parse(attrs),
            if_version: IfVersion::parse(attrs),
        }
    }
}

struct EnumAttrs {
    with_chunk_version: WithChunkVersion,
}

impl EnumAttrs {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        Self {
            with_chunk_version: WithChunkVersion::parse(attrs),
        }
    }
}

enum WithChunkVersion {
    Short,
    Big,
    None,
}

impl WithChunkVersion {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("with_chunk_version"))
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

enum VersionCmp {
    Eq(u8),
    Ne(u8),
    Lt(u8),
    Le(u8),
    Gt(u8),
    Ge(u8),
}

impl VersionCmp {
    fn parse(attr: &syn::Attribute) -> Option<Self> {
        match attr.parse_args::<syn::ExprCall>() {
            Ok(call) => {
                let number = match call.args.len() == 1 {
                    true => match call.args.first().unwrap() {
                        syn::Expr::Lit(lit) => match &lit.lit {
                            syn::Lit::Int(int) => int.base10_parse::<u8>().unwrap(),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                    false => panic!(),
                };
                match call.func.as_ref() {
                    syn::Expr::Path(path) => {
                        match path.path.get_ident().unwrap().to_string().as_str() {
                            "Eq" => Some(VersionCmp::Eq(number)),
                            "Ne" => Some(VersionCmp::Ne(number)),
                            "Lt" => Some(VersionCmp::Lt(number)),
                            "Le" => Some(VersionCmp::Le(number)),
                            "Gt" => Some(VersionCmp::Gt(number)),
                            "Ge" => Some(VersionCmp::Ge(number)),
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            Err(_) => panic!(),
        }
    }
}

fn generate_version_cmp(cmp: &VersionCmp) -> TokenStream2 {
    use VersionCmp::*;
    match cmp {
        Eq(v) => quote!( == #v),
        Ne(v) => quote!( != #v),
        Lt(v) => quote!( <  #v),
        Le(v) => quote!( <= #v),
        Gt(v) => quote!( > #v),
        Ge(v) => quote!( >= #v),
    }
}

struct IfVersion {
    major: Option<VersionCmp>,
    minor: Option<VersionCmp>,
}

impl IfVersion {
    fn parse(attrs: &Vec<syn::Attribute>) -> Self {
        let major = match attrs
            .iter()
            .find(|attr| attr.path.is_ident("if_major_version"))
        {
            Some(attr) => VersionCmp::parse(attr),
            None => None,
        };
        let minor = match attrs
            .iter()
            .find(|attr| attr.path.is_ident("if_minor_version"))
        {
            Some(attr) => VersionCmp::parse(attr),
            None => None,
        };
        Self { major, minor }
    }
}

struct IfChunkVersion {
    major: u8,
    minor: u8,
}

impl IfChunkVersion {
    fn parse(attrs: &Vec<syn::Attribute>) -> Option<Self> {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("if_chunk_version"))
        {
            Some(attr) => match attr.parse_args::<syn::ExprTuple>() {
                Ok(tuple) => match tuple.elems.len() {
                    2 => {
                        let major = match tuple.elems.first().unwrap() {
                            syn::Expr::Lit(lit) => match &lit.lit {
                                syn::Lit::Int(int) => int.base10_parse::<u8>().unwrap(),
                                _ => panic!(),
                            },
                            _ => panic!(),
                        };
                        let minor = match tuple.elems.last().unwrap() {
                            syn::Expr::Lit(lit) => match &lit.lit {
                                syn::Lit::Int(int) => int.base10_parse::<u8>().unwrap(),
                                _ => panic!(),
                            },
                            _ => panic!(),
                        };
                        Some(IfChunkVersion { major, minor })
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => None,
        }
    }
}

fn generate_if_version_condition(if_version: &IfVersion) -> TokenStream2 {
    match (&if_version.major, &if_version.minor) {
        (None, None) => quote!(true),
        (Some(major), None) => {
            let major_cmp = generate_version_cmp(major);
            quote!( version.major() # major_cmp )
        }
        (None, Some(minor)) => {
            let minor_cmp = generate_version_cmp(minor);
            quote!( version.minor() # minor_cmp )
        }
        (Some(major), Some(minor)) => {
            let major_cmp = generate_version_cmp(major);
            let minor_cmp = generate_version_cmp(minor);
            quote!( version.major() #major_cmp && version.minor() #minor_cmp )
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
    type_code: Option<syn::Type>,
    padding: Option<syn::Type>,
    underlying_type: Option<syn::Type>,
    if_version: IfVersion,
}

impl FieldAttrs {
    fn parse(field: &syn::Field) -> Self {
        Self {
            type_code: Self::parse_type_code(&field.attrs),
            padding: Self::parse_padding(&field.attrs),
            underlying_type: Self::parse_underlying_type(&field.attrs),
            if_version: IfVersion::parse(&field.attrs),
        }
    }

    fn parse_type_code(attrs: &Vec<syn::Attribute>) -> Option<syn::Type> {
        match attrs
            .iter()
            .find(|attr| attr.path.is_ident("field") || attr.path.is_ident("in_chunk"))
        {
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
        in_chunk,
        with_chunk_version,
        if_major_version,
        if_minor_version,
        if_chunk_version,
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
        Data::Enum(data_enum) => process_data_enum(&data_enum, &ident, &attrs),
        _ => {
            panic!("data type not supported")
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

fn process_data_enum(
    data: &syn::DataEnum,
    ident: &syn::Ident,
    attrs: &Vec<syn::Attribute>,
) -> TokenStream2 {
    let enum_attrs = EnumAttrs::parse(&attrs);
    generate_enum_deserialize(data, ident, &enum_attrs)
}

fn generate_enum_deserialize(
    data: &syn::DataEnum,
    ident: &syn::Ident,
    enum_attrs: &EnumAttrs,
) -> TokenStream2 {
    let header = generate_enum_header_deserialize(data, ident);
    let body = generate_enum_body_deserialize(data, ident, enum_attrs);
    quote! {
        #header
        {
            type Error = ErrorStack;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: once_io::OStream
            {
                #body
            }
        }
    }
}

fn generate_enum_header_deserialize(data: &syn::DataEnum, ident: &syn::Ident) -> TokenStream2 {
    let trait_bounds = generate_enum_trait_bounds_deserialize(data);
    quote! {
        impl<V> Deserialize<V> for #ident
        where
            V: FileVersion,
            #(#trait_bounds)*
    }
}

fn generate_enum_trait_bounds_deserialize(data: &syn::DataEnum) -> Vec<TokenStream2> {
    data.variants
        .iter()
        .map(|variant| match &variant.fields {
            Fields::Unnamed(unnamed_fields) => unnamed_fields
                .unnamed
                .iter()
                .map(|field| {
                    let ty = match &field.ty {
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
            Fields::Unit => Vec::<TokenStream2>::new(),
            _ => panic!(),
        })
        .flatten()
        .collect::<Vec<TokenStream2>>()
}

fn generate_enum_body_deserialize(
    data: &syn::DataEnum,
    ident: &syn::Ident,
    enum_attrs: &EnumAttrs,
) -> TokenStream2 {
    let chunk_version = generate_chunk_version_deserialize(&enum_attrs.with_chunk_version);
    let raw_variants = data
        .variants
        .iter()
        .map(|variant| generate_enum_variant_deserialize(&ident, &variant))
        .collect::<Vec<TokenStream2>>();
    let variants = raw_variants
        .iter()
        .filter(|item| !item.is_empty())
        .collect::<Vec<&TokenStream2>>();
    quote! {
        #chunk_version
        match (version.major, version.minor) {
            #(#variants),*,
            _ => Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidChunkVersion
            )))
        }
    }
}

fn generate_enum_variant_deserialize(ident: &syn::Ident, variant: &syn::Variant) -> TokenStream2 {
    let variant_ident = &variant.ident;
    let variant_ident_str = variant_ident.to_string();
    let fields = match &variant.fields {
        Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .map(|field| {
                let ty = match &field.ty {
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    _ => panic!(),
                };
                quote! {
                    deserialize!(#ty, V, ostream, #variant_ident_str)
                }
            })
            .collect::<Vec<TokenStream2>>(),
        Fields::Unit => Vec::<TokenStream2>::new(),
        _ => panic!(),
    };
    match fields.is_empty() {
        false => {
            let chunk_version = match IfChunkVersion::parse(&variant.attrs) {
                Some(v) => generate_if_chunk_version_deserialize(&v),
                None => panic!("1"),
            };
            quote! {
                #chunk_version => Ok(#ident::#variant_ident (#(#fields),*))
            }
        }
        true => quote!(),
    }
}

fn generate_if_chunk_version_deserialize(if_chunk_version: &IfChunkVersion) -> TokenStream2 {
    let major = if_chunk_version.major;
    let minor = if_chunk_version.minor;
    quote!( (#major, #minor) )
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
                T: once_io::OStream
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
        false => quote! {},
    };
    quote! {
        #chunk_begin_trait_bounds
    }
}

fn generate_type_trait_bounds_deserialize(fields: &syn::Fields) -> Vec<TokenStream2> {
    match fields {
        Fields::Named(raw_fields) => raw_fields
            .named
            .iter()
            .map(|raw_field| {
                let attrs = FieldAttrs::parse(raw_field);
                let ty = match attrs.underlying_type {
                    Some(underlying_ty) => quote!(#underlying_ty),
                    None => match &raw_field.ty {
                        syn::Type::Array(value) => {
                            quote!(#value)
                        }
                        syn::Type::Path(value) => {
                            quote!(#value)
                        }
                        syn::Type::Tuple(value) => match value.elems.len() {
                            0 => quote!(#value),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                };
                match attrs.type_code {
                    Some(_) => {
                        quote! {
                            chunk::Begin: Deserialize<V>,
                            ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
                            #ty: Deserialize<V>,
                            ErrorStack: From<<#ty as Deserialize<V>>::Error>,
                        }
                    }
                    None => {
                        quote! {
                            #ty: Deserialize<V>,
                            ErrorStack: From<<#ty as Deserialize<V>>::Error>,
                        }
                    }
                }
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}

fn generate_body_deserialize(data: &syn::DataStruct, struct_attrs: &StructAttrs) -> TokenStream2 {
    let version_deserialize = generate_chunk_version_deserialize(&struct_attrs.with_chunk_version);
    let body_core = generate_body_core_deserialize(data, struct_attrs);
    let condition = generate_if_version_condition(&struct_attrs.if_version);
    quote! {
        #version_deserialize
        if #condition {
            #body_core
        } else {
            Ok(Self::default())
        }
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
                    let type_code = deserialize!(Rollback<TypeCode>, V, ostream, "type_code").inner;
                    match type_code {
                        #(#field_deserializes)*
                        TypeCode::EndOfFile => {
                            deserialize!(Chunk<{TypeCode::EndOfFile as u32}, ()>, V, ostream, "end_of_file");
                            break;
                        }
                        TypeCode::EndOfTable => {
                            deserialize!(Chunk<{TypeCode::EndOfTable as u32}, ()>, V, ostream, "end_of_table");
                            break;
                        }
                        _ => {
                            deserialize!(Chunk<{TypeCode::Null as u32}, ()>, V, ostream, "unknown");
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

fn generate_chunk_version_deserialize(with_chunk_version: &WithChunkVersion) -> TokenStream2 {
    match with_chunk_version {
        WithChunkVersion::Big => {
            quote!(let version = deserialize!(chunk::BigVersion, V, ostream, "version");)
        }
        WithChunkVersion::Short => {
            quote!(let version = deserialize!(chunk::ShortVersion, V, ostream, "version");)
        }
        WithChunkVersion::None => quote!(),
    }
}

fn generate_padding_deserialize(field_attrs: &FieldAttrs) -> TokenStream2 {
    match &field_attrs.padding.as_ref() {
        Some(ty) => quote!(deserialize!(#ty, V, ostream, "padding");),
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
                    syn::Type::Array(value) => {
                        quote!(#value)
                    }
                    syn::Type::Path(value) => {
                        quote!(#value)
                    }
                    syn::Type::Tuple(value) => match value.elems.len() {
                        0 => quote!(#value),
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                let ident_str = ident.to_string();
                let attrs = FieldAttrs::parse(raw_field);
                let padding_deserialize = generate_padding_deserialize(&attrs);
                let if_version_conditions = generate_if_version_condition(&attrs.if_version);
                let deserialize = match &attrs.type_code {
                    Some(type_code) => {
                        match &attrs.underlying_type {
                            Some(underlying_ty) => quote! {
                                //TODO: improve deserialize! to allow #ty_str as parameter
                                deserialize!(Chunk<{TypeCode::#type_code as u32}, #underlying_ty>, V, ostream, #ident_str).inner.into()
                            },
                            None => quote! {
                                deserialize!(Chunk<{TypeCode::#type_code as u32}, #ty>, V, ostream, #ident_str).inner
                            },
                        }
                    }
                    None => {
                        match &attrs.underlying_type {
                            Some(underlying_ty) => quote! {
                                //TODO: improve deserialize! to allow #ty_str as parameter
                                deserialize!(#underlying_ty, V, ostream, #ident_str).into()
                            },
                            None => quote! {
                                deserialize!(#ty, V, ostream, #ident_str)
                            },
                        }
                    }
                };
                match struct_attrs.table.0 {
                    true => {
                        let type_code = attrs.type_code.as_ref().unwrap();
                        quote!(
                            TypeCode::#type_code => {
                                if #if_version_conditions {
                                    #padding_deserialize
                                    table.#ident = #deserialize;
                                }
                            }
                        )
                    }
                    false => {
                        quote! {
                            #ident: {
                                if #if_version_conditions {
                                    #padding_deserialize
                                    #deserialize
                                } else {
                                    <#ty>::default()
                                }
                            }
                        }
                    }
                }
            })
            .collect::<Vec<TokenStream2>>(),
        _ => Vec::<TokenStream2>::new(),
    }
}
