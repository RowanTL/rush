//! # Numeric Instructions
//!
//! This file contains numeric instructions for int and float.

// There has to be a better way to declare these functions.
// Just don't know enough Rust yet ig.

use crate::push::state::PushState;
use num_traits::ops::checked::CheckedMul;
use rust_decimal::Decimal;
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};

use super::utils::{CastingTrait, NumericTrait};

/// Adds two values together.
fn _add<T>(a: T, b: T) -> Option<T>
where
    T: Add<Output = T>,
{
    Some(b + a)
}

/// Subtracts two values from each other.
fn _sub<T>(a: T, b: T) -> Option<T>
where
    T: Sub<Output = T>,
{
    Some(b - a)
}

/// Multiplies two values with each other.
fn _mult<T>(a: T, b: T) -> Option<T>
where
    T: CheckedMul + Mul<Output = T> + Copy,
{
    // This is different from the numeric trait checked operators.
    b.checked_mul(&a)
}

/// Divides two values from each other.
fn _div<T>(a: T, b: T) -> Option<T>
where
    T: Div<Output = T> + NumericTrait,
{
    b.checked_div(a)
}

/// Takes the remainder of two values
fn _rem<T>(a: T, b: T) -> Option<T>
where
    T: Div<Output = T> + Copy + NumericTrait,
{
    b.checked_mod(a)
}

/// Takes the max of two values
fn _max<T>(a: T, b: T) -> Option<T>
where
    T: Ord,
{
    Some(max(a, b))
}

/// Takes the min of two values
fn _min<T>(a: T, b: T) -> Option<T>
where
    T: Ord,
{
    Some(min(a, b))
}

/// Increments a single value by 1
fn _inc<T>(a: T) -> Option<T>
where
    T: NumericTrait + Copy,
{
    Some(a.increment())
}

/// Decrements a single value by 1
fn _dec<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    Some(a.decrement())
}

/// Checks if the 2nd to top value is less than the top value
fn _lt<T>(a: T, b: T) -> Option<bool>
where
    T: Ord,
{
    Some(b < a)
}

/// Checks if the 2nd to top value is greater than the top value
fn _gt<T>(a: T, b: T) -> Option<bool>
where
    T: Ord,
{
    Some(b > a)
}

/// Checks if the 2nd to top value is less than or equal to the top value
fn _lte<T>(a: T, b: T) -> Option<bool>
where
    T: Ord + Copy,
{
    Some(b <= a)
}

/// Checks if the 2nd to top value is greater than or equal to the top value
fn _gte<T>(a: T, b: T) -> Option<bool>
where
    T: Ord,
{
    Some(b >= a)
}

/// Runs sin on a single item.
fn _sin<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_sin()
}

/// Runs arcsin on a single item.
fn _arcsin<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_sin()?.inverse()
}

/// Runs cos on a single item.
fn _cos<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_cos()
}

/// Runs arcsin on a single item.
fn _arccos<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_cos()?.inverse()
}

/// Runs tan on a single item.
fn _tan<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_tan()
}

/// Runs arctan on a single item.
fn _arctan<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_tan()?.inverse()
}

/// Converts a single value from an int to an arbitrary type.
fn _from_int<T>(a: i128) -> Option<T>
where
    T: CastingTrait,
{
    T::from_int(a)
}

/// Converts a single value from a float to an arbitrary type.
fn _from_float<T>(a: Decimal) -> Option<T>
where
    T: CastingTrait,
{
    T::from_float(a)
}

/// Converts a bool to a new type.
fn _from_boolean<T>(a: bool) -> Option<T>
where
    T: CastingTrait,
{
    T::from_bool(a)
}

/// Takes log base 10 of a single Decimal. Acts as a
/// NoOp if the value is 0. If the value is negative, takes
/// the absolute value of the number.
fn _log<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.absolute().safe_log10()
}

/// Takes the exp of a single value. Ints get truncated.
fn _exp<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_exp()
}

/// Takes the square root of the absolute value of a single value.
fn _sqrt<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.safe_sqrt()
}

/// Takes the inverse of a single value. If the number is 0,
/// does nothing (returns None). Truncates an int to 0.
fn _inv<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    a.inverse()
}

/// Takes the absolute value of the top number
fn _abs<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    Some(a.absolute())
}

/// Reverses the sign of the top number
fn _sign_reverse<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    Some(a.sign_reverse())
}

/// Squares the top number
fn _square<T>(a: T) -> Option<T>
where
    T: NumericTrait,
{
    Some(a.square())
}

