#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
extern crate std;

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// Network id for transaction-body network discriminants.
///
/// The Conway CDDL restricts transaction-body `network_id` to `0 / 1`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NetworkId {
    /// Testnet network id.
    Testnet,
    /// Mainnet network id.
    Mainnet,
}

impl NetworkId {
    /// Returns the protocol integer representation.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Testnet => 0,
            Self::Mainnet => 1,
        }
    }
}

impl TryFrom<u8> for NetworkId {
    type Error = PrimitiveError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Testnet),
            1 => Ok(Self::Mainnet),
            _ => Err(PrimitiveError::InvalidNetworkId { value }),
        }
    }
}

impl From<NetworkId> for u8 {
    fn from(value: NetworkId) -> Self {
        value.as_u8()
    }
}

/// Known Cardano ledger era.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Era {
    /// Byron era.
    Byron,
    /// Shelley era.
    Shelley,
    /// Allegra era.
    Allegra,
    /// Mary era.
    Mary,
    /// Alonzo era.
    Alonzo,
    /// Babbage era.
    Babbage,
    /// Conway era.
    Conway,
}

impl Era {
    /// Returns the stable lowercase era name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Byron => "byron",
            Self::Shelley => "shelley",
            Self::Allegra => "allegra",
            Self::Mary => "mary",
            Self::Alonzo => "alonzo",
            Self::Babbage => "babbage",
            Self::Conway => "conway",
        }
    }
}

impl From<Era> for &'static str {
    fn from(value: Era) -> Self {
        value.name()
    }
}

macro_rules! numeric_domain {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u64);

        impl $name {
            /// Creates a value from its raw unsigned integer representation.
            #[must_use]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the raw unsigned integer representation.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }
    };
}

numeric_domain!(
    Slot,
    "Cardano slot number.\n\nThe Conway CDDL represents `slot_no` as `uint .size 8`."
);
numeric_domain!(
    Epoch,
    "Cardano epoch number.\n\nThe Conway CDDL represents `epoch_no` as `uint .size 8`."
);
numeric_domain!(
    BlockNumber,
    "Cardano block number.\n\nThe Conway CDDL represents `block_no` as `uint .size 8`."
);
numeric_domain!(
    Coin,
    "Lovelace coin quantity.\n\nThis type is intentionally only a non-negative domain wrapper. Minimum-ADA, deposit, fee, and value-size rules belong in later ledger validation milestones."
);

macro_rules! fixed_bytes_domain {
    ($name:ident, $len:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; $len]);

        impl $name {
            /// Length in bytes.
            pub const LEN: usize = $len;

            /// Creates a value from fixed-width bytes.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; $len]) -> Self {
                Self(bytes)
            }

            /// Returns the fixed-width bytes.
            #[must_use]
            pub const fn to_bytes(self) -> [u8; $len] {
                self.0
            }

            /// Borrows the fixed-width bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; $len] {
                &self.0
            }

            /// Creates a value from a byte slice with the exact expected length.
            pub fn try_from_slice(bytes: &[u8]) -> Result<Self, PrimitiveError> {
                if bytes.len() != Self::LEN {
                    return Err(PrimitiveError::InvalidByteLength {
                        expected: Self::LEN,
                        actual: bytes.len(),
                    });
                }

                let mut out = [0u8; $len];
                out.copy_from_slice(bytes);
                Ok(Self(out))
            }
        }

        impl From<[u8; $len]> for $name {
            fn from(bytes: [u8; $len]) -> Self {
                Self::from_bytes(bytes)
            }
        }

        impl From<$name> for [u8; $len] {
            fn from(value: $name) -> Self {
                value.to_bytes()
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = PrimitiveError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Self::try_from_slice(value)
            }
        }
    };
}

fixed_bytes_domain!(
    TransactionId,
    32,
    "Transaction identifier hash bytes.\n\nThe Conway CDDL defines `transaction_id = hash32`."
);
fixed_bytes_domain!(
    BlockHash,
    32,
    "Block hash bytes.\n\nThe Conway CDDL uses `hash32` for previous block and block-body hashes."
);
fixed_bytes_domain!(
    DatumHash,
    32,
    "Datum hash bytes.\n\nThe Conway CDDL represents datum hashes as `hash32`."
);
fixed_bytes_domain!(
    ScriptHash,
    28,
    "Script hash bytes.\n\nThe Conway CDDL defines `script_hash = hash28`."
);
fixed_bytes_domain!(
    KeyHash,
    28,
    "Key hash bytes used by credential domains.\n\nThe Conway CDDL uses `addr_keyhash = hash28` and `pool_keyhash = hash28`."
);
fixed_bytes_domain!(
    PolicyId,
    28,
    "Native asset policy identifier.\n\nThe Conway CDDL defines `policy_id = script_hash`."
);

