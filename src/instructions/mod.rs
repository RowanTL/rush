#[macro_use]
pub mod macros {
    /// A macro that makes a push instruction given: the name of the input stack to use,
    /// the name of the output stack, an internal function to call, the type of a function,
    /// and the arity of the internal function call.
    ///
    /// The `in_stack` argument refers to which push stack should this operate on.
    /// The `out_stack` argument refers to which push stack should the result be pushed to.
    /// The `fn_name` argument refers to the name of the function that is to operate
    /// on the values popped from `in_stack`.
    /// The `fn_type` argument refers to the type of `in_stack`. For example, the
    /// int stack is type: *Vec<i128>*. `fn_type` is *i128* in this case.
    /// The `fn_arity` argument refers to how many popped stack items are needed to
    /// execute the instruction. If the amount of items in the stack is less than
    /// this value, the instruction does nothing. How many items exactly should be passed
    /// as a list to the functions used for calculations.
    ///
    /// What causes an instruction to NoOp:
    /// 1) There aren't enough values on a stack to execute an instruction.
    /// 2) The internal operation the instruction executes is unable to be ran without
    ///    erroring such as division by 0.
    #[macro_export]
    macro_rules! make_instruction {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    if in_stack_len < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n]);
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }

    /// The same as make_instruction above but prepends the output
    /// stack to the function name rather than the input stack.
    #[macro_export]
    macro_rules! make_instruction_out {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                pub fn [< $out_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    if in_stack_len < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }

    /// The same as make_instruction but uses clone() to fill the arguments
    /// to each function rather than a reference. Is slower, but will be okay
    /// for the time being.
    #[macro_export]
    macro_rules! make_instruction_clone {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                #[allow(clippy::reversed_empty_ranges, unused_comparisons)]
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    if in_stack_len < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! make_instruction_mult {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    if in_stack_len < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.extend(result.into_iter());
                    }
                }
            }
        };
    }

    /// Same as the make_instruction macro except it pushes nothing to the
    /// output stack.
    #[macro_export]
    macro_rules! make_instruction_no_out {
        ($in_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                #[allow(unused_comparisons)]
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    if in_stack_len < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(_) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                    }
                }
            }
        };
    }

    /// Same as `make_instruction!` but can work on two stacks.
    ///
    /// `aux_stack` is an auxiliary stack to be used as input to internal function.
    /// `aux_arity` is the amount of the auxiliary stack to use.
    /// `aux_type` is the type of the auxiliary stack
    #[macro_export]
    macro_rules! make_instruction_aux {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt, $aux_stack:ident, $aux_arity:stmt, $aux_type:ty) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                /// $aux_stack is also used and popped $aux_arity time(s).
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    let aux_stack_len = state.$aux_stack.len();
                    if in_stack_len < $fn_arity || aux_stack_len < $aux_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    let mut aux_inputs: Vec<$aux_type> = Vec::with_capacity($aux_arity);
                    for n in 1..=$aux_arity {
                        aux_inputs.push(state.$aux_stack[aux_stack_len - n].clone());
                    }
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs, aux_inputs) {
                        for _ in 0..$aux_arity {
                            state.$aux_stack.pop();
                        }
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }

    /// Same as make_instruction_mult but can handle one auxiliary variable.
    #[macro_export]
    macro_rules! make_instruction_mult_aux {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt, $aux_stack:ident, $aux_arity:stmt, $aux_type:ty) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                /// $aux_stack is also used and popped $aux_arity time(s).
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    let aux_stack_len = state.$aux_stack.len();
                    if in_stack_len < $fn_arity || aux_stack_len < $aux_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    let mut aux_inputs: Vec<$aux_type> = Vec::with_capacity($aux_arity);
                    for n in 1..=$aux_arity {
                        aux_inputs.push(state.$aux_stack[aux_stack_len - n].clone());
                    }
                    for n in 1..=$fn_arity {
                        if stringify!($fn_type) == stringify!($aux_type) {
                            inputs.push(state.$in_stack[in_stack_len - $aux_arity - n].clone());
                        } else {
                            inputs.push(state.$in_stack[in_stack_len - n].clone());
                        }
                        //inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs, aux_inputs) {
                        for _ in 0..$aux_arity {
                            state.$aux_stack.pop();
                        }
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.extend(result.into_iter());
                    }
                }
            }
        };
    }

    /// Same as `make_instruction!` but can work on three stacks. Is there a way
    /// to generalize even this?
    ///
    /// `aux_stack` is an auxiliary stack to be used as input to internal function.
    /// `aux_arity` is the amount of the auxiliary stack to use.
    /// `aux_type` is the type of the auxiliary stack
    #[macro_export]
    macro_rules! make_instruction_aux2 {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt, $aux0_stack:ident, $aux0_arity:stmt, $aux0_type:ty, $aux1_stack:ident, $aux1_arity:stmt, $aux1_type:ty) => {
            paste::item! {
                /// Runs the $fn_name function on the top $fn_arity items from
                /// the $in_stack and places the calculated value on the $out_stack.
                /// $aux_stack is also used and popped $aux_arity time(s).
                pub fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    let in_stack_len = state.$in_stack.len();
                    let aux0_stack_len = state.$aux0_stack.len();
                    let aux1_stack_len = state.$aux1_stack.len();
                    if in_stack_len < $fn_arity || aux0_stack_len < $aux0_arity || aux1_stack_len < $aux1_arity {
                        return;
                    }
                    if stringify!($aux0_type) == stringify!($aux1_type) {
                        if aux0_stack_len + aux1_stack_len < $aux0_arity + $aux1_arity {
                            return;
                        }
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    let mut aux0_inputs: Vec<$aux0_type> = Vec::with_capacity($aux0_arity);
                    let mut aux1_inputs: Vec<$aux1_type> = Vec::with_capacity($aux1_arity);
                    for n in 1..=$aux1_arity {
                        aux1_inputs.push(state.$aux1_stack[aux1_stack_len - n].clone());
                    }
                    for n in 1..=$aux0_arity {
                        if stringify!($aux0_type) == stringify!($aux1_type) {
                            aux0_inputs.push(state.$aux0_stack[aux0_stack_len - $aux1_arity - n].clone());
                        } else {
                            aux0_inputs.push(state.$aux0_stack[aux0_stack_len - n].clone());
                        }
                    }
                    // Stack shouldn't be the same for all three
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs, aux0_inputs, aux1_inputs) {
                        for _ in 0..$aux1_arity {
                            state.$aux1_stack.pop();
                        }
                        for _ in 0..$aux0_arity {
                            state.$aux0_stack.pop();
                        }
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }

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

    /// Runs a function and ensures needed variables are extracted from a state without error while
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
