//! # Logical Instructions
//!
//! This file holds instructions for the boolean stack.

use super::utils::{CastingTrait, LogicalTrait};
use crate::push::state::PushState;
use rust_decimal::Decimal;

/// Runs logical and on two values
fn _and<T>(a: T, b: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(b.logical_and(a))
}

/// Runs logical or on two values
fn _or<T>(a: T, b: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(b.logical_or(a))
}

/// Runs logical not on two values
fn _not<T>(a: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(a.logical_not())
}

/// Runs logical xor on two values
fn _xor<T>(a: T, b: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(b.logical_xor(a))
}

/// Inverts the first value and runs logical and on two values
fn _invert_first_then_and<T>(a: T, b: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(a.logical_not().logical_and(b))
}

/// Inverts the second value and runs logical and on two values
fn _invert_second_then_and<T>(a: T, b: T) -> Option<T>
where
    T: LogicalTrait,
{
    Some(a.logical_and(b.logical_not()))
}

fn _from_int<T>(a: i128) -> Option<T>
where
    T: CastingTrait,
{
    T::from_int(a)
}

fn _from_float<T>(a: Decimal) -> Option<T>
where
    T: CastingTrait,
{
    T::from_float(a)
}

macro_rules! make_logical_instructions {
    ($stack:ident) => {
        make_instruction_new!(_and, $stack, $stack, $stack, $stack);
        make_instruction_new!(_or, $stack, $stack, $stack, $stack);
        make_instruction_new!(_not, $stack, $stack, $stack);
        make_instruction_new!(_xor, $stack, $stack, $stack, $stack);
        make_instruction_new!(_invert_first_then_and, $stack, $stack, $stack, $stack);
        make_instruction_new!(_invert_second_then_and, $stack, $stack, $stack, $stack);
        make_instruction_new!(_from_int, $stack, $stack, int);
        make_instruction_new!(_from_float, $stack, $stack, float);
    };
}

macro_rules! all_logical_instructions {
    () => {
        make_logical_instructions!(boolean);
    };
}
all_logical_instructions!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;
    use rust_decimal::dec;

    #[test]
    fn and_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![true, false, true];
        boolean_and(&mut test_state);
        assert_eq!(vec![true, false], test_state.boolean);

        test_state.boolean = vec![true, true];
        boolean_and(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn or_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![true, false, true];
        boolean_or(&mut test_state);
        assert_eq!(vec![true, true], test_state.boolean);

        test_state.boolean = vec![false, false];
        boolean_or(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn not_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![true, false, true];
        boolean_not(&mut test_state);
        assert_eq!(vec![true, false, false], test_state.boolean);

        test_state.boolean = vec![false, false];
        boolean_not(&mut test_state);
        assert_eq!(vec![false, true], test_state.boolean);
    }

    #[test]
    fn xor_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![true, false, true];
        boolean_xor(&mut test_state);
        assert_eq!(vec![true, true], test_state.boolean);

        test_state.boolean = vec![false, false];
        boolean_xor(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.boolean = vec![true, true];
        boolean_xor(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn invert_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![true, false];
        boolean_invert_first_then_and(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.boolean = vec![false, false];
        boolean_invert_first_then_and(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.boolean = vec![true, false];
        boolean_invert_second_then_and(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.boolean = vec![false, true];
        boolean_invert_second_then_and(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn cast_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1];
        boolean_from_int(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.int = vec![0];
        boolean_from_int(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.float = vec![dec!(2.0)];
        boolean_from_float(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.float = vec![dec!(0.0)];
        boolean_from_float(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }
}
