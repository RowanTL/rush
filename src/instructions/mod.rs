#[macro_use]
pub mod macros {
    /// Runs a function and ensures the necessary variables are extracted from a state without error
    macro_rules! make_instruction_new {
        ($func:ident, $prefix:ident, $out_stack:ident, $($stacks:ident), *) => {
            paste::item! {
                pub fn [< $prefix $func >] (state: &mut PushState) {
                    rush_macro::run_instruction!($func, $out_stack, state, $($stacks), *);
                }
            }
        };
    }

    /// Runs a function and ensures the necessary variables are extracted from a state without error while
    /// returning multiple variables from the function
    macro_rules! make_instruction_new_aux {
        ($func:ident, $prefix:ident, $out_stack:ident, $($stacks:ident), *) => {
            paste::item! {
                pub fn [< $prefix $func >] (state: &mut PushState) {
                    rush_macro::run_instruction!($func, $out_stack, state, $($stacks), *, ;);
                }
            }
        };
    }

    /// Makes an instruction that takes no input stacks. Must specify a type for this
    /// one so because the result needs a type, and the compiler can't infer it here :(
    macro_rules! make_instruction_empty {
        ($func:ident, $prefix:ident, $out_stack:ident, $out_type:ty) => {
            paste::item! {
                pub fn [< $prefix $func >] (state: &mut PushState) {
                    if let Some(result) = $func::<$out_type>() {
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }
}

pub mod code;
pub mod common;
pub mod logical;
pub mod numeric;
pub mod utils;
pub mod vector;

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::push::state::{EMPTY_STATE, PushState};

    #[test]
    fn make_instruction_new_test() {
        fn _test_func(x: i128, y: i128) -> Option<i128> {
            Some(x + y)
        }

        fn _aux_test_func(x: i128, y: i128) -> Option<Vec<i128>> {
            Some(vec![x + y, x - y])
        }

        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        make_instruction_new!(_test_func, int, int, int, int);
        int_test_func(&mut test_state);
        assert_eq!(vec![3], test_state.int);

        test_state.int = vec![1, 2];
        make_instruction_new_aux!(_aux_test_func, int, int, int, int);
        int_aux_test_func(&mut test_state);
        assert_eq!(vec![3, 1], test_state.int);
    }
}
