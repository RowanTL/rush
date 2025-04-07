//! # Logical Instructions
//!
//! This file holds instructions for the boolean stack.

use super::utils::{CastingTrait, LogicalTrait};
use crate::push::state::PushState;
use rust_decimal::Decimal;

/// Runs logical and on two values
fn _and<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_and(vals[1]))
}
make_instruction!(boolean, boolean, _and, bool, 2);

/// Runs logical or on two values
fn _or<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_or(vals[1]))
}
make_instruction!(boolean, boolean, _or, bool, 2);

/// Runs logical not on two values
fn _not<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_not())
}
make_instruction!(boolean, boolean, _not, bool, 1);

/// Runs logical xor on two values
fn _xor<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_xor(vals[1]))
}
make_instruction!(boolean, boolean, _xor, bool, 2);

/// Inverts the first value and runs logical and on two values
fn _invert_first_then_and<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_not().logical_and(vals[1]))
}
make_instruction!(boolean, boolean, _invert_first_then_and, bool, 2);

/// Inverts the second value and runs logical and on two values
fn _invert_second_then_and<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + LogicalTrait,
{
    Some(vals[0].logical_and(vals[1].logical_not()))
}
make_instruction!(boolean, boolean, _invert_second_then_and, bool, 2);

fn _from_int<T>(vals: Vec<i128>) -> Option<T>
where
    T: Copy + CastingTrait,
{
    T::from_int(vals[0])
}
make_instruction_out!(int, boolean, _from_int, i128, 1);

fn _from_float<T>(vals: Vec<Decimal>) -> Option<T>
where
    T: Copy + CastingTrait,
{
    T::from_float(vals[0])
}
make_instruction_out!(float, boolean, _from_float, Decimal, 1);

pub fn boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        boolean_and,
        boolean_or,
        boolean_not,
        boolean_xor,
        boolean_invert_first_then_and,
        boolean_invert_second_then_and,
        boolean_from_int,
        boolean_from_float,
    ]
}

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
