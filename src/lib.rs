#![doc(html_logo_url = "http://test.fractal.global/img/logo.svg",
       html_favicon_url = "http://test.fractal.global/img/favicon32.png",
       html_root_url = "http://fractal.global/utils-rs/")]
//! This crate contains utilities to work with Fractal Global Credits. It contains the wallet
//! address representation and the currency amount type. Each of them is explained in its own
//! documentation and can be easily used by third parties.
//!
//! Using it is as simple as including this in the crate:
//!
//! ```
//! extern crate fractal_utils;
//! ```
#![forbid(missing_docs, warnings)]
#![deny(deprecated, improper_ctypes, non_shorthand_field_patterns, overflowing_literals,
    plugin_as_library, private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unused, unused_allocation, unused_attributes,
    unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused, unused_extern_crates, unused_import_braces,
    unused_qualifications, unused_results, variant_size_differences)]

extern crate rustc_serialize;
extern crate rust_base58;

pub mod amount;
pub mod wallet_address;
pub mod location;

pub use amount::Amount;
pub use wallet_address::{WALLET_ADDRESS_LEN, WalletAddress};
pub use location::Address;

/// The symbol of Fractal Global Credits
///
/// This symbol, `Ͼ` should be used whenever an amount of currency has to be represented. It is a
/// dotted lumate sigma symbol, the Unicode *U+03FE* character. It can easily be used when
/// formatting currencies:
///
/// ```
/// use fractal_utils::{CURRENCY_SYMBOL, Amount};
/// # assert_eq!(CURRENCY_SYMBOL, 'Ͼ');
/// # assert_eq!(CURRENCY_SYMBOL, '\u{03FE}');
/// let amount = Amount::from_repr(30_000); // 30.000
/// assert_eq!(format!("{} {}", CURRENCY_SYMBOL, amount), "Ͼ 30");
/// ```
pub const CURRENCY_SYMBOL: char = 'Ͼ';
