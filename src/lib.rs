//! A macro that defines a simple zero-based sequential enum.
//!
//! # Examples
//!
//! ```
//! use itsuki::zero_indexed_enum;
//!
//! zero_indexed_enum! {
//!     Quintuplets => [Ichika, Nino, Miku, Yotsuba, Itsuki]
//! }
//!
//! let miku = Quintuplets::Miku;
//!
//! assert_eq!(miku, Quintuplets::Miku);
//! assert_ne!(miku, Quintuplets::Nino);
//!
//! assert_eq!(Quintuplets::len(), 5);
//!
//! use Quintuplets::*;
//!
//! assert_eq!(
//!     Quintuplets::vars_vec(),
//!     vec![Ichika, Nino, Miku, Yotsuba, Itsuki]
//! );
//! assert_eq!(
//!     Quintuplets::vars_array(),
//!     [Ichika, Nino, Miku, Yotsuba, Itsuki]
//! );
//!
//! assert_eq!(Nino.next(), Miku);
//! assert_eq!(Itsuki.next(), Ichika);
//!
//! assert_eq!(Yotsuba.prev(), Miku);
//! assert_eq!(Ichika.prev(), Itsuki);
//!
//! assert_eq!(Ichika.next_in(|q| [Miku, Yotsuba].contains(&q)), Miku);
//! assert_eq!(Miku.next_in(|q| [Miku, Yotsuba].contains(&q)), Yotsuba);
//!
//! assert_eq!(Nino.prev_in(|q| [Miku, Yotsuba].contains(&q)), Yotsuba);
//! assert_eq!(Yotsuba.prev_in(|q| [Miku, Yotsuba].contains(&q)), Miku);
//!
//! assert_eq!(Miku.val(), 2);
//! assert_eq!(Yotsuba.val(), 3);
//!
//! assert_eq!(Quintuplets::try_from(0), Ok(Ichika));
//! assert_eq!(Quintuplets::try_from(4), Ok(Itsuki));
//! assert_eq!(Quintuplets::try_from(5), Err(()));
//!
//! assert_eq!(1.try_into(), Ok(Nino));
//! assert_eq!(3.try_into(), Ok(Yotsuba));
//! ```
//!

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    bracketed,
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    token::Bracket,
    Error, Result, Token,
};

/// Declare the enum type and variables as shown below:
/// ```no_run
/// use itsuki::zero_indexed_enum;
///
/// zero_indexed_enum! {
///     Quintuplets => [Ichika, Nino, Miku, Yotsuba, Itsuki]
/// }
/// ```
///
/// And then, the following enum will be defined.
/// ```no_run
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub enum Quintuplets {
///     Ichika = 0,
///     Nino = 1,
///     Miku = 2,
///     Yotsuba = 3,
///     Itsuki = 4,
/// }
/// impl Quintuplets {
///     // ...
/// }
/// ```
#[proc_macro]
pub fn zero_indexed_enum(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    zero_indexed_enum_impl(tokens.into()).into()
}

fn zero_indexed_enum_impl(tokens: TokenStream) -> TokenStream {
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
            fn try_from(value: usize) -> Result<Self, Self::Error> {
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
