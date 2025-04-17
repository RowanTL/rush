use rust_decimal::Decimal;
use std::cmp::{max, min};

use crate::push::state::{Gene, PushState};

fn min_max_bounds(ndx: i128, length: usize) -> usize {
    max(0, min(ndx.unsigned_abs() as usize, length - 1))
}

/// Acts as a NoOp, does nothing with the vals list.
fn _noop<T>(_: Vec<T>) -> Option<T> {
    None
}
make_instruction_clone!(code, code, _noop, Gene, 0);
make_instruction_clone!(exec, exec, _noop, Gene, 0);

fn _noop_block<T>(_: Vec<T>) -> Option<T> {
    None
}
make_instruction_clone!(code, code, _noop_block, Gene, 0);
make_instruction_clone!(exec, exec, _noop_block, Gene, 0);

/// Pops the top value from the stack
fn _pop<T>(vals: Vec<T>) -> Option<T>
where
    T: Clone,
{
    // This is suboptimal, how to re-write?
    // Calls for a complete overhaul later down the line.
    Some(vals[0].clone())
}
make_instruction_no_out!(int, _pop, i128, 1);
make_instruction_no_out!(float, _pop, Decimal, 1);
make_instruction_no_out!(string, _pop, Vec<char>, 1);
make_instruction_no_out!(boolean, _pop, bool, 1);
make_instruction_no_out!(char, _pop, char, 1);
make_instruction_no_out!(vector_int, _pop, Vec<i128>, 1);
make_instruction_no_out!(vector_float, _pop, Vec<Decimal>, 1);
make_instruction_no_out!(vector_string, _pop, Vec<Vec<char>>, 1);
make_instruction_no_out!(vector_boolean, _pop, Vec<bool>, 1);
make_instruction_no_out!(vector_char, _pop, Vec<char>, 1);
make_instruction_no_out!(code, _pop, Gene, 1);
make_instruction_no_out!(exec, _pop, Gene, 1);

/// Wraps a type in its respective Gene
macro_rules! make_code {
    ($in_stack:ident, $gene:ident) => {
        paste::item! {
            pub fn [< code_from_ $in_stack >] (state: &mut PushState) {
                if let Some(val) = state.$in_stack.pop() {
                    state.code.push(Gene::$gene(val));
                }
            }
        }
    };
}
make_code!(int, GeneInt);
make_code!(float, GeneFloat);
make_code!(string, GeneString);
make_code!(boolean, GeneBoolean);
make_code!(char, GeneChar);
make_code!(vector_int, GeneVectorInt);
make_code!(vector_float, GeneVectorFloat);
make_code!(vector_string, GeneVectorString);
make_code!(vector_boolean, GeneVectorBoolean);
make_code!(vector_char, GeneVectorChar);

pub fn code_from_exec(state: &mut PushState) {
    if let Some(gene) = state.exec.pop() {
        state.code.push(gene);
    }
}

/// Duplicates an item
pub fn _dup<T: Clone>(vals: Vec<T>) -> Option<Vec<T>> {
    Some(vec![vals[0].clone(), vals[0].clone()])
}
make_instruction_mult!(int, int, _dup, i128, 1);
make_instruction_mult!(float, float, _dup, Decimal, 1);
make_instruction_mult!(string, string, _dup, Vec<char>, 1);
make_instruction_mult!(boolean, boolean, _dup, bool, 1);
make_instruction_mult!(char, char, _dup, char, 1);
make_instruction_mult!(vector_int, vector_int, _dup, Vec<i128>, 1);
make_instruction_mult!(vector_float, vector_float, _dup, Vec<Decimal>, 1);
make_instruction_mult!(vector_string, vector_string, _dup, Vec<Vec<char>>, 1);
make_instruction_mult!(vector_boolean, vector_boolean, _dup, Vec<bool>, 1);
make_instruction_mult!(vector_char, vector_char, _dup, Vec<char>, 1);
make_instruction_mult!(code, code, _dup, Gene, 1);
make_instruction_mult!(exec, exec, _dup, Gene, 1);

