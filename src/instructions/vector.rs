use crate::push::state::PushState;
use rust_decimal::Decimal;

/// Generates an index between 0 and length. Takes abs(num) and then mods it by length.
fn bounded_idx(num: i128, length: usize) -> usize {
    (num.unsigned_abs() as usize) % length
}

/// Concats two vectors together.
pub fn _concat<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut concat_vec = vals[0].clone();
    concat_vec.extend(vals[1].clone().into_iter());
    Some(concat_vec)
}
make_instruction_clone!(vector_int, vector_int, _concat, Vec<i128>, 2);
make_instruction_clone!(vector_float, vector_float, _concat, Vec<Decimal>, 2);
make_instruction_clone!(vector_string, vector_string, _concat, Vec<Vec<char>>, 2);
make_instruction_clone!(vector_boolean, vector_boolean, _concat, Vec<bool>, 2);
make_instruction_clone!(vector_char, vector_char, _concat, Vec<char>, 2);
make_instruction_clone!(string, string, _concat, Vec<char>, 2);

/// Prepends a primitive value to a vector.
pub fn _conj<T>(vec_vals: Vec<Vec<T>>, prim_vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut t_vec = vec_vals[0].clone();
    t_vec.insert(0, prim_vals[0].clone());
    Some(t_vec)
}
make_instruction_aux!(vector_int, vector_int, _conj, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    vector_float,
    _conj,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _conj,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _conj,
    Vec<bool>,
    1,
    boolean,
    1,
    bool
);
make_instruction_aux!(vector_char, vector_char, _conj, Vec<char>, 1, char, 1, char);
make_instruction_aux!(string, string, _conj, Vec<char>, 1, char, 1, char);

/// Appends a primitive value to a vector.
pub fn _conj_end<T>(vec_vals: Vec<Vec<T>>, prim_vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut t_vec = vec_vals[0].clone();
    t_vec.push(prim_vals[0].clone());
    Some(t_vec)
}
make_instruction_aux!(
    vector_int,
    vector_int,
    _conj_end,
    Vec<i128>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_float,
    vector_float,
    _conj_end,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _conj_end,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _conj_end,
    Vec<bool>,
    1,
    boolean,
    1,
    bool
);
make_instruction_aux!(
    vector_char,
    vector_char,
    _conj_end,
    Vec<char>,
    1,
    char,
    1,
    char
);
make_instruction_aux!(string, string, _conj_end, Vec<char>, 1, char, 1, char);

/// Takes the first N items from a vector. N based on an int.
pub fn _take_n<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(ret_vec[0..bounded_idx(auxs[0], ret_vec.len())].to_vec())
}
make_instruction_aux!(vector_int, vector_int, _take_n, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    vector_float,
    _take_n,
    Vec<Decimal>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _take_n,
    Vec<Vec<char>>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _take_n,
    Vec<bool>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_char,
    vector_char,
    _take_n,
    Vec<char>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(string, string, _take_n, Vec<char>, 1, int, 1, i128);

/// Takes the first N items from a vector. N based on an int.
pub fn _take_last_n<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    let ret_vec_len = ret_vec.len();
    Some(ret_vec[ret_vec_len - bounded_idx(auxs[0], ret_vec_len)..ret_vec_len].to_vec())
}
make_instruction_aux!(
    vector_int,
    vector_int,
    _take_last_n,
    Vec<i128>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_float,
    vector_float,
    _take_last_n,
    Vec<Decimal>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _take_last_n,
    Vec<Vec<char>>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _take_last_n,
    Vec<bool>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_char,
    vector_char,
    _take_last_n,
    Vec<char>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(string, string, _take_last_n, Vec<char>, 1, int, 1, i128);

/// Takes a sublist of a vector based on two ints.
pub fn _sub<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    let (mut start, mut end): (usize, usize) = (
        auxs[0].unsigned_abs() as usize,
        auxs[1].unsigned_abs() as usize,
    );
    if start > end {
        (start, end) = (end, start)
    }
    let fin_start = start.min(ret_vec.len());
    let fin_end = end.min(ret_vec.len());
    Some(ret_vec[fin_start..fin_end].to_vec())
}
make_instruction_aux!(vector_int, vector_int, _sub, Vec<i128>, 1, int, 2, i128);
make_instruction_aux!(
    vector_float,
    vector_float,
    _sub,
    Vec<Decimal>,
    1,
    int,
    2,
    i128
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _sub,
    Vec<Vec<char>>,
    1,
    int,
    2,
    i128
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _sub,
    Vec<bool>,
    1,
    int,
    2,
    i128
);
make_instruction_aux!(vector_char, vector_char, _sub, Vec<char>, 1, int, 2, i128);
make_instruction_aux!(string, string, _sub, Vec<char>, 1, int, 2, i128);

