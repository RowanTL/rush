//! # Numeric Instructions
//!
//! This file contains numeric instructions for int and float.

use crate::push::state::*;
use paste::paste;
use std::ops::Add;

fn _add<T: Add<Output = T>>(val0: T, val1: T) -> T {
    val0 + val1
}

#[macro_export]
macro_rules! make_instruction {
    ($stack_name:ident, $fn_name:ident) => {
        paste::item! {
            fn [< $stack_name $fn_name >] (state: &mut PushState) -> Option< {
                let val0 = state.$stack_name[0];
                let val1 = state.$stack_name[1];
                $fn_name(val0, val1);
                println!("{val0} {val1}");
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut test_state = EMPTY_STATE;
        test_state.int = vec![1, 2];
        // assert_eq!(vec![1, 2, 3], _add(&mut test_state).int);
        make_instruction!(int, _add);
        int_add(&mut test_state);
    }
}
