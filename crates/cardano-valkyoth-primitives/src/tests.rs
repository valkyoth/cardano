use super::{
    AssetName, BlockHash, BlockNumber, Coin, Credential, DatumHash, Epoch, Era, ErrorCategory,
    ErrorCode, KeyHash, NetworkId, PolicyId, PrimitiveError, ScriptHash, Slot, TransactionId,
};
use core::hash::{Hash, Hasher};
use std::string::ToString;

#[test]
fn network_id_round_trips() {
    assert_eq!(NetworkId::try_from(0), Ok(NetworkId::Testnet));
    assert_eq!(NetworkId::try_from(1), Ok(NetworkId::Mainnet));
    assert_eq!(u8::from(NetworkId::Testnet), 0);
    assert_eq!(u8::from(NetworkId::Mainnet), 1);
    assert_eq!(
        NetworkId::try_from(2),
        Err(PrimitiveError::InvalidNetworkId { value: 2 })
    );
}

#[test]
fn era_names_are_stable() {
    assert_eq!(Era::Byron.name(), "byron");
    assert_eq!(Era::Shelley.name(), "shelley");
    assert_eq!(Era::Allegra.name(), "allegra");
    assert_eq!(Era::Mary.name(), "mary");
    assert_eq!(Era::Alonzo.name(), "alonzo");
    assert_eq!(Era::Babbage.name(), "babbage");
    assert_eq!(<&'static str>::from(Era::Conway), "conway");
}

#[test]
fn numeric_domains_round_trip() {
    assert_eq!(u64::from(Slot::new(42)), 42);
    assert_eq!(u64::from(Epoch::from(43)), 43);
    assert_eq!(u64::from(BlockNumber::new(44)), 44);
    assert_eq!(u64::from(Coin::from(45)), 45);
}

#[test]
fn fixed_hash_domains_round_trip() {
    let hash32 = [7u8; 32];
    let hash28 = [8u8; 28];

    assert_eq!(
        TransactionId::try_from_slice(&hash32),
        Ok(TransactionId::from(hash32))
    );
    assert_eq!(BlockHash::from(hash32).to_bytes(), hash32);
    assert_eq!(DatumHash::from(hash32).as_bytes(), &hash32);
    assert_eq!(
        ScriptHash::try_from_slice(&hash28),
        Ok(ScriptHash::from(hash28))
    );
    assert_eq!(KeyHash::from(hash28).to_bytes(), hash28);
    assert_eq!(PolicyId::from(hash28).as_bytes(), &hash28);
}

#[test]
fn fixed_hash_domains_reject_wrong_lengths() {
    let too_short = [0u8; 31];
    assert_eq!(
        TransactionId::try_from_slice(&too_short),
        Err(PrimitiveError::InvalidByteLength {
            expected: 32,
            actual: 31
        })
    );

    let too_long = [0u8; 29];
    assert_eq!(
        ScriptHash::try_from_slice(&too_long),
        Err(PrimitiveError::InvalidByteLength {
            expected: 28,
            actual: 29
        })
    );
}

#[test]
fn policy_id_converts_to_script_hash() {
    let script_hash = ScriptHash::from([9u8; 28]);
    let policy_id = PolicyId::from(script_hash);
    assert_eq!(ScriptHash::from(policy_id), script_hash);
}

#[test]
fn credential_round_trips_tagged_bytes() {
    let key = Credential::try_from_tagged_bytes(0, [1u8; 28]);
    let script = Credential::try_from_tagged_bytes(1, [2u8; 28]);

    assert_eq!(key, Ok(Credential::KeyHash(KeyHash::from([1u8; 28]))));
    assert_eq!(
        script,
        Ok(Credential::ScriptHash(ScriptHash::from([2u8; 28])))
    );
    assert_eq!(key.map(Credential::tag), Ok(0));
    assert_eq!(script.map(Credential::to_bytes), Ok([2u8; 28]));
    assert_eq!(
        Credential::try_from_tagged_bytes(2, [0u8; 28]),
        Err(PrimitiveError::InvalidCredentialTag { value: 2 })
    );
}

#[test]
fn asset_name_accepts_zero_to_thirty_two_bytes() {
    let empty = AssetName::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.as_bytes(), &[]);

    let max = [3u8; AssetName::MAX_LEN];
    let asset_name = AssetName::try_from_slice(&max);
    assert_eq!(asset_name.map(|value| value.as_bytes() == max), Ok(true));
}

#[test]
fn asset_name_rejects_more_than_thirty_two_bytes() {
    let too_long = [4u8; 33];
    assert_eq!(
        AssetName::try_from_slice(&too_long),
        Err(PrimitiveError::AssetNameTooLong {
            max: 32,
            actual: 33
        })
    );
}

