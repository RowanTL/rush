use rush_macro::run_func;

fn uadd(x: usize, y: usize, z: usize) -> usize {
    x + y + z
}

#[test]
fn run_func_test() {
    let res = run_func!(uadd, 1, 4, 2);
    assert_eq!(res, 7);
}
