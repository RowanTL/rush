use polars::prelude::*;
use rush::gp::args::PushArgs;
use rush::gp::simplification::auto_simplify_plushy;
use rush::instructions::numeric::*;
use rush::push::state::Gene;
use rush::push::utils::most_genes;
use rust_decimal::{Decimal, dec};

fn test_error_function(
    _push_args: &PushArgs,
    data: &DataFrame,
    _push_program: Vec<Gene>,
) -> Vec<Decimal> {
    let err_vec: Vec<Decimal> = vec![];

    let y = data.column("y").unwrap();
    let x = data.drop("y").unwrap();

    // println!("x: {x:#?}");
    // println!("y: {y:#?}");

    for n in 0..x.height() {
        let mut inputs: Vec<Gene> = Vec::with_capacity(x.width());
        let row = x.get_row(n).unwrap();
        for datum in row.0.iter() {
            // println!("{:?}", val);
            inputs.push(match datum {
                &AnyValue::Int32(val) => Gene::GeneInt(val as i128),
                _ => Gene::Close,
            });
        }
        println!("{:?}", inputs);
        inputs.clear();
    }

    vec![dec!(0.0)]
}

#[test]
fn simplification_function_test() {
    let train_df: DataFrame = df!(
        "x0" => [1, 2, 3],
        "x1" => [7, 8, 9],
        "y" => [8, 10, 12],
    )
    .unwrap();
    println!("{}", train_df);
    // println!("{:#?}", train_df["x0"]);

    // push program declaration
    let push_program: Vec<Gene> = vec![
        Gene::StateFunc(float_tan),
        Gene::StateFunc(float_sub),
        Gene::StateFunc(int_add),
        Gene::StateFunc(float_tan),
        Gene::StateFunc(float_sub),
        Gene::StateFunc(float_rem),
        Gene::StateFunc(float_inc),
    ];

    let mut args = PushArgs::new();
    args.training_data = Some(train_df.clone());
    args.instructions = Some(most_genes());
    args.simplification_steps = 100;
    args.error_function = Some(test_error_function);

    test_error_function(&args, &train_df, push_program);
}
