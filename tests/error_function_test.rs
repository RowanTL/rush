use polars::prelude::*;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rush::gp::args::SearchDirection;
use rush::gp::individual::Individual;
use rush::gp::selection::{Selection, select_parent};
use rush::gp::simplification::auto_simplify_plushy;
use rush::gp::utils::polars_to_gene;
use rush::instructions::numeric::*;
use rush::push::interpreter::interpret_program;
use rush::push::state::Gene;
use rush::push::utils::most_genes;
use rush::{gp::args::PushArgs, push::state::EMPTY_STATE};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::{Decimal, dec};

/// This is a prototype for an error function. I'm hoping to have some of this
/// refined later.
/// Will leave this for testing purposes.
fn test_error_function(
    push_args: &PushArgs,
    data: &DataFrame,
    push_program: Vec<Gene>,
) -> Vec<Decimal> {
    let mut error_vec: Vec<Decimal> = vec![];

    let y = data
        .column("y")
        .unwrap()
        .i32()
        .unwrap()
        .into_iter()
        .map(|opt| opt.map(|v| v as i128))
        .collect::<Option<Vec<_>>>()
        .unwrap(); // How to convert a series to a vector everybody
    let x = data.drop("y").unwrap();

    for n in 0..x.height() {
        let mut state = EMPTY_STATE;
        let mut inputs: Vec<Gene> = Vec::with_capacity(x.width());
        let row = x.get_row(n).unwrap();
        for datum in row.0.iter() {
            inputs.push(polars_to_gene(datum))
        }
        state.exec.extend(push_program.clone().into_iter()); // load the program
        state.input.extend(inputs.clone().into_iter()); // Make inputs available to the state
        interpret_program(&mut state, push_args.step_limit, push_args.max_stack_size);
        if let Some(top_int) = state.int.pop() {
            error_vec.push(Decimal::from_i128((y[n] - top_int).abs()).unwrap());
        } else {
            error_vec.push(dec!(999999.0)) // super large error if no stack item.
        }
    }

    error_vec
}

fn make_train_df() -> DataFrame {
    df!(
        "x0" => [4, 5, 6],
        "x1" => [7, 8, 9],
        "y" => [11, 13, 15],
    )
    .unwrap()
}

#[test]
fn simplification_function_test() {
    let train_df: DataFrame = make_train_df();

    // push program declaration
    let push_program: Vec<Gene> = vec![
        Gene::StateFunc(int_inc),   // Should get simplified out
        Gene::StateFunc(float_tan), // along with all these float instructions
        Gene::StateFunc(float_sub),
        Gene::StateFunc(int_add), // stays
        Gene::StateFunc(float_tan),
        Gene::StateFunc(float_sub),
        Gene::StateFunc(float_rem),
        Gene::StateFunc(float_inc),
        Gene::Place(0), // stays
        Gene::Place(1), // stays
    ];

    // Assuming minimization problem here
    let mut args = PushArgs::new();
    args.training_data = Some(train_df.clone());
    args.instructions = Some(most_genes());
    args.simplification_steps = 100;
    args.error_function = Some(test_error_function);

    // test_error_function(&args, &train_df, push_program);

    // test the auto simplification here
    let simplified_genome = auto_simplify_plushy(push_program, args.error_function.unwrap(), &args);
    assert_eq!(
        vec![Gene::StateFunc(int_add), Gene::Place(0), Gene::Place(1)],
        simplified_genome
    )
}

#[test]
fn tournament_selection_test() {
    let mut rng = StdRng::seed_from_u64(42);

    let train_df: DataFrame = make_train_df();

    let mut args = PushArgs::new();
    args.training_data = Some(train_df.clone());
    args.instructions = Some(most_genes());
    args.simplification_steps = 100;
    args.error_function = Some(test_error_function);
    args.parent_selection = Selection::Tournament;

    let mut individuals: Vec<Individual> = Vec::with_capacity(5);

    let plushy = vec![
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_add),
        Gene::GeneInt(9999),
        Gene::Place(0),
        Gene::Place(1),
    ];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_add),
        Gene::GeneInt(9000),
        Gene::Place(0),
        Gene::Place(1),
    ];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_add),
        Gene::GeneInt(420),
        Gene::Place(0),
        Gene::Place(1),
    ];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_add),
        Gene::GeneInt(69),
        Gene::Place(0),
        Gene::Place(1),
    ];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_add),
        Gene::GeneInt(42),
        Gene::Place(0),
        Gene::Place(1),
    ];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    // For minimization problem.
    let winning_ind = select_parent(individuals.clone(), &args, &mut rng);
    assert_eq!(Some(dec!(126.0)), winning_ind.total_fitness);

    // For maximization problem.
    args.search_direction = SearchDirection::Max;
    let winning_ind = select_parent(individuals.clone(), &args, &mut rng);
    assert_eq!(Some(dec!(29997.0)), winning_ind.total_fitness);
}

#[test]
fn lexicase_selection_test() {
    let mut rng = StdRng::seed_from_u64(42);

    let train_df = df!(
        "x0" => [10, 20, 30],
        "x1" => [40, 50, 60],
        "x2" => [70, 80, 90],
        "y" => [10, 50, 90],
    )
    .unwrap();

    let mut args = PushArgs::new();
    args.training_data = Some(train_df.clone());
    args.instructions = Some(most_genes());
    args.simplification_steps = 100;
    args.error_function = Some(test_error_function);
    args.parent_selection = Selection::Lexicase;

    let mut individuals: Vec<Individual> = Vec::with_capacity(5);

    let plushy = vec![Gene::Place(0)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![Gene::Place(1)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![Gene::Place(2)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![Gene::Place(0)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let plushy = vec![Gene::Place(1)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let winning_ind = select_parent(individuals.clone(), &args, &mut rng);
    assert_eq!(
        Individual::with_error(vec![Gene::Place(2)], test_error_function, &args, &train_df),
        winning_ind
    );

    let plushy = vec![Gene::Place(2)];
    let individual = Individual::with_error(plushy, test_error_function, &args, &train_df);
    individuals.push(individual);

    let winning_ind = select_parent(individuals.clone(), &args, &mut rng);
    assert_eq!(
        Individual::with_error(vec![Gene::Place(1)], test_error_function, &args, &train_df),
        winning_ind
    );
}
