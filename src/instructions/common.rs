use std::cmp::{max, min};

use crate::push::state::{Gene, PushState};

fn min_max_bounds(ndx: i128, length: usize) -> usize {
    max(0, min(ndx.unsigned_abs() as usize, length - 1))
}

/// Pops the top value from the stack
macro_rules! pop {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _pop >] (state: &mut PushState) {
                state.$in_stack.pop();
            }
        }
    };
}

macro_rules! gene_map {
    (int, $val:ident) => {
        Gene::GeneInt($val)
    };
    (float, $val:ident) => {
        Gene::GeneFloat($val)
    };
    (string, $val:ident) => {
        Gene::GeneString($val)
    };
    (boolean, $val:ident) => {
        Gene::GeneBoolean($val)
    };
    (char, $val:ident) => {
        Gene::GeneChar($val)
    };
    (vector_int, $val:ident) => {
        Gene::GeneVectorInt($val)
    };
    (vector_float, $val:ident) => {
        Gene::GeneVectorFloat($val)
    };
    (vector_string, $val:ident) => {
        Gene::GeneVectorString($val)
    };
    (vector_boolean, $val:ident) => {
        Gene::GeneVectorBoolean($val)
    };
    (vector_char, $val:ident) => {
        Gene::GeneVectorChar($val)
    };
    (code, $val:ident) => {
        $val
    };
    (exec, $val:ident) => {
        $val
    };
}

/// Wraps a type in its respective Gene
macro_rules! make_code {
    ($stack:ident) => {
        paste::item! {
            pub fn [< code_from_ $stack >] (state: &mut PushState) {
                if let Some(val) = state.$stack.pop() {
                    let push_val = gene_map!($stack, val);
                    state.code.push(push_val);
                }
            }
        }
    };
}

/// Duplicates an item
pub fn _dup<T: Clone>(val: T) -> Option<Vec<T>> {
    Some(vec![val.clone(), val])
}

pub fn _dup_times<T: Clone>(amt: i128, val: T) -> Option<Vec<T>> {
    Some(vec![val; amt as usize])
}

/// Swaps two values
pub fn _swap<T: Clone>(a: T, b: T) -> Option<Vec<T>> {
    Some(vec![a, b])
}

/// Rotates three values
pub fn _rotate<T>(a: T, b: T, c: T) -> Option<Vec<T>> {
    Some(vec![c, a, b])
}

/// Checks if two values are equal
pub fn _equal<T: Eq>(a: T, b: T) -> Option<bool> {
    Some(b == a)
}

/// Checks if two values are not equal
pub fn _not_equal<T: Clone + Eq>(a: T, b: T) -> Option<bool> {
    Some(b != a)
}

/// Removes all values from a stack
macro_rules! flush_state {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _flush >] (state: &mut PushState) {
                state.$in_stack.clear();
            }
        }
    };
}

/// Returns the depth of a stack
macro_rules! stack_depth {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _depth >] (state: &mut PushState) {
                state.int.push(state.$in_stack.len() as i128)
            }
        }
    };
}

macro_rules! yank {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _yank >] (state: &mut PushState) {
                if state.int.is_empty() || state.$in_stack.is_empty() {
                    return;
                }
                let in_stack_name = stringify!($in_stack);
                if in_stack_name == "int" && state.$in_stack.len() < 2 {
                    return;
                }
                // no -1 from in_stack_len, 1 subtracted within the min_max_bounds function
                let pre_idx = state.int.pop().unwrap();
                let idx = min_max_bounds(pre_idx, state.$in_stack.len());
                let item = state.$in_stack.remove(state.$in_stack.len() - 1 - idx);
                state.$in_stack.push(item);
            }
        }
    };
}

macro_rules! yank_dup {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _yank_dup >] (state: &mut PushState) {
                if state.int.is_empty() || state.$in_stack.is_empty() {
                    return;
                }
                let in_stack_name = stringify!($in_stack);
                if in_stack_name == "int" && state.$in_stack.len() < 2 {
                    return;
                }
                // no -1 from in_stack_len, 1 subtracted within the min_max_bounds function
                let pre_idx = state.int.pop().unwrap();
                let idx = min_max_bounds(pre_idx, state.$in_stack.len());
                let item = state.$in_stack[state.$in_stack.len() - 1 - idx].clone();
                state.$in_stack.push(item);
            }
        }
    };
}

macro_rules! shove {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _shove >] (state: &mut PushState) {
                if state.int.is_empty() || state.$in_stack.is_empty() {
                    return;
                }
                let in_stack_name = stringify!($in_stack);
                if in_stack_name == "int" && state.$in_stack.len() < 2 {
                    return;
                }
                let pre_idx = state.int.pop().unwrap();
                let idx = min_max_bounds(pre_idx, state.$in_stack.len());
                let item = state.$in_stack.pop().unwrap();
                state.$in_stack.insert(state.$in_stack.len() - idx, item);
            }
        }
    };
}

