use crate::gp::args::ClosingType;
use crate::gp::genome::OPEN_MAP;
use crate::push::state::Gene;
use rand::Rng;
use rand::seq::IndexedRandom;
use rust_decimal::prelude::*;

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
}
