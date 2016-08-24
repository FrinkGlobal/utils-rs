#![deny(deprecated, drop_with_repr_extern, improper_ctypes,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused, unused_extern_crates,
        unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]
#![allow(missing_docs)]

extern crate rand;
extern crate fractal_utils;

use std::str::FromStr;
use std::u64;

use rand::{Rng, thread_rng};

use fractal_utils::wallet_address::{WalletAddress, WALLET_ADDRESS_LEN};
use fractal_utils::amount::Amount;

#[cfg(test)]
#[test]
fn it_fromstr_walletaddress() {
    for _ in 0..50 {
        let mut random_addr = [0u8; WALLET_ADDRESS_LEN];
        thread_rng().fill_bytes(&mut random_addr[1..]);
        assert_eq!(WalletAddress::from_str(&format!("{}", WalletAddress::from_data(random_addr)))
            .unwrap().get_raw(), &random_addr);
    }
}

#[test]
#[should_panic]
fn it_invalid_wallet_address() {
    let _ = WalletAddress::from_data([1u8; WALLET_ADDRESS_LEN]);
}

#[test]
fn it_amount_parse() {
    let amount: Amount = "175.646".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_646));
    assert_eq!(format!("{}", amount), "175.646");

    let amount: Amount = "175.64".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_640));
    assert_eq!(format!("{}", amount), "175.64");

    let amount: Amount = "175.6".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_600));
    assert_eq!(format!("{}", amount), "175.6");

    let amount: Amount = "175.000".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_000));
    assert_eq!(format!("{}", amount), "175");

    let amount: Amount = "175.00".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_000));
    assert_eq!(format!("{}", amount), "175");

    let amount: Amount = "175.0".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_000));
    assert_eq!(format!("{}", amount), "175");

    let amount: Amount = "175".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_000));
    assert_eq!(format!("{}", amount), "175");

    let amount = Amount::max_value();
    assert_eq!(format!("{}", amount),
               format!("{}.{}", u64::MAX / 1_000, u64::MAX % 1_000));

    let amount: Amount = format!("{}", amount).parse().unwrap();
    assert_eq!(amount, Amount::max_value());

    let amount = Amount::min_value();
    assert_eq!(format!("{}", amount), "0");

    let amount: Amount = "0".parse().unwrap();
    assert_eq!(amount, Amount::min_value());

    let amount: Amount = "0.00012".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(0));
    assert_eq!(format!("{}", amount), "0");

    let amount: Amount = "175.6469".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_647));
    assert_eq!(format!("{}", amount), "175.647");

    let amount: Amount = "175.6465".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_647));
    assert_eq!(format!("{}", amount), "175.647");

    let amount: Amount = "175.6464".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(175_646));
    assert_eq!(format!("{}", amount), "175.646");

    let amount: Amount = ".6465".parse().unwrap();
    assert_eq!(amount, Amount::from_repr(647));
    assert_eq!(format!("{}", amount), "0.647");
}

#[test]
fn it_amount_other_format() {
    let amount = Amount::from_repr(56_000); // 56.000
    assert_eq!(format!("{}", amount), "56");
    assert_eq!(format!("{:.2}", amount), "56.00");
    assert_eq!(format!("{:.5}", amount), "56.00000");
    assert_eq!(format!("{:05}", amount), "00056");
    assert_eq!(format!("{:05.2}", amount), "56.00");
    assert_eq!(format!("{:05.1}", amount), "056.0");

    let amount = Amount::from_repr(56); // 0.056
    assert_eq!(format!("{:.0}", amount), "0");
    assert_eq!(format!("{:.2}", amount), "0.06");

    let amount = Amount::from_repr(1500); // 1.500
    assert_eq!(format!("{:.0}", amount), "2");
}

#[test]
fn it_amount_bad_format() {
    let amount: Result<Amount, _> = "175.".parse();
    assert!(amount.is_err());

    let amount: Result<Amount, _> = "175.837.9239".parse();
    assert!(amount.is_err());

    let amount: Result<Amount, _> = ".098320.2930".parse();
    assert!(amount.is_err());
}

#[test]
fn it_amount_ops() {
    let mut amount = Amount::min_value();
    let amount_10 = Amount::from_repr(10_000);
    assert_eq!(amount + amount_10, Amount::from_repr(10_000));
    amount += amount_10;
    assert_eq!(amount, Amount::from_repr(10_000));

    assert_eq!(amount - amount_10, Amount::min_value());
    amount -= amount_10;
    assert_eq!(amount, Amount::min_value());

    amount += amount_10;
    assert_eq!(amount * 10u32, Amount::from_repr(100_000));
    amount *= 10u32;
    assert_eq!(amount, Amount::from_repr(100_000));

    assert_eq!(amount / 10u32, Amount::from_repr(10_000));
    amount /= 10u32;
    assert_eq!(amount, Amount::from_repr(10_000));

    let mut amount = Amount::from_repr(12_345);
    assert_eq!(amount % 10u32, Amount::from_repr(2_345));
    amount %= 10u32;
    assert_eq!(amount, Amount::from_repr(2_345));
    assert_eq!(amount % 1u32, Amount::from_repr(345));
}
