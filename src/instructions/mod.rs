use crate::instructions::code::*;
use crate::instructions::common::*;
use crate::instructions::logical::*;
use crate::instructions::numeric::*;
use crate::instructions::vector::*;
use crate::push::state::PushState;

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

/// Call this if you want all integer instructions. Have to manually
/// add by hand. Wait until eager macro evaluation becomes more prevalent.
/// This can be simplified when that happens. (Or just know a lot about Rust)
pub fn int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        int_pop,
        int_dup,
        int_dup_times,
        int_swap,
        int_rotate,
        int_equal,
        int_flush,
        int_depth,
        int_yank,
        int_yank_dup,
        int_shove,
        int_shove_dup,
        int_is_empty,
        // Numeric instructions
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
        int_from_boolean,
        int_log,
        int_exp,
        int_sqrt,
        int_inv,
        int_abs,
        int_sign_reverse,
        int_square,
        int_from_float,
    ]
}

pub fn float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        float_pop,
        float_dup,
        float_dup_times,
        float_swap,
        float_rotate,
        float_equal,
        float_flush,
        float_depth,
        float_yank,
        float_yank_dup,
        float_shove,
        float_shove_dup,
        float_is_empty,
        // Numeric instructions
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
        float_from_boolean,
        float_log,
        float_exp,
        float_sqrt,
        float_inv,
        float_abs,
        float_sign_reverse,
        float_square,
        float_from_int,
    ]
}

pub fn string_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        string_pop,
        string_dup,
        string_dup_times,
        string_swap,
        string_rotate,
        string_equal,
        string_flush,
        string_depth,
        string_yank,
        string_yank_dup,
        string_shove,
        string_shove_dup,
        string_is_empty,
        // Vector instructions
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
        string_drop_last,
        string_length,
        string_reverse,
        string_push_all,
        string_is_vector_empty,
        string_contains,
        string_contains_vector_non_contiguous,
        string_contains_vector_contiguous,
        string_index_of,
        string_index_of_vector,
        string_occurrences_of,
        string_occurrences_of_vector,
        string_parse_to_prim,
        string_set_nth,
        string_split_on,
        string_replace,
        string_remove,
        string_insert,
        string_insert_vector,
        string_make_empty,
        string_iterate,
    ]
}

pub fn boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        boolean_pop,
        boolean_dup,
        boolean_dup_times,
        boolean_swap,
        boolean_rotate,
        boolean_equal,
        boolean_flush,
        boolean_depth,
        boolean_yank,
        boolean_yank_dup,
        boolean_shove,
        boolean_shove_dup,
        boolean_is_empty,
        // Logical instructions
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

pub fn char_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        char_pop,
        char_dup,
        char_dup_times,
        char_swap,
        char_rotate,
        char_equal,
        char_flush,
        char_depth,
        char_yank,
        char_yank_dup,
        char_shove,
        char_shove_dup,
        char_is_empty,
    ]
}

pub fn vector_int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        vector_int_pop,
        vector_int_dup,
        vector_int_dup_times,
        vector_int_swap,
        vector_int_rotate,
        vector_int_equal,
        vector_int_flush,
        vector_int_depth,
        vector_int_yank,
        vector_int_yank_dup,
        vector_int_shove,
        vector_int_shove_dup,
        vector_int_is_empty,
        // Vector instructions
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
        vector_int_drop_last,
        vector_int_length,
        vector_int_reverse,
        vector_int_push_all,
        vector_int_is_vector_empty,
        vector_int_contains,
        vector_int_contains_vector_non_contiguous,
        vector_int_contains_vector_contiguous,
        vector_int_index_of,
        vector_int_index_of_vector,
        vector_int_occurrences_of,
        vector_int_occurrences_of_vector,
        vector_int_parse_to_prim,
        vector_int_set_nth,
        vector_int_split_on,
        vector_int_replace,
        vector_int_remove,
        vector_int_insert,
        vector_int_insert_vector,
        vector_int_make_empty,
        vector_int_iterate,
        // Numeric vector instructions from vector.rs
        vector_int_sort,
        vector_int_sort_reverse,
        vector_int_mean,
        vector_int_maximum,
        vector_int_minimum,
        vector_int_sum,
        vector_int_mode,
        vector_int_two_norm,
        vector_int_cumulative_sum,
    ]
}

pub fn vector_float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        vector_float_pop,
        vector_float_dup,
        vector_float_dup_times,
        vector_float_swap,
        vector_float_rotate,
        vector_float_equal,
        vector_float_flush,
        vector_float_depth,
        vector_float_yank,
        vector_float_yank_dup,
        vector_float_shove,
        vector_float_shove_dup,
        vector_float_is_empty,
        // Vector instructions
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
        vector_float_drop_last,
        vector_float_length,
        vector_float_reverse,
        vector_float_push_all,
        vector_float_is_vector_empty,
        vector_float_contains,
        vector_float_contains_vector_non_contiguous,
        vector_float_contains_vector_contiguous,
        vector_float_index_of,
        vector_float_index_of_vector,
        vector_float_occurrences_of,
        vector_float_occurrences_of_vector,
        vector_float_parse_to_prim,
        vector_float_set_nth,
        vector_float_split_on,
        vector_float_replace,
        vector_float_remove,
        vector_float_insert,
        vector_float_insert_vector,
        vector_float_make_empty,
        vector_float_iterate,
        // Numeric vector instructions from vector.rs
        vector_float_sort,
        vector_float_sort_reverse,
        vector_float_mean,
        vector_float_maximum,
        vector_float_minimum,
        vector_float_sum,
        vector_float_mode,
        vector_float_two_norm,
        vector_float_cumulative_sum,
    ]
}

