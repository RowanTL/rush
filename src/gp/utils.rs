use crate::gp::args::ClosingType;
use crate::gp::genome::OPEN_MAP;
use crate::push::state::Gene;
use polars::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;
use rust_decimal::prelude::*;
use std::cmp::Ordering;

/// A helper function to convert from a polars type to a Gene
/// Note: **Does not support chars**. If you want chars, gonna have to
/// program that yourself in your error function.
pub fn polars_to_gene(polars_value: &AnyValue) -> Gene {
    match polars_value {
        AnyValue::Int8(val) => Gene::GeneInt(*val as i128),
        AnyValue::Int16(val) => Gene::GeneInt(*val as i128),
        AnyValue::Int32(val) => Gene::GeneInt(*val as i128),
        AnyValue::Int64(val) => Gene::GeneInt(*val as i128),
        AnyValue::Int128(val) => Gene::GeneInt(*val as i128),
        AnyValue::UInt8(val) => Gene::GeneInt(*val as i128),
        AnyValue::UInt16(val) => Gene::GeneInt(*val as i128),
        AnyValue::UInt32(val) => Gene::GeneInt(*val as i128),
        AnyValue::UInt64(val) => Gene::GeneInt(*val as i128),
        AnyValue::Float32(val) => Gene::GeneFloat(Decimal::from_f32(*val).unwrap()),
        AnyValue::Float64(val) => Gene::GeneFloat(Decimal::from_f64(*val).unwrap()),
        AnyValue::Boolean(val) => Gene::GeneBoolean(*val),
        AnyValue::String(val) => Gene::GeneString(val.chars().collect()),
        AnyValue::List(series) => match series.dtype() {
            DataType::Int8 => {
                let vec = series
                    .i8()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::Int16 => {
                let vec = series
                    .i16()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::Int32 => {
                let vec = series
                    .i32()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::Int64 => {
                let vec = series
                    .i64()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::Int128 => {
                let vec = series
                    .i64() // i64 will have to do
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::UInt8 => {
                let vec = series
                    .u8()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::UInt16 => {
                let vec = series
                    .u16()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::UInt32 => {
                let vec = series
                    .u32()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::UInt64 => {
                let vec = series
                    .u64()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as i128))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorInt(vec)
            }
            DataType::Float32 => {
                let vec = series
                    .f32()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| Decimal::from_f32(v).unwrap()))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorFloat(vec)
            }
            DataType::Float64 => {
                let vec = series
                    .f64()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| Decimal::from_f64(v).unwrap()))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorFloat(vec)
            }
            DataType::Boolean => {
                let vec = series
                    .bool()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v as bool))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorBoolean(vec)
            }
            DataType::String => {
                let vec = series
                    .str()
                    .unwrap()
                    .into_iter()
                    .map(|opt| opt.map(|v| v.chars().collect()))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
                Gene::GeneVectorString(vec)
            }
            _ => unimplemented!("Type {:?} not handled inside a vector", polars_value),
        },
        _ => unimplemented!("Type {:?} not handled", polars_value),
    }
}

pub fn random_instruction(
    instructions: Vec<Gene>,
    closing_type: ClosingType,
    rng: &mut impl Rng,
) -> Gene {
    match closing_type {
        ClosingType::Specified => return instructions.choose(rng).unwrap().clone(),
        ClosingType::Balanced => {
            let source: Vec<Gene> = instructions
                .iter()
                .filter(|instr| !matches!(instr, Gene::Close))
                .cloned()
                .collect();
            let total_opens: usize = source
                .iter()
                .filter_map(|instr| {
                    if let Gene::StateFunc(_) = instr {
                        OPEN_MAP.get(instr).copied().map(|val| val as usize)
                    } else {
                        None
                    }
                })
                .sum();
            let p = if source.is_empty() {
                0.0
            } else {
                total_opens as f64 / source.len() as f64
            };
            // Return Close or a random instruction based on probability
            if rng.random::<f64>() < p {
                Gene::Close
            } else {
                source.choose(rng).unwrap().clone()
            }
        }
        ClosingType::None => {
            // Find multi-block instructions (those with opens > 1)
            let multi_block_instructions: Vec<Gene> = instructions
                .iter()
                .filter(|instr| {
                    if let Gene::StateFunc(_) = instr {
                        OPEN_MAP.get(instr).map_or(false, |&opens| opens > 1)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            // Remove Close and multi-block instructions
            let source: Vec<Gene> = instructions
                .into_iter() // Take ownership of instructions
                .filter(|instr| {
                    !matches!(instr, Gene::Close) && !multi_block_instructions.contains(instr)
                })
                .collect();

            source.choose(rng).unwrap().clone()
        }
    }
}

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

fn median(data: &Vec<Decimal>) -> Decimal {
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let len = sorted.len();
    if len == 0 {
        panic!("Cannot compute median of an empty vector");
    }

    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        let mid = len / 2;
        (sorted[mid - 1] + sorted[mid]) / Decimal::TWO
    }
}

// Used in epsilon lexicase selection
pub fn absolute_median_deviation(data: &Vec<Decimal>) -> Decimal {
    let med = median(data);

    let deviations: Vec<Decimal> = data.iter().map(|x| (*x - med).abs()).collect();

    median(&deviations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::code::*;
    use crate::instructions::common::boolean_rotate;
    use crate::instructions::vector::*;
    use crate::push::utils::most_genes;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn random_instruction_test() {
        let mut rng = StdRng::seed_from_u64(42);
        let genes = most_genes();

        let rand_instruction = random_instruction(genes.clone(), ClosingType::Specified, &mut rng);
        assert_eq!(
            Gene::StateFunc(vector_float_from_last_prim),
            rand_instruction
        );

        let mut rng = StdRng::seed_from_u64(32038);
        let rand_instruction = random_instruction(genes.clone(), ClosingType::Balanced, &mut rng);
        assert_eq!(Gene::StateFunc(boolean_rotate), rand_instruction);

        let mut rng = StdRng::seed_from_u64(3203890821);
        let rand_instruction = random_instruction(genes, ClosingType::None, &mut rng);
        assert_eq!(Gene::StateFunc(code_insert), rand_instruction);
    }

    #[test]
    fn median_test() {
        let temp_vec = vec![dec!(1.0), dec!(2.0), dec!(3.0), dec!(4.0), dec!(5.0)];

        let res_median = median(&temp_vec);
        assert_eq!(dec!(3.0), res_median);
    }

    #[test]
    fn absolute_median_deviation_test() {
        // https://en.wikipedia.org/wiki/Median_absolute_deviation
        let temp_vec = vec![
            dec!(1.0),
            dec!(1.0),
            dec!(2.0),
            dec!(2.0),
            dec!(4.0),
            dec!(6.0),
            dec!(9.0),
        ];

        let res_abs_median_dev = absolute_median_deviation(&temp_vec);
        assert_eq!(dec!(1.0), res_abs_median_dev);
    }
}
