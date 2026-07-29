use super::{DecodeBudget, DecodeBudgetError, DecodeBudgetErrorCategory, DecodeBudgetTracker};
use std::string::ToString;

const BUDGET: DecodeBudget = DecodeBudget::new(4, 2, 3, 2, 5, 3);

#[test]
fn budget_accessors_are_stable() {
    assert_eq!(BUDGET.max_input_bytes(), 4);
    assert_eq!(BUDGET.max_nesting_depth(), 2);
    assert_eq!(BUDGET.max_items(), 3);
    assert_eq!(BUDGET.max_map_entries(), 2);
    assert_eq!(BUDGET.max_allocation_bytes(), 5);
    assert_eq!(BUDGET.max_values(), 3);
}

#[test]
fn tracker_accounts_input_bytes_at_construction() {
    let tracker = DecodeBudgetTracker::for_input(BUDGET, 4);

    assert_eq!(tracker.map(DecodeBudgetTracker::input_bytes), Ok(4));
    assert_eq!(
        DecodeBudgetTracker::for_input(BUDGET, 5),
        Err(DecodeBudgetError::InputTooLarge { max: 4, actual: 5 })
    );
}

#[test]
fn tracker_fails_closed_on_input_byte_overflow() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.account_input_bytes(2), Ok(()));
    assert_eq!(tracker.account_input_bytes(2), Ok(()));
    assert_eq!(
        tracker.account_input_bytes(1),
        Err(DecodeBudgetError::InputTooLarge { max: 4, actual: 5 })
    );
    assert_eq!(tracker.input_bytes(), 4);
}

#[test]
fn tracker_fails_closed_on_true_integer_overflow() {
    let budget = DecodeBudget::new(usize::MAX, 2, 3, 2, 5, 3);
    let mut tracker = DecodeBudgetTracker::new(budget);

    assert_eq!(tracker.account_input_bytes(usize::MAX), Ok(()));
    assert_eq!(
        tracker.account_input_bytes(1),
        Err(DecodeBudgetError::InputTooLarge {
            max: usize::MAX,
            actual: usize::MAX
        })
    );
    assert_eq!(tracker.input_bytes(), usize::MAX);
}

#[test]
fn tracker_fails_closed_on_nesting_depth() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.enter_nested(), Ok(()));
    assert_eq!(tracker.enter_nested(), Ok(()));
    assert_eq!(
        tracker.enter_nested(),
        Err(DecodeBudgetError::NestingTooDeep { max: 2, actual: 3 })
    );
    assert_eq!(tracker.nesting_depth(), 2);
    assert_eq!(tracker.leave_nested(), Ok(()));
    assert_eq!(tracker.nesting_depth(), 1);
}

#[test]
fn tracker_rejects_nesting_underflow() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(
        tracker.leave_nested(),
        Err(DecodeBudgetError::NestingUnderflow)
    );
}

#[test]
fn tracker_fails_closed_on_items() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.account_items(2), Ok(()));
    assert_eq!(tracker.account_item(), Ok(()));
    assert_eq!(
        tracker.account_item(),
        Err(DecodeBudgetError::TooManyItems { max: 3, actual: 4 })
    );
    assert_eq!(tracker.items(), 3);
}

#[test]
fn tracker_fails_closed_on_map_entries() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.account_map_entries(2), Ok(()));
    assert_eq!(
        tracker.account_map_entries(1),
        Err(DecodeBudgetError::TooManyMapEntries { max: 2, actual: 3 })
    );
    assert_eq!(tracker.map_entries(), 2);
}

#[test]
fn tracker_fails_closed_on_allocation_bytes() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.account_allocation_bytes(3), Ok(()));
    assert_eq!(tracker.account_allocation_bytes(2), Ok(()));
    assert_eq!(
        tracker.account_allocation_bytes(1),
        Err(DecodeBudgetError::AllocationTooLarge { max: 5, actual: 6 })
    );
    assert_eq!(tracker.allocation_bytes(), 5);
}

#[test]
fn tracker_fails_closed_on_values() {
    let mut tracker = DecodeBudgetTracker::new(BUDGET);

    assert_eq!(tracker.account_values(2), Ok(()));
    assert_eq!(tracker.account_value(), Ok(()));
    assert_eq!(
        tracker.account_value(),
        Err(DecodeBudgetError::TooManyValues { max: 3, actual: 4 })
    );
    assert_eq!(tracker.values(), 3);
}

#[test]
fn error_categories_codes_and_display_are_stable() {
    let error = DecodeBudgetError::TooManyMapEntries { max: 2, actual: 3 };

    assert_eq!(
        error.category(),
        DecodeBudgetErrorCategory::ResourceExhaustion
    );
    assert_eq!(
        DecodeBudgetError::NestingUnderflow.category(),
        DecodeBudgetErrorCategory::InvalidState
    );
    assert_eq!(error.code(), "cardano.cbor.budget.too_many_map_entries");
    assert_eq!(
        error.message(),
        "Cardano CBOR map-entry count exceeds decode budget"
    );
    assert_eq!(
        error.to_string(),
        "cardano.cbor.budget.too_many_map_entries: Cardano CBOR map-entry count exceeds decode budget (max=2, actual=3)"
    );
}
