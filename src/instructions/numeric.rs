//! # Numeric Instructions
//!
//! This file contains numeric instructions for int and float.

use crate::push::state::{EMPTY_STATE, PushState};
use std::ops::{Add, Sub};

/// Adds two addable values together.
fn _add<T>(vals: Vec<T>) -> Option<T>
where
    T: Add<Output = T>,
    T: Copy,
{
    Some(vals[1] + vals[0])
}

/// Subtracts two subtractable values from each other.
fn _sub<T>(vals: Vec<T>) -> Option<T>
where
    T: Sub<Output = T>,
    T: Copy,
{
    Some(vals[1] - vals[0])
}

/// Declares int_add
make_instruction!(int, _add, i64);

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the _add function.
    #[test]
    fn add_test() {
        let vals: Vec<i64> = vec![1, 2];
        assert_eq!(Some(3), _add(vals));

        let vals: Vec<f64> = vec![1.1, 2.2];
        assert_eq!(Some(3.3), _add(vals));
    }

    /// Tests the _sub function.
    #[test]
    fn sub_test() {
        let vals: Vec<i64> = vec![1, 2];
        assert_eq!(Some(1), _sub(vals));

        let vals: Vec<f64> = vec![1.1, 2.2];
        assert_eq!(Some(1.1), _sub(vals));
    }

    // Tests that the various state_add functions work.
    #[test]
    fn state_add() {
        let mut test_state = EMPTY_STATE;
        test_state.int = vec![1, 2];
        test_state.float = vec![1.1, 2.2];
        int_add(&mut test_state, 2);
        assert_eq!(test_state.int, vec![3]);
    }
}