impl From<ScriptHash> for PolicyId {
    fn from(value: ScriptHash) -> Self {
        Self::from_bytes(value.to_bytes())
    }
}

impl From<PolicyId> for ScriptHash {
    fn from(value: PolicyId) -> Self {
        Self::from_bytes(value.to_bytes())
    }
}

/// Credential hash domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Credential {
    /// Verification-key hash credential.
    KeyHash(KeyHash),
    /// Script-hash credential.
    ScriptHash(ScriptHash),
}

impl Credential {
    /// Creates a credential from a CDDL credential tag and hash bytes.
    pub fn try_from_tagged_bytes(tag: u8, bytes: [u8; 28]) -> Result<Self, PrimitiveError> {
        match tag {
            0 => Ok(Self::KeyHash(KeyHash::from_bytes(bytes))),
            1 => Ok(Self::ScriptHash(ScriptHash::from_bytes(bytes))),
            _ => Err(PrimitiveError::InvalidCredentialTag { value: tag }),
        }
    }

    /// Returns the CDDL credential tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::KeyHash(_) => 0,
            Self::ScriptHash(_) => 1,
        }
    }

    /// Returns the credential hash bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 28] {
        match self {
            Self::KeyHash(value) => value.to_bytes(),
            Self::ScriptHash(value) => value.to_bytes(),
        }
    }
}

/// Native asset name bytes.
#[derive(Clone, Copy, Debug)]
pub struct AssetName {
    bytes: [u8; Self::MAX_LEN],
    len: u8,
}

impl AssetName {
    /// Maximum asset-name length in bytes.
    ///
    /// The Conway CDDL defines `asset_name = bytes .size (0 .. 32)`.
    pub const MAX_LEN: usize = 32;

    /// Creates an asset name from bytes.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, PrimitiveError> {
        if bytes.len() > Self::MAX_LEN {
            return Err(PrimitiveError::AssetNameTooLong {
                max: Self::MAX_LEN,
                actual: bytes.len(),
            });
        }

        let mut out = [0u8; Self::MAX_LEN];
        let Some(target) = out.get_mut(..bytes.len()) else {
            return Err(PrimitiveError::AssetNameTooLong {
                max: Self::MAX_LEN,
                actual: bytes.len(),
            });
        };
        target.copy_from_slice(bytes);

        Ok(Self {
            bytes: out,
            len: bytes
                .len()
                .try_into()
                .map_err(|_| PrimitiveError::AssetNameTooLong {
                    max: Self::MAX_LEN,
                    actual: bytes.len(),
                })?,
        })
    }

    /// Creates the empty asset name.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            bytes: [0u8; Self::MAX_LEN],
            len: 0,
        }
    }

    /// Returns the asset-name length in bytes.
    #[must_use]
    pub const fn len(self) -> usize {
        self.len as usize
    }

    /// Returns `true` when this is the empty asset name.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Borrows the asset-name bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        match self.bytes.get(..self.len()) {
            Some(bytes) => bytes,
            None => &[],
        }
    }

    /// Returns the padded internal representation and significant length.
    #[must_use]
    pub const fn to_padded_bytes(self) -> ([u8; Self::MAX_LEN], usize) {
        (self.bytes, self.len())
    }
}

impl TryFrom<&[u8]> for AssetName {
    type Error = PrimitiveError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_slice(value)
    }
}

impl PartialEq for AssetName {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for AssetName {}

impl Hash for AssetName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

impl PartialOrd for AssetName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AssetName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

/// Primitive constructor failures.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveError {
    /// Network id is not one of the transaction-body network ids.
    InvalidNetworkId {
        /// Rejected value.
        value: u8,
    },
    /// Credential tag is not one of the CDDL credential tags.
    InvalidCredentialTag {
        /// Rejected value.
        value: u8,
    },
    /// Fixed-width byte value used the wrong length.
    InvalidByteLength {
        /// Expected byte length.
        expected: usize,
        /// Actual byte length.
        actual: usize,
    },
    /// Asset name exceeded the Cardano maximum length.
    AssetNameTooLong {
        /// Maximum accepted byte length.
        max: usize,
        /// Actual byte length.
        actual: usize,
    },
}

impl fmt::Display for PrimitiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNetworkId { value } => {
                write!(f, "invalid Cardano network id: {value}")
            }
            Self::InvalidCredentialTag { value } => {
                write!(f, "invalid Cardano credential tag: {value}")
            }
            Self::InvalidByteLength { expected, actual } => {
                write!(
                    f,
                    "invalid Cardano primitive byte length: expected {expected}, actual {actual}"
                )
            }
            Self::AssetNameTooLong { max, actual } => {
                write!(f, "Cardano asset name too long: max {max}, actual {actual}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PrimitiveError {}

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
