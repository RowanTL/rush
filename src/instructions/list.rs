use crate::instructions::common::*;
use crate::instructions::vector::*;
use crate::instructions::code::*;
use crate::instructions::numeric::*;
use crate::instructions::logical::*;
use crate::push::state::PushState;
use std::collections::HashMap;
use std::sync::LazyLock;

pub fn int_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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

pub fn code_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        code_pop,
        code_from_code,
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
        code_from_exec,
        code_do_then_pop,
        code_do_range,
        code_do_count,
        code_do_times,
        code_map,
        code_when,
        code_is_block,
        code_is_singular,
        code_length,
        code_first,
        code_last,
        code_rest,
        code_but_last,
        code_wrap_block,
        code_combine,
        code_if,
        code_member,
        code_nth,
        code_make_empty_block,
        code_is_empty_block,
        code_size,
        code_extract,
        code_insert,
        code_first_position,
        code_reverse,
        code_noop,
        code_noop_block,
    ]
}

pub fn float_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        string_iterate,
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
    ]
}

pub fn boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        vector_int_iterate,
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
        vector_float_iterate,
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
        vector_string_iterate,
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
    ]
}

pub fn vector_boolean_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        vector_boolean_iterate,
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
    ]
}

pub fn vector_char_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        vector_char_iterate,
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
    ]
}

