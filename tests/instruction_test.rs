use rush::push::state::EMPTY_STATE;
use rush_macro::{instruction_list, run_instruction};

fn iadd(x: i128, y: i128) -> Option<i128> {
    Some(x + y)
}

fn aux_iadd(x: i128, y: i128) -> Option<Vec<i128>> {
    Some(vec![x + y, x - y])
}

fn two_stacks(x: i128, y: i128, cond: bool) -> Option<i128> {
    if cond { Some(x + y) } else { Some(x - y) }
}

fn aux_char(ch: Vec<char>) -> Option<Vec<char>> {
    Some(ch)
}

#[test]
fn run_extract_test() {
    let mut test_state = EMPTY_STATE;

    test_state.int = vec![1, 2];
    run_instruction!(iadd, int, test_state, int, int);
    assert_eq!(vec![3], test_state.int);

    test_state.int = vec![1];
    run_instruction!(iadd, int, test_state, int, int);
    assert_eq!(vec![1], test_state.int);

    // If you're coming from the run_instruction docs, this is
    // the one.
    test_state.int = vec![1, 2];
    run_instruction!(aux_iadd, int, test_state, int, int;);
    assert_eq!(vec![3, 1], test_state.int);

    test_state.int = vec![1, 2];
    test_state.boolean = vec![true];
    run_instruction!(two_stacks, int, test_state, int, int, boolean);

    test_state.vector_char = vec![vec!['a', 'b']];
    run_instruction!(aux_char, char, test_state, vector_char;);
    assert_eq!(vec!['a', 'b'], test_state.char);
}

#[test]
fn instruction_list_test() {}
