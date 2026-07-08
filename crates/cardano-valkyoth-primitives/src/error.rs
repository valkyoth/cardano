use core::fmt;

/// Stable high-level error category for Cardano public APIs.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorCategory {
    /// Primitive domain construction failed.
    Primitive,
    /// Codec or wire-format operation failed.
    Codec,
    /// Address operation failed.
    Address,
    /// Ledger validation or state operation failed.
    Ledger,
    /// Script or Plutus-data operation failed.
    Script,
    /// Governance operation failed.
    Governance,
    /// Requested feature is not supported by the current crate configuration.
    Feature,
    /// Operation exceeded an explicit resource limit.
    Resource,
    /// Pinned source or spec-lock evidence is missing or mismatched.
    SourceLock,
    /// Cryptographic or conformance verification failed.
    Verification,
}

impl ErrorCategory {
    /// Returns the stable category label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primitive => "primitive",
            Self::Codec => "codec",
            Self::Address => "address",
            Self::Ledger => "ledger",
            Self::Script => "script",
            Self::Governance => "governance",
            Self::Feature => "feature",
            Self::Resource => "resource",
            Self::SourceLock => "source_lock",
            Self::Verification => "verification",
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable error code for Cardano public APIs.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorCode {
    /// Network id is not one of the admitted Cardano network ids.
    PrimitiveInvalidNetworkId,
    /// Credential tag is not one of the admitted credential tags.
    PrimitiveInvalidCredentialTag,
    /// Fixed-width primitive bytes used the wrong length.
    PrimitiveInvalidByteLength,
    /// Asset name exceeded the Cardano maximum length.
    PrimitiveAssetNameTooLong,
    /// Generic malformed codec input code reserved for codec milestones.
    CodecMalformedInput,
    /// Generic malformed address input code reserved for address milestones.
    AddressMalformedInput,
    /// Generic ledger validation failure code reserved for ledger milestones.
    LedgerValidationFailed,
    /// Generic script validation failure code reserved for script milestones.
    ScriptValidationFailed,
    /// Generic governance validation failure code reserved for governance milestones.
    GovernanceValidationFailed,
    /// Generic unsupported feature code reserved for optional-feature milestones.
    FeatureUnsupported,
    /// Generic resource exhaustion code reserved for bounded parser milestones.
    ResourceLimitExceeded,
    /// Generic source-lock mismatch code reserved for spec evidence gates.
    SourceLockMismatch,
    /// Generic verification failure code reserved for cryptographic checks.
    VerificationFailed,
}

impl ErrorCode {
    /// Returns the stable high-level category for this code.
    #[must_use]
    pub const fn category(self) -> ErrorCategory {
        match self {
            Self::PrimitiveInvalidNetworkId
            | Self::PrimitiveInvalidCredentialTag
            | Self::PrimitiveInvalidByteLength
            | Self::PrimitiveAssetNameTooLong => ErrorCategory::Primitive,
            Self::CodecMalformedInput => ErrorCategory::Codec,
            Self::AddressMalformedInput => ErrorCategory::Address,
            Self::LedgerValidationFailed => ErrorCategory::Ledger,
            Self::ScriptValidationFailed => ErrorCategory::Script,
            Self::GovernanceValidationFailed => ErrorCategory::Governance,
            Self::FeatureUnsupported => ErrorCategory::Feature,
            Self::ResourceLimitExceeded => ErrorCategory::Resource,
            Self::SourceLockMismatch => ErrorCategory::SourceLock,
            Self::VerificationFailed => ErrorCategory::Verification,
        }
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimitiveInvalidNetworkId => "cardano.primitive.invalid_network_id",
            Self::PrimitiveInvalidCredentialTag => "cardano.primitive.invalid_credential_tag",
            Self::PrimitiveInvalidByteLength => "cardano.primitive.invalid_byte_length",
            Self::PrimitiveAssetNameTooLong => "cardano.primitive.asset_name_too_long",
            Self::CodecMalformedInput => "cardano.codec.malformed_input",
            Self::AddressMalformedInput => "cardano.address.malformed_input",
            Self::LedgerValidationFailed => "cardano.ledger.validation_failed",
            Self::ScriptValidationFailed => "cardano.script.validation_failed",
            Self::GovernanceValidationFailed => "cardano.governance.validation_failed",
            Self::FeatureUnsupported => "cardano.feature.unsupported",
            Self::ResourceLimitExceeded => "cardano.resource.limit_exceeded",
            Self::SourceLockMismatch => "cardano.source_lock.mismatch",
            Self::VerificationFailed => "cardano.verification.failed",
        }
    }

    /// Returns the stable human-readable message for this code.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::PrimitiveInvalidNetworkId => "invalid Cardano network id",
            Self::PrimitiveInvalidCredentialTag => "invalid Cardano credential tag",
            Self::PrimitiveInvalidByteLength => "invalid Cardano primitive byte length",
            Self::PrimitiveAssetNameTooLong => "Cardano asset name exceeds 32 bytes",
            Self::CodecMalformedInput => "malformed Cardano codec input",
            Self::AddressMalformedInput => "malformed Cardano address input",
            Self::LedgerValidationFailed => "Cardano ledger validation failed",
            Self::ScriptValidationFailed => "Cardano script validation failed",
            Self::GovernanceValidationFailed => "Cardano governance validation failed",
            Self::FeatureUnsupported => "Cardano feature is not supported",
            Self::ResourceLimitExceeded => "Cardano operation exceeded resource limits",
            Self::SourceLockMismatch => "Cardano source-lock evidence mismatch",
            Self::VerificationFailed => "Cardano verification failed",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
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

impl PrimitiveError {
    /// Returns the stable high-level category.
    #[must_use]
    pub const fn category(self) -> ErrorCategory {
        self.code().category()
    }

    /// Returns the stable machine-readable error code.
    #[must_use]
    pub const fn code(self) -> ErrorCode {
        match self {
            Self::InvalidNetworkId { .. } => ErrorCode::PrimitiveInvalidNetworkId,
            Self::InvalidCredentialTag { .. } => ErrorCode::PrimitiveInvalidCredentialTag,
            Self::InvalidByteLength { .. } => ErrorCode::PrimitiveInvalidByteLength,
            Self::AssetNameTooLong { .. } => ErrorCode::PrimitiveAssetNameTooLong,
        }
    }

    /// Returns the stable human-readable message.
    #[must_use]
    pub const fn message(self) -> &'static str {
        self.code().message()
    }
}

impl fmt::Display for PrimitiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())?;
        match self {
            Self::InvalidNetworkId { value } => write!(f, " (value={value})"),
            Self::InvalidCredentialTag { value } => write!(f, " (value={value})"),
            Self::InvalidByteLength { expected, actual } => {
                write!(f, " (expected={expected}, actual={actual})")
            }
            Self::AssetNameTooLong { max, actual } => {
                write!(f, " (max={max}, actual={actual})")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PrimitiveError {}