macro_rules! make_numeric_instructions {
    ($stack:ident) => {
        make_instruction_new!(_add, $stack, $stack, $stack, $stack);
        make_instruction_new!(_sub, $stack, $stack, $stack, $stack);
        make_instruction_new!(_mult, $stack, $stack, $stack, $stack);
        make_instruction_new!(_div, $stack, $stack, $stack, $stack);
        make_instruction_new!(_rem, $stack, $stack, $stack, $stack);
        make_instruction_new!(_max, $stack, $stack, $stack, $stack);
        make_instruction_new!(_min, $stack, $stack, $stack, $stack);
        make_instruction_new!(_inc, $stack, $stack, $stack);
        make_instruction_new!(_dec, $stack, $stack, $stack);
        make_instruction_new!(_lt, $stack, boolean, $stack, $stack);
        make_instruction_new!(_gt, $stack, boolean, $stack, $stack);
        make_instruction_new!(_lte, $stack, boolean, $stack, $stack);
        make_instruction_new!(_gte, $stack, boolean, $stack, $stack);
        make_instruction_new!(_sin, $stack, $stack, $stack);
        make_instruction_new!(_arcsin, $stack, $stack, $stack);
        make_instruction_new!(_cos, $stack, $stack, $stack);
        make_instruction_new!(_arccos, $stack, $stack, $stack);
        make_instruction_new!(_tan, $stack, $stack, $stack);
        make_instruction_new!(_arctan, $stack, $stack, $stack);
        make_instruction_new!(_from_boolean, $stack, $stack, boolean);
        make_instruction_new!(_log, $stack, $stack, $stack);
        make_instruction_new!(_exp, $stack, $stack, $stack);
        make_instruction_new!(_sqrt, $stack, $stack, $stack);
        make_instruction_new!(_inv, $stack, $stack, $stack);
        make_instruction_new!(_abs, $stack, $stack, $stack);
        make_instruction_new!(_sign_reverse, $stack, $stack, $stack);
        make_instruction_new!(_square, $stack, $stack, $stack);
    };
}

