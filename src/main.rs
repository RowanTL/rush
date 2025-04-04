use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

mod instructions;
mod push;

fn main() {
    // let arr: Vec<i32> = vec![];
    // let slice = &arr[..2];
    // println!("{:?}", slice);

    // let arr: Vec<Decimal> = vec![dec!(2.2), dec!(1.1)];
    // println!("{arr:?}");

    // let result = dec!(1.0) / dec!(0.0);
    let result = dec!(2.7) % dec!(1.2);
    println!("{result}");
}