pub fn _dup_times<T: Clone>(vals: Vec<T>, auxs: Vec<i128>) -> Option<Vec<T>> {
    Some(vec![vals[0].clone(); auxs[0] as usize])
}
make_instruction_mult_aux!(int, int, _dup_times, i128, 1, int, 1, i128);
make_instruction_mult_aux!(float, float, _dup_times, Decimal, 1, int, 1, i128);
make_instruction_mult_aux!(string, string, _dup_times, Vec<char>, 1, int, 1, i128);
make_instruction_mult_aux!(boolean, boolean, _dup_times, bool, 1, int, 1, i128);
make_instruction_mult_aux!(char, char, _dup_times, char, 1, int, 1, i128);
make_instruction_mult_aux!(
    vector_int,
    vector_int,
    _dup_times,
    Vec<i128>,
    1,
    int,
    1,
    i128
);
make_instruction_mult_aux!(
    vector_float,
    vector_float,
    _dup_times,
    Vec<Decimal>,
    1,
    int,
    1,
    i128
);
make_instruction_mult_aux!(
    vector_string,
    vector_string,
    _dup_times,
    Vec<Vec<char>>,
    1,
    int,
    1,
    i128
);
make_instruction_mult_aux!(
    vector_boolean,
    vector_boolean,
    _dup_times,
    Vec<bool>,
    1,
    int,
    1,
    i128
);
make_instruction_mult_aux!(
    vector_char,
    vector_char,
    _dup_times,
    Vec<char>,
    1,
    int,
    1,
    i128
);
make_instruction_mult_aux!(code, code, _dup_times, Gene, 1, int, 1, i128);
make_instruction_mult_aux!(exec, exec, _dup_times, Gene, 1, int, 1, i128);

/// Swaps two values
pub fn _swap<T: Clone>(vals: Vec<T>) -> Option<Vec<T>> {
    Some(vec![vals[0].clone(), vals[1].clone()])
}
make_instruction_mult!(int, int, _swap, i128, 2);
make_instruction_mult!(float, float, _swap, Decimal, 2);
make_instruction_mult!(string, string, _swap, Vec<char>, 2);
make_instruction_mult!(boolean, boolean, _swap, bool, 2);
make_instruction_mult!(char, char, _swap, char, 2);
make_instruction_mult!(vector_int, vector_int, _swap, Vec<i128>, 2);
make_instruction_mult!(vector_float, vector_float, _swap, Vec<Decimal>, 2);
make_instruction_mult!(vector_string, vector_string, _swap, Vec<Vec<char>>, 2);
make_instruction_mult!(vector_boolean, vector_boolean, _swap, Vec<bool>, 2);
make_instruction_mult!(vector_char, vector_char, _swap, Vec<char>, 2);
make_instruction_mult!(code, code, _swap, Gene, 2);
make_instruction_mult!(exec, exec, _swap, Gene, 2);

/// Rotates three values
pub fn _rotate<T: Clone>(vals: Vec<T>) -> Option<Vec<T>> {
    Some(vec![vals[2].clone(), vals[0].clone(), vals[1].clone()])
}
make_instruction_mult!(int, int, _rotate, i128, 3);
make_instruction_mult!(float, float, _rotate, Decimal, 3);
make_instruction_mult!(string, string, _rotate, Vec<char>, 3);
make_instruction_mult!(boolean, boolean, _rotate, bool, 3);
make_instruction_mult!(char, char, _rotate, char, 3);
make_instruction_mult!(vector_int, vector_int, _rotate, Vec<i128>, 3);
make_instruction_mult!(vector_float, vector_float, _rotate, Vec<Decimal>, 3);
make_instruction_mult!(vector_string, vector_string, _rotate, Vec<Vec<char>>, 3);
make_instruction_mult!(vector_boolean, vector_boolean, _rotate, Vec<bool>, 3);
make_instruction_mult!(vector_char, vector_char, _rotate, Vec<char>, 3);
make_instruction_mult!(code, code, _rotate, Gene, 3);
make_instruction_mult!(exec, exec, _rotate, Gene, 3);

/// Checks if two values are equal
pub fn _equal<T: Clone + Eq>(vals: Vec<T>) -> Option<bool> {
    Some(vals[1] == vals[0])
}
make_instruction!(int, boolean, _equal, i128, 2);
make_instruction!(float, boolean, _equal, Decimal, 2);
make_instruction_clone!(string, boolean, _equal, Vec<char>, 2);
make_instruction!(boolean, boolean, _equal, bool, 2);
make_instruction!(char, boolean, _equal, char, 2);
make_instruction_clone!(vector_int, boolean, _equal, Vec<i128>, 2);
make_instruction_clone!(vector_float, boolean, _equal, Vec<Decimal>, 2);
make_instruction_clone!(vector_string, boolean, _equal, Vec<Vec<char>>, 2);
make_instruction_clone!(vector_boolean, boolean, _equal, Vec<bool>, 2);
make_instruction_clone!(vector_char, boolean, _equal, Vec<char>, 2);
make_instruction_clone!(code, boolean, _equal, Gene, 2);
make_instruction_clone!(exec, boolean, _equal, Gene, 2);

