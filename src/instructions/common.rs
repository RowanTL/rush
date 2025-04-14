use rust_decimal::Decimal;

use crate::push::state::{Gene, PushState};

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
}
