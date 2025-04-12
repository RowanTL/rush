use instructions::utils::NumericTrait;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::*;

mod instructions;
mod push;

fn test_func() {}
fn another_test_func() {}

fn main() {
    // let sixth_pi = Decimal::PI / dec!(6.0);
    // let result = dec!(1).sin();
    // let result = Decimal::PI.sin().checked_div(Decimal::PI.cos());
    // let result = dec!(1.0) / Decimal::HALF_PI.sin();
    // let result = sixth_pi.sin();
    // let result = Decimal::HALF_PI.cos();
    // let result = Decimal::PI.sin();
    // let result = Decimal::PI.tan();
    // let result = dec!(1.0) / Decimal::QUARTER_PI.tan();
    // let result = dec!(1.0) / Decimal::QUARTER_PI.cos();
    // let result = dec!(1.2).checked_exp();
    // let result = dec!(2).log10();
    /*let result = vec![0, 1, 2];
    let r_len = result.len();
    let fin_result = &result[..r_len - 1];
    println!("{fin_result:?}");*/

    // println!("{result:?}");
    // println!("{sixth_pi}");

    // casting a function call to a usize is a way to
    // test for function equality.
    // let test_func_result = test_func as usize == test_func as usize;
    // println!("{test_func_result}");

    //let temp_vec = vec![0, 1, 2, 3];
    //temp_vec[0..9].to_vec();

    //let res = 3 % 2;
    //println!("res is {res}");

    let mut test_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    test_vec.drain(..15);
    println!("{:?}", test_vec);
}
