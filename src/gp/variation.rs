use crate::gp::args::PushArgs;
use crate::gp::individual::Individual;
use crate::gp::selection::select_parent;
use crate::gp::utils::gaussian_noise_factor;
use crate::push::state::Gene;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use std::iter::zip;

use super::args::ClosingType;
use super::utils::random_instruction;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Variation {
    Crossover,
    Alternation,
    TailAlignedCrossover,
    UniformAddition,
    UniformReplacement,
    UniformDeletion,
    Reproduction,
    UMAD,
}

fn crossover(plushy0: Vec<Gene>, plushy1: Vec<Gene>, mut rng: impl Rng) -> Vec<Gene> {
    let mut shorter: Vec<Gene>;
    let longer: Vec<Gene>;
    let mut new_plushy: Vec<Gene> = vec![];

    if plushy0.len() >= plushy1.len() {
        shorter = plushy1;
        longer = plushy0;
    } else {
        shorter = plushy0;
        longer = plushy1;
    }

    for _ in 0..(longer.len() - shorter.len()) {
        shorter.push(Gene::CrossoverPadding)
    }

    // Add genes here
    for (sgene, lgene) in zip(shorter, longer) {
        if rng.random_range(0..=99) < 50 {
            new_plushy.push(sgene)
        } else {
            new_plushy.push(lgene)
        }
    }

    new_plushy
        .into_iter()
        .filter(|gene| !matches!(gene, Gene::CrossoverPadding))
        .collect()
}

fn tail_aligned_crossover(plushy0: Vec<Gene>, plushy1: Vec<Gene>, mut rng: impl Rng) -> Vec<Gene> {
    let mut shorter: Vec<Gene>;
    let longer: Vec<Gene>;
    let mut new_plushy: Vec<Gene> = vec![];

    if plushy0.len() >= plushy1.len() {
        shorter = plushy1;
        longer = plushy0;
    } else {
        shorter = plushy0;
        longer = plushy1;
    }

    for _ in 0..(longer.len() - shorter.len()) {
        shorter.insert(0, Gene::CrossoverPadding)
    }

    // Add genes here
    for (sgene, lgene) in zip(shorter, longer) {
        if rng.random_range(0..=99) < 50 {
            new_plushy.push(sgene)
        } else {
            new_plushy.push(lgene)
        }
    }

    new_plushy
        .into_iter()
        .filter(|gene| !matches!(gene, Gene::CrossoverPadding))
        .collect()
}

fn alternation(
    plushy0: Vec<Gene>,
    plushy1: Vec<Gene>,
    alternation_rate: usize,
    alignment_deviation: Decimal,
    mut rng: impl Rng,
) -> Vec<Gene> {
    let mut use_plushy0: bool = true;
    let mut result_plushy: Vec<Gene> = vec![];
    let mut iteration_budget = plushy0.len() + plushy1.len();
    let mut n: i128 = 0;
    loop {
        if use_plushy0 {
            if n >= plushy0.len() as i128 {
                return result_plushy;
            }
        } else {
            if n >= plushy1.len() as i128 {
                return result_plushy;
            }
        }
        if iteration_budget <= 0 {
            return result_plushy;
        }
        if rng.random_range(0..=99) < alternation_rate {
            let pre_usize = alignment_deviation * gaussian_noise_factor(&mut rng);
            n = 0i128.max(n + ToPrimitive::to_i128(&(pre_usize).round()).unwrap());
            use_plushy0 = !use_plushy0;
        } else {
            result_plushy.push(if use_plushy0 {
                plushy0[n as usize].clone()
            } else {
                plushy1[n as usize].clone()
            });
            n += 1;
        }
        iteration_budget -= 1;
    }
}

fn uniform_addition(
    plushy: Vec<Gene>,
    instructions: Vec<Gene>,
    umad_rate: f64,
    closing_type: ClosingType,
    rng: &mut impl Rng,
) -> Vec<Gene> {
    let mut new_plushy: Vec<Gene> = vec![];

    for gene in plushy {
        if rng.random::<f64>() < umad_rate {
            let new_instruction = random_instruction(instructions.clone(), closing_type, rng);

            // Randomly decide order (original first or new first)
            if rng.random::<bool>() {
                new_plushy.push(gene);
                new_plushy.push(new_instruction);
            } else {
                new_plushy.push(new_instruction);
                new_plushy.push(gene);
            }
        } else {
            new_plushy.push(gene);
        }
    }

    new_plushy
}

