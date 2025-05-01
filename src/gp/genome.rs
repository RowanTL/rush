use crate::instructions::code::{
    exec_do_count, exec_do_range, exec_do_times, exec_do_while, exec_if, exec_when, exec_while,
};
use crate::instructions::common::{
    exec_dup, exec_dup_times, exec_pop, exec_rotate, exec_shove, exec_swap,
};
use crate::instructions::vector::{
    string_iterate, vector_boolean_iterate, vector_char_iterate, vector_float_iterate,
    vector_int_iterate, vector_string_iterate,
};
use crate::push::state::Gene;
use crate::push::state::Gene::StateFunc;
use rand::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Generates a random plushy.
pub fn make_random_plushy(
    genes: Vec<Gene>,
    max_init_plushy_size: usize,
    rng: &mut impl Rng,
) -> Vec<Gene> {
    let plushy_size = rng.random_range(0..=max_init_plushy_size);
    let mut plushy = Vec::with_capacity(plushy_size);
    for _ in 0..plushy_size {
        plushy.push(genes[rng.random_range(0..genes.len())].clone());
    }
    plushy
}

/// A map of genes to their number of blocks they open.
pub static OPEN_MAP: LazyLock<HashMap<Gene, u8>> = LazyLock::new(|| {
    let mut temp_map = HashMap::default();
    temp_map.insert(StateFunc(exec_dup), 1u8);
    temp_map.insert(StateFunc(exec_dup_times), 1u8);
    temp_map.insert(StateFunc(exec_pop), 1u8);
    temp_map.insert(StateFunc(exec_rotate), 3u8);
    temp_map.insert(StateFunc(exec_shove), 1u8);
    temp_map.insert(StateFunc(exec_swap), 2u8);
    temp_map.insert(StateFunc(exec_if), 2u8);
    temp_map.insert(StateFunc(exec_when), 1u8);
    temp_map.insert(StateFunc(exec_while), 1u8);
    temp_map.insert(StateFunc(exec_do_while), 1u8);
    temp_map.insert(StateFunc(exec_do_range), 1u8);
    temp_map.insert(StateFunc(exec_do_count), 1u8);
    temp_map.insert(StateFunc(exec_do_times), 1u8);
    //exec_k, 2
    //exec_s 3
    //exec_y 1
    temp_map.insert(StateFunc(string_iterate), 1u8);
    temp_map.insert(StateFunc(vector_int_iterate), 1u8);
    temp_map.insert(StateFunc(vector_float_iterate), 1u8);
    temp_map.insert(StateFunc(vector_string_iterate), 1u8);
    temp_map.insert(StateFunc(vector_boolean_iterate), 1u8);
    temp_map.insert(StateFunc(vector_char_iterate), 1u8);
    temp_map
});

fn has_openers(genes: &[Gene]) -> bool {
    for gene in genes {
        if is_opener(gene) {
            return true;
        }
    }
    false
}

fn is_opener(gene: &Gene) -> bool {
    match gene {
        Gene::Open(_) => return true,
        _ => (),
    }
    false
}

fn get_opener_count(gene: &Gene) -> &u8 {
    match gene {
        Gene::Open(val) => val,
        _ => &0u8,
    }
}

fn dec_opener(gene: Gene) -> Gene {
    match gene {
        Gene::Open(val) => Gene::Open(val - 1),
        _ => gene,
    }
}

/// Converts a plushy to a push program.
pub fn plushy_to_push(genes: Vec<Gene>) -> Vec<Gene> {
    let mut plushy_buffer: Vec<Gene> = Vec::with_capacity(genes.len() * 2);
    for gene in genes.into_iter() {
        let open = OPEN_MAP.get(&gene);
        plushy_buffer.push(gene);
        if let Some(amt) = open {
            plushy_buffer.push(Gene::Open(*amt))
        }
    }
    let mut push_buffer = Vec::with_capacity(plushy_buffer.len());
    loop {
        // recur with one more close if there are openers
        if plushy_buffer.is_empty() && has_openers(&push_buffer) {
            plushy_buffer.push(Gene::Close);
        } else if plushy_buffer.is_empty() {
            return push_buffer;
        } else {
            let first_gene = plushy_buffer.remove(0);
            match &first_gene {
                Gene::Close => {
                    if has_openers(&push_buffer) {
                        let mut index: Option<usize> = None;
                        let mut opener: Option<Gene> = None;
                        // not the most optimal iterating through the entire genome.
                        // Will do for now.
                        for (ndx, el) in push_buffer.clone().into_iter().enumerate() {
                            if is_opener(&el) {
                                index = Some(ndx);
                                opener = Some(el);
                            }
                        }
                        let post_open: Vec<_> = push_buffer.drain((index.unwrap() + 1)..).collect();
                        push_buffer.pop(); // Pop the close here
                        push_buffer.push(Gene::Block(post_open));
                        if get_opener_count(&opener.clone().unwrap()) > &1u8 {
                            let opener_new = dec_opener(opener.unwrap().clone());
                            push_buffer.push(opener_new);
                        }
                    }
                }
                _ => push_buffer.push(first_gene),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::instructions::vector::{string_iterate, vector_float_maximum};
    use crate::instructions::common::*;
    use crate::instructions::numeric::*;
    use crate::push::state::*;
    // use crate::push::utils::most_genes;
    // use rand::SeedableRng;

    #[test]
    fn make_random_plushy_test() {
        // let rng = StdRng::seed_from_u64(42);
        // let rand_plushy = make_random_plushy(most_genes(), 15, rng);
        // let fin_result = vec![StateFunc(string_iterate), StateFunc(vector_float_maximum)];
        // Make this consistent later
        // assert_eq!(fin_result, rand_plushy);
    }

    #[test]
    fn plushy_to_push_test() {
        let plushy = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
            Gene::StateFunc(exec_flush),
            Gene::Close,
            Gene::StateFunc(boolean_pop),
        ];
        let res_push = plushy_to_push(plushy);
        assert_eq!(
            res_push,
            vec![
                StateFunc(exec_swap),
                Gene::Block(vec![Gene::StateFunc(float_tan), Gene::StateFunc(int_pop)]),
                Gene::Block(vec![Gene::StateFunc(exec_flush)]),
                Gene::StateFunc(boolean_pop),
            ]
        );

        let plushy = vec![
            Gene::StateFunc(exec_swap),
            Gene::StateFunc(float_tan),
            Gene::StateFunc(int_pop),
            Gene::Close,
        ];
        let res_push = plushy_to_push(plushy);
        assert_eq!(
            res_push,
            vec![
                StateFunc(exec_swap),
                Gene::Block(vec![Gene::StateFunc(float_tan), Gene::StateFunc(int_pop)]),
                Gene::Block(vec![]),
            ]
        )
    }
}