/// Takes the first item from a vector.
pub fn _first<T>(vals: Vec<Vec<T>>) -> Option<T>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vals[0][0].clone())
}
make_instruction_clone!(vector_int, int, _first, Vec<i128>, 1);
make_instruction_clone!(vector_float, float, _first, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, string, _first, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, boolean, _first, Vec<bool>, 1);
make_instruction_clone!(vector_char, char, _first, Vec<char>, 1);
make_instruction_clone!(string, char, _first, Vec<char>, 1);

/// Takes the first item from a vector, wraps it into a vector, and pushes it back
/// to the same stack.
pub fn _from_first_prim<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vec![vals[0][0].clone()])
}
make_instruction_clone!(vector_int, vector_int, _from_first_prim, Vec<i128>, 1);
make_instruction_clone!(
    vector_float,
    vector_float,
    _from_first_prim,
    Vec<Decimal>,
    1
);
make_instruction_clone!(
    vector_string,
    vector_string,
    _from_first_prim,
    Vec<Vec<char>>,
    1
);
make_instruction_clone!(
    vector_boolean,
    vector_boolean,
    _from_first_prim,
    Vec<bool>,
    1
);
make_instruction_clone!(vector_char, vector_char, _from_first_prim, Vec<char>, 1);
make_instruction_clone!(string, string, _from_first_prim, Vec<char>, 1);

/// Places the top of a primitive type into a vector
pub fn _from_prim<T>(vals: Vec<T>) -> Option<Vec<T>>
where
    T: Clone,
{
    Some(vec![vals[0].clone()])
}
make_instruction_out!(int, vector_int, _from_prim, i128, 1);
make_instruction_out!(float, vector_float, _from_prim, Decimal, 1);
make_instruction_out!(string, vector_string, _from_prim, Vec<char>, 1);
make_instruction_out!(boolean, vector_boolean, _from_prim, bool, 1);
make_instruction_out!(char, vector_char, _from_prim, char, 1);
make_instruction_out!(char, string, _from_prim, char, 1);

/// Takes the last item from a vector.
pub fn _last<T>(vals: Vec<Vec<T>>) -> Option<T>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vals[0][ret_vec.len() - 1].clone())
}
make_instruction_clone!(vector_int, int, _last, Vec<i128>, 1);
make_instruction_clone!(vector_float, float, _last, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, string, _last, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, boolean, _last, Vec<bool>, 1);
make_instruction_clone!(vector_char, char, _last, Vec<char>, 1);
make_instruction_clone!(string, char, _last, Vec<char>, 1);

/// Takes the last item from a vector, wraps it into a vector, and pushes it back
/// to the same stack.
pub fn _from_last_prim<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vec![vals[0][ret_vec.len() - 1].clone()])
}
make_instruction_clone!(vector_int, vector_int, _from_last_prim, Vec<i128>, 1);
make_instruction_clone!(vector_float, vector_float, _from_last_prim, Vec<Decimal>, 1);
make_instruction_clone!(
    vector_string,
    vector_string,
    _from_last_prim,
    Vec<Vec<char>>,
    1
);
make_instruction_clone!(
    vector_boolean,
    vector_boolean,
    _from_last_prim,
    Vec<bool>,
    1
);
make_instruction_clone!(vector_char, vector_char, _from_last_prim, Vec<char>, 1);
make_instruction_clone!(string, string, _from_last_prim, Vec<char>, 1);

