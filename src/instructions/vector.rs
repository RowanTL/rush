use crate::instructions::utils::NumericTrait;
use crate::push::state::{Gene, PushState};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Generates an index between 0 and length. Takes abs(num) and then mods it by length.
fn bounded_idx(num: i128, length: usize) -> usize {
    (num.unsigned_abs() as usize) % length
}

/// Concats two vectors together.
fn _concat<T>(a: Vec<T>, b: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut concat_vec = a;
    concat_vec.extend(b.into_iter());
    Some(concat_vec)
}

/// Prepends a primitive value to a vector.
fn _conj<T>(vect: Vec<T>, prim: T) -> Option<Vec<T>> {
    let mut t_vec = vect;
    t_vec.insert(0, prim);
    Some(t_vec)
}

/// Appends a primitive value to a vector.
fn _conj_end<T>(mut vals: Vec<T>, prim: T) -> Option<Vec<T>> {
    vals.push(prim);
    Some(vals)
}

/// Takes the first N items from a vector. N based on an int.
fn _take_n<T>(vals: Vec<T>, amt: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[0..bounded_idx(amt, vals.len())].to_vec())
}

/// Takes the first N items from a vector. N based on an int.
fn _take_last_n<T>(vals: Vec<T>, amt: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    let vals_len = vals.len();
    Some(vals[vals_len - bounded_idx(amt, vals_len)..vals_len].to_vec())
}

/// Takes a sublist of a vector based on two ints.
fn _sub<T>(vals: Vec<T>, idx0: i128, idx1: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    let (mut start, mut end): (usize, usize) =
        (idx0.unsigned_abs() as usize, idx1.unsigned_abs() as usize);
    if start > end {
        (start, end) = (end, start)
    }
    let fin_start = start.min(vals.len());
    let fin_end = end.min(vals.len());
    Some(vals[fin_start..fin_end].to_vec())
}

/// Takes the first item from a vector.
fn _first<T>(vals: Vec<T>) -> Option<T>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[0].clone())
}

/// Takes the first item from a vector, wraps it into a vector, and pushes it back
/// to the same stack.
fn _from_first_prim<T>(vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vec![vals[0].clone()])
}

/// Places the top of a primitive type into a vector
fn _from_prim<T>(prim: T) -> Option<Vec<T>> {
    Some(vec![prim])
}

/// Takes the last item from a vector.
fn _last<T>(vals: Vec<T>) -> Option<T>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[vals.len() - 1].clone())
}

/// Takes the last item from a vector, wraps it into a vector, and pushes it back
/// to the same stack.
fn _from_last_prim<T>(vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vec![vals[vals.len() - 1].clone()])
}

/// Takes the nth item from a vector. N from int stack.
fn _nth<T>(vals: Vec<T>, idx: i128) -> Option<T>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[bounded_idx(idx, vals.len())].clone())
}

/// Takes the nth item from a vector, wraps it into a vector, and pushes it back
/// to the same stack. N from int stack
fn _from_nth_prim<T>(vals: Vec<T>, idx: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vec![vals[bounded_idx(idx, vals.len())].clone()])
}

/// Takes a vector and removes the first element.
fn _rest<T>(vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[1..].to_vec())
}

/// Takes a vector and removes the last element.
fn _but_last<T>(vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    Some(vals[0..vals.len() - 1].to_vec())
}

/// Removes the first n items from a vector. n from the int stack.
fn _drop<T>(mut vals: Vec<T>, idx: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    if vals.is_empty() {
        return None;
    }
    vals.drain(0..idx.abs().min(vals.len() as i128) as usize);
    Some(vals)
}

fn _drop_last<T>(mut vals: Vec<T>, idx: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    let valslen = vals.len(); //Ret_Vec Len
    if vals.is_empty() {
        return None;
    }
    vals.drain((valslen - (idx.abs().min(valslen as i128) as usize))..valslen);
    Some(vals)
}

/// Takes the length of a vector.
fn _length<T>(vals: Vec<T>) -> Option<i128> {
    Some(vals.len() as i128)
}

/// Reverses a vector
fn _reverse<T>(mut vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    vals.reverse();
    Some(vals)
}

/// Pushes all values of a vector into a primitive stack
fn _push_all<T>(vals: Vec<T>) -> Option<Vec<T>> {
    Some(vals)
}

/// Creates an empty vector
fn _make_empty<T>() -> Option<Vec<T>> {
    let empty_vec: Vec<T> = Vec::new();
    Some(empty_vec)
}

/// Checks if a vector is empty. Pushes true if is, false otherwise
fn _is_vector_empty<T>(vals: Vec<T>) -> Option<bool> {
    Some(vals.is_empty())
}

