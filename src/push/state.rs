use rust_decimal::prelude::*;

/// The declaration of the state that push operates on.
///
/// I chose to use `rust_decimal` crate here because
/// there are round off errors with the build in `f64`.
#[derive(Debug, Clone)]
pub struct PushState {
    pub int: Vec<i128>,
    pub float: Vec<Decimal>,
    pub string: Vec<Vec<u8>>,
    pub bool: Vec<bool>,
    pub char: Vec<u8>,
    pub vector_int: Vec<Vec<i128>>,
    pub vector_float: Vec<Vec<Decimal>>,
    pub vector_string: Vec<Vec<Vec<u8>>>,
    pub vector_bool: Vec<Vec<bool>>,
    pub vector_char: Vec<u8>,
}

pub const EMPTY_STATE: PushState = PushState {
    int: vec![],
    float: vec![],
    string: vec![],
    bool: vec![],
    char: vec![],
    vector_int: vec![],
    vector_float: vec![],
    vector_string: vec![],
    vector_bool: vec![],
    vector_char: vec![],
};
