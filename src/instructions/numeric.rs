//! # Numeric Instructions
//!
//! This file contains numeric instructions for int and float.

// There has to be a better way to declare these functions.
// Just don't know enough Rust yet ig.

use crate::push::state::PushState;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};

use super::utils::InstructionTrait;

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
    T: Div<Output = T> + Copy + InstructionTrait,
{
    vals[1].checked_div(vals[0])
}
make_instruction!(int, int, _div, i128, 2);
make_instruction!(float, float, _div, Decimal, 2);

/// Takes the remainder of two values
fn _rem<T>(vals: Vec<T>) -> Option<T>
where
    T: Div<Output = T> + Copy + InstructionTrait,
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
make_instruction!(int, int, _max, i128, 2);
make_instruction!(float, float, _max, Decimal, 2);

/// Takes the min of two values
fn _min<T>(vals: Vec<T>) -> Option<T>
where
    T: Ord + Copy,
{
    Some(min(vals[1], vals[0]))
}
make_instruction!(int, int, _min, i128, 2);
make_instruction!(float, float, _min, Decimal, 2);

/// Increments a single value by 1
fn _inc<T>(vals: Vec<T>) -> Option<T>
where
    T: InstructionTrait + Copy,
{
    Some(vals[0].increment())
}
make_instruction!(int, int, _inc, i128, 1);
make_instruction!(float, float, _inc, Decimal, 1);

/// Decrements a single value by 1
fn _dec<T>(vals: Vec<T>) -> Option<T>
where
    T: InstructionTrait + Copy,
{
    Some(vals[0].decrement())
}
make_instruction!(int, int, _dec, i128, 1);
make_instruction!(float, float, _dec, Decimal, 1);

/// Checks if the 2nd to top value is less than the top value
fn _lt<T>(vals: Vec<T>) -> Option<bool>
where
    T: Ord + Copy,
{
    Some(vals[1] < vals[0])
}
make_instruction!(int, boolean, _lt, i128, 2);
make_instruction!(float, boolean, _lt, Decimal, 2);

/// Checks if the 2nd to top value is greater than the top value
fn _gt<T>(vals: Vec<T>) -> Option<bool>
where
    T: Ord + Copy,
{
    Some(vals[1] > vals[0])
}
make_instruction!(int, boolean, _gt, i128, 2);
make_instruction!(float, boolean, _gt, Decimal, 2);

/// Checks if the 2nd to top value is less than or equal to the top value
fn _lte<T>(vals: Vec<T>) -> Option<bool>
where
    T: Ord + Copy,
{
    Some(vals[1] <= vals[0])
}
make_instruction!(int, boolean, _lte, i128, 2);
make_instruction!(float, boolean, _lte, Decimal, 2);

/// Checks if the 2nd to top value is greater than or equal to the top value
fn _gte<T>(vals: Vec<T>) -> Option<bool>
where
    T: Ord + Copy,
{
    Some(vals[1] >= vals[0])
}
make_instruction!(int, boolean, _gte, i128, 2);
make_instruction!(float, boolean, _gte, Decimal, 2);

/// Runs sin on a single item.
fn _sin<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_sin()
}
make_instruction!(int, int, _sin, i128, 1);
make_instruction!(float, float, _sin, Decimal, 1);

/// Runs arcsin on a single item.
fn _arcsin<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_sin()?.inverse()
}
make_instruction!(int, int, _arcsin, i128, 1);
make_instruction!(float, float, _arcsin, Decimal, 1);

/// Runs cos on a single item.
fn _cos<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_cos()
}
make_instruction!(int, int, _cos, i128, 1);
make_instruction!(float, float, _cos, Decimal, 1);

/// Runs arcsin on a single item.
fn _arccos<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_cos()?.inverse()
}
make_instruction!(int, int, _arccos, i128, 1);
make_instruction!(float, float, _arccos, Decimal, 1);