fn uniform_replacement(
    plushy: Vec<Gene>,
    instructions: Vec<Gene>,
    replacement_rate: f64,
    closing_type: ClosingType,
    rng: &mut impl Rng,
) -> Vec<Gene> {
    plushy
        .into_iter()
        .map(|gene| {
            if rng.random::<f64>() < replacement_rate {
                // Replace with random instruction
                random_instruction(instructions.to_vec(), closing_type, rng)
            } else {
                // Keep original gene
                gene
            }
        })
        .collect()
}

fn uniform_deletion(plushy: Vec<Gene>, umad_rate: f64, rng: &mut impl Rng) -> Vec<Gene> {
    // If umad_rate is zero, return the original vector
    if umad_rate == 0.0 {
        return plushy;
    }

    // Calculate the adjusted deletion rate
    let adjusted_rate = 1.0 / (1.0 + (1.0 / umad_rate));

    // Filter the vector, keeping items that are either Gene::Skip or pass the random test
    plushy
        .into_iter()
        .filter(|_| rng.random::<f64>() >= adjusted_rate)
        .collect()
}

/// Selects a variation operator based on the probabilities
fn select_variation_op(variation_ops: &HashMap<Variation, f64>, r: f64) -> Variation {
    let mut accum = 0.0;

    for (op, prob) in variation_ops {
        accum += prob;
        if accum >= r {
            return op.clone();
        }
    }

    // Default to reproduction if no match (or probabilities don't sum to 1.0)
    Variation::Reproduction
}

