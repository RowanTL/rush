use polars::prelude::*;
use rush::gp::args::PushArgs;
use rush::gp::args::SearchDirection;
use rush::gp::gp_loop;
use rush::gp::selection::Selection;
use rush::gp::utils::polars_to_gene;
use rush::instructions::code::exec_if;
use rush::instructions::common::*;
use rush::instructions::numeric::*;
use rush::push::interpreter::interpret_program;
use rush::push::state::{EMPTY_STATE, Gene};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::{Decimal, dec};

fn target_function(x: i64) -> i64 {
    (x * x * x) + (2 * x * x) + 3
}

fn train_data() -> DataFrame {
    let range: Vec<i64> = (-10..=10).collect();
    df!(
        "x0" => range.clone(),
        "y" => range.iter().map(|x| target_function(*x)).collect::<Vec<i64>>(),
    )
    .unwrap()
}

fn instructions() -> Vec<Gene> {
    vec![
        Gene::Place(0),
        Gene::StateFunc(int_add),
        Gene::StateFunc(int_sub),
        Gene::StateFunc(int_mult),
        Gene::StateFunc(int_div),
        Gene::StateFunc(int_dup),
        Gene::StateFunc(int_equal),
        Gene::StateFunc(exec_dup),
        // Gene::StateFunc(exec_if),
        Gene::Close,
        Gene::GeneInt(1),
        Gene::GeneInt(0),
    ]
}

fn error_function(push_args: &PushArgs, data: &DataFrame, push_program: Vec<Gene>) -> Vec<Decimal> {
    let mut error_vec: Vec<Decimal> = vec![];

    let y = data
        .column("y")
        .unwrap()
        .i64()
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

fn main() {
    let mut push_args = PushArgs::new();
    push_args.training_data = Some(train_data());
    push_args.instructions = Some(instructions());
    push_args.simplification_steps = 100;
    push_args.error_function = Some(error_function);
    push_args.parent_selection = Selection::Tournament;
    // push_args.max_generations = 5;
    push_args.elitism = true;
    // push_args.search_direction = SearchDirection::Max;

    gp_loop(push_args);
}
