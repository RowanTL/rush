use std::ops::Not;

use crate::push::state::{Gene, PushState};

use super::common::{code_from_exec, code_pop};

/// Checks to see if a single gene is a block.
fn _is_block(vals: Vec<Gene>) -> Option<bool> {
    Some(match vals[0] {
        Gene::Block(_) => true,
        _ => false,
    })
}
make_instruction_clone!(code, boolean, _is_block, Gene, 1);

/// Checks to see if a single gene is not a block.
fn _is_singular(vals: Vec<Gene>) -> Option<bool> {
    Some(_is_block(vals)?.not())
}
make_instruction_clone!(code, boolean, _is_singular, Gene, 1);

/// Returns the length of a block, else 1 if not a block
fn _length(vals: Vec<Gene>) -> Option<i128> {
    Some(match &vals[0] {
        Gene::Block(x) => x.len() as i128,
        _ => 1,
    })
}
make_instruction_clone!(code, int, _length, Gene, 1);

/// Returns the first item in a block if doable, else None
fn _first(vals: Vec<Gene>) -> Option<Gene> {
    match &vals[0] {
        Gene::Block(x) => {
            if x.len() > 1 {
                Some(x[0].clone())
            } else {
                None
            }
        }
        _ => None,
    }
}
make_instruction_clone!(code, code, _first, Gene, 1);