/// Checks if a vector contains a primitive. True if does, false otherwise
fn _contains<T>(vals: Vec<T>, prim: T) -> Option<bool>
where
    T: Eq,
{
    Some(vals.contains(&prim))
}

/// Checks if a vector contains another vector in no order. True if does, false otherwise
fn _contains_vector_non_contiguous<T>(vec0: Vec<T>, vec1: Vec<T>) -> Option<bool>
where
    T: Eq + Hash,
{
    let hashset: HashSet<&T> = vec1.iter().collect();
    Some(vec0.iter().all(|x| hashset.contains(x)))
}

/// Checks if a vector contains another contiguous vector. True if does, false otherwise
fn _contains_vector_contiguous<T>(vec0: Vec<T>, vec1: Vec<T>) -> Option<bool>
where
    T: Eq,
{
    if vec0.is_empty() {
        return Some(true); // would argue the empty set is in everything
    }
    Some(vec1.windows(vec0.len()).any(|x| x == vec0))
}

/// Returns the index of a primitive in a vector, pushes result to int stack
fn _index_of<T>(vals: Vec<T>, prim: T) -> Option<i128>
where
    T: Clone + Eq,
{
    let temp_vec = &vals;
    let temp_aux = &prim;
    if let Some(idx) = temp_vec.iter().position(|r| r == temp_aux) {
        return Some(idx as i128);
    }
    Some(-1)
}

/// Finds the index of the start of one vector in another. Searches in contiguous space.
fn _index_of_vector<T>(vec0: Vec<T>, vec1: Vec<T>) -> Option<i128>
where
    T: Eq,
{
    if vec0.is_empty() {
        return Some(0);
    }
    if let Some(val) = vec1.windows(vec0.len()).position(|x| x == vec0) {
        return Some(val as i128);
    }
    Some(-1)
}

/// Counts the amount of a primitive in a vector
fn _occurrences_of<T>(vals: Vec<T>, prim: T) -> Option<i128>
where
    T: Clone + Eq,
{
    Some(vals.into_iter().filter(|r| r == &prim).count() as i128)
}

/// Counts the amount of continuous occurrences one vector appears in another.
fn _occurrences_of_vector<T>(vec0: Vec<T>, vec1: Vec<T>) -> Option<i128>
where
    T: Eq,
{
    if vec0.is_empty() {
        return Some(0);
    }
    Some(vec1.windows(vec0.len()).filter(|x| x == &vec0).count() as i128)
}

/// Pushes the values inside a vector separated into individual vectors back to
/// the stack.
fn _parse_to_prim<T>(vals: Vec<T>) -> Option<Vec<Vec<T>>>
where
    T: Clone,
    Vec<T>: FromIterator<T>,
{
    Some(vals.clone().into_iter().map(|x| vec![x]).collect())
}

/// Sets the nth index in a vector. N from the int stack.
fn _set_nth<T>(vals: Vec<T>, idx: i128, prim: T) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut temp_vec = vals.clone();
    let idx = bounded_idx(idx, temp_vec.len());
    temp_vec.insert(idx, prim);
    Some(temp_vec)
}

/// Splits a vector based on the first occurence of a primitive
fn _split_on<T>(vals: Vec<T>, prim: T) -> Option<Vec<Vec<T>>>
where
    T: Clone + Eq,
    Vec<T>: FromIterator<T>,
{
    let mut final_vec = vec![];
    let mut temp_vec = vec![];
    for val in vals.iter() {
        if &prim == val {
            final_vec.push(temp_vec.clone());
            temp_vec.clear();
            continue;
        }
        temp_vec.push(val.clone());
    }
    if !temp_vec.is_empty() {
        final_vec.push(temp_vec);
    }
    Some(final_vec)
}

/*/// Splits a vector based the first occurrence of a primitive
fn _split_on_vector<T>(vals: Vec<Vec<T>>) -> Option<Vec<Vec<T>>>
where
    T: Clone + Eq,
{
    if vals[0].is_empty() {
        return Some(vec![vals[1]]);
    }
    let mut final_vec = vec![];
    let mut temp_vec = vec![];
    for val in vals[1].windows(vals[0].len()) {
        if &auxs[0] == val {
            final_vec.push(temp_vec.clone());
            temp_vec.clear();
            continue;
        }
        temp_vec.push(val.clone());
    }
    if !temp_vec.is_empty() {
        final_vec.push(temp_vec);
    }
    Some(final_vec)
}*/