/// Runs tan on a single item.
fn _tan<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_tan()
}
make_instruction!(int, int, _tan, i128, 1);
make_instruction!(float, float, _tan, Decimal, 1);

/// Runs arctan on a single item.
fn _arctan<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_tan()?.inverse()
}
make_instruction!(int, int, _arctan, i128, 1);
make_instruction!(float, float, _arctan, Decimal, 1);

/// Converts the top int to a float.
fn _to_int(vals: Vec<Decimal>) -> Option<i128> {
    vals[0].to_i128()
}
make_instruction!(float, int, _to_int, Decimal, 1);

/// Converts the top float to an int.
fn _to_float(vals: Vec<i128>) -> Option<Decimal> {
    Decimal::from_i128(vals[0])
}
make_instruction!(int, float, _to_float, i128, 1);

/// Converts a single number to a bool.
fn _to_bool<T>(vals: Vec<T>) -> Option<bool>
where
    T: Copy + InstructionTrait,
{
    Some(vals[0].to_bool())
}
make_instruction!(int, boolean, _to_bool, i128, 1);
make_instruction!(float, boolean, _to_bool, Decimal, 1);

/// Takes the log base 10 of a single Decimal. Acts as a
/// NoOp if the value is 0. If the value is negative, takes
/// the absolute value of the number.
fn _log<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].absolute().safe_log10()
}
make_instruction!(int, int, _log, i128, 1);
make_instruction!(float, float, _log, Decimal, 1);

/// Takes the exp of a single value. Ints get truncated.
fn _exp<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_exp()
}
make_instruction!(int, int, _exp, i128, 1);
make_instruction!(float, float, _exp, Decimal, 1);

/// Takes the square root of the absolute value of a single value.
fn _sqrt<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].safe_sqrt()
}
make_instruction!(int, int, _sqrt, i128, 1);
make_instruction!(float, float, _sqrt, Decimal, 1);

/// Takes the inverse of a single value. If the number is 0,
/// does nothing (returns None). Truncates an int to 0.
fn _inv<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    vals[0].inverse()
}
make_instruction!(int, int, _inv, i128, 1);
make_instruction!(float, float, _inv, Decimal, 1);

/// Takes the absolute value of the top number
fn _abs<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    Some(vals[0].absolute())
}
make_instruction!(int, int, _abs, i128, 1);
make_instruction!(float, float, _abs, Decimal, 1);

/// Reverses the sign of the top number
fn _sign_reverse<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    Some(vals[0].sign_reverse())
}
make_instruction!(int, int, _sign_reverse, i128, 1);
make_instruction!(float, float, _sign_reverse, Decimal, 1);

/// Squares the top number
fn _square<T>(vals: Vec<T>) -> Option<T>
where
    T: Copy + InstructionTrait,
{
    Some(vals[0].square())
}
make_instruction!(int, int, _square, i128, 1);
make_instruction!(float, float, _square, Decimal, 1);

/// A list of all of the defined int functions in this file.
/// Must manually register functions in this list if added.
pub fn int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        int_add,
        int_sub,
        int_mult,
        int_div,
        int_rem,
        int_max,
        int_min,
        int_inc,
        int_dec,
        int_lt,
        int_gt,
        int_lte,
        int_gte,
        int_sin,
        int_arcsin,
        int_cos,
        int_arccos,
        int_tan,
        int_arctan,
        int_to_float,
        int_to_bool,
        int_log,
        int_exp,
        int_sqrt,
        int_inv,
        int_abs,
        int_sign_reverse,
        int_square,
    ]
}

