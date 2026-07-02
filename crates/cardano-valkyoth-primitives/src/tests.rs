use super::{
    AssetName, BlockHash, BlockNumber, Coin, Credential, DatumHash, Epoch, Era, KeyHash, NetworkId,
    PolicyId, PrimitiveError, ScriptHash, Slot, TransactionId,
};

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