#[test]
fn stable_error_categories_have_stable_labels() {
    assert_eq!(ErrorCategory::Primitive.as_str(), "primitive");
    assert_eq!(ErrorCategory::Codec.as_str(), "codec");
    assert_eq!(ErrorCategory::Address.as_str(), "address");
    assert_eq!(ErrorCategory::Ledger.as_str(), "ledger");
    assert_eq!(ErrorCategory::Script.as_str(), "script");
    assert_eq!(ErrorCategory::Governance.as_str(), "governance");
    assert_eq!(ErrorCategory::Feature.as_str(), "feature");
    assert_eq!(ErrorCategory::Resource.as_str(), "resource");
    assert_eq!(ErrorCategory::SourceLock.as_str(), "source_lock");
    assert_eq!(ErrorCategory::Verification.as_str(), "verification");
}

#[test]
fn stable_error_codes_have_categories_and_messages() {
    assert_eq!(
        ErrorCode::PrimitiveInvalidNetworkId.as_str(),
        "cardano.primitive.invalid_network_id"
    );
    assert_eq!(
        ErrorCode::CodecMalformedInput.category(),
        ErrorCategory::Codec
    );
    assert_eq!(
        ErrorCode::AddressMalformedInput.category(),
        ErrorCategory::Address
    );
    assert_eq!(
        ErrorCode::LedgerValidationFailed.category(),
        ErrorCategory::Ledger
    );
    assert_eq!(
        ErrorCode::ScriptValidationFailed.category(),
        ErrorCategory::Script
    );
    assert_eq!(
        ErrorCode::GovernanceValidationFailed.category(),
        ErrorCategory::Governance
    );
    assert_eq!(
        ErrorCode::FeatureUnsupported.category(),
        ErrorCategory::Feature
    );
    assert_eq!(
        ErrorCode::ResourceLimitExceeded.category(),
        ErrorCategory::Resource
    );
    assert_eq!(
        ErrorCode::SourceLockMismatch.category(),
        ErrorCategory::SourceLock
    );
    assert_eq!(
        ErrorCode::VerificationFailed.category(),
        ErrorCategory::Verification
    );
    assert_eq!(
        ErrorCode::PrimitiveAssetNameTooLong.message(),
        "Cardano asset name exceeds 32 bytes"
    );
}

#[test]
fn primitive_errors_expose_stable_codes_and_messages() {
    let error = PrimitiveError::InvalidByteLength {
        expected: 32,
        actual: 31,
    };

    assert_eq!(error.category(), ErrorCategory::Primitive);
    assert_eq!(error.code(), ErrorCode::PrimitiveInvalidByteLength);
    assert_eq!(error.message(), "invalid Cardano primitive byte length");
    assert_eq!(
        error.to_string(),
        "cardano.primitive.invalid_byte_length: invalid Cardano primitive byte length (expected=32, actual=31)"
    );
}

#[test]
fn primitive_error_display_keeps_non_secret_diagnostics() {
    assert_eq!(
        PrimitiveError::InvalidNetworkId { value: 5 }.to_string(),
        "cardano.primitive.invalid_network_id: invalid Cardano network id (value=5)"
    );
    assert_eq!(
        PrimitiveError::InvalidCredentialTag { value: 9 }.to_string(),
        "cardano.primitive.invalid_credential_tag: invalid Cardano credential tag (value=9)"
    );
    assert_eq!(
        PrimitiveError::AssetNameTooLong {
            max: 32,
            actual: 33
        }
        .to_string(),
        "cardano.primitive.asset_name_too_long: Cardano asset name exceeds 32 bytes (max=32, actual=33)"
    );
}

#[test]
fn asset_name_identity_uses_significant_bytes_only() {
    let clean_result = AssetName::try_from_slice(&[9]);
    assert!(clean_result.is_ok());
    let Ok(clean) = clean_result else {
        return;
    };
    let mut padded = [0u8; AssetName::MAX_LEN];
    padded[0] = 9;
    padded[1] = 99;
    let dirty_tail = AssetName {
        bytes: padded,
        len: 1,
    };

    assert_eq!(clean, dirty_tail);
    assert_eq!(clean.cmp(&dirty_tail), core::cmp::Ordering::Equal);
    assert_eq!(stable_hash(clean), stable_hash(dirty_tail));
}

fn stable_hash(value: AssetName) -> u64 {
    let mut hasher = TestHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}

#[derive(Default)]
struct TestHasher(u64);

impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = self.0.wrapping_mul(257).wrapping_add(u64::from(*byte));
        }
    }
}