pub fn vector_string_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        vector_string_pop,
        vector_string_dup,
        vector_string_dup_times,
        vector_string_swap,
        vector_string_rotate,
        vector_string_equal,
        vector_string_flush,
        vector_string_depth,
        vector_string_yank,
        vector_string_yank_dup,
        vector_string_shove,
        vector_string_shove_dup,
        vector_string_is_empty,
        // Vector instructions
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
        vector_string_drop_last,
        vector_string_length,
        vector_string_reverse,
        vector_string_push_all,
        vector_string_is_vector_empty,
        vector_string_contains,
        vector_string_contains_vector_non_contiguous,
        vector_string_contains_vector_contiguous,
        vector_string_index_of,
        vector_string_index_of_vector,
        vector_string_occurrences_of,
        vector_string_occurrences_of_vector,
        vector_string_parse_to_prim,
        vector_string_set_nth,
        vector_string_split_on,
        vector_string_replace,
        vector_string_remove,
        vector_string_insert,
        vector_string_insert_vector,
        vector_string_make_empty,
        vector_string_iterate,
    ]
}

pub fn vector_boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        vector_boolean_pop,
        vector_boolean_dup,
        vector_boolean_dup_times,
        vector_boolean_swap,
        vector_boolean_rotate,
        vector_boolean_equal,
        vector_boolean_flush,
        vector_boolean_depth,
        vector_boolean_yank,
        vector_boolean_yank_dup,
        vector_boolean_shove,
        vector_boolean_shove_dup,
        vector_boolean_is_empty,
        // Vector instructions
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
        vector_boolean_drop_last,
        vector_boolean_length,
        vector_boolean_reverse,
        vector_boolean_push_all,
        vector_boolean_is_vector_empty,
        vector_boolean_contains,
        vector_boolean_contains_vector_non_contiguous,
        vector_boolean_contains_vector_contiguous,
        vector_boolean_index_of,
        vector_boolean_index_of_vector,
        vector_boolean_occurrences_of,
        vector_boolean_occurrences_of_vector,
        vector_boolean_parse_to_prim,
        vector_boolean_set_nth,
        vector_boolean_split_on,
        vector_boolean_replace,
        vector_boolean_remove,
        vector_boolean_insert,
        vector_boolean_insert_vector,
        vector_boolean_make_empty,
        vector_boolean_iterate,
    ]
}

pub fn vector_char_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        vector_char_pop,
        vector_char_dup,
        vector_char_dup_times,
        vector_char_swap,
        vector_char_rotate,
        vector_char_equal,
        vector_char_flush,
        vector_char_depth,
        vector_char_yank,
        vector_char_yank_dup,
        vector_char_shove,
        vector_char_shove_dup,
        vector_char_is_empty,
        // Vector instructions
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
        vector_char_drop_last,
        vector_char_length,
        vector_char_reverse,
        vector_char_push_all,
        vector_char_is_vector_empty,
        vector_char_contains,
        vector_char_contains_vector_non_contiguous,
        vector_char_contains_vector_contiguous,
        vector_char_index_of,
        vector_char_index_of_vector,
        vector_char_occurrences_of,
        vector_char_occurrences_of_vector,
        vector_char_parse_to_prim,
        vector_char_set_nth,
        vector_char_split_on,
        vector_char_replace,
        vector_char_remove,
        vector_char_insert,
        vector_char_insert_vector,
        vector_char_make_empty,
        vector_char_iterate,
    ]
}

pub fn code_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        code_pop,
        code_dup,
        code_dup_times,
        code_swap,
        code_rotate,
        code_equal,
        code_flush,
        code_depth,
        code_yank,
        code_yank_dup,
        code_shove,
        code_shove_dup,
        code_is_empty,
        // Code instructions
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
        code_first_position,
        code_reverse,
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
        code_from_code, // this is a thing lol
        code_from_exec,
        code_noop,
        code_noop_block,
    ]
}

pub fn exec_instructions() -> Vec<fn(&mut PushState)> {
    vec![
        // Common instructions
        exec_pop,
        exec_dup,
        exec_dup_times,
        exec_swap,
        exec_rotate,
        exec_equal,
        exec_flush,
        exec_depth,
        exec_yank,
        exec_yank_dup,
        exec_shove,
        exec_shove_dup,
        exec_is_empty,
        // exec instructions
        exec_is_block,
        exec_is_singular,
        exec_length,
        exec_first,
        exec_last,
        exec_rest,
        exec_but_last,
        exec_wrap_block,
        exec_combine,
        exec_do_range,
        exec_do_count,
        exec_do_times,
        exec_while,
        exec_do_while,
        exec_if,
        exec_when,
        exec_member,
        exec_nth,
        exec_make_empty_block,
        exec_is_empty_block,
        exec_size,
        exec_extract,
        exec_insert,
        exec_first_position,
        exec_reverse,
        exec_noop,
        exec_noop_block,
    ]
}

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
