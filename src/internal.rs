use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    bracketed,
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    token::Bracket,
    Error, Result, Token,
};

pub(crate) fn zero_indexed_enum_impl(tokens: TokenStream) -> TokenStream {
    zero_indexed_enum_parse
        .parse2(tokens)
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

    let ts = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #type_name {
            #(#variants = #variants_ns),*
        }
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
        impl TryFrom<usize> for #type_name {
            type Error = ();
            fn try_from(value: usize) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    #(#variants_ns => Ok(#type_name::#variants),)*
                    _ => Err(()),
                }
            }
        }
    };
    Ok(ts)
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