macro_rules! all_numeric_instructions {
    () => {
        make_numeric_instructions!(int);
        make_numeric_instructions!(float);
        make_instruction_new!(_from_int, float, float, int);
        make_instruction_new!(_from_float, int, int, float);
    };
}
all_numeric_instructions!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;
    use rust_decimal::dec;

    /// Tests the _add function.
    #[test]
    fn add_test() {
        assert_eq!(Some(3), _add(1, 2));
        assert_eq!(Some(dec!(3.3)), _add(dec!(1.1), dec!(2.2)));
    }

    /// Tests the _sub function.
    #[test]
    fn sub_test() {
        assert_eq!(Some(1), _sub(1, 2));
        assert_eq!(Some(dec!(1.1)), _sub(dec!(1.1), dec!(2.2)));
    }

    /// Tests the _mult function.
    #[test]
    fn mult_test() {
        assert_eq!(Some(20), _mult(5, 4));
        assert_eq!(Some(dec!(2.42)), _mult(dec!(2.2), dec!(1.1)));
    }

    /// Tests the _div function
    #[test]
    fn div_test() {
        assert_eq!(Some(5), _div(4, 20));
        assert_eq!(Some(6), _div(3, 20));
        assert_eq!(Some(dec!(1.375)), _div(dec!(1.6), dec!(2.2)));
        assert_eq!(None, _div(0, 1));
    }

    /// Tests the _rem function
    #[test]
    fn rem_test() {
        assert_eq!(Some(2), _rem(3, 20));
        assert_eq!(Some(0), _rem(20, 20));
        assert_eq!(None, _rem(0, 9));
    }

    /// Tests the _max function
    #[test]
    fn max_test() {
        assert_eq!(Some(2), _max(1, 2));
        assert_eq!(Some(3), _max(3, 0));
        assert_eq!(Some(dec!(2.2)), _max(dec!(2.2), dec!(1.1)));
        assert_eq!(Some(dec!(3.3)), _max(dec!(3.3), dec!(-1.1)));
    }

    /// Tests the _min function
    #[test]
    fn min_test() {
        assert_eq!(Some(1), _min(1, 2));
        assert_eq!(Some(0), _min(3, 0));
        assert_eq!(Some(dec!(1.1)), _min(dec!(2.2), dec!(1.1)));
        assert_eq!(Some(dec!(-1.1)), _min(dec!(3.3), dec!(-1.1)));
    }

    /// Tests the _inc and _dec functions
    #[test]
    fn inc_dec_test() {
        assert_eq!(Some(3), _inc(2));
        assert_eq!(Some(9), _dec(10));
        assert_eq!(Some(dec!(3.2)), _inc(dec!(2.2)));
        assert_eq!(Some(dec!(4.6)), _dec(dec!(5.6)));
    }

    /// Tests the _lt, _gt, _lte, and _gte functions
    #[test]
    fn lt_gt_lte_gte_test() {
        assert_eq!(Some(true), _lt(3, 2));
        assert_eq!(Some(false), _lt(1, 4));
        assert_eq!(Some(false), _lt(3, 3));
        assert_eq!(Some(true), _gt(2, 3));
        assert_eq!(Some(false), _gt(4, 1));
        assert_eq!(Some(false), _gt(3, 3));
        assert_eq!(Some(true), _lte(3, 2));
        assert_eq!(Some(false), _lte(1, 4));
        assert_eq!(Some(true), _lte(3, 3));
        assert_eq!(Some(true), _gte(2, 3));
        assert_eq!(Some(false), _gte(4, 1));
        assert_eq!(Some(true), _gte(3, 3));
    }

    /// Tests the various trig functions.
    #[test]
    fn trig_tests() {
        assert_eq!(Some(dec!(0.0)), _sin(Decimal::PI));
        assert_eq!(
            Some(dec!(1.4142135623869512272301701717)),
            _arcsin(Decimal::QUARTER_PI)
        );
        assert_eq!(Some(dec!(-1.0)), _cos(Decimal::PI));
        assert_eq!(Some(dec!(-1.0)), _arccos(Decimal::PI));
        assert_eq!(Some(dec!(0.0)), _tan(Decimal::PI));
        assert_eq!(
            Some(dec!(1.0000000043184676055890307049)),
            _arctan(Decimal::QUARTER_PI)
        );
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

    /// Tests the min and max functions in the state
    #[test]
    fn state_min_max() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0];
        int_max(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.int = vec![0];
        int_min(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.int = vec![1, 2, 3];
        int_max(&mut test_state);
        assert_eq!(vec![1, 3], test_state.int);

        test_state.int = vec![1, 2, 3];
        int_min(&mut test_state);
        assert_eq!(vec![1, 2], test_state.int);

        test_state.float = vec![dec!(1.2), dec!(4.6)];
        float_max(&mut test_state);
        assert_eq!(vec![dec!(4.6)], test_state.float);

        test_state.float = vec![dec!(0.0), dec!(1.2), dec!(4.6)];
        float_min(&mut test_state);
        assert_eq!(vec![dec!(0.0), dec!(1.2)], test_state.float);
    }

    /// Tests the inc and dec functions in the state
    #[test]
    fn state_inc_dec() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![];
        int_inc(&mut test_state);
        let empty_vec: Vec<i128> = vec![];
        assert_eq!(empty_vec, test_state.int);

        drop(empty_vec);

        test_state.int = vec![-2, 1];
        int_inc(&mut test_state);
        assert_eq!(vec![-2, 2], test_state.int);

        test_state.int = vec![-2, 1];
        int_dec(&mut test_state);
        assert_eq!(vec![-2, 0], test_state.int);

        test_state.float = vec![dec!(1.1)];
        float_inc(&mut test_state);
        assert_eq!(vec![dec!(2.1)], test_state.float);

        test_state.float = vec![dec!(1.1)];
        float_dec(&mut test_state);
        assert_eq!(vec![dec!(0.1)], test_state.float);
    }

    /// Tests the lt, gt, lte, gte functions in the state
    #[test]
    fn state_lt_gt_lte_gte() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![2, 3];
        int_lt(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.int = vec![4, 1];
        test_state.boolean = vec![];
        int_lt(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![3, 3];
        test_state.boolean = vec![];
        int_lt(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![3, 2];
        test_state.boolean = vec![];
        int_gt(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.int = vec![1, 4];
        test_state.boolean = vec![];
        int_gt(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![3, 3];
        test_state.boolean = vec![];
        int_gt(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![2, 3];
        test_state.boolean = vec![];
        int_lte(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.int = vec![4, 1];
        test_state.boolean = vec![];
        int_lte(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![3, 3];
        test_state.boolean = vec![];
        int_lte(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.int = vec![3, 2];
        test_state.boolean = vec![];
        int_gte(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);

        test_state.int = vec![1, 4];
        test_state.boolean = vec![];
        int_gte(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);

        test_state.int = vec![3, 3];
        test_state.boolean = vec![];
        int_gte(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    /// Tests the various trig functions when they should revert.
    #[test]
    fn state_trig() {
        let mut test_state = EMPTY_STATE;

        test_state.float = vec![Decimal::HALF_PI];
        float_tan(&mut test_state);
        assert_eq!(vec![Decimal::HALF_PI], test_state.float);

        test_state.float = vec![Decimal::HALF_PI];
        float_arccos(&mut test_state);
        assert_eq!(vec![Decimal::HALF_PI], test_state.float);

        test_state.float = vec![dec!(3.4), Decimal::PI];
        float_arcsin(&mut test_state);
        assert_eq!(vec![dec!(3.4), Decimal::PI], test_state.float);
    }

    /// Tests the int and float casting functions
    #[test]
    fn state_cast() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0, 1];
        float_from_int(&mut test_state);
        assert_eq!(vec![dec!(1.0)], test_state.float);
        test_state.int.clear();

        test_state.float = vec![dec!(2.1)];
        int_from_float(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.float.clear();
        test_state.int.clear();

        test_state.boolean = vec![true];
        int_from_boolean(&mut test_state);
        assert_eq!(vec![1], test_state.int);
        test_state.boolean.clear();
    }

    /// Tests the log function
    #[test]
    fn state_log() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1];
        int_log(&mut test_state);
        assert_eq!(vec![0], test_state.int);
        test_state.int.clear();

        test_state.float = vec![dec!(2)];
        float_log(&mut test_state);
        assert_eq!(vec![dec!(0.3010299956639811952137388949)], test_state.float);
        test_state.float.clear();

        test_state.int = vec![6, 7, 0];
        int_log(&mut test_state);
        assert_eq!(vec![6, 7, 0], test_state.int);

        test_state.float = vec![dec!(-4.5)];
        float_log(&mut test_state);
        assert_eq!(vec![dec!(0.6532125137753436793763169119)], test_state.float);
    }

    /// Tests the exp function
    #[test]
    fn state_exp() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![0];
        int_exp(&mut test_state);
        assert_eq!(vec![1], test_state.int);

        test_state.int = vec![0, 2];
        int_exp(&mut test_state);
        assert_eq!(vec![0, 7], test_state.int);

        test_state.int.clear();
        test_state.float = vec![dec!(1.2)];
        float_exp(&mut test_state);
        assert_eq!(vec![dec!(3.3201169022444051948051948052)], test_state.float);
    }

    /// Tests the sqrt function
    #[test]
    fn state_sqrt() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![4];
        int_sqrt(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.int = vec![5];
        int_sqrt(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.float = vec![dec!(4.84)];
        float_sqrt(&mut test_state);
        assert_eq!(vec![dec!(2.2)], test_state.float);

        test_state.int = vec![-1];
        int_sqrt(&mut test_state);
        assert_eq!(vec![1], test_state.int);

        test_state.float = vec![dec!(-1.0)];
        float_sqrt(&mut test_state);
        assert_eq!(vec![dec!(1.0)], test_state.float);
    }

    /// Tests the inv function
    #[test]
    fn state_inv() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![-1, 10];
        int_inv(&mut test_state);
        assert_eq!(vec![-1, 0], test_state.int);

        test_state.float = vec![dec!(-10)];
        float_inv(&mut test_state);
        assert_eq!(vec![dec!(-0.1)], test_state.float);

        test_state.int = vec![0];
        int_inv(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    /// Tests the abs function
    #[test]
    fn state_abs() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![-1];
        int_abs(&mut test_state);
        assert_eq!(vec![1], test_state.int);

        test_state.float = vec![dec!(-2.7)];
        float_abs(&mut test_state);
        assert_eq!(vec![dec!(2.7)], test_state.float);
    }

    /// Tests the sign reverse function
    #[test]
    fn state_sign_reverse() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![-2];
        int_sign_reverse(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.int = vec![3];
        int_sign_reverse(&mut test_state);
        assert_eq!(vec![-3], test_state.int);

        test_state.float = vec![dec!(3.0), dec!(-2.0)];
        float_sign_reverse(&mut test_state);
        assert_eq!(vec![dec!(3.0), dec!(2.0)], test_state.float);

        test_state.float = vec![dec!(3.0)];
        float_sign_reverse(&mut test_state);
        assert_eq!(vec![dec!(-3.0)], test_state.float);
    }

    /// Tests the square function
    #[test]
    fn state_square() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![2, 3];
        int_square(&mut test_state);
        assert_eq!(vec![2, 9], test_state.int);

        test_state.float = vec![dec!(-4.0)];
        float_square(&mut test_state);
        assert_eq!(vec![dec!(16.0)], test_state.float);
    }
}