pub fn exec_instructions() -> Vec<fn(&mut PushState)> {
    vec![
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
        exec_do_range,
        exec_do_count,
        exec_do_times,
        exec_while,
        exec_do_while,
        exec_when,
        exec_is_block,
        exec_is_singular,
        exec_length,
        exec_first,
        exec_last,
        exec_rest,
        exec_but_last,
        exec_wrap_block,
        exec_combine,
        exec_if,
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

pub fn all_instructions() -> Vec<fn(&mut PushState)> {
    let mut all_vec = vec![];
    all_vec.extend(int_instructions().iter());
    all_vec.extend(code_instructions().iter());
    all_vec.extend(float_instructions().iter());
    all_vec.extend(string_instructions().iter());
    all_vec.extend(boolean_instructions().iter());
    all_vec.extend(char_instructions().iter());
    all_vec.extend(vector_int_instructions().iter());
    all_vec.extend(vector_float_instructions().iter());
    all_vec.extend(vector_string_instructions().iter());
    all_vec.extend(vector_boolean_instructions().iter());
    all_vec.extend(vector_char_instructions().iter());
    all_vec.extend(exec_instructions().iter());
    all_vec
}

pub static INSTR_NAME_MAP: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {
   let mut temp_map = HashMap::default();
   temp_map.insert(int_pop as usize, "int_pop".to_string());
   temp_map.insert(int_dup as usize, "int_dup".to_string());
   temp_map.insert(int_dup_times as usize, "int_dup_times".to_string());
   temp_map.insert(int_swap as usize, "int_swap".to_string());
   temp_map.insert(int_rotate as usize, "int_rotate".to_string());
   temp_map.insert(int_equal as usize, "int_equal".to_string());
   temp_map.insert(int_flush as usize, "int_flush".to_string());
   temp_map.insert(int_depth as usize, "int_depth".to_string());
   temp_map.insert(int_yank as usize, "int_yank".to_string());
   temp_map.insert(int_yank_dup as usize, "int_yank_dup".to_string());
   temp_map.insert(int_shove as usize, "int_shove".to_string());
   temp_map.insert(int_shove_dup as usize, "int_shove_dup".to_string());
   temp_map.insert(int_is_empty as usize, "int_is_empty".to_string());
   temp_map.insert(int_add as usize, "int_add".to_string());
   temp_map.insert(int_sub as usize, "int_sub".to_string());
   temp_map.insert(int_mult as usize, "int_mult".to_string());
   temp_map.insert(int_div as usize, "int_div".to_string());
   temp_map.insert(int_rem as usize, "int_rem".to_string());
   temp_map.insert(int_max as usize, "int_max".to_string());
   temp_map.insert(int_min as usize, "int_min".to_string());
   temp_map.insert(int_inc as usize, "int_inc".to_string());
   temp_map.insert(int_dec as usize, "int_dec".to_string());
   temp_map.insert(int_lt as usize, "int_lt".to_string());
   temp_map.insert(int_gt as usize, "int_gt".to_string());
   temp_map.insert(int_lte as usize, "int_lte".to_string());
   temp_map.insert(int_gte as usize, "int_gte".to_string());
   temp_map.insert(int_sin as usize, "int_sin".to_string());
   temp_map.insert(int_arcsin as usize, "int_arcsin".to_string());
   temp_map.insert(int_cos as usize, "int_cos".to_string());
   temp_map.insert(int_arccos as usize, "int_arccos".to_string());
   temp_map.insert(int_tan as usize, "int_tan".to_string());
   temp_map.insert(int_arctan as usize, "int_arctan".to_string());
   temp_map.insert(int_from_boolean as usize, "int_from_boolean".to_string());
   temp_map.insert(int_log as usize, "int_log".to_string());
   temp_map.insert(int_exp as usize, "int_exp".to_string());
   temp_map.insert(int_sqrt as usize, "int_sqrt".to_string());
   temp_map.insert(int_inv as usize, "int_inv".to_string());
   temp_map.insert(int_abs as usize, "int_abs".to_string());
   temp_map.insert(int_sign_reverse as usize, "int_sign_reverse".to_string());
   temp_map.insert(int_square as usize, "int_square".to_string());
   temp_map.insert(int_from_float as usize, "int_from_float".to_string());
   temp_map.insert(code_from_int as usize, "code_from_int".to_string());
   temp_map.insert(code_from_float as usize, "code_from_float".to_string());
   temp_map.insert(code_from_string as usize, "code_from_string".to_string());
   temp_map.insert(code_from_boolean as usize, "code_from_boolean".to_string());
   temp_map.insert(code_from_char as usize, "code_from_char".to_string());
   temp_map.insert(code_from_vector_int as usize, "code_from_vector_int".to_string());
   temp_map.insert(code_from_vector_float as usize, "code_from_vector_float".to_string());
   temp_map.insert(code_from_vector_string as usize, "code_from_vector_string".to_string());
   temp_map.insert(code_from_vector_boolean as usize, "code_from_vector_boolean".to_string());
   temp_map.insert(code_from_vector_char as usize, "code_from_vector_char".to_string());
   temp_map.insert(code_pop as usize, "code_pop".to_string());
   temp_map.insert(code_from_code as usize, "code_from_code".to_string());
   temp_map.insert(code_dup as usize, "code_dup".to_string());
   temp_map.insert(code_dup_times as usize, "code_dup_times".to_string());
   temp_map.insert(code_swap as usize, "code_swap".to_string());
   temp_map.insert(code_rotate as usize, "code_rotate".to_string());
   temp_map.insert(code_equal as usize, "code_equal".to_string());
   temp_map.insert(code_flush as usize, "code_flush".to_string());
   temp_map.insert(code_depth as usize, "code_depth".to_string());
   temp_map.insert(code_yank as usize, "code_yank".to_string());
   temp_map.insert(code_yank_dup as usize, "code_yank_dup".to_string());
   temp_map.insert(code_shove as usize, "code_shove".to_string());
   temp_map.insert(code_shove_dup as usize, "code_shove_dup".to_string());
   temp_map.insert(code_is_empty as usize, "code_is_empty".to_string());
   temp_map.insert(code_from_exec as usize, "code_from_exec".to_string());
   temp_map.insert(code_do_then_pop as usize, "code_do_then_pop".to_string());
   temp_map.insert(code_do_range as usize, "code_do_range".to_string());
   temp_map.insert(code_do_count as usize, "code_do_count".to_string());
   temp_map.insert(code_do_times as usize, "code_do_times".to_string());
   temp_map.insert(code_map as usize, "code_map".to_string());
   temp_map.insert(code_when as usize, "code_when".to_string());
   temp_map.insert(code_is_block as usize, "code_is_block".to_string());
   temp_map.insert(code_is_singular as usize, "code_is_singular".to_string());
   temp_map.insert(code_length as usize, "code_length".to_string());
   temp_map.insert(code_first as usize, "code_first".to_string());
   temp_map.insert(code_last as usize, "code_last".to_string());
   temp_map.insert(code_rest as usize, "code_rest".to_string());
   temp_map.insert(code_but_last as usize, "code_but_last".to_string());
   temp_map.insert(code_wrap_block as usize, "code_wrap_block".to_string());
   temp_map.insert(code_combine as usize, "code_combine".to_string());
   temp_map.insert(code_if as usize, "code_if".to_string());
   temp_map.insert(code_member as usize, "code_member".to_string());
   temp_map.insert(code_nth as usize, "code_nth".to_string());
   temp_map.insert(code_make_empty_block as usize, "code_make_empty_block".to_string());
   temp_map.insert(code_is_empty_block as usize, "code_is_empty_block".to_string());
   temp_map.insert(code_size as usize, "code_size".to_string());
   temp_map.insert(code_extract as usize, "code_extract".to_string());
   temp_map.insert(code_insert as usize, "code_insert".to_string());
   temp_map.insert(code_first_position as usize, "code_first_position".to_string());
   temp_map.insert(code_reverse as usize, "code_reverse".to_string());
   temp_map.insert(code_noop as usize, "code_noop".to_string());
   temp_map.insert(code_noop_block as usize, "code_noop_block".to_string());
   temp_map.insert(float_pop as usize, "float_pop".to_string());
   temp_map.insert(float_dup as usize, "float_dup".to_string());
   temp_map.insert(float_dup_times as usize, "float_dup_times".to_string());
   temp_map.insert(float_swap as usize, "float_swap".to_string());
   temp_map.insert(float_rotate as usize, "float_rotate".to_string());
   temp_map.insert(float_equal as usize, "float_equal".to_string());
   temp_map.insert(float_flush as usize, "float_flush".to_string());
   temp_map.insert(float_depth as usize, "float_depth".to_string());
   temp_map.insert(float_yank as usize, "float_yank".to_string());
   temp_map.insert(float_yank_dup as usize, "float_yank_dup".to_string());
   temp_map.insert(float_shove as usize, "float_shove".to_string());
   temp_map.insert(float_shove_dup as usize, "float_shove_dup".to_string());
   temp_map.insert(float_is_empty as usize, "float_is_empty".to_string());
   temp_map.insert(float_add as usize, "float_add".to_string());
   temp_map.insert(float_sub as usize, "float_sub".to_string());
   temp_map.insert(float_mult as usize, "float_mult".to_string());
   temp_map.insert(float_div as usize, "float_div".to_string());
   temp_map.insert(float_rem as usize, "float_rem".to_string());
   temp_map.insert(float_max as usize, "float_max".to_string());
   temp_map.insert(float_min as usize, "float_min".to_string());
   temp_map.insert(float_inc as usize, "float_inc".to_string());
   temp_map.insert(float_dec as usize, "float_dec".to_string());
   temp_map.insert(float_lt as usize, "float_lt".to_string());
   temp_map.insert(float_gt as usize, "float_gt".to_string());
   temp_map.insert(float_lte as usize, "float_lte".to_string());
   temp_map.insert(float_gte as usize, "float_gte".to_string());
   temp_map.insert(float_sin as usize, "float_sin".to_string());
   temp_map.insert(float_arcsin as usize, "float_arcsin".to_string());
   temp_map.insert(float_cos as usize, "float_cos".to_string());
   temp_map.insert(float_arccos as usize, "float_arccos".to_string());
   temp_map.insert(float_tan as usize, "float_tan".to_string());
   temp_map.insert(float_arctan as usize, "float_arctan".to_string());
   temp_map.insert(float_from_boolean as usize, "float_from_boolean".to_string());
   temp_map.insert(float_log as usize, "float_log".to_string());
   temp_map.insert(float_exp as usize, "float_exp".to_string());
   temp_map.insert(float_sqrt as usize, "float_sqrt".to_string());
   temp_map.insert(float_inv as usize, "float_inv".to_string());
   temp_map.insert(float_abs as usize, "float_abs".to_string());
   temp_map.insert(float_sign_reverse as usize, "float_sign_reverse".to_string());
   temp_map.insert(float_square as usize, "float_square".to_string());
   temp_map.insert(float_from_int as usize, "float_from_int".to_string());
   temp_map.insert(string_pop as usize, "string_pop".to_string());
   temp_map.insert(string_dup as usize, "string_dup".to_string());
   temp_map.insert(string_dup_times as usize, "string_dup_times".to_string());
   temp_map.insert(string_swap as usize, "string_swap".to_string());
   temp_map.insert(string_rotate as usize, "string_rotate".to_string());
   temp_map.insert(string_equal as usize, "string_equal".to_string());
   temp_map.insert(string_flush as usize, "string_flush".to_string());
   temp_map.insert(string_depth as usize, "string_depth".to_string());
   temp_map.insert(string_yank as usize, "string_yank".to_string());
   temp_map.insert(string_yank_dup as usize, "string_yank_dup".to_string());
   temp_map.insert(string_shove as usize, "string_shove".to_string());
   temp_map.insert(string_shove_dup as usize, "string_shove_dup".to_string());
   temp_map.insert(string_is_empty as usize, "string_is_empty".to_string());
   temp_map.insert(string_iterate as usize, "string_iterate".to_string());
   temp_map.insert(string_concat as usize, "string_concat".to_string());
   temp_map.insert(string_conj as usize, "string_conj".to_string());
   temp_map.insert(string_conj_end as usize, "string_conj_end".to_string());
   temp_map.insert(string_take_n as usize, "string_take_n".to_string());
   temp_map.insert(string_take_last_n as usize, "string_take_last_n".to_string());
   temp_map.insert(string_sub as usize, "string_sub".to_string());
   temp_map.insert(string_first as usize, "string_first".to_string());
   temp_map.insert(string_from_first_prim as usize, "string_from_first_prim".to_string());
   temp_map.insert(string_from_prim as usize, "string_from_prim".to_string());
   temp_map.insert(string_last as usize, "string_last".to_string());
   temp_map.insert(string_from_last_prim as usize, "string_from_last_prim".to_string());
   temp_map.insert(string_nth as usize, "string_nth".to_string());
   temp_map.insert(string_from_nth_prim as usize, "string_from_nth_prim".to_string());
   temp_map.insert(string_rest as usize, "string_rest".to_string());
   temp_map.insert(string_but_last as usize, "string_but_last".to_string());
   temp_map.insert(string_drop as usize, "string_drop".to_string());
   temp_map.insert(string_drop_last as usize, "string_drop_last".to_string());
   temp_map.insert(string_length as usize, "string_length".to_string());
   temp_map.insert(string_reverse as usize, "string_reverse".to_string());
   temp_map.insert(string_push_all as usize, "string_push_all".to_string());
   temp_map.insert(string_is_vector_empty as usize, "string_is_vector_empty".to_string());
   temp_map.insert(string_contains as usize, "string_contains".to_string());
   temp_map.insert(string_contains_vector_non_contiguous as usize, "string_contains_vector_non_contiguous".to_string());
   temp_map.insert(string_contains_vector_contiguous as usize, "string_contains_vector_contiguous".to_string());
   temp_map.insert(string_index_of as usize, "string_index_of".to_string());
   temp_map.insert(string_index_of_vector as usize, "string_index_of_vector".to_string());
   temp_map.insert(string_occurrences_of as usize, "string_occurrences_of".to_string());
   temp_map.insert(string_occurrences_of_vector as usize, "string_occurrences_of_vector".to_string());
   temp_map.insert(string_parse_to_prim as usize, "string_parse_to_prim".to_string());
   temp_map.insert(string_set_nth as usize, "string_set_nth".to_string());
   temp_map.insert(string_split_on as usize, "string_split_on".to_string());
   temp_map.insert(string_replace as usize, "string_replace".to_string());
   temp_map.insert(string_remove as usize, "string_remove".to_string());
   temp_map.insert(string_insert as usize, "string_insert".to_string());
   temp_map.insert(string_insert_vector as usize, "string_insert_vector".to_string());
   temp_map.insert(string_make_empty as usize, "string_make_empty".to_string());
   temp_map.insert(boolean_pop as usize, "boolean_pop".to_string());
   temp_map.insert(boolean_dup as usize, "boolean_dup".to_string());
   temp_map.insert(boolean_dup_times as usize, "boolean_dup_times".to_string());
   temp_map.insert(boolean_swap as usize, "boolean_swap".to_string());
   temp_map.insert(boolean_rotate as usize, "boolean_rotate".to_string());
   temp_map.insert(boolean_equal as usize, "boolean_equal".to_string());
   temp_map.insert(boolean_flush as usize, "boolean_flush".to_string());
   temp_map.insert(boolean_depth as usize, "boolean_depth".to_string());
   temp_map.insert(boolean_yank as usize, "boolean_yank".to_string());
   temp_map.insert(boolean_yank_dup as usize, "boolean_yank_dup".to_string());
   temp_map.insert(boolean_shove as usize, "boolean_shove".to_string());
   temp_map.insert(boolean_shove_dup as usize, "boolean_shove_dup".to_string());
   temp_map.insert(boolean_is_empty as usize, "boolean_is_empty".to_string());
   temp_map.insert(boolean_and as usize, "boolean_and".to_string());
   temp_map.insert(boolean_or as usize, "boolean_or".to_string());
   temp_map.insert(boolean_not as usize, "boolean_not".to_string());
   temp_map.insert(boolean_xor as usize, "boolean_xor".to_string());
   temp_map.insert(boolean_invert_first_then_and as usize, "boolean_invert_first_then_and".to_string());
   temp_map.insert(boolean_invert_second_then_and as usize, "boolean_invert_second_then_and".to_string());
   temp_map.insert(boolean_from_int as usize, "boolean_from_int".to_string());
   temp_map.insert(boolean_from_float as usize, "boolean_from_float".to_string());
   temp_map.insert(char_pop as usize, "char_pop".to_string());
   temp_map.insert(char_dup as usize, "char_dup".to_string());
   temp_map.insert(char_dup_times as usize, "char_dup_times".to_string());
   temp_map.insert(char_swap as usize, "char_swap".to_string());
   temp_map.insert(char_rotate as usize, "char_rotate".to_string());
   temp_map.insert(char_equal as usize, "char_equal".to_string());
   temp_map.insert(char_flush as usize, "char_flush".to_string());
   temp_map.insert(char_depth as usize, "char_depth".to_string());
   temp_map.insert(char_yank as usize, "char_yank".to_string());
   temp_map.insert(char_yank_dup as usize, "char_yank_dup".to_string());
   temp_map.insert(char_shove as usize, "char_shove".to_string());
   temp_map.insert(char_shove_dup as usize, "char_shove_dup".to_string());
   temp_map.insert(char_is_empty as usize, "char_is_empty".to_string());
   temp_map.insert(vector_int_pop as usize, "vector_int_pop".to_string());
   temp_map.insert(vector_int_dup as usize, "vector_int_dup".to_string());
   temp_map.insert(vector_int_dup_times as usize, "vector_int_dup_times".to_string());
   temp_map.insert(vector_int_swap as usize, "vector_int_swap".to_string());
   temp_map.insert(vector_int_rotate as usize, "vector_int_rotate".to_string());
   temp_map.insert(vector_int_equal as usize, "vector_int_equal".to_string());
   temp_map.insert(vector_int_flush as usize, "vector_int_flush".to_string());
   temp_map.insert(vector_int_depth as usize, "vector_int_depth".to_string());
   temp_map.insert(vector_int_yank as usize, "vector_int_yank".to_string());
   temp_map.insert(vector_int_yank_dup as usize, "vector_int_yank_dup".to_string());
   temp_map.insert(vector_int_shove as usize, "vector_int_shove".to_string());
   temp_map.insert(vector_int_shove_dup as usize, "vector_int_shove_dup".to_string());
   temp_map.insert(vector_int_is_empty as usize, "vector_int_is_empty".to_string());
   temp_map.insert(vector_int_iterate as usize, "vector_int_iterate".to_string());
   temp_map.insert(vector_int_concat as usize, "vector_int_concat".to_string());
   temp_map.insert(vector_int_conj as usize, "vector_int_conj".to_string());
   temp_map.insert(vector_int_conj_end as usize, "vector_int_conj_end".to_string());
   temp_map.insert(vector_int_take_n as usize, "vector_int_take_n".to_string());
   temp_map.insert(vector_int_take_last_n as usize, "vector_int_take_last_n".to_string());
   temp_map.insert(vector_int_sub as usize, "vector_int_sub".to_string());
   temp_map.insert(vector_int_first as usize, "vector_int_first".to_string());
   temp_map.insert(vector_int_from_first_prim as usize, "vector_int_from_first_prim".to_string());
   temp_map.insert(vector_int_from_prim as usize, "vector_int_from_prim".to_string());
   temp_map.insert(vector_int_last as usize, "vector_int_last".to_string());
   temp_map.insert(vector_int_from_last_prim as usize, "vector_int_from_last_prim".to_string());
   temp_map.insert(vector_int_nth as usize, "vector_int_nth".to_string());
   temp_map.insert(vector_int_from_nth_prim as usize, "vector_int_from_nth_prim".to_string());
   temp_map.insert(vector_int_rest as usize, "vector_int_rest".to_string());
   temp_map.insert(vector_int_but_last as usize, "vector_int_but_last".to_string());
   temp_map.insert(vector_int_drop as usize, "vector_int_drop".to_string());
   temp_map.insert(vector_int_drop_last as usize, "vector_int_drop_last".to_string());
   temp_map.insert(vector_int_length as usize, "vector_int_length".to_string());
   temp_map.insert(vector_int_reverse as usize, "vector_int_reverse".to_string());
   temp_map.insert(vector_int_push_all as usize, "vector_int_push_all".to_string());
   temp_map.insert(vector_int_is_vector_empty as usize, "vector_int_is_vector_empty".to_string());
   temp_map.insert(vector_int_contains as usize, "vector_int_contains".to_string());
   temp_map.insert(vector_int_contains_vector_non_contiguous as usize, "vector_int_contains_vector_non_contiguous".to_string());
   temp_map.insert(vector_int_contains_vector_contiguous as usize, "vector_int_contains_vector_contiguous".to_string());
   temp_map.insert(vector_int_index_of as usize, "vector_int_index_of".to_string());
   temp_map.insert(vector_int_index_of_vector as usize, "vector_int_index_of_vector".to_string());
   temp_map.insert(vector_int_occurrences_of as usize, "vector_int_occurrences_of".to_string());
   temp_map.insert(vector_int_occurrences_of_vector as usize, "vector_int_occurrences_of_vector".to_string());
   temp_map.insert(vector_int_parse_to_prim as usize, "vector_int_parse_to_prim".to_string());
   temp_map.insert(vector_int_set_nth as usize, "vector_int_set_nth".to_string());
   temp_map.insert(vector_int_split_on as usize, "vector_int_split_on".to_string());
   temp_map.insert(vector_int_replace as usize, "vector_int_replace".to_string());
   temp_map.insert(vector_int_remove as usize, "vector_int_remove".to_string());
   temp_map.insert(vector_int_insert as usize, "vector_int_insert".to_string());
   temp_map.insert(vector_int_insert_vector as usize, "vector_int_insert_vector".to_string());
   temp_map.insert(vector_int_make_empty as usize, "vector_int_make_empty".to_string());
   temp_map.insert(vector_int_sort as usize, "vector_int_sort".to_string());
   temp_map.insert(vector_int_sort_reverse as usize, "vector_int_sort_reverse".to_string());
   temp_map.insert(vector_int_mean as usize, "vector_int_mean".to_string());
   temp_map.insert(vector_int_maximum as usize, "vector_int_maximum".to_string());
   temp_map.insert(vector_int_minimum as usize, "vector_int_minimum".to_string());
   temp_map.insert(vector_int_sum as usize, "vector_int_sum".to_string());
   temp_map.insert(vector_int_mode as usize, "vector_int_mode".to_string());
   temp_map.insert(vector_int_two_norm as usize, "vector_int_two_norm".to_string());
   temp_map.insert(vector_int_cumulative_sum as usize, "vector_int_cumulative_sum".to_string());
   temp_map.insert(vector_float_pop as usize, "vector_float_pop".to_string());
   temp_map.insert(vector_float_dup as usize, "vector_float_dup".to_string());
   temp_map.insert(vector_float_dup_times as usize, "vector_float_dup_times".to_string());
   temp_map.insert(vector_float_swap as usize, "vector_float_swap".to_string());
   temp_map.insert(vector_float_rotate as usize, "vector_float_rotate".to_string());
   temp_map.insert(vector_float_equal as usize, "vector_float_equal".to_string());
   temp_map.insert(vector_float_flush as usize, "vector_float_flush".to_string());
   temp_map.insert(vector_float_depth as usize, "vector_float_depth".to_string());
   temp_map.insert(vector_float_yank as usize, "vector_float_yank".to_string());
   temp_map.insert(vector_float_yank_dup as usize, "vector_float_yank_dup".to_string());
   temp_map.insert(vector_float_shove as usize, "vector_float_shove".to_string());
   temp_map.insert(vector_float_shove_dup as usize, "vector_float_shove_dup".to_string());
   temp_map.insert(vector_float_is_empty as usize, "vector_float_is_empty".to_string());
   temp_map.insert(vector_float_iterate as usize, "vector_float_iterate".to_string());
   temp_map.insert(vector_float_concat as usize, "vector_float_concat".to_string());
   temp_map.insert(vector_float_conj as usize, "vector_float_conj".to_string());
   temp_map.insert(vector_float_conj_end as usize, "vector_float_conj_end".to_string());
   temp_map.insert(vector_float_take_n as usize, "vector_float_take_n".to_string());
   temp_map.insert(vector_float_take_last_n as usize, "vector_float_take_last_n".to_string());
   temp_map.insert(vector_float_sub as usize, "vector_float_sub".to_string());
   temp_map.insert(vector_float_first as usize, "vector_float_first".to_string());
   temp_map.insert(vector_float_from_first_prim as usize, "vector_float_from_first_prim".to_string());
   temp_map.insert(vector_float_from_prim as usize, "vector_float_from_prim".to_string());
   temp_map.insert(vector_float_last as usize, "vector_float_last".to_string());
   temp_map.insert(vector_float_from_last_prim as usize, "vector_float_from_last_prim".to_string());
   temp_map.insert(vector_float_nth as usize, "vector_float_nth".to_string());
   temp_map.insert(vector_float_from_nth_prim as usize, "vector_float_from_nth_prim".to_string());
   temp_map.insert(vector_float_rest as usize, "vector_float_rest".to_string());
   temp_map.insert(vector_float_but_last as usize, "vector_float_but_last".to_string());
   temp_map.insert(vector_float_drop as usize, "vector_float_drop".to_string());
   temp_map.insert(vector_float_drop_last as usize, "vector_float_drop_last".to_string());
   temp_map.insert(vector_float_length as usize, "vector_float_length".to_string());
   temp_map.insert(vector_float_reverse as usize, "vector_float_reverse".to_string());
   temp_map.insert(vector_float_push_all as usize, "vector_float_push_all".to_string());
   temp_map.insert(vector_float_is_vector_empty as usize, "vector_float_is_vector_empty".to_string());
   temp_map.insert(vector_float_contains as usize, "vector_float_contains".to_string());
   temp_map.insert(vector_float_contains_vector_non_contiguous as usize, "vector_float_contains_vector_non_contiguous".to_string());
   temp_map.insert(vector_float_contains_vector_contiguous as usize, "vector_float_contains_vector_contiguous".to_string());
   temp_map.insert(vector_float_index_of as usize, "vector_float_index_of".to_string());
   temp_map.insert(vector_float_index_of_vector as usize, "vector_float_index_of_vector".to_string());
   temp_map.insert(vector_float_occurrences_of as usize, "vector_float_occurrences_of".to_string());
   temp_map.insert(vector_float_occurrences_of_vector as usize, "vector_float_occurrences_of_vector".to_string());
   temp_map.insert(vector_float_parse_to_prim as usize, "vector_float_parse_to_prim".to_string());
   temp_map.insert(vector_float_set_nth as usize, "vector_float_set_nth".to_string());
   temp_map.insert(vector_float_split_on as usize, "vector_float_split_on".to_string());
   temp_map.insert(vector_float_replace as usize, "vector_float_replace".to_string());
   temp_map.insert(vector_float_remove as usize, "vector_float_remove".to_string());
   temp_map.insert(vector_float_insert as usize, "vector_float_insert".to_string());
   temp_map.insert(vector_float_insert_vector as usize, "vector_float_insert_vector".to_string());
   temp_map.insert(vector_float_make_empty as usize, "vector_float_make_empty".to_string());
   temp_map.insert(vector_float_sort as usize, "vector_float_sort".to_string());
   temp_map.insert(vector_float_sort_reverse as usize, "vector_float_sort_reverse".to_string());
   temp_map.insert(vector_float_mean as usize, "vector_float_mean".to_string());
   temp_map.insert(vector_float_maximum as usize, "vector_float_maximum".to_string());
   temp_map.insert(vector_float_minimum as usize, "vector_float_minimum".to_string());
   temp_map.insert(vector_float_sum as usize, "vector_float_sum".to_string());
   temp_map.insert(vector_float_mode as usize, "vector_float_mode".to_string());
   temp_map.insert(vector_float_two_norm as usize, "vector_float_two_norm".to_string());
   temp_map.insert(vector_float_cumulative_sum as usize, "vector_float_cumulative_sum".to_string());
   temp_map.insert(vector_string_pop as usize, "vector_string_pop".to_string());
   temp_map.insert(vector_string_dup as usize, "vector_string_dup".to_string());
   temp_map.insert(vector_string_dup_times as usize, "vector_string_dup_times".to_string());
   temp_map.insert(vector_string_swap as usize, "vector_string_swap".to_string());
   temp_map.insert(vector_string_rotate as usize, "vector_string_rotate".to_string());
   temp_map.insert(vector_string_equal as usize, "vector_string_equal".to_string());
   temp_map.insert(vector_string_flush as usize, "vector_string_flush".to_string());
   temp_map.insert(vector_string_depth as usize, "vector_string_depth".to_string());
   temp_map.insert(vector_string_yank as usize, "vector_string_yank".to_string());
   temp_map.insert(vector_string_yank_dup as usize, "vector_string_yank_dup".to_string());
   temp_map.insert(vector_string_shove as usize, "vector_string_shove".to_string());
   temp_map.insert(vector_string_shove_dup as usize, "vector_string_shove_dup".to_string());
   temp_map.insert(vector_string_is_empty as usize, "vector_string_is_empty".to_string());
   temp_map.insert(vector_string_iterate as usize, "vector_string_iterate".to_string());
   temp_map.insert(vector_string_concat as usize, "vector_string_concat".to_string());
   temp_map.insert(vector_string_conj as usize, "vector_string_conj".to_string());
   temp_map.insert(vector_string_conj_end as usize, "vector_string_conj_end".to_string());
   temp_map.insert(vector_string_take_n as usize, "vector_string_take_n".to_string());
   temp_map.insert(vector_string_take_last_n as usize, "vector_string_take_last_n".to_string());
   temp_map.insert(vector_string_sub as usize, "vector_string_sub".to_string());
   temp_map.insert(vector_string_first as usize, "vector_string_first".to_string());
   temp_map.insert(vector_string_from_first_prim as usize, "vector_string_from_first_prim".to_string());
   temp_map.insert(vector_string_from_prim as usize, "vector_string_from_prim".to_string());
   temp_map.insert(vector_string_last as usize, "vector_string_last".to_string());
   temp_map.insert(vector_string_from_last_prim as usize, "vector_string_from_last_prim".to_string());
   temp_map.insert(vector_string_nth as usize, "vector_string_nth".to_string());
   temp_map.insert(vector_string_from_nth_prim as usize, "vector_string_from_nth_prim".to_string());
   temp_map.insert(vector_string_rest as usize, "vector_string_rest".to_string());
   temp_map.insert(vector_string_but_last as usize, "vector_string_but_last".to_string());
   temp_map.insert(vector_string_drop as usize, "vector_string_drop".to_string());
   temp_map.insert(vector_string_drop_last as usize, "vector_string_drop_last".to_string());
   temp_map.insert(vector_string_length as usize, "vector_string_length".to_string());
   temp_map.insert(vector_string_reverse as usize, "vector_string_reverse".to_string());
   temp_map.insert(vector_string_push_all as usize, "vector_string_push_all".to_string());
   temp_map.insert(vector_string_is_vector_empty as usize, "vector_string_is_vector_empty".to_string());
   temp_map.insert(vector_string_contains as usize, "vector_string_contains".to_string());
   temp_map.insert(vector_string_contains_vector_non_contiguous as usize, "vector_string_contains_vector_non_contiguous".to_string());
   temp_map.insert(vector_string_contains_vector_contiguous as usize, "vector_string_contains_vector_contiguous".to_string());
   temp_map.insert(vector_string_index_of as usize, "vector_string_index_of".to_string());
   temp_map.insert(vector_string_index_of_vector as usize, "vector_string_index_of_vector".to_string());
   temp_map.insert(vector_string_occurrences_of as usize, "vector_string_occurrences_of".to_string());
   temp_map.insert(vector_string_occurrences_of_vector as usize, "vector_string_occurrences_of_vector".to_string());
   temp_map.insert(vector_string_parse_to_prim as usize, "vector_string_parse_to_prim".to_string());
   temp_map.insert(vector_string_set_nth as usize, "vector_string_set_nth".to_string());
   temp_map.insert(vector_string_split_on as usize, "vector_string_split_on".to_string());
   temp_map.insert(vector_string_replace as usize, "vector_string_replace".to_string());
   temp_map.insert(vector_string_remove as usize, "vector_string_remove".to_string());
   temp_map.insert(vector_string_insert as usize, "vector_string_insert".to_string());
   temp_map.insert(vector_string_insert_vector as usize, "vector_string_insert_vector".to_string());
   temp_map.insert(vector_string_make_empty as usize, "vector_string_make_empty".to_string());
   temp_map.insert(vector_boolean_pop as usize, "vector_boolean_pop".to_string());
   temp_map.insert(vector_boolean_dup as usize, "vector_boolean_dup".to_string());
   temp_map.insert(vector_boolean_dup_times as usize, "vector_boolean_dup_times".to_string());
   temp_map.insert(vector_boolean_swap as usize, "vector_boolean_swap".to_string());
   temp_map.insert(vector_boolean_rotate as usize, "vector_boolean_rotate".to_string());
   temp_map.insert(vector_boolean_equal as usize, "vector_boolean_equal".to_string());
   temp_map.insert(vector_boolean_flush as usize, "vector_boolean_flush".to_string());
   temp_map.insert(vector_boolean_depth as usize, "vector_boolean_depth".to_string());
   temp_map.insert(vector_boolean_yank as usize, "vector_boolean_yank".to_string());
   temp_map.insert(vector_boolean_yank_dup as usize, "vector_boolean_yank_dup".to_string());
   temp_map.insert(vector_boolean_shove as usize, "vector_boolean_shove".to_string());
   temp_map.insert(vector_boolean_shove_dup as usize, "vector_boolean_shove_dup".to_string());
   temp_map.insert(vector_boolean_is_empty as usize, "vector_boolean_is_empty".to_string());
   temp_map.insert(vector_boolean_iterate as usize, "vector_boolean_iterate".to_string());
   temp_map.insert(vector_boolean_concat as usize, "vector_boolean_concat".to_string());
   temp_map.insert(vector_boolean_conj as usize, "vector_boolean_conj".to_string());
   temp_map.insert(vector_boolean_conj_end as usize, "vector_boolean_conj_end".to_string());
   temp_map.insert(vector_boolean_take_n as usize, "vector_boolean_take_n".to_string());
   temp_map.insert(vector_boolean_take_last_n as usize, "vector_boolean_take_last_n".to_string());
   temp_map.insert(vector_boolean_sub as usize, "vector_boolean_sub".to_string());
   temp_map.insert(vector_boolean_first as usize, "vector_boolean_first".to_string());
   temp_map.insert(vector_boolean_from_first_prim as usize, "vector_boolean_from_first_prim".to_string());
   temp_map.insert(vector_boolean_from_prim as usize, "vector_boolean_from_prim".to_string());
   temp_map.insert(vector_boolean_last as usize, "vector_boolean_last".to_string());
   temp_map.insert(vector_boolean_from_last_prim as usize, "vector_boolean_from_last_prim".to_string());
   temp_map.insert(vector_boolean_nth as usize, "vector_boolean_nth".to_string());
   temp_map.insert(vector_boolean_from_nth_prim as usize, "vector_boolean_from_nth_prim".to_string());
   temp_map.insert(vector_boolean_rest as usize, "vector_boolean_rest".to_string());
   temp_map.insert(vector_boolean_but_last as usize, "vector_boolean_but_last".to_string());
   temp_map.insert(vector_boolean_drop as usize, "vector_boolean_drop".to_string());
   temp_map.insert(vector_boolean_drop_last as usize, "vector_boolean_drop_last".to_string());
   temp_map.insert(vector_boolean_length as usize, "vector_boolean_length".to_string());
   temp_map.insert(vector_boolean_reverse as usize, "vector_boolean_reverse".to_string());
   temp_map.insert(vector_boolean_push_all as usize, "vector_boolean_push_all".to_string());
   temp_map.insert(vector_boolean_is_vector_empty as usize, "vector_boolean_is_vector_empty".to_string());
   temp_map.insert(vector_boolean_contains as usize, "vector_boolean_contains".to_string());
   temp_map.insert(vector_boolean_contains_vector_non_contiguous as usize, "vector_boolean_contains_vector_non_contiguous".to_string());
   temp_map.insert(vector_boolean_contains_vector_contiguous as usize, "vector_boolean_contains_vector_contiguous".to_string());
   temp_map.insert(vector_boolean_index_of as usize, "vector_boolean_index_of".to_string());
   temp_map.insert(vector_boolean_index_of_vector as usize, "vector_boolean_index_of_vector".to_string());
   temp_map.insert(vector_boolean_occurrences_of as usize, "vector_boolean_occurrences_of".to_string());
   temp_map.insert(vector_boolean_occurrences_of_vector as usize, "vector_boolean_occurrences_of_vector".to_string());
   temp_map.insert(vector_boolean_parse_to_prim as usize, "vector_boolean_parse_to_prim".to_string());
   temp_map.insert(vector_boolean_set_nth as usize, "vector_boolean_set_nth".to_string());
   temp_map.insert(vector_boolean_split_on as usize, "vector_boolean_split_on".to_string());
   temp_map.insert(vector_boolean_replace as usize, "vector_boolean_replace".to_string());
   temp_map.insert(vector_boolean_remove as usize, "vector_boolean_remove".to_string());
   temp_map.insert(vector_boolean_insert as usize, "vector_boolean_insert".to_string());
   temp_map.insert(vector_boolean_insert_vector as usize, "vector_boolean_insert_vector".to_string());
   temp_map.insert(vector_boolean_make_empty as usize, "vector_boolean_make_empty".to_string());
   temp_map.insert(vector_char_pop as usize, "vector_char_pop".to_string());
   temp_map.insert(vector_char_dup as usize, "vector_char_dup".to_string());
   temp_map.insert(vector_char_dup_times as usize, "vector_char_dup_times".to_string());
   temp_map.insert(vector_char_swap as usize, "vector_char_swap".to_string());
   temp_map.insert(vector_char_rotate as usize, "vector_char_rotate".to_string());
   temp_map.insert(vector_char_equal as usize, "vector_char_equal".to_string());
   temp_map.insert(vector_char_flush as usize, "vector_char_flush".to_string());
   temp_map.insert(vector_char_depth as usize, "vector_char_depth".to_string());
   temp_map.insert(vector_char_yank as usize, "vector_char_yank".to_string());
   temp_map.insert(vector_char_yank_dup as usize, "vector_char_yank_dup".to_string());
   temp_map.insert(vector_char_shove as usize, "vector_char_shove".to_string());
   temp_map.insert(vector_char_shove_dup as usize, "vector_char_shove_dup".to_string());
   temp_map.insert(vector_char_is_empty as usize, "vector_char_is_empty".to_string());
   temp_map.insert(vector_char_iterate as usize, "vector_char_iterate".to_string());
   temp_map.insert(vector_char_concat as usize, "vector_char_concat".to_string());
   temp_map.insert(vector_char_conj as usize, "vector_char_conj".to_string());
   temp_map.insert(vector_char_conj_end as usize, "vector_char_conj_end".to_string());
   temp_map.insert(vector_char_take_n as usize, "vector_char_take_n".to_string());
   temp_map.insert(vector_char_take_last_n as usize, "vector_char_take_last_n".to_string());
   temp_map.insert(vector_char_sub as usize, "vector_char_sub".to_string());
   temp_map.insert(vector_char_first as usize, "vector_char_first".to_string());
   temp_map.insert(vector_char_from_first_prim as usize, "vector_char_from_first_prim".to_string());
   temp_map.insert(vector_char_from_prim as usize, "vector_char_from_prim".to_string());
   temp_map.insert(vector_char_last as usize, "vector_char_last".to_string());
   temp_map.insert(vector_char_from_last_prim as usize, "vector_char_from_last_prim".to_string());
   temp_map.insert(vector_char_nth as usize, "vector_char_nth".to_string());
   temp_map.insert(vector_char_from_nth_prim as usize, "vector_char_from_nth_prim".to_string());
   temp_map.insert(vector_char_rest as usize, "vector_char_rest".to_string());
   temp_map.insert(vector_char_but_last as usize, "vector_char_but_last".to_string());
   temp_map.insert(vector_char_drop as usize, "vector_char_drop".to_string());
   temp_map.insert(vector_char_drop_last as usize, "vector_char_drop_last".to_string());
   temp_map.insert(vector_char_length as usize, "vector_char_length".to_string());
   temp_map.insert(vector_char_reverse as usize, "vector_char_reverse".to_string());
   temp_map.insert(vector_char_push_all as usize, "vector_char_push_all".to_string());
   temp_map.insert(vector_char_is_vector_empty as usize, "vector_char_is_vector_empty".to_string());
   temp_map.insert(vector_char_contains as usize, "vector_char_contains".to_string());
   temp_map.insert(vector_char_contains_vector_non_contiguous as usize, "vector_char_contains_vector_non_contiguous".to_string());
   temp_map.insert(vector_char_contains_vector_contiguous as usize, "vector_char_contains_vector_contiguous".to_string());
   temp_map.insert(vector_char_index_of as usize, "vector_char_index_of".to_string());
   temp_map.insert(vector_char_index_of_vector as usize, "vector_char_index_of_vector".to_string());
   temp_map.insert(vector_char_occurrences_of as usize, "vector_char_occurrences_of".to_string());
   temp_map.insert(vector_char_occurrences_of_vector as usize, "vector_char_occurrences_of_vector".to_string());
   temp_map.insert(vector_char_parse_to_prim as usize, "vector_char_parse_to_prim".to_string());
   temp_map.insert(vector_char_set_nth as usize, "vector_char_set_nth".to_string());
   temp_map.insert(vector_char_split_on as usize, "vector_char_split_on".to_string());
   temp_map.insert(vector_char_replace as usize, "vector_char_replace".to_string());
   temp_map.insert(vector_char_remove as usize, "vector_char_remove".to_string());
   temp_map.insert(vector_char_insert as usize, "vector_char_insert".to_string());
   temp_map.insert(vector_char_insert_vector as usize, "vector_char_insert_vector".to_string());
   temp_map.insert(vector_char_make_empty as usize, "vector_char_make_empty".to_string());
   temp_map.insert(exec_pop as usize, "exec_pop".to_string());
   temp_map.insert(exec_dup as usize, "exec_dup".to_string());
   temp_map.insert(exec_dup_times as usize, "exec_dup_times".to_string());
   temp_map.insert(exec_swap as usize, "exec_swap".to_string());
   temp_map.insert(exec_rotate as usize, "exec_rotate".to_string());
   temp_map.insert(exec_equal as usize, "exec_equal".to_string());
   temp_map.insert(exec_flush as usize, "exec_flush".to_string());
   temp_map.insert(exec_depth as usize, "exec_depth".to_string());
   temp_map.insert(exec_yank as usize, "exec_yank".to_string());
   temp_map.insert(exec_yank_dup as usize, "exec_yank_dup".to_string());
   temp_map.insert(exec_shove as usize, "exec_shove".to_string());
   temp_map.insert(exec_shove_dup as usize, "exec_shove_dup".to_string());
   temp_map.insert(exec_is_empty as usize, "exec_is_empty".to_string());
   temp_map.insert(exec_do_range as usize, "exec_do_range".to_string());
   temp_map.insert(exec_do_count as usize, "exec_do_count".to_string());
   temp_map.insert(exec_do_times as usize, "exec_do_times".to_string());
   temp_map.insert(exec_while as usize, "exec_while".to_string());
   temp_map.insert(exec_do_while as usize, "exec_do_while".to_string());
   temp_map.insert(exec_when as usize, "exec_when".to_string());
   temp_map.insert(exec_is_block as usize, "exec_is_block".to_string());
   temp_map.insert(exec_is_singular as usize, "exec_is_singular".to_string());
   temp_map.insert(exec_length as usize, "exec_length".to_string());
   temp_map.insert(exec_first as usize, "exec_first".to_string());
   temp_map.insert(exec_last as usize, "exec_last".to_string());
   temp_map.insert(exec_rest as usize, "exec_rest".to_string());
   temp_map.insert(exec_but_last as usize, "exec_but_last".to_string());
   temp_map.insert(exec_wrap_block as usize, "exec_wrap_block".to_string());
   temp_map.insert(exec_combine as usize, "exec_combine".to_string());
   temp_map.insert(exec_if as usize, "exec_if".to_string());
   temp_map.insert(exec_member as usize, "exec_member".to_string());
   temp_map.insert(exec_nth as usize, "exec_nth".to_string());
   temp_map.insert(exec_make_empty_block as usize, "exec_make_empty_block".to_string());
   temp_map.insert(exec_is_empty_block as usize, "exec_is_empty_block".to_string());
   temp_map.insert(exec_size as usize, "exec_size".to_string());
   temp_map.insert(exec_extract as usize, "exec_extract".to_string());
   temp_map.insert(exec_insert as usize, "exec_insert".to_string());
   temp_map.insert(exec_first_position as usize, "exec_first_position".to_string());
   temp_map.insert(exec_reverse as usize, "exec_reverse".to_string());
   temp_map.insert(exec_noop as usize, "exec_noop".to_string());
   temp_map.insert(exec_noop_block as usize, "exec_noop_block".to_string());
   temp_map
});