/// Creates a new individual based on an argmap variation
pub fn new_individual(pop: Vec<Individual>, argmap: &PushArgs, rng: &mut impl Rng) -> Individual {
    // Select variation operator based on probabilities
    let r = rng.random::<f64>();
    let op = select_variation_op(&argmap.variation, r);

    let plushy = match op {
        Variation::Crossover => {
            let parent1 = select_parent(pop.clone(), argmap, rng);
            let parent2 = select_parent(pop, argmap, rng);
            crossover(parent1.plushy, parent2.plushy, rng)
        }

        Variation::TailAlignedCrossover => {
            let parent1 = select_parent(pop.clone(), argmap, rng);
            let parent2 = select_parent(pop, argmap, rng);
            tail_aligned_crossover(parent1.plushy, parent2.plushy, rng)
        }

        Variation::UniformAddition => {
            let parent = select_parent(pop, argmap, rng);
            uniform_addition(
                parent.plushy.clone(),
                argmap
                    .instructions
                    .clone()
                    .expect("Must provide instructions"),
                argmap.umad_rate,
                argmap.closes,
                rng,
            )
        }

        Variation::UniformReplacement => {
            let parent = select_parent(pop, argmap, rng);
            uniform_replacement(
                parent.plushy.clone(),
                argmap
                    .instructions
                    .clone()
                    .expect("Must provide instructions!"),
                argmap.replacement_rate,
                argmap.closes,
                rng,
            )
        }

        Variation::UniformDeletion => {
            let parent = select_parent(pop, argmap, rng);
            uniform_deletion(parent.plushy.clone(), argmap.umad_rate, rng)
        }

        Variation::Alternation => {
            let parent1 = select_parent(pop.clone(), argmap, rng);
            let parent2 = select_parent(pop, argmap, rng);
            alternation(
                parent1.plushy,
                parent2.plushy,
                argmap.alternation_rate,
                argmap.alignment_deviation,
                rng,
            )
        }

        Variation::UMAD => {
            let parent = select_parent(pop, argmap, rng);
            let parent_plushy = parent.plushy.clone();

            // Apply uniform addition followed by uniform deletion
            let after_addition = uniform_addition(
                parent_plushy,
                argmap
                    .instructions
                    .clone()
                    .expect("Must provide instructions"),
                argmap.umad_rate,
                argmap.closes,
                rng,
            );

            uniform_deletion(after_addition, argmap.umad_rate, rng)
        }

        Variation::Reproduction => {
            let parent = select_parent(pop, argmap, rng);
            parent.plushy.clone()
        }
    };

    Individual {
        plushy,
        push_program: None,
        total_fitness: None,
        fitness_cases: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::common::*;
    use crate::instructions::numeric::*;
    use crate::instructions::vector::*;
    use crate::push::state::Gene;
    use crate::push::utils::most_genes;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rust_decimal::dec;

    #[test]
    fn crossover_test() {
        let rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::StateFunc(exec_flush),
            Gene::Close,
            Gene::StateFunc(boolean_pop),
            Gene::StateFunc(vector_int_swap),
            Gene::StateFunc(vector_char_pop),
        ];
        let plushy1 = vec![
            Gene::StateFunc(string_swap),
            Gene::StateFunc(float_arctan),
            Gene::StateFunc(char_pop),
            Gene::GeneChar('a'),
            Gene::StateFunc(code_flush),
            Gene::GeneInt(1),
            Gene::StateFunc(float_pop),
        ];
        let res_plushy = crossover(plushy0, plushy1, rng);
        assert_eq!(
            vec![
                Gene::StateFunc(string_swap),
                Gene::StateFunc(float_tan),
                Gene::StateFunc(char_pop),
                Gene::Close,
                Gene::StateFunc(exec_flush),
                Gene::Close,
                Gene::StateFunc(boolean_pop),
                Gene::StateFunc(vector_char_pop),
            ],
            res_plushy
        )
    }

    #[test]
    fn tail_aligned_crossover_test() {
        let rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::StateFunc(exec_flush),
            Gene::Close,
            Gene::StateFunc(boolean_pop),
            Gene::StateFunc(vector_int_swap),
            Gene::StateFunc(vector_char_pop),
        ];
        let plushy1 = vec![
            Gene::StateFunc(string_swap),
            Gene::StateFunc(float_arctan),
            Gene::StateFunc(char_pop),
            Gene::GeneChar('a'),
            Gene::StateFunc(code_flush),
            Gene::GeneInt(1),
            Gene::StateFunc(float_pop),
        ];
        let res_plushy = tail_aligned_crossover(plushy0, plushy1, rng);
        assert_eq!(
            vec![
                Gene::StateFunc(float_tan),
                Gene::StateFunc(string_swap),
                Gene::Close,
                Gene::StateFunc(exec_flush),
                Gene::Close,
                Gene::StateFunc(boolean_pop),
                Gene::GeneInt(1),
                Gene::StateFunc(vector_char_pop),
            ],
            res_plushy
        )
    }

    #[test]
    fn alternation_test() {
        let rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::StateFunc(exec_flush),
            Gene::Close,
            Gene::StateFunc(boolean_pop),
            Gene::StateFunc(vector_int_swap),
            Gene::StateFunc(vector_char_pop),
        ];
        let plushy1 = vec![
            Gene::StateFunc(string_swap),
            Gene::StateFunc(float_arctan),
            Gene::StateFunc(char_pop),
            Gene::GeneChar('a'),
            Gene::StateFunc(code_flush),
            Gene::GeneInt(1),
            Gene::StateFunc(float_pop),
        ];
        let res_plushy = alternation(plushy0, plushy1, 50, dec!(2.0), rng);
        assert_eq!(
            vec![
                Gene::StateFunc(char_pop),
                Gene::GeneChar('a'),
                Gene::StateFunc(boolean_pop),
                Gene::StateFunc(vector_int_swap),
                Gene::StateFunc(vector_char_pop),
            ],
            res_plushy
        );
    }

    #[test]
    fn uniform_addition_test() {
        let mut rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
        ];
        let res_plushy =
            uniform_addition(plushy0, most_genes(), 0.75, ClosingType::Balanced, &mut rng);
        assert_eq!(
            vec![
                Gene::StateFunc(exec_swap),
                Gene::StateFunc(float_min),
                Gene::StateFunc(float_tan),
                Gene::Close,
                Gene::StateFunc(int_pop),
                Gene::StateFunc(int_yank_dup),
                Gene::Close,
                Gene::StateFunc(float_is_empty),
            ],
            res_plushy
        );
    }

    #[test]
    fn uniform_replacement_test() {
        let mut rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::Close,
            Gene::GeneInt(1),
        ];
        let res_plushy =
            uniform_replacement(plushy0, most_genes(), 0.5, ClosingType::Balanced, &mut rng);
        assert_eq!(
            vec![
                Gene::StateFunc(exec_swap),
                Gene::StateFunc(float_tan),
                Gene::StateFunc(int_pop),
                Gene::Close,
                Gene::StateFunc(vector_float_sort_reverse),
                Gene::GeneInt(1),
            ],
            res_plushy
        );
    }

    #[test]
    fn uniform_deletion_test() {
        let mut rng = StdRng::seed_from_u64(42);
        let plushy0 = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::Close,
            Gene::GeneInt(1),
        ];
        let res_plushy = uniform_deletion(plushy0, 0.5, &mut rng);
        assert_eq!(
            vec![
                Gene::StateFunc(exec_swap),
                Gene::StateFunc(float_tan),
                Gene::StateFunc(int_pop),
                Gene::Close,
                Gene::GeneInt(1),
            ],
            res_plushy
        );
    }
}
