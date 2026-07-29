use core::{
    fmt,
    ops::{Deref, DerefMut},
};

/// Static resource limits for untrusted Cardano byte decoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodeBudget {
    max_input_bytes: usize,
    max_nesting_depth: usize,
    max_items: usize,
    max_map_entries: usize,
    max_allocation_bytes: usize,
    max_values: usize,
}

impl DecodeBudget {
    /// Conservative default budget for tests and explicitly reviewed callers.
    pub const DEFAULT: Self = Self::new(1_048_576, 64, 65_536, 32_768, 1_048_576, 65_536);

    /// Creates a decode budget with explicit resource ceilings.
    #[must_use]
    pub const fn new(
        max_input_bytes: usize,
        max_nesting_depth: usize,
        max_items: usize,
        max_map_entries: usize,
        max_allocation_bytes: usize,
        max_values: usize,
    ) -> Self {
        Self {
            max_input_bytes,
            max_nesting_depth,
            max_items,
            max_map_entries,
            max_allocation_bytes,
            max_values,
        }
    }

    /// Maximum admitted input bytes.
    #[must_use]
    pub const fn max_input_bytes(self) -> usize {
        self.max_input_bytes
    }

    /// Maximum nested array/map/tag/container depth.
    #[must_use]
    pub const fn max_nesting_depth(self) -> usize {
        self.max_nesting_depth
    }

    /// Maximum cumulative items admitted across decoded containers.
    #[must_use]
    pub const fn max_items(self) -> usize {
        self.max_items
    }

    /// Maximum cumulative map entries admitted across decoded maps.
    #[must_use]
    pub const fn max_map_entries(self) -> usize {
        self.max_map_entries
    }

    /// Maximum cumulative bytes requested for decoded owned output.
    #[must_use]
    pub const fn max_allocation_bytes(self) -> usize {
        self.max_allocation_bytes
    }

    /// Maximum cumulative decoded values admitted.
    #[must_use]
    pub const fn max_values(self) -> usize {
        self.max_values
    }
}

/// Stateful accounting for one untrusted decode operation.
#[derive(Debug, Eq, PartialEq)]
pub struct DecodeBudgetTracker {
    budget: DecodeBudget,
    input_bytes: usize,
    nesting_depth: usize,
    items: usize,
    map_entries: usize,
    allocation_bytes: usize,
    values: usize,
}

impl DecodeBudgetTracker {
    const fn new_internal(budget: DecodeBudget) -> Self {
        Self {
            budget,
            input_bytes: 0,
            nesting_depth: 0,
            items: 0,
            map_entries: 0,
            allocation_bytes: 0,
            values: 0,
        }
    }

    /// Creates a tracker and immediately accounts the input byte length.
    pub fn for_input(budget: DecodeBudget, input: &[u8]) -> Result<Self, DecodeBudgetError> {
        let mut tracker = Self::new_internal(budget);
        tracker.account_input_bytes(input.len())?;
        Ok(tracker)
    }

    /// Returns the configured budget.
    #[must_use]
    pub const fn budget(&self) -> DecodeBudget {
        self.budget
    }

    /// Returns accounted input bytes.
    #[must_use]
    pub const fn input_bytes(&self) -> usize {
        self.input_bytes
    }

    /// Returns the current nesting depth.
    #[must_use]
    pub const fn nesting_depth(&self) -> usize {
        self.nesting_depth
    }

    /// Returns accounted cumulative container items.
    #[must_use]
    pub const fn items(&self) -> usize {
        self.items
    }

    /// Returns accounted cumulative map entries.
    #[must_use]
    pub const fn map_entries(&self) -> usize {
        self.map_entries
    }

    /// Returns accounted cumulative allocation bytes.
    #[must_use]
    pub const fn allocation_bytes(&self) -> usize {
        self.allocation_bytes
    }

    /// Returns accounted cumulative decoded values.
    #[must_use]
    pub const fn values(&self) -> usize {
        self.values
    }

    fn account_input_bytes(&mut self, bytes: usize) -> Result<(), DecodeBudgetError> {
        self.input_bytes = checked_total(
            BudgetField::InputBytes,
            self.input_bytes,
            bytes,
            self.budget.max_input_bytes,
        )?;
        Ok(())
    }

    /// Enters one nested value scope for the lifetime of the returned guard.
    pub fn nested(&mut self) -> Result<NestedBudget<'_>, DecodeBudgetError> {
        self.nesting_depth = checked_total(
            BudgetField::NestingDepth,
            self.nesting_depth,
            1,
            self.budget.max_nesting_depth,
        )?;
        Ok(NestedBudget { tracker: self })
    }

    /// Accounts cumulative decoded container items.
    pub fn account_items(&mut self, items: usize) -> Result<(), DecodeBudgetError> {
        self.items = checked_total(BudgetField::Items, self.items, items, self.budget.max_items)?;
        Ok(())
    }

    /// Accounts one decoded container item.
    pub fn account_item(&mut self) -> Result<(), DecodeBudgetError> {
        self.account_items(1)
    }

    /// Accounts cumulative decoded map entries.
    pub fn account_map_entries(&mut self, entries: usize) -> Result<(), DecodeBudgetError> {
        self.map_entries = checked_total(
            BudgetField::MapEntries,
            self.map_entries,
            entries,
            self.budget.max_map_entries,
        )?;
        Ok(())
    }

    /// Accounts cumulative owned-output allocation bytes.
    pub fn account_allocation_bytes(&mut self, bytes: usize) -> Result<(), DecodeBudgetError> {
        self.allocation_bytes = checked_total(
            BudgetField::AllocationBytes,
            self.allocation_bytes,
            bytes,
            self.budget.max_allocation_bytes,
        )?;
        Ok(())
    }

    /// Accounts cumulative decoded values.
    pub fn account_values(&mut self, values: usize) -> Result<(), DecodeBudgetError> {
        self.values = checked_total(
            BudgetField::Values,
            self.values,
            values,
            self.budget.max_values,
        )?;
        Ok(())
    }

    /// Accounts one decoded value.
    pub fn account_value(&mut self) -> Result<(), DecodeBudgetError> {
        self.account_values(1)
    }
}