/// All of the float instructions declared in this file.
pub fn float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        float_add,
        float_sub,
        float_mult,
        float_div,
        float_rem,
        float_max,
        float_min,
        float_inc,
        float_dec,
        float_lt,
        float_gt,
        float_lte,
        float_gte,
        float_sin,
        float_arcsin,
        float_cos,
        float_arccos,
        float_tan,
        float_arctan,
        float_to_int,
        float_to_bool,
        float_log,
        float_exp,
        float_sqrt,
        float_inv,
        float_abs,
        float_sign_reverse,
        float_square,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;
    use rust_decimal::dec;

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

    /// Tests the _inc and _dec functions
    #[test]
    fn inc_dec_test() {
        let vals: Vec<i128> = vec![2];
        assert_eq!(Some(3), _inc(vals));

        let vals: Vec<i128> = vec![10];
        assert_eq!(Some(9), _dec(vals));

        let vals: Vec<Decimal> = vec![dec!(2.2)];
        assert_eq!(Some(dec!(3.2)), _inc(vals));

        let vals: Vec<Decimal> = vec![dec!(5.6)];
        assert_eq!(Some(dec!(4.6)), _dec(vals));
    }

    /// Tests the _lt, _gt, _lte, and _gte functions
    #[test]
    fn lt_gt_lte_gte_test() {
        let vals: Vec<i128> = vec![3, 2];
        assert_eq!(Some(true), _lt(vals));

        let vals: Vec<i128> = vec![1, 4];
        assert_eq!(Some(false), _lt(vals));

        let vals: Vec<i128> = vec![3, 3];
        assert_eq!(Some(false), _lt(vals));

        let vals: Vec<i128> = vec![2, 3];
        assert_eq!(Some(true), _gt(vals));

        let vals: Vec<i128> = vec![4, 1];
        assert_eq!(Some(false), _gt(vals));

        let vals: Vec<i128> = vec![3, 3];
        assert_eq!(Some(false), _gt(vals));

        let vals: Vec<i128> = vec![3, 2];
        assert_eq!(Some(true), _lte(vals));

        let vals: Vec<i128> = vec![1, 4];
        assert_eq!(Some(false), _lte(vals));

        let vals: Vec<i128> = vec![3, 3];
        assert_eq!(Some(true), _lte(vals));

        let vals: Vec<i128> = vec![2, 3];
        assert_eq!(Some(true), _gte(vals));

        let vals: Vec<i128> = vec![4, 1];
        assert_eq!(Some(false), _gte(vals));

        let vals: Vec<i128> = vec![3, 3];
        assert_eq!(Some(true), _gte(vals));
    }

    /// Tests the various trig functions.
    #[test]
    fn trig_tests() {
        let vals = vec![Decimal::PI];
        assert_eq!(Some(dec!(0.0)), _sin(vals));

        let vals = vec![Decimal::QUARTER_PI];
        assert_eq!(Some(dec!(1.4142135623869512272301701717)), _arcsin(vals));

        let vals = vec![Decimal::PI];
        assert_eq!(Some(dec!(-1.0)), _cos(vals));

        let vals = vec![Decimal::QUARTER_PI];
        assert_eq!(Some(dec!(1.4142135626023406165042434783)), _arccos(vals));

        let vals = vec![Decimal::PI];
        assert_eq!(Some(dec!(0.0)), _tan(vals));

        let vals = vec![Decimal::QUARTER_PI];
        assert_eq!(Some(dec!(1.0000000043184676055890307049)), _arctan(vals));
    }

    /// Tests the functions that cast from one numeric type
    /// to another
    #[test]
    fn cast_tests() {
        let vals = vec![dec!(1.2)];
        assert_eq!(Some(1), _to_int(vals));

        let vals: Vec<i128> = vec![2];
        assert_eq!(Some(dec!(2.0)), _to_float(vals));
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
        int_to_float(&mut test_state);
        assert_eq!(vec![dec!(1.0)], test_state.float);

        test_state.int.clear();
        test_state.float = vec![dec!(2.1)];
        float_to_int(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.int = vec![1];
        int_to_bool(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.int = vec![0];
        int_to_bool(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.float = vec![dec!(2.0)];
        float_to_bool(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.float = vec![dec!(0.0)];
        float_to_bool(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
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