/// Returns the first item in a block if applicable, else None
fn _last(vals: Vec<Gene>) -> Option<Gene> {
    match &vals[0] {
        Gene::Block(x) => {
            if x.len() > 1 {
                Some(x.last()?.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}
make_instruction_clone!(code, code, _last, Gene, 1);

/// Returns all but the first code item in a block if applicable, else None
fn _rest(vals: Vec<Gene>) -> Option<Gene> {
    match &vals[0] {
        Gene::Block(x) => {
            if x.len() > 1 {
                Some(Gene::Block(Box::new(x[1..].to_vec())))
            } else {
                None
            }
        }
        _ => None,
    }
}
make_instruction_clone!(code, code, _rest, Gene, 1);

/// Returns all but the first code item in a block if applicable, else None
fn _but_last(vals: Vec<Gene>) -> Option<Gene> {
    match &vals[0] {
        Gene::Block(x) => {
            let x_len = x.len();
            if x_len > 1 {
                Some(Gene::Block(Box::new(x[..x_len - 1].to_vec())))
            } else {
                None
            }
        }
        _ => None,
    }
}
make_instruction_clone!(code, code, _but_last, Gene, 1);

/// Returns all the vals wrapped in a code block
fn _wrap_block(vals: Vec<Gene>) -> Option<Gene> {
    Some(Gene::Block(Box::new(vals)))
}
make_instruction_clone!(code, code, _wrap_block, Gene, 1);

/// Combines two genes into one. Accounts for blocks.
/// If the second gene is a block and the first one isn't,
/// appends the first gene to the second gene.
fn _combine(vals: Vec<Gene>) -> Option<Gene> {
    match (&vals[0], &vals[1]) {
        (Gene::Block(x), Gene::Block(y)) => {
            let x_clone = x.clone();
            let mut y_clone = y.clone();
            y_clone.extend(x_clone.into_iter());
            Some(Gene::Block(y_clone))
        }
        (Gene::Block(x), y) => {
            let mut x_clone = x.clone();
            x_clone.push(y.clone());
            Some(Gene::Block(x_clone))
        }
        (x, Gene::Block(y)) => {
            let mut y_clone = y.clone();
            y_clone.push(x.clone());
            Some(Gene::Block(y_clone))
        }
        (x, y) => Some(Gene::Block(Box::new(vec![x.clone(), y.clone()]))),
    }
}
make_instruction_clone!(code, code, _combine, Gene, 2);

/// Pushes `code_pop` and the top item of the code stack to the exec stack.
/// Top code item gets executed before being removed from code stack.
pub fn code_do_then_pop(state: &mut PushState) {
    if state.code.is_empty() {
        return;
    }
    let c = state.code[state.code.len() - 1].clone();
    state.exec.push(Gene::StateFunc(code_pop));
    state.exec.push(c);
}

/// Evaluates the top item on the code stack based off
/// the range of two ints from the int stack.
pub fn code_do_range(state: &mut PushState) {
    if state.code.is_empty() || state.int.len() < 2 {
        return;
    }
    let to_do = state.code.pop().unwrap();
    let dest_idx = state.int.pop().unwrap();
    let current_idx = state.int.pop().unwrap();

    let mut increment = 0;
    if current_idx < dest_idx {
        increment = 1
    } else if current_idx > dest_idx {
        increment = -1
    }

    if increment != 0 {
        state.exec.push(Gene::Block(Box::new(vec![
            Gene::GeneInt(current_idx + increment),
            Gene::GeneInt(dest_idx),
            Gene::StateFunc(code_from_exec),
            to_do.clone(),
            Gene::StateFunc(code_do_range),
        ])));
        state.int.push(current_idx);
        state.exec.push(to_do);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instructions::numeric::int_add,
        push::{interpreter::interpret_program, state::EMPTY_STATE},
    };
    use rust_decimal::dec;

    const STEP_LIMIT: usize = 1000;
    const MAX_STACK_SIZE: usize = 1000;

    #[test]
    fn is_block_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![]))];
        code_is_block(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![(Gene::GeneInt(1))];
        code_is_block(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn is_singular_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![]))];
        code_is_singular(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![(Gene::GeneInt(1))];
        code_is_singular(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn length_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ]))];
        code_length(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::Block(Box::new(vec![]))];
        code_length(&mut test_state);
        assert_eq!(vec![0], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::GeneInt(3)];
        code_length(&mut test_state);
        assert_eq!(vec![1], test_state.int);
    }

    #[test]
    fn first_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ]))];
        code_first(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);

        test_state.code = vec![];
        code_first(&mut test_state);
        let empty_vec: Vec<Gene> = vec![];
        assert_eq!(empty_vec, test_state.code);
        drop(empty_vec);

        test_state.code = vec![Gene::GeneInt(1)];
        code_first(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);
    }

    #[test]
    fn last_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ]))];
        code_last(&mut test_state);
        assert_eq!(vec![Gene::GeneFloat(dec!(3.8))], test_state.code);

        test_state.code = vec![];
        code_last(&mut test_state);
        let empty_vec: Vec<Gene> = vec![];
        assert_eq!(empty_vec, test_state.code);
        drop(empty_vec);

        test_state.code = vec![Gene::GeneInt(1)];
        code_last(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);
    }

    #[test]
    fn rest_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ]))];
        code_rest(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneFloat(dec!(3.8)),
                Gene::GeneBoolean(true)
            ]))],
            test_state.code
        );

        test_state.code = vec![];
        code_rest(&mut test_state);
        let empty_vec: Vec<Gene> = vec![];
        assert_eq!(empty_vec, test_state.code);
        drop(empty_vec);

        test_state.code = vec![Gene::GeneInt(1)];
        code_rest(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);
    }

    #[test]
    fn but_last_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ]))];
        code_but_last(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(3.8)),
            ]))],
            test_state.code
        );

        test_state.code = vec![];
        code_but_last(&mut test_state);
        let empty_vec: Vec<Gene> = vec![];
        assert_eq!(empty_vec, test_state.code);
        drop(empty_vec);

        test_state.code = vec![Gene::GeneInt(1)];
        code_but_last(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);
    }

    #[test]
    fn wrap_block_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::GeneInt(1)];
        code_wrap_block(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![Gene::GeneInt(1)]))],
            test_state.code
        );
    }

    #[test]
    fn combine_test() {
        let mut test_state = EMPTY_STATE;

        test_state
            .code
            .push(Gene::Block(Box::new(vec![Gene::GeneInt(1)])));
        test_state.code.push(Gene::Block(Box::new(vec![
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ])));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(3.8)),
                Gene::GeneBoolean(true),
            ]))],
            test_state.code
        );
        test_state.code.clear();

        test_state
            .code
            .push(Gene::Block(Box::new(vec![Gene::GeneInt(1)])));
        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(4.0)),
            ]))],
            test_state.code
        );
        test_state.code.clear();

        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        test_state
            .code
            .push(Gene::Block(Box::new(vec![Gene::GeneInt(1)])));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(4.0)),
            ]))],
            test_state.code
        );
        test_state.code.clear();

        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        test_state.code.push(Gene::GeneChar('z'));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(Box::new(vec![
                Gene::GeneChar('z'),
                Gene::GeneFloat(dec!(4.0)),
            ]))],
            test_state.code
        );
    }

    #[test]
    fn code_do_then_pop_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code.push(Gene::StateFunc(int_add));
        code_do_then_pop(&mut test_state);
        assert_eq!(vec![Gene::StateFunc(int_add)], test_state.code);
        assert_eq!(
            vec![Gene::StateFunc(code_pop), Gene::StateFunc(int_add)],
            test_state.exec
        );
    }

    #[test]
    fn code_do_range_test() {
        let mut test_state = EMPTY_STATE;

        let code_do_range_addr = format!("0x{:x}", code_do_range as usize);
        let int_add_addr = format!("0x{:x}", int_add as usize);
        let code_from_exec_addr = format!("0x{:x}", code_from_exec as usize);

        test_state.exec = vec![
            Gene::StateFunc(code_do_range),
            Gene::StateFunc(int_add),
            Gene::StateFunc(code_from_exec),
            Gene::GeneInt(6),
            Gene::GeneInt(3),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![18], test_state.int);
    }
}
