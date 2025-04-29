use super::common::{code_from_exec, code_pop, int_pop};
use crate::push::state::{Gene, PushState};
use std::ops::Not;

/// Checks to see if a single gene is a block.
fn _is_block(a: Gene) -> Option<bool> {
    Some(matches!(a, Gene::Block(_)))
}

/// Checks to see if a single gene is not a block.
fn _is_singular(a: Gene) -> Option<bool> {
    Some(_is_block(a)?.not())
}

/// Returns the length of a block, else 1 if not a block
fn _length(a: Gene) -> Option<i128> {
    Some(match &a {
        Gene::Block(x) => x.len() as i128,
        _ => 1,
    })
}

/// Returns the first item in a block if doable, else None
fn _first(a: Gene) -> Option<Gene> {
    match &a {
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

/// Returns the first item in a block if applicable, else None
fn _last(a: Gene) -> Option<Gene> {
    match &a {
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

/// Returns all but the first code item in a block if applicable, else None
fn _rest(a: Gene) -> Option<Gene> {
    match &a {
        Gene::Block(x) => {
            if x.len() > 1 {
                Some(Gene::Block(x[1..].to_vec()))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Returns all but the first code item in a block if applicable, else None
fn _but_last(a: Gene) -> Option<Gene> {
    match &a {
        Gene::Block(x) => {
            let x_len = x.len();
            if x_len > 1 {
                Some(Gene::Block(x[..x_len - 1].to_vec()))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Returns a gene wrapped in a block
fn _wrap_block(a: Gene) -> Option<Gene> {
    Some(Gene::Block(vec![a]))
}

/// Combines two genes into one. Accounts for blocks.
/// If the second gene is a block and the first one isn't,
/// appends the first gene to the second gene.
fn _combine(a: Gene, b: Gene) -> Option<Gene> {
    match (&a, &b) {
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
        (x, y) => Some(Gene::Block(vec![x.clone(), y.clone()])),
    }
}

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
        state.exec.push(Gene::Block(vec![
            Gene::GeneInt(current_idx + increment),
            Gene::GeneInt(dest_idx),
            Gene::StateFunc(code_from_exec),
            to_do.clone(),
            Gene::StateFunc(code_do_range),
        ]));
    }
    state.int.push(current_idx);
    state.exec.push(to_do);
}

/// Evaluates the top item on the exec stack based off
/// the range of two ints from the int stack.
pub fn exec_do_range(state: &mut PushState) {
    if state.exec.is_empty() || state.int.len() < 2 {
        return;
    }
    let to_do = state.exec.pop().unwrap();
    let dest_idx = state.int.pop().unwrap();
    let current_idx = state.int.pop().unwrap();

    let mut increment = 0;
    if current_idx < dest_idx {
        increment = 1
    } else if current_idx > dest_idx {
        increment = -1
    }

    if increment != 0 {
        state.exec.push(Gene::Block(vec![
            Gene::GeneInt(current_idx + increment),
            Gene::GeneInt(dest_idx),
            Gene::StateFunc(exec_do_range),
            to_do.clone(),
        ]));
    }
    state.int.push(current_idx);
    state.exec.push(to_do);
}

/// Evaluates the top item on the code stack n times. N pulled from
/// top of int stack.
pub fn code_do_count(state: &mut PushState) {
    if state.code.is_empty() || state.int.is_empty() {
        return;
    }
    if state.int[state.int.len() - 1] < 1 {
        return;
    }
    let code = state.code.pop().unwrap();
    let count = state.int.pop().unwrap();
    state.exec.push(Gene::Block(vec![
        Gene::GeneInt(0),
        Gene::GeneInt(count - 1),
        Gene::StateFunc(code_from_exec),
        code,
        Gene::StateFunc(code_do_range),
    ]));
}

/// Evaluates the top item on the exec stack n times. N pulled from top
/// of int stack.
pub fn exec_do_count(state: &mut PushState) {
    if state.exec.is_empty() || state.int.is_empty() {
        return;
    }
    if state.int[state.int.len() - 1] < 1 {
        return;
    }
    let code = state.exec.pop().unwrap();
    let count = state.int.pop().unwrap();
    state.exec.push(Gene::Block(vec![
        Gene::GeneInt(0),
        Gene::GeneInt(count - 1),
        Gene::StateFunc(exec_do_range),
        code,
    ]));
}

/// Evaluates the top item on the code stack n times but differently that
/// than `code_do_count`. Don't ask, it uses a block for some reason.
pub fn code_do_times(state: &mut PushState) {
    if state.code.is_empty() || state.int.is_empty() {
        return;
    }
    if state.int[state.int.len() - 1] < 1 {
        return;
    }
    let code = state.code.pop().unwrap();
    let times = state.int.pop().unwrap();
    let nested_block = Gene::Block(vec![Gene::StateFunc(int_pop), code]);
    state.exec.push(Gene::Block(vec![
        Gene::GeneInt(0),
        Gene::GeneInt(times - 1),
        Gene::StateFunc(code_from_exec),
        nested_block,
        Gene::StateFunc(code_do_range),
    ]));
}

/// Evaluates the top item on the code stack n times, also different :shrug:
pub fn exec_do_times(state: &mut PushState) {
    if state.exec.is_empty() || state.int.is_empty() {
        return;
    }
    if state.int[state.int.len() - 1] < 1 {
        return;
    }
    let code = state.exec.pop().unwrap();
    let times = state.int.pop().unwrap();
    let nested_block = Gene::Block(vec![Gene::StateFunc(int_pop), code]);
    state.exec.push(Gene::Block(vec![
        Gene::GeneInt(0),
        Gene::GeneInt(times - 1),
        Gene::StateFunc(exec_do_range),
        nested_block,
    ]));
}

/// Evaluates the top item on the exec stack until the top bool isn't true
pub fn exec_while(state: &mut PushState) {
    if state.exec.is_empty() {
        return;
    }
    if state.boolean.is_empty() {
        state.exec.pop().unwrap();
        return;
    }
    let code = state.exec[state.exec.len() - 1].clone();
    if state.boolean.pop().unwrap() {
        state.exec.push(Gene::StateFunc(exec_while));
        state.exec.push(code);
    } else {
        state.exec.pop().unwrap();
    }
}

/// Evaluates the top item on the exec stack at least once until the top bool
/// isn't true
pub fn exec_do_while(state: &mut PushState) {
    if state.exec.is_empty() {
        return;
    }
    let code = state.exec[state.exec.len() - 1].clone();
    state.exec.push(Gene::StateFunc(exec_while));
    state.exec.push(code);
}

/// Evaluates the top exec item for each element of the top block on the code stack.
/// If top item isn't a block, wrapped in one.
pub fn code_map(state: &mut PushState) {
    if state.exec.is_empty() || state.code.is_empty() {
        return;
    }
    let e = state.exec.pop().unwrap();
    let c = state.code.pop().unwrap();
    let c_vec = match c {
        Gene::Block(val) => val,
        val => vec![val],
    };

    let mut contents = Vec::new();

    for item in c_vec.clone().into_iter() {
        let code_block = vec![Gene::StateFunc(code_from_exec), item, e.clone()];
        contents.push(Gene::Block(code_block));
    }

    contents.push(Gene::StateFunc(code_wrap_block));

    for _ in c_vec.into_iter().skip(1) {
        contents.push(Gene::StateFunc(code_combine));
    }

    state.exec.push(Gene::Block(contents));
}

/// If top bool is true, execute top element of code/exec stack and skip the second.
/// If false, execute second element and skip the top.
fn _if(a: Gene, b: Gene, cond: bool) -> Option<Gene> {
    Some(if cond { a } else { b })
}

/// Evaluates the top code item if the top code is true, else pops it.
pub fn code_when(state: &mut PushState) {
    if state.code.is_empty() || state.boolean.is_empty() {
        return;
    }
    let code = state.code.pop().unwrap();
    if state.boolean.pop().unwrap() {
        state.exec.push(code);
    }
}

/// Pops the next item on the exec stack without evaluating it if the top
/// bool is False, otherwise has no effect.
pub fn exec_when(state: &mut PushState) {
    if state.exec.is_empty() || state.boolean.is_empty() {
        return;
    }
    if !state.boolean.pop().unwrap() {
        state.exec.pop();
    }
}

/// Pushes true if the second code item is found within the first item.
/// If the first item isn't a block, coerced into one.
fn _member(a: Gene, b: Gene) -> Option<bool> {
    let block = match b {
        Gene::Block(val) => val,
        val => vec![val],
    };

    Some(block.contains(&a))
}

/// Pushes the nth item of the top element of the code stack.
/// If top code item isn't a block, wrap one around it.
fn _nth(a: Gene, idx: i128) -> Option<Gene> {
    let gene_vec = match a {
        Gene::Block(val) => val,
        val => vec![val],
    };
    let gene_vec_len = gene_vec.len();
    if gene_vec_len == 0 {
        return None;
    }
    let ndx = idx.abs() as usize % gene_vec_len;
    Some(gene_vec[ndx].clone())
}

/// Pushes an empty block to the top of a stack.
fn _make_empty_block<T>() -> Option<Gene> {
    Some(Gene::Block(vec![]))
}

/// Checks to see if the top item on the code/exec stack is an empty block.
/// True if is, False if not.
fn _is_empty_block(a: Gene) -> Option<bool> {
    Some(match a {
        Gene::Block(val) => val.is_empty(),
        _ => false,
    })
}

/// Returns the size of the top item on the code/exec stack.
fn _size(a: Gene) -> Option<i128> {
    Some(match a.clone() {
        Gene::Block(val) => val.len() as i128,
        _ => 1,
    })
}

/// Returns a nested element inside a block based on an int.
fn _extract(a: Gene, idx: i128) -> Option<Gene> {
    match &a {
        block @ Gene::Block(_) => {
            let block_len = block.rec_len();
            if block_len == 0 {
                None
            } else {
                let ndx = (idx % block_len as i128).abs() as usize;
                Some(a.code_at_point(ndx)?)
            }
        }
        _ => Some(a),
    }
}

/// Inserts a gene at a given position in into the top block based off an
/// int from the top of the int stack. The top code item is coerced into a block
/// if needed.
fn _insert(a: Gene, b: Gene, idx: i128) -> Option<Gene> {
    let mut block = match a.clone() {
        iblock @ Gene::Block(_) => iblock,
        val => Gene::Block(vec![val]),
    };
    if block.rec_len() == 0 {
        return _combine(block, b);
    }
    let ndx = idx.abs() as usize % block.rec_len();
    block.with_code_inserted_at_point(b, ndx);
    Some(block)
}

/// Pushes the first position of the 2nd code item within the top code item.
/// If not found, pushes -1. If top code item isn't a block, returns 0 if top
/// two code items equal, -1 otherwise.
fn _first_position(a: Gene, b: Gene) -> Option<i128> {
    let bad_cond: bool = match &a {
        Gene::Block(val) => val.len() == 0,
        _ => true,
    };
    if bad_cond {
        if a == b {
            return Some(0);
        }
    } else {
        match &a {
            Gene::Block(val) => {
                for (idx, el) in val.iter().enumerate() {
                    if el == &b {
                        return Some(idx as i128);
                    }
                }
            }
            _ => panic!("Error: Invariant of only a block failed in _first_position!"),
        }
    }
    Some(-1)
}

/// Reverses the top block. Does nothing if not a block.
fn _reverse(a: Gene) -> Option<Gene> {
    Some(match a {
        Gene::Block(mut val) => {
            val.reverse();
            Gene::Block(val)
        }
        val => val,
    })
}

/// Acts as a noop, does nothing to the state internally.
macro_rules! noop {
    ($stack:ident, $name:ident) => {
        paste::item! {
            pub fn [< $stack $name >] (_state: &mut PushState) {
                ()
            }
        }
    };
}

macro_rules! make_code_instructions {
    ($stack:ident) => {
        make_instruction_new!(_is_block, $stack, boolean, $stack);
        make_instruction_new!(_is_singular, $stack, boolean, $stack);
        make_instruction_new!(_length, $stack, int, $stack);
        make_instruction_new!(_first, $stack, $stack, $stack);
        make_instruction_new!(_last, $stack, $stack, $stack);
        make_instruction_new!(_rest, $stack, $stack, $stack);
        make_instruction_new!(_but_last, $stack, $stack, $stack);
        make_instruction_new!(_wrap_block, $stack, $stack, $stack);
        make_instruction_new!(_combine, $stack, $stack, $stack, $stack);
        make_instruction_new!(_if, $stack, exec, $stack, $stack, boolean);
        make_instruction_new!(_member, $stack, boolean, $stack, $stack);
        make_instruction_new!(_nth, $stack, $stack, $stack, int);
        make_instruction_empty!(_make_empty_block, $stack, $stack, Gene);
        make_instruction_new!(_is_empty_block, $stack, boolean, $stack);
        make_instruction_new!(_size, $stack, int, $stack);
        make_instruction_new!(_extract, $stack, $stack, $stack, int);
        make_instruction_new!(_insert, $stack, $stack, $stack, $stack, int);
        make_instruction_new!(_first_position, $stack, int, $stack, $stack);
        make_instruction_new!(_reverse, $stack, $stack, $stack);
    };
}

macro_rules! all_code_instructions {
    () => {
        make_code_instructions!(code);
        make_code_instructions!(exec);

        // Misc instructions
        noop!(code, _noop);
        noop!(exec, _noop);
        noop!(code, _noop_block);
        noop!(exec, _noop_block);
    };
}
all_code_instructions!();

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

        test_state.code = vec![Gene::Block(vec![])];
        code_is_block(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![Gene::GeneInt(1)];
        code_is_block(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn is_singular_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(vec![])];
        code_is_singular(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![Gene::GeneInt(1)];
        code_is_singular(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn length_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ])];
        code_length(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::Block(vec![])];
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

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ])];
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

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
        ])];
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

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ])];
        code_rest(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneFloat(dec!(3.8)),
                Gene::GeneBoolean(true)
            ])],
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

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ])];
        code_but_last(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(3.8)),
            ])],
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
        assert_eq!(vec![Gene::Block(vec![Gene::GeneInt(1)])], test_state.code);
    }

    #[test]
    fn combine_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code.push(Gene::Block(vec![Gene::GeneInt(1)]));
        test_state.code.push(Gene::Block(vec![
            Gene::GeneFloat(dec!(3.8)),
            Gene::GeneBoolean(true),
        ]));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(3.8)),
                Gene::GeneBoolean(true),
            ])],
            test_state.code
        );
        test_state.code.clear();

        test_state.code.push(Gene::Block(vec![Gene::GeneInt(1)]));
        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(4.0)),
            ])],
            test_state.code
        );
        test_state.code.clear();

        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        test_state.code.push(Gene::Block(vec![Gene::GeneInt(1)]));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneInt(1),
                Gene::GeneFloat(dec!(4.0)),
            ])],
            test_state.code
        );
        test_state.code.clear();

        test_state.code.push(Gene::GeneFloat(dec!(4.0)));
        test_state.code.push(Gene::GeneChar('z'));
        code_combine(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneChar('z'),
                Gene::GeneFloat(dec!(4.0)),
            ])],
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

    #[test]
    fn exec_do_range_test() {
        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(int_add),
            Gene::StateFunc(exec_do_range),
            Gene::GeneInt(5),
            Gene::GeneInt(3),
            Gene::GeneInt(8),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![20], test_state.int);
    }

    #[test]
    fn code_do_count_test() {
        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(code_do_count),
            Gene::StateFunc(int_add),
            Gene::StateFunc(code_from_exec),
            Gene::GeneInt(6),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![15], test_state.int);
    }

    #[test]
    fn exec_do_count_test() {
        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(int_add),
            Gene::StateFunc(exec_do_count),
            Gene::GeneInt(5),
            Gene::GeneInt(3),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![13], test_state.int);
    }

    #[test]
    fn code_do_times_test() {
        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(code_do_times),
            Gene::StateFunc(int_add),
            Gene::StateFunc(code_from_exec),
            Gene::GeneInt(2),
            Gene::GeneInt(4),
            Gene::GeneInt(3),
            Gene::GeneInt(6),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![13], test_state.int);
    }

    #[test]
    fn exec_do_times_test() {
        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(int_add),
            Gene::StateFunc(exec_do_times),
            Gene::GeneInt(7),
            Gene::GeneInt(4),
            Gene::GeneInt(5),
            Gene::GeneInt(3),
        ];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![12], test_state.int);
    }

    #[test]
    fn exec_while_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![false, true, false, true, true, true];
        test_state.int = vec![1, 1, 1, 1];
        test_state.exec = vec![Gene::StateFunc(int_add), Gene::StateFunc(exec_while)];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![4], test_state.int);
        assert_eq!(vec![false, true], test_state.boolean);
        test_state.int.clear();
        test_state.boolean.clear();

        test_state.boolean = vec![false, true, false, true, true, false];
        test_state.int = vec![1, 1, 1, 1];
        test_state.exec = vec![Gene::StateFunc(int_add), Gene::StateFunc(exec_while)];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![1, 1, 1, 1], test_state.int);
    }

    #[test]
    fn exec_do_while_test() {
        let mut test_state = EMPTY_STATE;

        test_state.boolean = vec![false, true, false, true, true, false];
        test_state.int = vec![1, 1, 1, 1];
        test_state.exec = vec![Gene::StateFunc(int_add), Gene::StateFunc(exec_do_while)];
        interpret_program(&mut test_state, STEP_LIMIT, MAX_STACK_SIZE);
        assert_eq!(vec![1, 1, 2], test_state.int);
    }

    #[test]
    fn code_map_test() {
        let mut test_state = EMPTY_STATE;

        // Pulled from pyshgp test in instruction_test_specs.py
        test_state.code = vec![Gene::GeneInt(5)];
        test_state.exec = vec![Gene::GeneInt(-1)];
        code_map(&mut test_state);
        test_state.exec = vec![Gene::Block(vec![Gene::Block(vec![
            Gene::StateFunc(code_from_exec),
            Gene::GeneInt(5),
            Gene::GeneInt(-1),
            Gene::StateFunc(code_wrap_block),
        ])])]
    }

    #[test]
    fn if_test() {
        let mut test_state = EMPTY_STATE;

        // Code tests
        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![true];
        code_if(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.exec);
        test_state.exec.clear();

        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![false];
        code_if(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(0)], test_state.exec);
        test_state.exec.clear();

        // Exec tests
        test_state.exec = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![true];
        exec_if(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.exec);
        test_state.exec.clear();

        test_state.exec = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![false];
        exec_if(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(0)], test_state.exec);
        test_state.exec.clear();
    }

    #[test]
    fn when_test() {
        let mut test_state = EMPTY_STATE;

        // Code stack
        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![true];
        code_when(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.exec);
        test_state.exec.clear();

        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![false];
        code_when(&mut test_state);
        let empty_vec: Vec<Gene> = Vec::new();
        assert_eq!(empty_vec, test_state.exec);
        drop(empty_vec);
        test_state.exec.clear();

        // Exec stack
        test_state.exec = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![true];
        exec_when(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(0), Gene::GeneInt(1)], test_state.exec);

        test_state.exec = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean = vec![false];
        exec_when(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(0)], test_state.exec);

        test_state.exec = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        test_state.boolean.clear(); // testing nothing on this one
        exec_when(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(0), Gene::GeneInt(1)], test_state.exec);
    }

    #[test]
    fn code_exec_member_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![
            Gene::Block(vec![
                Gene::GeneInt(0),
                Gene::GeneInt(4),
                Gene::StateFunc(exec_do_range),
            ]),
            Gene::GeneInt(0),
        ];
        code_member(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![
            Gene::Block(vec![
                Gene::GeneInt(5),
                Gene::GeneInt(4),
                Gene::StateFunc(exec_do_range),
            ]),
            Gene::GeneInt(0),
        ];
        code_member(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(0)];
        code_member(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![Gene::GeneInt(0), Gene::GeneInt(1)];
        code_member(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn code_exec_nth_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(0),
            Gene::GeneInt(1),
            Gene::GeneInt(2),
        ])];
        test_state.int = vec![1];
        code_nth(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);

        test_state.code = vec![Gene::GeneInt(1)];
        test_state.int = vec![0];
        code_nth(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneInt(0),
            Gene::GeneInt(1),
            Gene::GeneInt(2),
        ])];
        test_state.int = vec![4];
        code_nth(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);
    }

    #[test]
    fn code_exec_make_empty_block_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::GeneInt(0)];
        code_make_empty_block(&mut test_state);
        let empty_vec: Vec<Gene> = Vec::new();
        assert_eq!(
            vec![Gene::GeneInt(0), Gene::Block(empty_vec.clone())],
            test_state.code
        );
        test_state.code.clear();

        exec_make_empty_block(&mut test_state);
        assert_eq!(empty_vec, test_state.code);
    }

    #[test]
    fn code_exec_is_empty_block_test() {
        let mut test_state = EMPTY_STATE;

        // code
        test_state.code = vec![Gene::GeneInt(0)];
        code_is_empty_block(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        let empty_vec: Vec<Gene> = Vec::new();
        test_state.code = vec![Gene::Block(empty_vec.clone())];
        code_is_empty_block(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.code = vec![Gene::Block(vec![Gene::GeneBoolean(false)])];
        code_is_empty_block(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        // exec
        test_state.exec = vec![Gene::GeneInt(0)];
        exec_is_empty_block(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        let empty_vec: Vec<Gene> = Vec::new();
        test_state.exec = vec![Gene::Block(empty_vec.clone())];
        exec_is_empty_block(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
    }

    #[test]
    fn code_exec_size_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::GeneInt(1)];
        code_size(&mut test_state);
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneBoolean(false),
            Gene::GeneInt(42),
        ])];
        code_size(&mut test_state);
        assert_eq!(vec![2], test_state.int);
    }

    #[test]
    fn extract_test() {
        let mut test_state = EMPTY_STATE;

        let test_code = vec![Gene::Block(vec![
            Gene::GeneBoolean(true),
            Gene::GeneInt(1),
            Gene::Block(vec![
                Gene::GeneInt(4),
                Gene::GeneFloat(dec!(6.0)),
                Gene::Block(vec![Gene::GeneString(vec!['t'])]),
            ]),
            Gene::GeneInt(10),
            Gene::Block(vec![Gene::GeneBoolean(false)]),
        ])];

        test_state.code = test_code.clone();
        test_state.int = vec![0];
        code_member(&mut test_state);
        assert_eq!(test_code.clone(), test_state.code);

        test_state.code = test_code.clone();
        test_state.int = vec![2];
        code_extract(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);

        test_state.code = test_code.clone();
        test_state.int = vec![4];
        code_extract(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(4)], test_state.code);

        test_state.code = test_code.clone();
        test_state.int = vec![9];
        code_extract(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![Gene::GeneBoolean(false)])],
            test_state.code
        );
    }

    #[test]
    fn insert_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![
            Gene::GeneInt(20),
            Gene::Block(vec![
                Gene::GeneInt(1),
                Gene::Block(vec![Gene::GeneInt(1), Gene::GeneInt(1)]),
            ]),
        ];
        test_state.int = vec![1];
        let inserted_block = vec![Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneInt(20),
            Gene::Block(vec![Gene::GeneInt(1), Gene::GeneInt(1)]),
        ])];
        code_insert(&mut test_state);
        assert_eq!(inserted_block, test_state.code);

        test_state.code = vec![
            Gene::GeneInt(20),
            Gene::Block(vec![
                Gene::GeneBoolean(true),
                Gene::GeneInt(1),
                Gene::Block(vec![
                    Gene::GeneInt(4),
                    Gene::GeneFloat(dec!(6.0)),
                    Gene::Block(vec![Gene::GeneString(vec!['t'])]),
                ]),
                Gene::GeneInt(10),
                Gene::Block(vec![Gene::GeneBoolean(false)]),
            ]),
        ];
        test_state.int = vec![4];
        let inserted_block = vec![Gene::Block(vec![
            Gene::GeneBoolean(true),
            Gene::GeneInt(1),
            Gene::Block(vec![
                Gene::GeneInt(4),
                Gene::GeneInt(20),
                Gene::GeneFloat(dec!(6.0)),
                Gene::Block(vec![Gene::GeneString(vec!['t'])]),
            ]),
            Gene::GeneInt(10),
            Gene::Block(vec![Gene::GeneBoolean(false)]),
        ])];
        code_insert(&mut test_state);
        assert_eq!(inserted_block, test_state.code);
    }

    #[test]
    fn first_position_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![
            Gene::GeneInt(20),
            Gene::Block(vec![
                Gene::GeneBoolean(true),
                Gene::GeneInt(1),
                Gene::GeneInt(20),
                Gene::GeneFloat(dec!(6.0)),
            ]),
        ];
        code_first_position(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::GeneBoolean(false), Gene::GeneBoolean(false)];
        code_first_position(&mut test_state);
        assert_eq!(vec![0], test_state.int);
        test_state.int.clear();

        test_state.code = vec![Gene::GeneBoolean(false), Gene::GeneBoolean(true)];
        code_first_position(&mut test_state);
        assert_eq!(vec![-1], test_state.int);
        test_state.int.clear();

        test_state.code = vec![
            Gene::GeneInt(-6),
            Gene::Block(vec![
                Gene::GeneBoolean(true),
                Gene::GeneInt(1),
                Gene::GeneInt(20),
                Gene::GeneFloat(dec!(6.0)),
            ]),
        ];
        code_first_position(&mut test_state);
        assert_eq!(vec![-1], test_state.int);
        test_state.int.clear();
    }

    #[test]
    fn code_reverse_test() {
        let mut test_state = EMPTY_STATE;

        test_state.code = vec![Gene::GeneInt(1)];
        code_reverse(&mut test_state);
        assert_eq!(vec![Gene::GeneInt(1)], test_state.code);

        test_state.code = vec![Gene::Block(vec![
            Gene::GeneBoolean(false),
            Gene::GeneInt(1),
            Gene::GeneInt(20),
        ])];
        code_reverse(&mut test_state);
        assert_eq!(
            vec![Gene::Block(vec![
                Gene::GeneInt(20),
                Gene::GeneInt(1),
                Gene::GeneBoolean(false),
            ])],
            test_state.code
        );
    }

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
}