/// Takes the nth item from a vector. N from int stack.
pub fn _nth<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<T>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vals[0][bounded_idx(auxs[0], ret_vec.len())].clone())
}
make_instruction_aux!(vector_int, int, _nth, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(vector_float, float, _nth, Vec<Decimal>, 1, int, 1, i128);
make_instruction_aux!(vector_string, string, _nth, Vec<Vec<char>>, 1, int, 1, i128);
make_instruction_aux!(vector_boolean, boolean, _nth, Vec<bool>, 1, int, 1, i128);
make_instruction_aux!(vector_char, char, _nth, Vec<char>, 1, int, 1, i128);
make_instruction_aux!(string, char, _nth, Vec<char>, 1, int, 1, i128);

/// Takes the nth item from a vector, wraps it into a vector, and pushes it back
/// to the same stack. N from int stack
pub fn _from_nth_prim<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(vec![vals[0][bounded_idx(auxs[0], ret_vec.len())].clone()])
}
make_instruction_aux!(
    vector_int,
    vector_int,
    _from_nth_prim,
    Vec<i128>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_float,
    vector_float,
    _from_nth_prim,
    Vec<Decimal>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _from_nth_prim,
    Vec<Vec<char>>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _from_nth_prim,
    Vec<bool>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_char,
    vector_char,
    _from_nth_prim,
    Vec<char>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    string,
    vector_char,
    _from_nth_prim,
    Vec<char>,
    1,
    int,
    1,
    i128
);

/// Takes a vector and removes the first element.
pub fn _rest<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(ret_vec[1..].to_vec())
}
make_instruction_clone!(vector_int, vector_int, _rest, Vec<i128>, 1);
make_instruction_clone!(vector_float, vector_float, _rest, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, vector_string, _rest, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, vector_boolean, _rest, Vec<bool>, 1);
make_instruction_clone!(vector_char, vector_char, _rest, Vec<char>, 1);
make_instruction_clone!(string, string, _rest, Vec<char>, 1);

/// Takes a vector and removes the last element.
pub fn _but_last<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    Some(ret_vec[0..ret_vec.len() - 1].to_vec())
}
make_instruction_clone!(vector_int, vector_int, _but_last, Vec<i128>, 1);
make_instruction_clone!(vector_float, vector_float, _but_last, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, vector_string, _but_last, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, vector_boolean, _but_last, Vec<bool>, 1);
make_instruction_clone!(vector_char, vector_char, _but_last, Vec<char>, 1);
make_instruction_clone!(string, string, _but_last, Vec<char>, 1);

/// Removes the first n items from a vector. n from the int stack.
pub fn _drop<T>(vals: Vec<Vec<T>>, auxs: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut ret_vec = vals[0].clone();
    if ret_vec.is_empty() {
        return None;
    }
    ret_vec.drain(0..auxs[0].abs().min(ret_vec.len() as i128) as usize);
    Some(ret_vec)
}
make_instruction_aux!(vector_int, vector_int, _drop, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    vector_float,
    _drop,
    Vec<Decimal>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_string,
    vector_string,
    _drop,
    Vec<Vec<char>>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(
    vector_boolean,
    vector_boolean,
    _drop,
    Vec<bool>,
    1,
    int,
    1,
    i128
);
make_instruction_aux!(vector_char, vector_char, _drop, Vec<char>, 1, int, 1, i128);
make_instruction_aux!(string, string, _drop, Vec<char>, 1, int, 1, i128);

/// Takes the length of a vector.
pub fn _length<T>(vals: Vec<Vec<T>>) -> Option<i128> {
    Some(vals[0].len() as i128)
}
make_instruction_clone!(vector_int, int, _length, Vec<i128>, 1);
make_instruction_clone!(vector_float, int, _length, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, int, _length, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, int, _length, Vec<bool>, 1);
make_instruction_clone!(vector_char, int, _length, Vec<char>, 1);
make_instruction_clone!(string, int, _length, Vec<char>, 1);

/// Reverses a vector
pub fn _reverse<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut rev_vec = vals[0].clone();
    rev_vec.reverse();
    Some(rev_vec)
}
make_instruction_clone!(vector_int, vector_int, _reverse, Vec<i128>, 1);
make_instruction_clone!(vector_float, vector_float, _reverse, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, vector_string, _reverse, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, vector_boolean, _reverse, Vec<bool>, 1);
make_instruction_clone!(vector_char, vector_char, _reverse, Vec<char>, 1);
make_instruction_clone!(string, string, _reverse, Vec<char>, 1);

