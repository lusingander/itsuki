//! A macro that defines a simple zero-based sequential enum.
//!
//! # Examples
//!
//! ```
//! use itsuki::define_zero_indexed_enum;
//!
//! define_zero_indexed_enum! {
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

mod internal;

/// Declare the enum type and variables as shown below:
/// ```no_run
/// use itsuki::define_zero_indexed_enum;
///
/// define_zero_indexed_enum! {
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
pub fn define_zero_indexed_enum(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::define_zero_indexed_enum_impl(tokens.into()).into()
}

/// ```no_run
/// use itsuki::ZeroIndexedEnum;
///
/// #[derive(ZeroIndexedEnum, Debug, Clone, Copy, PartialEq, Eq)]
/// enum Quintuplets {
///     Ichika,
///     Nino,
///     Miku,
///     Yotsuba,
///     Itsuki,
/// }
/// ```
///
/// And then, impl block same as [`define_zero_indexed_enum!`] will be generated.
#[proc_macro_derive(ZeroIndexedEnum)]
pub fn zero_indexed_enum_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::zero_indexed_enum_derive_impl(tokens.into()).into()
}

/// ```no_run
/// use itsuki::zero_indexed_enum;
///
/// #[zero_indexed_enum]
/// enum Quintuplets {
///     Ichika,
///     Nino,
///     Miku,
///     Yotsuba,
///     Itsuki,
/// }
/// ```
///
/// And then, impl block and derives same as [`define_zero_indexed_enum!`] will be generated.
#[proc_macro_attribute]
pub fn zero_indexed_enum(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    internal::zero_indexed_enum_impl(item.into()).into()
}
