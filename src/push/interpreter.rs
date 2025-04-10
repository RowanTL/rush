use crate::push::state::*;

/// The main function that disperses the exec stack Genes into
/// the respective stacks. Also is where the individual instructions
/// (such as int_add) is ran.
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
        Gene::Block(x) => state.exec.extend(x.into_iter().rev()),
        Gene::Close => panic!("Close found in the exec stack, this should not happen!"),
        Gene::Open(_) => panic!("Open found in the exec stack, this should not happen!"),
        Gene::Skip => panic!("Skip found in the exec stack, this should not happen!"),
        Gene::CrossoverPadding => {
            panic!("CrossoverPadding found in the exec stack, this should not happen!")
        }
    }
}

/// Where a push program's exec stack is interpreted to completion.
/// TODO: Decide where to place loading in a push program.
pub fn interpret_program(state: &mut PushState, step_limit: usize, max_stack_size: usize) {
    let mut steps: usize = 0;
    while state.exec.len() > 0 && steps < step_limit {
        if let Some(gene) = state.exec.pop() {
            gene_to_stack(state, gene);
            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::numeric::int_add;

    use super::*;
    use rust_decimal::dec;
    use crate::instructions::common::code_from_exec;

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

        gene_to_stack(&mut test_state, Gene::GeneString(vec!['t', 'e', 's', 't']));
        assert_eq!(vec![vec!['t', 'e', 's', 't']], test_state.string);
        test_state.string.clear();

        gene_to_stack(&mut test_state, Gene::GeneChar('a'));
        gene_to_stack(&mut test_state, Gene::GeneChar('b'));
        gene_to_stack(&mut test_state, Gene::GeneChar('c'));
        assert_eq!(vec!['a', 'b', 'c'], test_state.char);
        test_state.char.clear();

        gene_to_stack(&mut test_state, Gene::GeneVectorInt(vec![1, 2, 3]));
        gene_to_stack(&mut test_state, Gene::GeneVectorInt(vec![4, 5, 6]));
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6]], test_state.vector_int);
        test_state.vector_int.clear();

        gene_to_stack(
            &mut test_state,
            Gene::GeneVectorFloat(vec![dec!(1.7), dec!(2.4), dec!(3.9)]),
        );
        gene_to_stack(
            &mut test_state,
            Gene::GeneVectorFloat(vec![dec!(4.7), dec!(5.4), dec!(6.9)]),
        );
        assert_eq!(
            vec![
                vec![dec!(1.7), dec!(2.4), dec!(3.9)],
                vec![dec!(4.7), dec!(5.4), dec!(6.9)]
            ],
            test_state.vector_float
        );
        test_state.vector_float.clear();

        gene_to_stack(&mut test_state, Gene::GeneVectorBoolean(vec![true, false]));
        assert_eq!(vec![vec![true, false]], test_state.vector_boolean);
        test_state.vector_boolean.clear();

        gene_to_stack(
            &mut test_state,
            Gene::GeneVectorString(vec![vec!['t', 'e', 's', 't', '0']]),
        );
        gene_to_stack(
            &mut test_state,
            Gene::GeneVectorString(vec![
                vec!['t', 'e', 's', 't', '1'],
                vec!['t', 'e', 's', 't', '2'],
            ]),
        );
        assert_eq!(
            vec![
                vec![vec!['t', 'e', 's', 't', '0']],
                vec![vec!['t', 'e', 's', 't', '1'], vec!['t', 'e', 's', 't', '2']]
            ],
            test_state.vector_string
        );
        test_state.vector_string.clear();

        gene_to_stack(&mut test_state, Gene::GeneVectorChar(vec!['a', 'b']));
        gene_to_stack(&mut test_state, Gene::GeneVectorChar(vec!['b', 'c', 'd']));
        assert_eq!(
            vec![vec!['a', 'b'], vec!['b', 'c', 'd']],
            test_state.vector_char
        );
        test_state.vector_char.clear();

        let test_block: Gene = Gene::Block(Box::new(vec![
            Gene::GeneInt(1),
            Gene::GeneFloat(dec!(2.3)),
            Gene::StateFunc(int_add),
        ]));
        test_state.exec.push(Gene::GeneInt(2));
        gene_to_stack(&mut test_state, test_block);
        assert_eq!(
            //vec![
            //    Gene::GeneInt(2),
            //    Gene::GeneInt(1),
            //    Gene::GeneFloat(dec!(2.3)),
            //    Gene::StateFunc(int_add)
            //],
            vec![
                Gene::GeneInt(2),
                Gene::StateFunc(int_add),
                Gene::GeneFloat(dec!(2.3)),
                Gene::GeneInt(1),
            ],
            test_state.exec
        );
    }

    #[test]
    fn interpret_program_test() {
        use crate::instructions::numeric::int_add;

        let mut test_state = EMPTY_STATE;

        test_state.exec = vec![
            Gene::StateFunc(int_add),
            Gene::StateFunc(int_add),
            Gene::GeneInt(2),
            Gene::GeneInt(3),
            Gene::GeneInt(4),
        ];
        interpret_program(&mut test_state, 1000, 1000);
        assert_eq!(vec![9], test_state.int);
    }
}
