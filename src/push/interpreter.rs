use crate::push::state::*;

pub fn gene_to_stack(state: &mut PushState, gene: Gene) {
    match gene {
        Gene::GeneInt(x) => state.int.push(x),
        Gene::GeneFloat(x) => state.float.push(x),
        Gene::GeneBoolean(x) => state.boolean.push(x),
        Gene::GeneString(x) => state.string.push(x),
        Gene::GeneChar(x) => state.char.push(x),
        Gene::GeneVectorInt(x) => state.vector_int.push(x),
        Gene::GeneVectorFloat(x) => state.vector_float.push(x),
        Gene::GeneVectorBoolean(x) => state.vector_boolean.push(x),
        Gene::GeneVectorString(x) => state.vector_string.push(x),
        Gene::GeneVectorChar(x) => state.vector_char.push(x),
        Gene::StateFunc(func) => func(state),
        Gene::Block(x) => state.exec.extend(x.into_iter()),
        Gene::Close => panic!("Close found in the exec stack, this should not happen!"),
        Gene::Open(_) => panic!("Open found in the exec stack, this should not happen!"),
        Gene::Skip => panic!("Skip found in the exec stack, this should not happen!"),
        Gene::CrossoverPadding => {
            panic!("CrossoverPadding found in the exec stack, this should not happen!")
        }
    }
}

pub fn interpret_program(state: &mut PushState, step_limit: usize, max_stack_size: isize) {
    let mut steps: usize = 0;
    while state.exec.len() > 0 && steps < step_limit {
        if let Some(val) = state.exec.pop() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn gene_to_stack_test() {
        let mut test_state = EMPTY_STATE;

        gene_to_stack(&mut test_state, Gene::GeneInt(1));
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();

        gene_to_stack(&mut test_state, Gene::GeneFloat(dec!(1.2)));
        gene_to_stack(&mut test_state, Gene::GeneFloat(dec!(2.4)));
        assert_eq!(vec![dec!(1.2), dec!(2.4)], test_state.float);
        test_state.float.clear();

        gene_to_stack(&mut test_state, Gene::GeneBoolean(true));
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        // Need to finish these tests later.
    }
}
