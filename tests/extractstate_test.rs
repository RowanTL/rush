use rush::push::state::EMPTY_STATE;
use rush_macro::run_instruction;

fn iadd(x: i128, y: i128) -> i128 {
    x + y
}

#[test]
fn run_extract_test() {
    let mut test_state = EMPTY_STATE;

    test_state.int = vec![1, 2];
    run_instruction!(iadd, int, test_state, int, int);
    assert_eq!(vec![3], test_state.int);

    /*test_state.int = vec![1];
    run_instruction!(iadd, int, test_state, int, int);
    assert_eq!(vec![1], test_state.int);*/
}