/// Pushes all values of a vector into a primitive stack
pub fn _push_all<T>(vals: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Clone,
{
    Some(vals[0].clone())
}
make_instruction_mult!(vector_int, int, _push_all, Vec<i128>, 1);
make_instruction_mult!(vector_float, float, _push_all, Vec<Decimal>, 1);
// make_instruction_mult!(vector_string, string, _push_all, Vec<Vec<char>>, 1); // Optional
make_instruction_mult!(vector_boolean, boolean, _push_all, Vec<bool>, 1);
make_instruction_mult!(vector_char, char, _push_all, Vec<char>, 1);
make_instruction_mult!(string, char, _push_all, Vec<char>, 1);

/// Creates an empty vector
pub fn _make_empty<T>(_: Vec<Vec<T>>) -> Option<Vec<T>> {
    let empty_vec: Vec<T> = Vec::new();
    Some(empty_vec)
}
make_instruction_clone!(vector_int, vector_int, _make_empty, Vec<i128>, 0);
make_instruction_clone!(vector_float, vector_float, _make_empty, Vec<Decimal>, 0);
make_instruction_clone!(vector_string, vector_string, _make_empty, Vec<Vec<char>>, 0);
make_instruction_clone!(vector_boolean, vector_boolean, _make_empty, Vec<bool>, 0);
make_instruction_clone!(vector_char, vector_char, _make_empty, Vec<char>, 0);
make_instruction_clone!(string, string, _make_empty, Vec<char>, 0);

/// Checks if a vector is empty. Pushes true if is, false otherwise
pub fn _is_empty<T>(vals: Vec<Vec<T>>) -> Option<bool> {
    Some(vals[0].is_empty())
}
make_instruction_clone!(vector_int, boolean, _is_empty, Vec<i128>, 1);
make_instruction_clone!(vector_float, boolean, _is_empty, Vec<Decimal>, 1);
make_instruction_clone!(vector_string, boolean, _is_empty, Vec<Vec<char>>, 1);
make_instruction_clone!(vector_boolean, boolean, _is_empty, Vec<bool>, 1);
make_instruction_clone!(vector_char, boolean, _is_empty, Vec<char>, 1);
make_instruction_clone!(string, boolean, _is_empty, Vec<char>, 1);

/// Checks if a vector contains a primitive. True if does, false otherwise
pub fn _contains<T>(vals: Vec<Vec<T>>, auxs: Vec<T>) -> Option<bool>
where
    T: Eq,
{
    Some(vals[0].contains(&auxs[0]))
}
make_instruction_aux!(vector_int, boolean, _contains, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    boolean,
    _contains,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal
);
make_instruction_aux!(
    vector_string,
    boolean,
    _contains,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>
);
make_instruction_aux!(
    vector_boolean,
    boolean,
    _contains,
    Vec<bool>,
    1,
    boolean,
    1,
    bool
);
make_instruction_aux!(vector_char, boolean, _contains, Vec<char>, 1, char, 1, char);
make_instruction_aux!(string, boolean, _contains, Vec<char>, 1, char, 1, char);

/// Returns the index of a primitive in a vector, pushes result to int stack
pub fn _index_of<T>(vals: Vec<Vec<T>>, auxs: Vec<T>) -> Option<i128>
where
    T: Clone + Eq,
{
    let temp_vec = &vals[0];
    let temp_aux = &auxs[0];
    if let Some(idx) = temp_vec.iter().position(|r| r == temp_aux) {
        return Some(idx as i128);
    }
    Some(-1)
}
make_instruction_aux!(vector_int, int, _index_of, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    int,
    _index_of,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal
);
make_instruction_aux!(
    vector_string,
    int,
    _index_of,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>
);
make_instruction_aux!(
    vector_boolean,
    int,
    _index_of,
    Vec<bool>,
    1,
    boolean,
    1,
    bool
);
make_instruction_aux!(vector_char, int, _index_of, Vec<char>, 1, char, 1, char);
make_instruction_aux!(string, int, _index_of, Vec<char>, 1, char, 1, char);

/// Counts the amount of a primitive in a vector
pub fn _occurrences_of<T>(vals: Vec<Vec<T>>, auxs: Vec<T>) -> Option<i128>
where
    T: Clone + Eq,
{
    let temp_aux = &auxs[0];
    Some(
        vals[0]
            .clone()
            .into_iter()
            .filter(|r| r == temp_aux)
            .count() as i128,
    )
}
make_instruction_aux!(vector_int, int, _occurrences_of, Vec<i128>, 1, int, 1, i128);
make_instruction_aux!(
    vector_float,
    int,
    _occurrences_of,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal
);
make_instruction_aux!(
    vector_string,
    int,
    _occurrences_of,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>
);
make_instruction_aux!(
    vector_boolean,
    int,
    _occurrences_of,
    Vec<bool>,
    1,
    boolean,
    1,
    bool
);
make_instruction_aux!(
    vector_char,
    int,
    _occurrences_of,
    Vec<char>,
    1,
    char,
    1,
    char
);
make_instruction_aux!(string, int, _occurrences_of, Vec<char>, 1, char, 1, char);

/// Sets the nth index in a vector. N from the int stack.
pub fn _set_nth<T>(vals: Vec<Vec<T>>, aux0: Vec<T>, aux1: Vec<i128>) -> Option<Vec<T>>
where
    T: Clone,
{
    let mut temp_vec = vals[0].clone();
    let idx = bounded_idx(aux1[0], temp_vec.len());
    temp_vec.insert(idx, aux0[0].clone());
    Some(temp_vec)
}
make_instruction_aux2!(
    vector_int,
    vector_int,
    _set_nth,
    Vec<i128>,
    1,
    int,
    1,
    i128,
    int,
    1,
    i128
);
make_instruction_aux2!(
    vector_float,
    vector_float,
    _set_nth,
    Vec<Decimal>,
    1,
    float,
    1,
    Decimal,
    int,
    1,
    i128
);
make_instruction_aux2!(
    vector_string,
    vector_string,
    _set_nth,
    Vec<Vec<char>>,
    1,
    string,
    1,
    Vec<char>,
    int,
    1,
    i128
);
make_instruction_aux2!(
    vector_boolean,
    vector_boolean,
    _set_nth,
    Vec<bool>,
    1,
    boolean,
    1,
    bool,
    int,
    1,
    i128
);
make_instruction_aux2!(
    vector_char,
    vector_char,
    _set_nth,
    Vec<char>,
    1,
    char,
    1,
    char,
    int,
    1,
    i128
);
make_instruction_aux2!(
    string,
    string,
    _set_nth,
    Vec<char>,
    1,
    char,
    1,
    char,
    int,
    1,
    i128
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;

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
        vector_int_is_empty(&mut test_state);
        assert_eq!(vec![true], test_state.boolean);
        test_state.boolean.clear();

        test_state.vector_int = vec![vec![1, 2]];
        vector_int_is_empty(&mut test_state);
        assert_eq!(vec![false], test_state.boolean);
    }

    #[test]
    fn contains_test() {
        let mut test_state = EMPTY_STATE;
        let empty_vec: Vec<i128> = vec![];

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
    fn set_nth_test() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![0, 1, 2, 3, 4, 5]];
        test_state.int = vec![99, 1];
        vector_int_set_nth(&mut test_state);
        assert_eq!(vec![vec![0, 99, 1, 2, 3, 4, 5]], test_state.vector_int);

        // Write more tests tmo!
    }
}
