use rush::gp::individual::Individual;
use rush::instructions::code::{code_but_last, code_combine};
use rush::push::state::Gene;

#[test]
fn run_display_test() {
    let ind: Individual = Individual {
        plushy: vec![
            Gene::GeneInt(1),
            Gene::StateFunc(code_but_last),
            Gene::Block(vec![Gene::GeneBoolean(true), Gene::StateFunc(code_combine)]),
            Gene::GeneInt(9),
        ],
        push_program: None,
        total_fitness: None,
        fitness_cases: None,
    };

    println!("{}", ind);
}
