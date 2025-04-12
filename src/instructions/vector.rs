use rust_decimal::Decimal;
use crate::push::state::PushState;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::state::EMPTY_STATE;

    #[test]
    fn test_vector_concat() {
        let mut test_state = EMPTY_STATE;

        test_state.vector_int = vec![vec![4, 5, 6], vec![1, 2, 3]];
        vector_int_concat(&mut test_state);
        assert_eq!(vec![vec![1, 2, 3, 4, 5, 6]], test_state.vector_int);

        test_state.string = vec![vec!['s', 't'], vec!['t', 'e']];
        string_concat(&mut test_state);
        assert_eq!(vec![vec!['t', 'e', 's', 't']], test_state.string);
    }
}
