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
    let tracker = DecodeBudgetTracker::for_input(BUDGET, &[0, 1, 2, 3]);

    assert_eq!(
        tracker.as_ref().map(DecodeBudgetTracker::input_bytes),
        Ok(4)
    );
    assert_eq!(
        DecodeBudgetTracker::for_input(BUDGET, &[0, 1, 2, 3, 4]),
        Err(DecodeBudgetError::InputTooLarge { max: 4, actual: 5 })
    );
}

#[test]
fn tracker_fails_closed_on_input_byte_overflow() {
    assert_eq!(
        DecodeBudgetTracker::for_input(BUDGET, &[0, 1, 2, 3, 4]),
        Err(DecodeBudgetError::InputTooLarge { max: 4, actual: 5 })
    );
}

#[test]
fn tracker_fails_closed_on_true_integer_overflow() -> Result<(), DecodeBudgetError> {
    let budget = DecodeBudget::new(4, 2, 3, 2, usize::MAX, 3);
    let mut tracker = DecodeBudgetTracker::for_input(budget, &[])?;

    assert_eq!(tracker.account_allocation_bytes(usize::MAX), Ok(()));
    assert_eq!(
        tracker.account_allocation_bytes(1),
        Err(DecodeBudgetError::AllocationTooLarge {
            max: usize::MAX,
            actual: usize::MAX
        })
    );
    assert_eq!(tracker.allocation_bytes(), usize::MAX);
    Ok(())
}

#[test]
fn nested_guard_tracks_depth_for_scope_lifetime() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    {
        let mut nested = tracker.nested()?;
        assert_eq!(nested.nesting_depth(), 1);
        assert_eq!(nested.account_value(), Ok(()));
        {
            let deeper = nested.nested()?;
            assert_eq!(deeper.nesting_depth(), 2);
        }
        assert_eq!(nested.nesting_depth(), 1);
    }
    assert_eq!(tracker.nesting_depth(), 0);
    Ok(())
}

#[test]
fn nested_guard_rejects_excess_depth() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;
    let mut first = tracker.nested()?;
    let mut second = first.nested()?;

    assert!(matches!(
        second.nested(),
        Err(DecodeBudgetError::NestingTooDeep { max: 2, actual: 3 })
    ));
    assert_eq!(second.nesting_depth(), 2);
    Ok(())
}

#[test]
fn nested_guard_restores_depth_after_early_error() -> Result<(), DecodeBudgetError> {
    fn fail_while_nested(tracker: &mut DecodeBudgetTracker) -> Result<(), DecodeBudgetError> {
        let _nested = tracker.nested()?;
        Err(DecodeBudgetError::TooManyValues { max: 0, actual: 1 })
    }

    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    assert_eq!(
        fail_while_nested(&mut tracker),
        Err(DecodeBudgetError::TooManyValues { max: 0, actual: 1 })
    );
    assert_eq!(tracker.nesting_depth(), 0);
    Ok(())
}

#[test]
fn tracker_fails_closed_on_items() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    assert_eq!(tracker.account_items(2), Ok(()));
    assert_eq!(tracker.account_item(), Ok(()));
    assert_eq!(
        tracker.account_item(),
        Err(DecodeBudgetError::TooManyItems { max: 3, actual: 4 })
    );
    assert_eq!(tracker.items(), 3);
    Ok(())
}

#[test]
fn tracker_fails_closed_on_map_entries() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    assert_eq!(tracker.account_map_entries(2), Ok(()));
    assert_eq!(
        tracker.account_map_entries(1),
        Err(DecodeBudgetError::TooManyMapEntries { max: 2, actual: 3 })
    );
    assert_eq!(tracker.map_entries(), 2);
    Ok(())
}

#[test]
fn tracker_fails_closed_on_allocation_bytes() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    assert_eq!(tracker.account_allocation_bytes(3), Ok(()));
    assert_eq!(tracker.account_allocation_bytes(2), Ok(()));
    assert_eq!(
        tracker.account_allocation_bytes(1),
        Err(DecodeBudgetError::AllocationTooLarge { max: 5, actual: 6 })
    );
    assert_eq!(tracker.allocation_bytes(), 5);
    Ok(())
}

#[test]
fn tracker_fails_closed_on_values() -> Result<(), DecodeBudgetError> {
    let mut tracker = DecodeBudgetTracker::for_input(BUDGET, &[])?;

    assert_eq!(tracker.account_values(2), Ok(()));
    assert_eq!(tracker.account_value(), Ok(()));
    assert_eq!(
        tracker.account_value(),
        Err(DecodeBudgetError::TooManyValues { max: 3, actual: 4 })
    );
    assert_eq!(tracker.values(), 3);
    Ok(())
}

#[test]
fn error_categories_codes_and_display_are_stable() {
    let error = DecodeBudgetError::TooManyMapEntries { max: 2, actual: 3 };

    assert_eq!(
        error.category(),
        DecodeBudgetErrorCategory::ResourceExhaustion
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