macro_rules! shove_dup {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _shove_dup >] (state: &mut PushState) {
                if state.int.is_empty() || state.$in_stack.is_empty() {
                    return;
                }
                let in_stack_name = stringify!($in_stack);
                if in_stack_name == "int" && state.$in_stack.len() < 2 {
                    return;
                }
                let pre_idx = state.int.pop().unwrap();
                let idx = min_max_bounds(pre_idx, state.$in_stack.len());
                let item = state.$in_stack[state.$in_stack.len() - 1].clone();
                state.$in_stack.insert(state.$in_stack.len() - idx, item);
            }
        }
    };
}

macro_rules! is_state_empty {
    ($in_stack:ident) => {
        paste::item! {
            pub fn [< $in_stack _is_empty >] (state: &mut PushState) {
                state.boolean.push(state.$in_stack.is_empty());
            }
        }
    };
}

macro_rules! make_common_instructions {
    ($stack:ident) => {
        pop!($stack);
        make_code!($stack);
        make_instruction_new_aux!(_dup, $stack, $stack, $stack);
        make_instruction_new_aux!(_dup_times, $stack, $stack, int, $stack);
        make_instruction_new_aux!(_swap, $stack, $stack, $stack, $stack);
        make_instruction_new_aux!(_rotate, $stack, $stack, $stack, $stack, $stack);
        make_instruction_new!(_equal, $stack, boolean, $stack, $stack);
        flush_state!($stack);
        stack_depth!($stack);
        yank!($stack);
        yank_dup!($stack);
        shove!($stack);
        shove_dup!($stack);
        is_state_empty!($stack);
    };
}

make_common_instructions!(int);
make_common_instructions!(float);
make_common_instructions!(string);
make_common_instructions!(boolean);
make_common_instructions!(char);
make_common_instructions!(vector_int);
make_common_instructions!(vector_float);
make_common_instructions!(vector_string);
make_common_instructions!(vector_boolean);
make_common_instructions!(vector_char);
make_common_instructions!(code);
make_common_instructions!(exec);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;

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
        assert_eq!(vec![1, 2, 3, 4, 5, 7, 8, 6], test_state.int);

        test_state.int = vec![1, 2];
        int_yank(&mut test_state);
        assert_eq!(vec![1], test_state.int);

        test_state.int = vec![0];
        test_state.boolean = vec![true, true, true, false];
        boolean_yank(&mut test_state);
        assert_eq!(vec![true, true, true, false], test_state.boolean);
    }

    #[test]
    fn yank_dup_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3, 4, 5, 6, 7, 8, 2];
        int_yank_dup(&mut test_state);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 6], test_state.int);

        test_state.int = vec![1, 2];
        int_yank_dup(&mut test_state);
        assert_eq!(vec![1, 1], test_state.int);

        test_state.int = vec![0];
        test_state.boolean = vec![true, true, true, false];
        boolean_yank_dup(&mut test_state);
        assert_eq!(vec![true, true, true, false, false], test_state.boolean);

        test_state.int = vec![0];
        int_yank_dup(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    #[test]
    fn shove_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2, 3, 4, 5, 1];
        int_shove(&mut test_state);
        assert_eq!(vec![1, 2, 3, 5, 4], test_state.int);

        test_state.int = vec![1, 2, 3, 4, 5, 0];
        int_shove(&mut test_state);
        assert_eq!(vec![1, 2, 3, 4, 5], test_state.int);

        test_state.int = vec![-1];
        test_state.boolean = vec![true, true, true, false];
        boolean_shove(&mut test_state);
        assert_eq!(vec![true, true, false, true], test_state.boolean);
    }

    #[test]
    fn shove_dup_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 1, 2, 3, 2];
        int_shove_dup(&mut test_state);
        assert_eq!(vec![1, 1, 3, 2, 3], test_state.int);

        test_state.int = vec![1, 2, 3, 4, 5, 1];
        int_shove_dup(&mut test_state);
        assert_eq!(vec![1, 2, 3, 4, 5, 5], test_state.int);

        test_state.int = vec![1, 2, 3, 4, 5, 0];
        int_shove_dup(&mut test_state);
        assert_eq!(vec![1, 2, 3, 4, 5, 5], test_state.int);

        test_state.int = vec![-1];
        test_state.boolean = vec![true, true, true, false];
        boolean_shove_dup(&mut test_state);
        assert_eq!(vec![true, true, true, false, false], test_state.boolean);
    }

    #[test]
    fn is_state_empty_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        int_is_empty(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.int = vec![];
        int_is_empty(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }
}
