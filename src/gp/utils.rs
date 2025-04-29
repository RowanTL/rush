use rand::Rng;
use rust_decimal::prelude::*;

pub fn gaussian_noise_factor(rng: &mut impl Rng) -> Decimal {
    let u0f64: f64 = rng.random();
    let u1f64: f64 = rng.random();

    let u0: Decimal = FromPrimitive::from_f64(u0f64).unwrap();
    let u1: Decimal = FromPrimitive::from_f64(u1f64).unwrap();

    let u0 = if u0 == dec!(0.0) {
        FromPrimitive::from_f64(f64::EPSILON).unwrap()
    } else {
        u0
    };

    (dec!(-2.0) * u0.ln()).sqrt().unwrap() * (dec!(2.0) * rust_decimal::Decimal::PI * u1).cos()
}