/// Replaces all values in a vector with respect to two primitives. The first primitive is
/// the search value and the second value is the one to replace.
fn _replace<T>(mut vals: Vec<T>, from: T, to: T) -> Option<Vec<T>>
where
    T: Clone,
    for<'a> &'a T: Eq,
    Vec<T>: FromIterator<T>,
{
    let temp_vec = &mut vals;
    let ret_vec: Vec<T> = temp_vec
        .iter()
        .map(|x| if x == &from { to.clone() } else { x.clone() })
        .collect();
    Some(ret_vec)
}

/// Removes all values in a vector with respect to a primitive. If is equal, remove it.
fn _remove<T>(vals: Vec<T>, prim: T) -> Option<Vec<T>>
where
    T: Clone,
    for<'a> &'a T: Eq,
    Vec<T>: FromIterator<T>,
{
    let temp_vec = &vals;
    let ret_vec = temp_vec.iter().filter(|&x| x != &prim).cloned().collect();
    Some(ret_vec)
}

/// Iterates over a vector using an instruction from the exec stack.
macro_rules! make_iterate {
    ($vec_stack:ident, $prim_stack:ident, $vec_gene:ident) => {
        paste::item! {
            pub fn [< $vec_stack _iterate >] (state: &mut PushState) {
                if state.$vec_stack.is_empty() || state.exec.is_empty() {
                    return;
                }
                let first_vec = state.$vec_stack.pop().unwrap();
                if first_vec.is_empty() {
                    state.exec.pop();
                    return;
                } else if first_vec.len() == 1 {
                    state.$prim_stack.push(first_vec[0].clone());
                    return;
                } else {
                    let top_exec = state.exec[state.exec.len() - 1].clone();
                    let first_prim = first_vec[0].clone();
                    state.exec.push(Gene::StateFunc([< $vec_stack _iterate >]));
                    state.exec.push(Gene::$vec_gene(first_vec[1..].to_vec()));
                    state.exec.push(top_exec);
                    state.$prim_stack.push(first_prim);
                }
            }
        }
    };
}
make_iterate!(vector_int, int, GeneVectorInt);
make_iterate!(vector_float, float, GeneVectorFloat);
make_iterate!(vector_string, string, GeneVectorString);
make_iterate!(vector_boolean, boolean, GeneVectorBoolean);
make_iterate!(vector_char, char, GeneVectorChar);
make_iterate!(string, char, GeneString);

/// Sorts a vector
fn _sort<T>(mut vals: Vec<T>) -> Option<Vec<T>>
where
    T: NumericTrait,
{
    vals.sort();
    Some(vals)
}

/// Sorts a vector and reverses it
fn _sort_reverse<T>(mut vals: Vec<T>) -> Option<Vec<T>>
where
    T: NumericTrait,
{
    vals.sort();
    vals.reverse();
    Some(vals)
}

/// Inserts a primitive into a vector at a given point from the int stack
fn _insert<T>(mut vals: Vec<T>, idx: i128, prim: T) -> Option<Vec<T>> {
    let vec_len = vals.len();
    vals.insert(bounded_idx(idx, vec_len), prim);
    Some(vals)
}

/// Inserts one vector into another based on an index.
fn _insert_vector<T>(vec0: Vec<T>, mut vec1: Vec<T>, idx: i128) -> Option<Vec<T>>
where
    T: Clone,
{
    let bound_idx = bounded_idx(idx, vec0.len());
    vec1.splice(bound_idx..bound_idx, vec0);
    Some(vec1)
}

/// Takes the mean of a vector
fn _mean<T: NumericTrait + Clone>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    let mut fin_num = T::zero();
    for num in vals.clone().into_iter() {
        fin_num = fin_num + num;
    }
    Some(fin_num.div(T::from_usize(vals.len())))
}

/// Takes the max of a vector
fn _maximum<T: NumericTrait>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    vals.into_iter().max()
}

/// Takes the min of a vector
fn _minimum<T: NumericTrait>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    vals.into_iter().min()
}

/// Takes the sum of a vector
fn _sum<T: NumericTrait + Clone>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    let mut fin_num = T::zero();
    for num in vals.clone().into_iter() {
        fin_num = fin_num + num;
    }
    Some(fin_num)
}

/// Takes the mode of a vector
fn _mode<T: NumericTrait + Clone + Hash + Copy>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    let mut counts = HashMap::new();
    vals.iter()
        .max_by_key(|&x| {
            let count = counts.entry(x).or_insert(0);
            *count += 1;
            *count
        })
        .copied()
}

