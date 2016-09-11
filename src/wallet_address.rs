//! Fractal Global Wallet Address
//!
//! This module holds the Fractal Global wallet address format, along with its parsing error
//! representing struct.

use std::convert::From;
use std::result::Result;
use std::error::Error;
use std::{fmt, str};
use std::str::FromStr;

use rust_base58::{ToBase58, FromBase58};
use rust_base58::base58::FromBase58Error;
#[cfg(feature = "json-types")]
use rustc_serialize::json;

/// The wallet address size.
///
/// This is the length, in bytes of the wallet addresses. It can be used to create arrays to store
/// complete addresses. Note: an address stored as a `[u8, WALLET_ADDRESS_LEN]` won't have any sort
/// of checksum verification, and as such, it should be used with extreme care, never using is as
/// an input or output mechanism, and only as an internal representation of the wallet address.
pub const WALLET_ADDRESS_LEN: usize = 7;

/// The object representation of a wallet address.
///
/// Wallet addresses are structs that act as as an easy manipulation object for wallet addresses.
/// Addresses that come from user input can be verified, and made sure they are correct.
///
/// Address can be used as strings or displayed using the `Display` trait:
///
/// ```
/// use fractal_utils::{WalletAddress, WALLET_ADDRESS_LEN};
///
/// let addr = WalletAddress::from_data([0u8; WALLET_ADDRESS_LEN]);
/// let addr_str = format!("{}", addr);
/// assert_eq!(addr_str, "fr111111111");
/// ```
///
/// All Fractal wallet addresses start with `fr`, and then they have a base-58 encoded string
/// representing `WALLET_ADDRESS_LEN+2` bytes. The first byte will be `0x00`, that the rest bytes
/// until `WALLET_ADDRESS_LEN` will compose the actual address, while the other two are the
/// checksum. That way addresses coming from user input can be verified:
///
/// ```
/// use std::str::FromStr;
/// use std::result::Result;
/// use fractal_utils::{WalletAddress, WALLET_ADDRESS_LEN};
///
/// let wallet: Result<WalletAddress, _> = "fr111111111".parse();
/// assert!(wallet.is_ok());
///
/// let wallet: Result<WalletAddress, _> = "fr111111112".parse();
/// assert!(wallet.is_err());
/// ```
///
/// The checksums are calculated by doing the `XOR` operation in all the bytes of the wallet address
/// and doing `XOR` of the checksum's first byte with the second one for each byte:
///
/// ```
/// # use fractal_utils::WALLET_ADDRESS_LEN;
/// #
/// let check_addr = [0x00, 0x11, 0x2A, 0x44, 0xCD, 0xFF, 0xE0];
/// # assert_eq!(check_addr.len(), WALLET_ADDRESS_LEN);
/// let mut checksum = [0u8; 2];
///
/// for b in &check_addr {
///     checksum[0] ^= *b;
///     checksum[1] ^= checksum[0];
/// }
///
/// assert_eq!(checksum, [0xAD, 0x07]);
/// ```
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct WalletAddress {
    address: [u8; WALLET_ADDRESS_LEN],
}

impl WalletAddress {
    /// Creates a new wallet address from raw data.
    ///
    /// This should only be used if the raw input data is verified to be correct, ir it could lead
    /// o a false address.
    ///
    /// It will panic if the address does not start with byte `0x00`.
    pub fn from_data(addr: [u8; WALLET_ADDRESS_LEN]) -> WalletAddress {
        assert_eq!(addr[0],
                   0x00,
                   "the provided address is not a correct Fractal Global wallet address, its \
                    first byt should be 0x00");
        WalletAddress { address: addr }
    }

    /// Returns the wallet address bytes.
    ///
    /// This could be useful to store the bytes in databases where space can be an issue, or where
    /// fast search is required. It does not contain checksums nor any other verification mechanism.
    pub fn get_raw(&self) -> &[u8] {
        &self.address
    }
}

impl From<[u8; WALLET_ADDRESS_LEN]> for WalletAddress {
    fn from(other: [u8; WALLET_ADDRESS_LEN]) -> WalletAddress {
        WalletAddress { address: other }
    }
}

impl FromStr for WalletAddress {
    type Err = WalletAddressParseError;
    fn from_str(s: &str) -> Result<WalletAddress, WalletAddressParseError> {
        if &s[0..2] != "fr" {
            return Err(WalletAddressParseError::new(s,
                                                    "the address does not start with \"fr\"",
                                                    None));
        }
        let bytes = match s[2..].from_base58() {
            Ok(b) => b,
            Err(FromBase58Error::InvalidBase58Byte(c, i)) => {
                let new_error = FromBase58Error::InvalidBase58Byte(c, i + 2);
                return Err(WalletAddressParseError::new(s,
                                                        &format!("the address is not a valid \
                                                                  base-58 encoded string: {}",
                                                                 new_error),
                                                        Some(new_error)));
            }
        };
        if bytes[0] != 0x00 {
            return Err(WalletAddressParseError::new(s,
                                                    "the first byte of the address is not 0x00",
                                                    None));
        }

        let mut checksum = [0u8; 2];
        for byte in &bytes[..WALLET_ADDRESS_LEN] {
            checksum[0] ^= *byte;
            checksum[1] ^= checksum[0];
        }

        if checksum[0] != bytes[WALLET_ADDRESS_LEN] ||
           checksum[1] != bytes[WALLET_ADDRESS_LEN + 1] {
            Err(WalletAddressParseError::new(s, "checksum fail", None))
        } else {
            let mut address = [0u8; WALLET_ADDRESS_LEN];
            address.clone_from_slice(&bytes[..WALLET_ADDRESS_LEN]);
            Ok(WalletAddress::from_data(address))
        }

    }
}

#[cfg(feature = "json-types")]
/// The `WalletAddress` type can easily be converted to json, using its `to_json()` method. Note
/// that this will return a `Json::String` with the wallet address as a string in it.
impl json::ToJson for WalletAddress {
    fn to_json(&self) -> json::Json {
        json::Json::String(format!("{}", self))
    }
}

impl fmt::Display for WalletAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut arr = [0u8; WALLET_ADDRESS_LEN + 2];
        arr[0..WALLET_ADDRESS_LEN].clone_from_slice(&self.address);

        for byte in &self.address {
            arr[WALLET_ADDRESS_LEN] ^= *byte;
            arr[WALLET_ADDRESS_LEN + 1] ^= arr[WALLET_ADDRESS_LEN];
        }

        write!(f, "fr{}", arr.to_base58())
    }
}

/// Wallet address parsing error.
///
/// This struct represents a wallet address parsing error. It can be used to check the validity of
/// wallet address strings, and implements common `Error` and `Display` traits.
#[derive(Debug)]
pub struct WalletAddressParseError {
    description: String,
    cause: Option<FromBase58Error>,
}

impl WalletAddressParseError {
    fn new<S: AsRef<str>>(wallet_address: S,
                          error: S,
                          cause: Option<FromBase58Error>)
                          -> WalletAddressParseError {
        WalletAddressParseError {
            description: format!("the wallet address {:?} is not a valid Fractal Global wallet \
                                  address, {}",
                                 wallet_address.as_ref(),
                                 error.as_ref()),
            cause: cause,
        }
    }
}

impl fmt::Display for WalletAddressParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for WalletAddressParseError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
