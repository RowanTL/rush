//! # Numeric Instructions
//!
//! This file contains numeric instructions for int and float.

use crate::push::state::{EMPTY_STATE, PushState};
use rust_decimal::Decimal;
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};

use super::utils::CheckedDiv;

/// Adds two addable values together.
fn _add<T>(vals: Vec<T>) -> Option<T>
where
    T: Add<Output = T> + Copy,
{
    Some(vals[1] + vals[0])
}
make_instruction!(int, int, _add, i128, 2);
make_instruction!(float, float, _add, Decimal, 2);

/// Subtracts two subtractable values from each other.
fn _sub<T>(vals: Vec<T>) -> Option<T>
where
    T: Sub<Output = T> + Copy,
{
    Some(vals[1] - vals[0])
}
make_instruction!(int, int, _sub, i128, 2);
make_instruction!(float, float, _sub, Decimal, 2);

/// Multiplies two multipliable values together.
fn _mult<T>(vals: Vec<T>) -> Option<T>
where
    T: Mul<Output = T> + Copy,
{
    Some(vals[1] * vals[0])
}
make_instruction!(int, int, _mult, i128, 2);
make_instruction!(float, float, _mult, Decimal, 2);

/// Divides two values from each other.
fn _div<T>(vals: Vec<T>) -> Option<T>
where
    T: Div<Output = T> + Copy + CheckedDiv,
{
    vals[1].checked_div(vals[0])
}
make_instruction!(int, int, _div, i128, 2);
make_instruction!(float, float, _div, Decimal, 2);

/// Takes the remainder of two values
fn _rem<T>(vals: Vec<T>) -> Option<T>
where
    T: Div<Output = T> + Copy + CheckedDiv,
{
    vals[1].checked_mod(vals[0])
}
make_instruction!(int, int, _rem, i128, 2);
make_instruction!(float, float, _rem, Decimal, 2);

/// Takes the max of two values
fn _max<T>(vals: Vec<T>) -> Option<T>
where
    T: Ord + Copy,
{
    Some(max(vals[1], vals[0]))
}

/// Takes the min of two values
fn _min<T>(vals: Vec<T>) -> Option<T>
where
    T: Ord + Copy,
{
    Some(min(vals[1], vals[0]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    /// Tests the _add function.
    #[test]
    fn add_test() {
        let vals: Vec<i64> = vec![1, 2];
        assert_eq!(Some(3), _add(vals));

        let vals: Vec<Decimal> = vec![dec!(1.1), dec!(2.2)];
        assert_eq!(Some(dec!(3.3)), _add(vals));
    }

    /// Tests the _sub function.
    #[test]
    fn sub_test() {
        let vals: Vec<i64> = vec![1, 2];
        assert_eq!(Some(1), _sub(vals));

        let vals: Vec<Decimal> = vec![dec!(1.1), dec!(2.2)];
        assert_eq!(Some(dec!(1.1)), _sub(vals));
    }

    /// Tests the _mult function.
    #[test]
    fn mult_test() {
        let vals: Vec<i128> = vec![4, 5];
        assert_eq!(Some(20), _mult(vals));

        let vals: Vec<Decimal> = vec![dec!(1.1), dec!(2.2)];
        assert_eq!(Some(dec!(2.42)), _mult(vals));
    }

    /// Tests the _div function
    #[test]
    fn div_test() {
        let vals: Vec<i128> = vec![4, 20];
        assert_eq!(Some(5), _div(vals));

        let vals: Vec<i128> = vec![3, 20];
        assert_eq!(Some(6), _div(vals));

        let vals: Vec<Decimal> = vec![dec!(1.6), dec!(2.2)];
        assert_eq!(Some(dec!(1.375)), _div(vals));

        let vals: Vec<i128> = vec![0, 1];
        assert_eq!(None, _div(vals));
    }

    /// Tests the _rem function
    #[test]
    fn rem_test() {
        let vals: Vec<i128> = vec![3, 20];
        assert_eq!(Some(2), _rem(vals));

        let vals: Vec<i128> = vec![20, 20];
        assert_eq!(Some(0), _rem(vals));

        let vals: Vec<i128> = vec![0, 9];
        assert_eq!(None, _rem(vals));
    }

    /// Tests the _max function
    #[test]
    fn max_test() {
        let vals: Vec<i128> = vec![1, 2];
        assert_eq!(Some(2), _max(vals));

        let vals: Vec<i128> = vec![3, 0];
        assert_eq!(Some(3), _max(vals));

        let vals: Vec<Decimal> = vec![dec!(2.2), dec!(1.1)];
        assert_eq!(Some(dec!(2.2)), _max(vals));

        let vals: Vec<Decimal> = vec![dec!(3.3), dec!(-1.1)];
        assert_eq!(Some(dec!(3.3)), _max(vals));
    }

    /// Tests the _min function
    #[test]
    fn min_test() {
        let vals: Vec<i128> = vec![1, 2];
        assert_eq!(Some(1), _min(vals));

        let vals: Vec<i128> = vec![3, 0];
        assert_eq!(Some(0), _min(vals));

        let vals: Vec<Decimal> = vec![dec!(2.2), dec!(1.1)];
        assert_eq!(Some(dec!(1.1)), _min(vals));

        let vals: Vec<Decimal> = vec![dec!(3.3), dec!(-1.1)];
        assert_eq!(Some(dec!(-1.1)), _min(vals));
    }

    /// Tests that the various addition functions.
    #[test]
    fn state_add() {
        let mut test_state = EMPTY_STATE;
        test_state.int = vec![1, 2];
        test_state.float = vec![dec!(1.1), dec!(2.2)];

        int_add(&mut test_state);
        assert_eq!(vec![3], test_state.int);

        float_add(&mut test_state);
        assert_eq!(vec![dec!(3.3)], test_state.float);
    }

    /// Tests the various subtraction functions.
    #[test]
    fn state_sub() {
        let mut test_state = EMPTY_STATE;
        test_state.int = vec![1, 2];
        test_state.float = vec![dec!(1.1), dec!(2.2)];

        int_sub(&mut test_state);
        assert_eq!(vec![-1], test_state.int);

        float_sub(&mut test_state);
        assert_eq!(vec![dec!(-1.1)], test_state.float);
    }

    /// Tests the various multiplication functions.
    #[test]
    fn state_mult() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0];
        int_mult(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.int = vec![10, 3, 2];
        test_state.float = vec![dec!(1.1), dec!(2.2)];

        int_mult(&mut test_state);
        assert_eq!(vec![10, 6], test_state.int);

        float_mult(&mut test_state);
        assert_eq!(vec![dec!(2.42)], test_state.float);
    }

    /// Tests the division functions in the state
    #[test]
    fn state_div() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0];
        int_div(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.int = vec![2, 0];
        int_div(&mut test_state);
        assert_eq!(vec![2, 0], test_state.int);

        test_state.int = vec![6, 3];

        int_div(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.float = vec![dec!(2.2), dec!(1.6)];
        float_div(&mut test_state);
        assert_eq!(vec![dec!(1.375)], test_state.float);
    }

    /// Tests the remainder functions in the state.
    #[test]
    fn state_rem() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0];
        int_rem(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.int = vec![2, 0];
        int_rem(&mut test_state);
        assert_eq!(vec![2, 0], test_state.int);

        test_state.int = vec![60, 80, 20, 3];
        int_rem(&mut test_state);
        assert_eq!(vec![60, 80, 2], test_state.int);

        test_state.float = vec![dec!(2.7), dec!(1.2)];
        float_rem(&mut test_state);
        assert_eq!(vec![dec!(0.3)], test_state.float);
    }
}
