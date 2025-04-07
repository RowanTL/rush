use crate::push::state::{Gene, PushState};

/// Acts as a NoOp, does nothing with the vals list.
fn _noop<T>(_: Vec<T>) -> Option<T> {
    None
}
make_instruction_clone!(code, code, _noop, Gene, 0);
make_instruction_clone!(exec, exec, _noop, Gene, 0);

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
}
