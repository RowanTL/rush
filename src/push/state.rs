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
    pub boolean: Vec<bool>,
    pub char: Vec<u8>,
    pub vector_int: Vec<Vec<i128>>,
    pub vector_float: Vec<Vec<Decimal>>,
    pub vector_string: Vec<Vec<Vec<u8>>>,
    pub vector_boolean: Vec<Vec<bool>>,
    pub vector_char: Vec<Vec<u8>>,
    pub exec: Vec<Gene>,
    pub code: Vec<Gene>,
}

pub const EMPTY_STATE: PushState = PushState {
    int: vec![],
    float: vec![],
    string: vec![],
    boolean: vec![],
    char: vec![],
    vector_int: vec![],
    vector_float: vec![],
    vector_string: vec![],
    vector_boolean: vec![],
    vector_char: vec![],
    exec: vec![],
    code: vec![],
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Gene {
    GeneInt(i128),
    GeneFloat(Decimal),
    GeneBoolean(bool),
    GeneString(Vec<u8>),
    GeneChar(u8),
    GeneVectorInt(Vec<i128>),
    GeneVectorFloat(Vec<Decimal>),
    GeneVectorBoolean(Vec<bool>),
    GeneVectorString(Vec<Vec<u8>>),
    GeneVectorChar(Vec<u8>),
    StateFunc(fn(&mut PushState)),
    Close,
    Open(u8),
    Skip,
    Block(Box<Vec<Gene>>),
    CrossoverPadding,
}
