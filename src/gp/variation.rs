use crate::gp::utils::gaussian_noise_factor;
use crate::push::state::Gene;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::iter::zip;

pub enum Variation {
    Crossover,
    Alternation,
    TailAlignedCrossover,
    UniformAddition,
    UniformReplacement,
    UniformDeletion,
}

fn is_crossover_padding(gene: &Gene) -> bool {
    match gene {
        Gene::CrossoverPadding => true,
        _ => false,
    }
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
        .filter(|gene| !is_crossover_padding(gene))
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
        .filter(|gene| !is_crossover_padding(gene))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::common::*;
    use crate::instructions::numeric::*;
    use crate::push::state::Gene;
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
}