/// Checks if two values are not equal
pub fn _not_equal<T: Clone + Eq>(vals: Vec<T>) -> Option<bool> {
    Some(vals[1] != vals[0])
}
make_instruction!(int, boolean, _not_equal, i128, 2);
make_instruction!(float, boolean, _not_equal, Decimal, 2);
make_instruction_clone!(string, boolean, _not_equal, Vec<char>, 2);
make_instruction!(boolean, boolean, _not_equal, bool, 2);
make_instruction!(char, boolean, _not_equal, char, 2);
make_instruction_clone!(vector_int, boolean, _not_equal, Vec<i128>, 2);
make_instruction_clone!(vector_float, boolean, _not_equal, Vec<Decimal>, 2);
make_instruction_clone!(vector_string, boolean, _not_equal, Vec<Vec<char>>, 2);
make_instruction_clone!(vector_boolean, boolean, _not_equal, Vec<bool>, 2);
make_instruction_clone!(vector_char, boolean, _not_equal, Vec<char>, 2);
make_instruction_clone!(code, boolean, _not_equal, Gene, 2);
make_instruction_clone!(exec, boolean, _not_equal, Gene, 2);

macro_rules! flush_state {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _flush >] (state: &mut PushState) {
                state.$in_stack.clear();
            }
        }
    };
}
flush_state!(int);
flush_state!(float);
flush_state!(string);
flush_state!(boolean);
flush_state!(char);
flush_state!(vector_int);
flush_state!(vector_float);
flush_state!(vector_string);
flush_state!(vector_boolean);
flush_state!(vector_char);
flush_state!(code);
flush_state!(exec);

macro_rules! stack_depth {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _depth >] (state: &mut PushState) {
                state.int.push(state.$in_stack.len() as i128)
            }
        }
    };
}
stack_depth!(int);
stack_depth!(float);
stack_depth!(string);
stack_depth!(boolean);
stack_depth!(char);
stack_depth!(vector_int);
stack_depth!(vector_float);
stack_depth!(vector_string);
stack_depth!(vector_boolean);
stack_depth!(vector_char);
stack_depth!(code);
stack_depth!(exec);

macro_rules! yank {
    ($in_stack:ident, $in_type:expr) => {
        paste::item! {
            pub fn [< $in_stack _yank >] (state: &mut PushState) {
                if state.int.is_empty() || state.$in_stack.is_empty() {
                    return;
                }
                let in_stack_len = state.$in_stack.len();
                if $in_type == "i128" && in_stack_len < 2 {
                    return;
                }
                // no -1 at the end, handled in the min_max_bounds function
                let idx = min_max_bounds(state.int.pop().unwrap(), in_stack_len);
                let item = state.$in_stack.remove(in_stack_len - idx);
                state.$in_stack.push(item);
            }
        }
    };
}
yank!(int, "i128");
yank!(float, "");
yank!(string, "");
yank!(boolean, "");
yank!(char, "");
yank!(vector_int, "");
yank!(vector_float, "");
yank!(vector_string, "");
yank!(vector_boolean, "");
yank!(vector_char, "");
yank!(code, "");
yank!(exec, "");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;

    #[test]
    fn noop_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        let test_state_copy = test_state.clone();
        code_noop(&mut test_state);
        assert_eq!(test_state, test_state_copy);

        test_state.int = vec![1, 2];
        let test_state_copy = test_state.clone();
        exec_noop(&mut test_state);
        assert_eq!(test_state, test_state_copy);
    }

    #[test]
    fn pop_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        int_pop(&mut test_state);
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::GeneInt(4)];
        code_pop(&mut test_state);
        let empty_vec: Vec<Gene> = vec![];
        assert_eq!(empty_vec, test_state.code);
    }

    #[test]
    fn from_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        code_from_int(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(2)], test_state.code);
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();
        test_state.code.clear();

        test_state.exec.push(Gene::GeneInt(5));
        code_from_exec(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(5)], test_state.code);
    }

    #[test]
    fn dup_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        int_dup(&mut test_state);
        assert_eq!(vec![1, 2, 2], test_state.int);
    }

    #[test]
    fn dup_times_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![3, 2];
        int_dup_times(&mut test_state);
        assert_eq!(vec![3, 3], test_state.int);

        test_state.char = vec!['a', 'b'];
        test_state.int = vec![3];
        char_dup_times(&mut test_state);
        assert_eq!(vec!['a', 'b', 'b', 'b'], test_state.char);
    }

    #[test]
    fn swap_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        int_swap(&mut test_state);
        assert_eq!(vec![2, 1], test_state.int);
    }

    #[test]
    fn rotate_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3];
        int_rotate(&mut test_state);
        assert_eq!(vec![1, 3, 2], test_state.int);
    }

    #[test]
    fn equal_not_equal_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3];
        int_equal(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.int = vec![1, 2, 2];
        int_equal(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn flush_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3];
        int_flush(&mut test_state);
        assert_eq!(Vec::<i128>::new(), test_state.int);
    }

    #[test]
    fn stack_depth_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3];
        int_depth(&mut test_state);
        assert_eq!(vec![1, 2, 3, 3], test_state.int);
    }

    #[test]
    fn yank_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3, 4, 5, 6, 7, 8, 2];
        int_yank(&mut test_state);
        //assert_eq!(vec![1, 2, 3, 4, 5, 7, 8, 6], test_state.int);
    }
}
