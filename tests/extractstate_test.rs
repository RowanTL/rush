use rush::push::state::EMPTY_STATE;
use rush_macro::run_canextract;

#[test]
fn run_extract_test() {
    let mut test_state = EMPTY_STATE;

    test_state.int = vec![1, 2];
    let res = run_canextract!(test_state, int, int, float);
    assert_eq!(false, res);

    test_state.int = vec![1, 2];
    let res = run_canextract!(test_state, int, int);
    assert_eq!(true, res);
}
