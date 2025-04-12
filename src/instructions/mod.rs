#[macro_use]
pub mod macros {
    /// A macro that makes a push instruction given: the name of the input stack to use,
    /// the name of the output stack, an internal function to call, the type of a function,
    /// and the arity of the internal function call.
    ///
    /// The `in_stack` argument refers to which push stack should this operate on.
    /// The `out_stack` argument refers to which push stack should the result be pushed to.
    /// The `fn_name` argement refers to the name of the function that is to operate
    /// on the values popped from `in_stack`.
    /// The `fn_type` argument refers to the type of `in_stack`. For example, the
    /// int stack is type: *Vec<i128>*. `fn_type` is *i128* in this case.
    /// The `fn_arity` argument refers to how many popped stack items are needed to
    /// execute the instruction. If the amount of items in the stack is less than
    /// this value, the instruction does nothing.
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

    /// The same as make_instruction but uses clone() to fill the arguments
    /// to each function rather than a reference. Is slower, but will be okay
    /// for the time being.
    #[macro_export]
    macro_rules! make_instruction_clone {
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
                        inputs.push(state.$in_stack[in_stack_len - n]);
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.extend(result.iter());
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

    /// Same as `make_instruction!` but has two extra parameters.
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
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[in_stack_len - n].clone());
                    }
                    for n in 1..=$aux_arity {
                        aux_inputs.push(state.$aux_stack[aux_stack_len - n].clone());
                    }
                    if let Some(result) = $fn_name(inputs, aux_inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        for _ in 0..$aux_arity {
                            state.$aux_stack.pop();
                        }
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
