use crate::instructions::code::*;
use crate::instructions::common::*;
use crate::instructions::logical::*;
use crate::instructions::numeric::*;
use crate::instructions::vector::*;
use crate::push::state::PushState;

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
                    // This is crazy jank, not meant for use in actual code :)
                    // https://doc.rust-lang.org/std/any/fn.type_name.html
                    if std::any::type_name::<$aux0_type>() == std::any::type_name::<$aux1_type>() {
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
                        if std::any::type_name::<$aux0_type>() == std::any::type_name::<$aux1_type>() {
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
}

pub mod code;
pub mod common;
pub mod logical;
pub mod numeric;
pub mod utils;
pub mod vector;

// unsure how to procedurally read a file and put all functions
// into a vector. Probably need to use procedural macros, but I'm not there yet.
pub fn int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // numeric.rs
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
        int_from_float,
        int_from_boolean,
        int_log,
        int_exp,
        int_sqrt,
        int_inv,
        int_abs,
        int_sign_reverse,
        int_square,
        // common.rs
        int_pop,
    ]
}
pub fn float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // numeric
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
        float_from_int,
        float_from_boolean,
        float_log,
        float_exp,
        float_sqrt,
        float_inv,
        float_abs,
        float_sign_reverse,
        float_square,
        // common.rs
        float_pop,
    ]
}
pub fn string_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // numeric.rs
        string_concat,
        string_conj,
        string_conj_end,
        string_take_n,
        string_take_last_n,
        string_sub,
        string_first,
        string_from_first_prim,
        string_from_prim,
        string_last,
        string_from_last_prim,
        string_nth,
        string_from_nth_prim,
        string_rest,
        string_but_last,
        string_drop,
        string_length,
        string_reverse,
        string_push_all,
        string_make_empty,
        string_is_empty,
        string_contains,
        string_index_of,
        string_occurrences_of,
        string_set_nth,
        string_replace,
        // common.rs
        string_pop,
    ]
}
pub fn boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // logical.rs
        boolean_and,
        boolean_or,
        boolean_not,
        boolean_xor,
        boolean_invert_first_then_and,
        boolean_invert_second_then_and,
        boolean_from_int,
        boolean_from_float,
        // common.rs
        boolean_pop,
    ]
}
pub fn char_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // common.rs
        char_pop,
    ]
}
pub fn vector_int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // vector.rs
        vector_int_concat,
        vector_int_conj,
        vector_int_conj_end,
        vector_int_take_n,
        vector_int_take_last_n,
        vector_int_sub,
        vector_int_first,
        vector_int_from_first_prim,
        vector_int_from_prim,
        vector_int_last,
        vector_int_from_last_prim,
        vector_int_nth,
        vector_int_from_nth_prim,
        vector_int_rest,
        vector_int_but_last,
        vector_int_drop,
        vector_int_length,
        vector_int_reverse,
        vector_int_push_all,
        vector_int_make_empty,
        vector_int_is_empty,
        vector_int_contains,
        vector_int_index_of,
        vector_int_occurrences_of,
        vector_int_set_nth,
        vector_int_replace,
        // common.rs
        vector_int_pop,
    ]
}
pub fn vector_float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // vector.rs
        vector_float_concat,
        vector_float_conj,
        vector_float_conj_end,
        vector_float_take_n,
        vector_float_take_last_n,
        vector_float_sub,
        vector_float_first,
        vector_float_from_first_prim,
        vector_float_from_prim,
        vector_float_last,
        vector_float_from_last_prim,
        vector_float_nth,
        vector_float_from_nth_prim,
        vector_float_rest,
        vector_float_but_last,
        vector_float_drop,
        vector_float_length,
        vector_float_reverse,
        vector_float_push_all,
        vector_float_make_empty,
        vector_float_is_empty,
        vector_float_contains,
        vector_float_index_of,
        vector_float_occurrences_of,
        vector_float_set_nth,
        vector_float_replace,
        // common.rs
        vector_float_pop,
    ]
}
pub fn vector_string_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // vector.rs
        vector_string_concat,
        vector_string_conj,
        vector_string_conj_end,
        vector_string_take_n,
        vector_string_take_last_n,
        vector_string_sub,
        vector_string_first,
        vector_string_from_first_prim,
        vector_string_from_prim,
        vector_string_last,
        vector_string_from_last_prim,
        vector_string_nth,
        vector_string_from_nth_prim,
        vector_string_rest,
        vector_string_but_last,
        vector_string_drop,
        vector_string_length,
        vector_string_reverse,
        vector_string_make_empty,
        vector_string_is_empty,
        vector_string_contains,
        vector_string_index_of,
        vector_string_occurrences_of,
        vector_string_set_nth,
        vector_string_replace,
        // common.rs
        vector_string_pop,
    ]
}
pub fn vector_boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // vector.rs
        vector_boolean_concat,
        vector_boolean_conj,
        vector_boolean_conj_end,
        vector_boolean_take_n,
        vector_boolean_take_last_n,
        vector_boolean_sub,
        vector_boolean_first,
        vector_boolean_from_first_prim,
        vector_boolean_from_prim,
        vector_boolean_last,
        vector_boolean_from_last_prim,
        vector_boolean_nth,
        vector_boolean_from_nth_prim,
        vector_boolean_rest,
        vector_boolean_but_last,
        vector_boolean_drop,
        vector_boolean_length,
        vector_boolean_reverse,
        vector_boolean_push_all,
        vector_boolean_make_empty,
        vector_boolean_is_empty,
        vector_boolean_contains,
        vector_boolean_index_of,
        vector_boolean_occurrences_of,
        vector_boolean_set_nth,
        vector_boolean_replace,
        // common.rs
        vector_boolean_pop,
    ]
}
pub fn vector_char_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // vector.rs
        vector_char_concat,
        vector_char_conj,
        vector_char_conj_end,
        vector_char_take_n,
        vector_char_take_last_n,
        vector_char_sub,
        vector_char_first,
        vector_char_from_first_prim,
        vector_char_from_prim,
        vector_char_last,
        vector_char_from_last_prim,
        vector_char_nth,
        vector_char_from_nth_prim,
        vector_char_rest,
        vector_char_but_last,
        vector_char_drop,
        vector_char_length,
        vector_char_reverse,
        vector_char_push_all,
        vector_char_make_empty,
        vector_char_is_empty,
        vector_char_contains,
        vector_char_index_of,
        vector_char_occurrences_of,
        vector_char_set_nth,
        vector_char_replace,
        // common.rs
        vector_char_pop,
    ]
}
pub fn exec_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // code.rs
        exec_do_range,
        exec_do_count,
        exec_do_times,
        exec_while,
        exec_do_while,
        exec_if,
        exec_when,
        exec_make_empty_block,
        exec_is_empty_block,
        exec_size,
        // common.rs
        exec_noop,
        exec_noop_block,
        exec_pop,
    ]
}
pub fn code_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // code.rs
        code_is_block,
        code_is_singular,
        code_length,
        code_first,
        code_last,
        code_rest,
        code_but_last,
        code_wrap_block,
        code_combine,
        code_do_then_pop,
        code_do_range,
        code_do_count,
        code_do_times,
        code_map,
        code_if,
        code_when,
        code_member,
        code_nth,
        code_make_empty_block,
        code_is_empty_block,
        code_size,
        code_extract,
        code_insert,
        code_insert,
        code_first_position,
        code_reverse,
        // common.rs
        code_noop,
        code_noop_block,
        code_pop,
        code_from_int,
        code_from_float,
        code_from_string,
        code_from_boolean,
        code_from_char,
        code_from_vector_int,
        code_from_vector_float,
        code_from_vector_string,
        code_from_vector_boolean,
        code_from_vector_char,
        code_from_exec,
    ]
}