/// Adds the squares of all values in a vector and then takes the square root
fn _two_norm<T: NumericTrait + Clone>(vals: Vec<T>) -> Option<T> {
    if vals.is_empty() {
        return Some(T::zero());
    }
    let mut fin_num = T::zero();
    for num in vals.clone().into_iter() {
        fin_num = fin_num + (num.clone() * num);
    }
    fin_num.safe_sqrt()
}

/// Takes the cumulative sum of a vector
fn _cumulative_sum<T: NumericTrait + Clone>(vals: Vec<T>) -> Option<Vec<T>> {
    if vals.is_empty() {
        return Some(vec![]);
    }
    let mut fin_num = T::zero();
    let mut ret_vec = vec![];
    for num in vals.clone().into_iter() {
        fin_num = fin_num + num;
        ret_vec.push(fin_num.clone());
    }
    Some(ret_vec)
}

/* /// Takes the cumulative mean of a vector
fn _cumulative_mean<T: NumericTrait + Clone>(vals: Vec<Vec<T>>) -> Option<Vec<T>> {
    if vals[0].is_empty() {
        return Some(vec![]);
    }
    // This is not an efficient implementation
    let mut ret_vec = vec![];
    let mut cum_vec = vec![];
    for (idx, val) in vals[0].iter().enumerate() {
        cum_vec.push(val.clone());
        let mut temp_sum = T::zero();
        for num in &cum_vec {
            temp_sum = temp_sum + num.clone();
        }
        temp_sum
    }
    Some(ret_vec)
}
make_instruction_clone!(vector_int, vector_int, _cumulative_mean, Vec<i128>, 1);
make_instruction_clone!(
    vector_float,
    vector_float,
    _cumulative_mean,
    Vec<Decimal>,
    1
);*/

macro_rules! make_vector_instructions {
    ($stack:ident, $prim_stack:ident) => {
        make_instruction_new!(_concat, $stack, $stack, $stack, $stack);
        make_instruction_new!(_conj, $stack, $stack, $stack, $prim_stack);
        make_instruction_new!(_conj_end, $stack, $stack, $stack, $prim_stack);
        make_instruction_new!(_take_n, $stack, $stack, $stack, int);
        make_instruction_new!(_take_last_n, $stack, $stack, $stack, int);
        make_instruction_new!(_sub, $stack, $stack, $stack, int, int);
        make_instruction_new!(_first, $stack, $prim_stack, $stack);
        make_instruction_new!(_from_first_prim, $stack, $stack, $stack);
        make_instruction_new!(_from_prim, $stack, $stack, $prim_stack);
        make_instruction_new!(_last, $stack, $prim_stack, $stack);
        make_instruction_new!(_from_last_prim, $stack, $stack, $stack);
        make_instruction_new!(_nth, $stack, $prim_stack, $stack, int);
        make_instruction_new!(_from_nth_prim, $stack, $stack, $stack, int);
        make_instruction_new!(_rest, $stack, $stack, $stack);
        make_instruction_new!(_but_last, $stack, $stack, $stack);
        make_instruction_new!(_drop, $stack, $stack, $stack, int);
        make_instruction_new!(_drop_last, $stack, $stack, $stack, int);
        make_instruction_new!(_length, $stack, int, $stack);
        make_instruction_new!(_reverse, $stack, $stack, $stack);
        make_instruction_new_aux!(_push_all, $stack, $prim_stack, $stack);
        // _make_empty would go here
        make_instruction_new!(_is_vector_empty, $stack, boolean, $stack);
        make_instruction_new!(_contains, $stack, boolean, $stack, $prim_stack);
        make_instruction_new!(
            _contains_vector_non_contiguous,
            $stack,
            boolean,
            $stack,
            $stack
        );
        make_instruction_new!(_contains_vector_contiguous, $stack, boolean, $stack, $stack);
        make_instruction_new!(_index_of, $stack, int, $stack, $prim_stack);
        make_instruction_new!(_index_of_vector, $stack, int, $stack, $stack);
        make_instruction_new!(_occurrences_of, $stack, int, $stack, $prim_stack);
        make_instruction_new!(_occurrences_of_vector, $stack, int, $stack, $stack);
        make_instruction_new_aux!(_parse_to_prim, $stack, $stack, $stack);
        make_instruction_new!(_set_nth, $stack, $stack, $stack, int, $prim_stack);
        make_instruction_new_aux!(_split_on, $stack, $stack, $stack, $prim_stack);
        make_instruction_new!(_replace, $stack, $stack, $stack, $prim_stack, $prim_stack);
        make_instruction_new!(_remove, $stack, $stack, $stack, $prim_stack);
        make_instruction_new!(_insert, $stack, $stack, $stack, int, $prim_stack);
        make_instruction_new!(_insert_vector, $stack, $stack, $stack, $stack, int);
    };
}

