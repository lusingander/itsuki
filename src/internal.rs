use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    bracketed,
    parse::{ParseStream, Parser},
    parse2,
    punctuated::Punctuated,
    token::Bracket,
    DataEnum, DeriveInput, Error, Result, Token,
};

pub(crate) fn zero_indexed_enum_impl(tokens: TokenStream) -> TokenStream {
    zero_indexed_enum_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

pub(crate) fn zero_indexed_enum_derive_impl(tokens: TokenStream) -> TokenStream {
    parse2::<DeriveInput>(tokens)
        .and_then(zero_indexed_enum_derive_parse)
        .unwrap_or_else(Error::into_compile_error)
}

fn zero_indexed_enum_parse(input: ParseStream) -> Result<TokenStream> {
    let type_name: Ident = input.parse()?;
    let _: Token![=>] = input.parse()?;
    let content;
    let _: Bracket = bracketed!(content in input);
    let variants = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;

    let variants: Vec<Ident> = variants.iter().cloned().collect();
    let variants_ns = variants_ns(&variants);
    let variants_nexts = variants_nexts(&variants);
    let variants_prevs = variants_prevs(&variants);
    let variants_len = variants.len();

    let enum_defintion_block = build_enum_defintion_block(&type_name, &variants, &variants_ns);
    let impl_block = build_impl_block(
        &type_name,
        &variants,
        &variants_nexts,
        &variants_prevs,
        variants_len,
    );
    let try_into_impl_block = build_try_into_impl_block(&type_name, &variants_ns, &variants);

    let ts = quote! {
        #enum_defintion_block
        #impl_block
        #try_into_impl_block
    };
    Ok(ts)
}

fn zero_indexed_enum_derive_parse(input: DeriveInput) -> Result<TokenStream> {
    let data_enum: DataEnum = if let syn::Data::Enum(data) = input.data {
        data
    } else {
        return Err(Error::new_spanned(
            input,
            "ZeroIndexedEnum only supports enums",
        ));
    };

    let type_name = input.ident;
    let variants: Vec<Ident> = data_enum
        .variants
        .iter()
        .map(ident_from_variant)
        .collect::<Result<_>>()?;
    let variants_ns = variants_ns(&variants);
    let variants_nexts = variants_nexts(&variants);
    let variants_prevs = variants_prevs(&variants);
    let variants_len = variants.len();

    let impl_block = build_impl_block(
        &type_name,
        &variants,
        &variants_nexts,
        &variants_prevs,
        variants_len,
    );
    let try_into_impl_block = build_try_into_impl_block(&type_name, &variants_ns, &variants);

    let ts = quote! {
        #impl_block
        #try_into_impl_block
    };
    Ok(ts)
}

fn build_enum_defintion_block(
    type_name: &Ident,
    variants: &Vec<Ident>,
    variants_ns: &Vec<syn::Index>,
) -> TokenStream {
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #type_name {
            #(#variants = #variants_ns),*
        }
    }
}

fn build_impl_block(
    type_name: &Ident,
    variants: &Vec<Ident>,
    variants_nexts: &Vec<Ident>,
    variants_prevs: &Vec<Ident>,
    variants_len: usize,
) -> TokenStream {
    quote! {
        impl #type_name {
            pub fn len() -> usize {
                #variants_len
            }
            pub fn vars_vec() -> Vec<#type_name> {
                vec![
                    #(#type_name::#variants),*
                ]
            }
            pub fn vars_array() -> [#type_name; #variants_len] {
                [
                    #(#type_name::#variants),*
                ]
            }
            pub fn next(&self) -> #type_name {
                match self {
                    #( #type_name::#variants => #type_name::#variants_nexts ),*
                }
            }
            pub fn prev(&self) -> #type_name {
                match self {
                    #( #type_name::#variants => #type_name::#variants_prevs ),*
                }
            }
            fn next_in<F>(&self, f: F) -> #type_name
            where
                F: Fn(#type_name) -> bool,
            {
                let mut item = self.next();
                while !f(item) {
                    item = item.next();
                }
                item
            }
            fn prev_in<F>(&self, f: F) -> #type_name
            where
                F: Fn(#type_name) -> bool,
            {
                let mut item = self.prev();
                while !f(item) {
                    item = item.prev();
                }
                item
            }
            pub fn val(&self) -> usize {
                *self as usize
            }
        }
    }
}

fn build_try_into_impl_block(
    type_name: &Ident,
    variants_ns: &Vec<syn::Index>,
    variants: &Vec<Ident>,
) -> TokenStream {
    quote! {
        impl TryFrom<usize> for #type_name {
            type Error = ();
            fn try_from(value: usize) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    #(#variants_ns => Ok(#type_name::#variants),)*
                    _ => Err(()),
                }
            }
        }
    }
}

fn ident_from_variant(variant: &syn::Variant) -> Result<Ident> {
    if variant.fields.is_empty() {
        Ok(variant.ident.clone())
    } else {
        Err(Error::new_spanned(
            variant,
            "ZeroIndexedEnum does not support variants with fields",
        ))
    }
}

fn variants_ns(vs: &[Ident]) -> Vec<syn::Index> {
    (0..vs.len()).map(syn::Index::from).collect()
}

fn variants_nexts(vs: &[Ident]) -> Vec<Ident> {
    let l = vs.len();
    vs.iter().cloned().cycle().skip(1).take(l).collect()
}

fn variants_prevs(vs: &[Ident]) -> Vec<Ident> {
    let l = vs.len();
    vs.iter().cloned().cycle().skip(l - 1).take(l).collect()
}