/// Active nested decode-budget scope.
#[must_use = "the nesting guard must be held for the entire nested decode"]
#[derive(Debug)]
pub struct NestedBudget<'a> {
    tracker: &'a mut DecodeBudgetTracker,
}

impl Deref for NestedBudget<'_> {
    type Target = DecodeBudgetTracker;

    fn deref(&self) -> &Self::Target {
        self.tracker
    }
}

impl DerefMut for NestedBudget<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tracker
    }
}

impl Drop for NestedBudget<'_> {
    fn drop(&mut self) {
        self.tracker.nesting_depth = self.tracker.nesting_depth.saturating_sub(1);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BudgetField {
    InputBytes,
    NestingDepth,
    Items,
    MapEntries,
    AllocationBytes,
    Values,
}

fn checked_total(
    field: BudgetField,
    current: usize,
    amount: usize,
    max: usize,
) -> Result<usize, DecodeBudgetError> {
    let Some(actual) = current.checked_add(amount) else {
        return Err(error_for(field, max, usize::MAX));
    };
    if actual > max {
        return Err(error_for(field, max, actual));
    }
    Ok(actual)
}

fn error_for(field: BudgetField, max: usize, actual: usize) -> DecodeBudgetError {
    match field {
        BudgetField::InputBytes => DecodeBudgetError::InputTooLarge { max, actual },
        BudgetField::NestingDepth => DecodeBudgetError::NestingTooDeep { max, actual },
        BudgetField::Items => DecodeBudgetError::TooManyItems { max, actual },
        BudgetField::MapEntries => DecodeBudgetError::TooManyMapEntries { max, actual },
        BudgetField::AllocationBytes => DecodeBudgetError::AllocationTooLarge { max, actual },
        BudgetField::Values => DecodeBudgetError::TooManyValues { max, actual },
    }
}

/// Stable high-level decode-budget error category.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DecodeBudgetErrorCategory {
    /// The operation exceeded an explicit resource limit.
    ResourceExhaustion,
}

/// Decode-budget accounting failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeBudgetError {
    /// Input bytes exceeded the configured limit.
    InputTooLarge {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
    /// Nesting depth exceeded the configured limit.
    NestingTooDeep {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
    /// Item count exceeded the configured limit.
    TooManyItems {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
    /// Map-entry count exceeded the configured limit.
    TooManyMapEntries {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
    /// Allocation bytes exceeded the configured limit.
    AllocationTooLarge {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
    /// Decoded value count exceeded the configured limit.
    TooManyValues {
        /// Configured maximum.
        max: usize,
        /// Actual attempted total.
        actual: usize,
    },
}

impl DecodeBudgetError {
    /// Returns the stable high-level error category.
    #[must_use]
    pub const fn category(self) -> DecodeBudgetErrorCategory {
        match self {
            Self::InputTooLarge { .. }
            | Self::NestingTooDeep { .. }
            | Self::TooManyItems { .. }
            | Self::TooManyMapEntries { .. }
            | Self::AllocationTooLarge { .. }
            | Self::TooManyValues { .. } => DecodeBudgetErrorCategory::ResourceExhaustion,
        }
    }

    /// Returns the stable machine-readable error code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InputTooLarge { .. } => "cardano.cbor.budget.input_too_large",
            Self::NestingTooDeep { .. } => "cardano.cbor.budget.nesting_too_deep",
            Self::TooManyItems { .. } => "cardano.cbor.budget.too_many_items",
            Self::TooManyMapEntries { .. } => "cardano.cbor.budget.too_many_map_entries",
            Self::AllocationTooLarge { .. } => "cardano.cbor.budget.allocation_too_large",
            Self::TooManyValues { .. } => "cardano.cbor.budget.too_many_values",
        }
    }

    /// Returns the stable human-readable message.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::InputTooLarge { .. } => "Cardano CBOR input exceeds decode budget",
            Self::NestingTooDeep { .. } => "Cardano CBOR nesting exceeds decode budget",
            Self::TooManyItems { .. } => "Cardano CBOR item count exceeds decode budget",
            Self::TooManyMapEntries { .. } => "Cardano CBOR map-entry count exceeds decode budget",
            Self::AllocationTooLarge { .. } => "Cardano CBOR allocation bytes exceed decode budget",
            Self::TooManyValues { .. } => "Cardano CBOR value count exceeds decode budget",
        }
    }
}

impl fmt::Display for DecodeBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())?;
        match self {
            Self::InputTooLarge { max, actual }
            | Self::NestingTooDeep { max, actual }
            | Self::TooManyItems { max, actual }
            | Self::TooManyMapEntries { max, actual }
            | Self::AllocationTooLarge { max, actual }
            | Self::TooManyValues { max, actual } => {
                write!(f, " (max={max}, actual={actual})")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeBudgetError {}