macro_rules! make_numeric_vector_instructions {
    ($stack:ident, $prim_stack:ident) => {
        make_instruction_new!(_sort, $stack, $stack, $stack);
        make_instruction_new!(_sort_reverse, $stack, $stack, $stack);
        make_instruction_new!(_mean, $stack, $prim_stack, $stack);
        make_instruction_new!(_maximum, $stack, $prim_stack, $stack);
        make_instruction_new!(_minimum, $stack, $prim_stack, $stack);
        make_instruction_new!(_sum, $stack, $prim_stack, $stack);
        make_instruction_new!(_mode, $stack, $prim_stack, $stack);
        make_instruction_new!(_two_norm, $stack, $prim_stack, $stack);
        make_instruction_new!(_cumulative_sum, $stack, $stack, $stack);
    };
}

macro_rules! all_vector_instructions {
    () => {
        make_vector_instructions!(vector_int, int);
        make_vector_instructions!(vector_float, float);
        make_vector_instructions!(vector_string, string);
        make_vector_instructions!(vector_boolean, boolean);
        make_vector_instructions!(vector_char, char);
        make_vector_instructions!(string, char);

        // Need to pass a stack type to the empty! macro,
        // wont work in the make_vector_instructions macro without
        // bloating it a bit more
        make_instruction_empty!(_make_empty, vector_int, vector_int, i128);
        make_instruction_empty!(_make_empty, vector_float, vector_float, Decimal);
        make_instruction_empty!(_make_empty, vector_string, vector_string, Vec<char>);
        make_instruction_empty!(_make_empty, vector_boolean, vector_boolean, bool);
        make_instruction_empty!(_make_empty, vector_char, vector_char, char);
        make_instruction_empty!(_make_empty, string, string, char);

        // Numeric only vector instructions
        make_numeric_vector_instructions!(vector_int, int);
        make_numeric_vector_instructions!(vector_float, float);
    };
}
all_vector_instructions!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::numeric::int_inc;
    use crate::push::interpreter::interpret_program;
    use crate::push::state::EMPTY_STATE;
    use rust_decimal::dec;

    #[test]
    fn vector_concat_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![4, 5, 6], vec![1, 2, 3]];
        vector_int_concat(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3, 4, 5, 6]], test_state.vector_int);

        test_state.string = vec![vec!['s', 't'], vec!['t', 'e']];
        string_concat(&mut test_state);
        assert_eq!(vec![vec!['t', 'e', 's', 't']], test_state.string);

        test_state.vector_int = vec![empty_vec.clone(), empty_vec.clone()];
        vector_int_concat(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_conj_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![0];
        vector_int_conj(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 3]], test_state.vector_int);

        test_state.vector_int = vec![empty_vec];
        test_state.int = vec![0];
        vector_int_conj(&mut test_state);
        assert_eq!(vec![vec![0]], test_state.vector_int);
    }

    #[test]
    fn vector_conj_end_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![0];
        vector_int_conj_end(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3, 0]], test_state.vector_int);

        test_state.vector_int = vec![empty_vec];
        test_state.int = vec![0];
        vector_int_conj_end(&mut test_state);
        assert_eq!(vec![vec![0]], test_state.vector_int);
    }

    /// Tests take_n and take_last_n
    #[test]
    fn vector_takes_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        // n
        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![2];
        vector_int_take_n(&mut test_state);
        assert_eq!(vec![vec![1, 2]], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![0];
        vector_int_take_n(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![-5];
        vector_int_take_n(&mut test_state);
        assert_eq!(vec![vec![1, 2]], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![2];
        vector_int_take_n(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        // last n
        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![2];
        vector_int_take_last_n(&mut test_state);
        assert_eq!(vec![vec![2, 3]], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![0];
        vector_int_take_last_n(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3]];
        test_state.int = vec![-5];
        vector_int_take_last_n(&mut test_state);
        assert_eq!(vec![vec![2, 3]], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![2];
        vector_int_take_last_n(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);
    }

    #[test]
    fn vector_sub_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![1, 4];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![1, 10];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3, 4, 5]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![-1, 4];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![-4, -1];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![0, 0];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![2, 6];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![vec![0]];
        test_state.int = vec![2, 10];
        vector_int_sub(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_first_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_first(&mut test_state);
        assert_eq!(vec![0], test_state.int);

        test_state.string = vec![vec!['t', 'e', 's', 't']];
        string_first(&mut test_state);
        assert_eq!(vec!['t'], test_state.char);

        let empty_vec: Vec<i128> = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_first(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_from_first_prim_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_from_first_prim(&mut test_state);
        assert_eq!(vec![vec![0]], test_state.vector_int);

        let empty_vec = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_from_first_prim(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_from_prim_test() {
        let mut test_state = EMPTY_STATE;

        test_state.int = vec![1, 2];
        vector_int_from_prim(&mut test_state);
        assert_eq!(vec![vec![2]], test_state.vector_int);
    }

    #[test]
    fn vector_last_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_last(&mut test_state);
        assert_eq!(vec![5], test_state.int);

        test_state.string = vec![vec!['t', 'e', 's', 't', 's']];
        string_last(&mut test_state);
        assert_eq!(vec!['s'], test_state.char);

        let empty_vec: Vec<i128> = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_last(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_from_last_prim_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_from_last_prim(&mut test_state);
        assert_eq!(vec![vec![5]], test_state.vector_int);

        let empty_vec = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_from_last_prim(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_nth_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_nth(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.string = vec![vec!['t', 'e', 's', 't', 's']];
        test_state.int = vec![3];
        string_nth(&mut test_state);
        assert_eq!(vec!['t'], test_state.char);

        let empty_vec: Vec<i128> = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![10];
        vector_int_nth(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_from_nth_prim_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_from_nth_prim(&mut test_state);
        assert_eq!(vec![vec![2]], test_state.vector_int);

        let empty_vec = vec![];
        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![20];
        vector_int_from_nth_prim(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_rest_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_rest(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3, 4, 5]], test_state.vector_int);

        test_state.vector_int = vec![vec![0]];
        vector_int_rest(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_rest(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_but_last_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_but_last(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 3, 4]], test_state.vector_int);

        test_state.vector_int = vec![vec![0]];
        vector_int_but_last(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_but_last(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_drop_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_drop(&mut test_state);
        assert_eq!(vec![vec![2, 3, 4, 5]], test_state.vector_int);

        test_state.vector_int = vec![vec![0]];
        test_state.int = vec![2];
        vector_int_drop(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3, 4]];
        test_state.int = vec![4];
        vector_int_drop(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![30];
        vector_int_drop(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_drop_last_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_drop_last(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 3]], test_state.vector_int);

        test_state.vector_int = vec![vec![0]];
        test_state.int = vec![2];
        vector_int_drop_last(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![vec![1, 2, 3, 4]];
        test_state.int = vec![4];
        vector_int_drop_last(&mut test_state);
        assert_eq!(vec![empty_vec.clone()], test_state.vector_int);

        test_state.vector_int = vec![empty_vec.clone()];
        test_state.int = vec![30];
        vector_int_drop_last(&mut test_state);
        assert_eq!(vec![empty_vec], test_state.vector_int);
    }

    #[test]
    fn vector_length_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_length(&mut test_state);
        assert_eq!(vec![6], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_length(&mut test_state);
        assert_eq!(vec![0], test_state.int);
        test_state.int.clear();

        test_state.string = vec![vec!['t', 'e', 's']];
        string_length(&mut test_state);
        assert_eq!(vec![3], test_state.int);
    }

    #[test]
    fn vector_reverse() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        vector_int_reverse(&mut test_state);
        assert_eq!(vec![vec![5, 4, 3, 2, 1, 0]], test_state.vector_int);
    }

    #[test]
    fn push_all_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![1, 2, 3]];
        vector_int_push_all(&mut test_state);
        assert_eq!(vec![1, 2, 3], test_state.int);
    }

    #[test]
    fn make_empty_vec_test() {
        let mut test_state = EMPTY_STATE;

        vector_int_make_empty(&mut test_state);
        vector_float_make_empty(&mut test_state);
        string_make_empty(&mut test_state);
        assert_eq!(vec![Vec::<i128>::new()], test_state.vector_int);
        assert_eq!(vec![Vec::<Decimal>::new()], test_state.vector_float);
        assert_eq!(vec![Vec::<char>::new()], test_state.string);
    }

    #[test]
    fn is_empty_vec_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

        test_state.vector_int = vec![empty_vec.clone()];
        vector_int_is_vector_empty(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![1, 2]];
        vector_int_is_vector_empty(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn contains_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_contains(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![]];
        test_state.int = vec![2];
        vector_int_contains(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![9];
        vector_int_contains(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn contains_vector_non_contiguous_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![0, 2, 3, 4, 5, 1]];
        vector_int_contains_vector_non_contiguous(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![], vec![]];
        vector_int_contains_vector_non_contiguous(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![1, 2, 3], vec![0, 1, 2, 3, 4, 5]];
        vector_int_contains_vector_non_contiguous(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn contains_vector_contiguous() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![0, 1, 2, 3, 4, 5]];
        vector_int_contains_vector_contiguous(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![0, 2, 3, 4, 5, 1]];
        vector_int_contains_vector_contiguous(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![], vec![]];
        vector_int_contains_vector_contiguous(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![1, 2, 3], vec![0, 1, 2, 3, 4, 5]];
        vector_int_contains_vector_contiguous(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn index_of_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![2];
        vector_int_index_of(&mut test_state);
        assert_eq!(vec![2], test_state.int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![10];
        vector_int_index_of(&mut test_state);
        assert_eq!(vec![-1], test_state.int);

        test_state.vector_int = vec![vec![]];
        test_state.int = vec![10];
        vector_int_index_of(&mut test_state);
        assert_eq!(vec![-1], test_state.int);
    }

    #[test]
    fn index_of_vector_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5], vec![1, 2, 3]];
        vector_int_index_of_vector(&mut test_state);
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5], vec![10]];
        vector_int_index_of_vector(&mut test_state);
        assert_eq!(vec![-1], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![vec![], vec![]];
        vector_int_index_of_vector(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    #[test]
    fn occurrences_of_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![1];
        vector_int_occurrences_of(&mut test_state);
        assert_eq!(vec![1], test_state.int);

        test_state.vector_int = vec![vec![1], vec![1, 2, 3, 2, 2, 5]];
        test_state.int = vec![2];
        vector_int_occurrences_of(&mut test_state);
        assert_eq!(vec![3], test_state.int);

        test_state.vector_int = vec![vec![]];
        test_state.int = vec![1];
        vector_int_occurrences_of(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    #[test]
    fn occurrences_of_vector_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 1, 2], vec![1, 2]];
        vector_int_occurrences_of_vector(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![vec![1], vec![1, 2, 3, 2, 2, 5], vec![2, 2]];
        vector_int_occurrences_of_vector(&mut test_state);
        assert_eq!(vec![1], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![vec![], vec![]];
        vector_int_occurrences_of_vector(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    #[test]
    fn parse_to_prim_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2]];
        vector_int_parse_to_prim(&mut test_state);
        assert_eq!(vec![vec![0], vec![1], vec![2]], test_state.vector_int);
    }

    #[test]
    fn set_nth_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![99, 1];
        vector_int_set_nth(&mut test_state);
        assert_eq!(vec![vec![0, 99, 1, 2, 3, 4, 5]], test_state.vector_int);

        test_state.string = vec![vec!['t', 'e', 's', 't']];
        test_state.int = vec![2];
        test_state.char = vec!['z'];
        string_set_nth(&mut test_state);
        assert_eq!(vec![vec!['t', 'e', 'z', 's', 't']], test_state.string);

        test_state.vector_boolean = vec![vec![true, false, true]];
        test_state.int = vec![];
        test_state.boolean = vec![true];
        vector_boolean_set_nth(&mut test_state);
        assert_eq!(vec![vec![true, false, true]], test_state.vector_boolean);
    }

    #[test]
    fn split_on_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2]];
        test_state.int = vec![1];
        vector_int_split_on(&mut test_state);
        assert_eq!(vec![vec![0], vec![2]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 1, 5]];
        test_state.int = vec![1];
        vector_int_split_on(&mut test_state);
        assert_eq!(vec![vec![0], vec![2], vec![5]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 1]];
        test_state.int = vec![1];
        vector_int_split_on(&mut test_state);
        assert_eq!(vec![vec![0], vec![2]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 1]];
        test_state.int = vec![9];
        vector_int_split_on(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 1]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3]];
        test_state.int = vec![3];
        vector_int_split_on(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2]], test_state.vector_int);
    }

    #[test]
    fn replace_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        test_state.int = vec![3, 2];
        vector_int_replace(&mut test_state);
        assert_eq!(vec![vec![0, 1, 3, 3, 4, 5, 3]], test_state.vector_int);
    }

    #[test]
    fn remove_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        test_state.int = vec![3];
        vector_int_remove(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 4, 5, 2]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        test_state.int = vec![2];
        vector_int_remove(&mut test_state);
        assert_eq!(vec![vec![0, 1, 3, 4, 5]], test_state.vector_int);

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        test_state.int = vec![9];
        vector_int_remove(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 3, 4, 5, 2]], test_state.vector_int);
    }

    #[test]
    fn iterate_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        test_state.exec = vec![
            Gene::StateFunc(int_inc),
            Gene::StateFunc(vector_int_iterate),
        ];
        interpret_program(&mut test_state, 1000, 1000);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 3], test_state.int);
    }

    #[test]
    fn sort_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_sort(&mut test_state);
        assert_eq!(vec![vec![0, 1, 2, 2, 3, 4, 5]], test_state.vector_int);

        test_state.vector_float = vec![vec![dec!(0.0), dec!(1.2), dec!(-3.4)]];
        vector_float_sort(&mut test_state);
        assert_eq!(
            vec![vec![dec!(-3.4), dec!(0.0), dec!(1.2)]],
            test_state.vector_float
        );

        test_state.vector_int = vec![vec![]];
        vector_int_sort(&mut test_state);
        assert_eq!(vec![Vec::<i128>::new()], test_state.vector_int);
    }

    #[test]
    fn sort_reverse_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_sort_reverse(&mut test_state);
        assert_eq!(vec![vec![5, 4, 3, 2, 2, 1, 0]], test_state.vector_int);

        test_state.vector_float = vec![vec![dec!(0.0), dec!(1.2), dec!(-3.4)]];
        vector_float_sort_reverse(&mut test_state);
        assert_eq!(
            vec![vec![dec!(1.2), dec!(0.0), dec!(-3.4)]],
            test_state.vector_float
        );

        test_state.vector_int = vec![vec![]];
        vector_int_sort_reverse(&mut test_state);
        assert_eq!(vec![Vec::<i128>::new()], test_state.vector_int);
    }

    #[test]
    fn insert_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3]];
        test_state.int = vec![9, 1];
        vector_int_insert(&mut test_state);
        assert_eq!(vec![vec![0, 9, 1, 2, 3]], test_state.vector_int);

        test_state.vector_boolean = vec![vec![false, true, false]];
        test_state.int = vec![0];
        test_state.boolean = vec![false];
        vector_boolean_insert(&mut test_state);
        assert_eq!(
            vec![vec![false, false, true, false]],
            test_state.vector_boolean
        );

        test_state.vector_int = vec![vec![0, 1, 2, 3]];
        test_state.int = vec![9, 5];
        vector_int_insert(&mut test_state);
        assert_eq!(vec![vec![0, 9, 1, 2, 3]], test_state.vector_int);
    }

    #[test]
    fn insert_vector_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3], vec![69, 69]];
        test_state.int = vec![1];
        vector_int_insert_vector(&mut test_state);
        assert_eq!(vec![vec![0, 69, 69, 1, 2, 3]], test_state.vector_int);

        test_state.vector_boolean = vec![vec![false, true, false], vec![false, true]];
        test_state.int = vec![0];
        vector_boolean_insert_vector(&mut test_state);
        assert_eq!(
            vec![vec![false, true, false, true, false]],
            test_state.vector_boolean
        );

        test_state.vector_int = vec![vec![0, 1, 2, 3], vec![69, 69]];
        test_state.int = vec![5];
        vector_int_insert_vector(&mut test_state);
        assert_eq!(vec![vec![0, 69, 69, 1, 2, 3]], test_state.vector_int);
    }

    #[test]
    fn mean_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![6, 5, 4]];
        vector_int_mean(&mut test_state);
        assert_eq!(vec![5], test_state.int);

        test_state.vector_float = vec![vec![dec!(6.0), dec!(5.0), dec!(4.0)]];
        vector_float_mean(&mut test_state);
        assert_eq!(vec![dec!(5.0)], test_state.float);
    }

    #[test]
    fn minimum_maximum_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_maximum(&mut test_state);
        assert_eq!(vec![5], test_state.int);
        test_state.int.clear();

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_minimum(&mut test_state);
        assert_eq!(vec![0], test_state.int);
    }

    #[test]
    fn sum_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_sum(&mut test_state);
        assert_eq!(vec![17], test_state.int);
    }

    #[test]
    fn mode_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_mode(&mut test_state);
        assert_eq!(vec![2], test_state.int);
        test_state.int.clear();

        // returns the last mode to be calculated
        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2, 4, 3]];
        vector_int_mode(&mut test_state);
        assert_eq!(vec![3], test_state.int);
    }

    #[test]
    fn two_norm_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![5, 5, 5, 5]];
        vector_int_two_norm(&mut test_state);
        assert_eq!(vec![10], test_state.int);

        test_state.vector_float = vec![vec![dec!(5.0), dec!(5.0), dec!(5.0), dec!(5.0)]];
        vector_float_two_norm(&mut test_state);
        assert_eq!(vec!(dec!(10.0)), test_state.float);
    }

    #[test]
    fn cumulative_sum_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5, 2]];
        vector_int_cumulative_sum(&mut test_state);
        assert_eq!(vec![vec![0, 1, 3, 6, 10, 15, 17]], test_state.vector_int);
    }
}
