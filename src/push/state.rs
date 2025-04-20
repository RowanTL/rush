use rust_decimal::prelude::*;

/// The declaration of the state that push operates on.
///
/// I chose to use `rust_decimal` crate here because
/// there are round off errors with the build in `f64`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushState {
    pub int: Vec<i128>,
    pub float: Vec<Decimal>,
    pub string: Vec<Vec<char>>,
    pub boolean: Vec<bool>,
    pub char: Vec<char>,
    pub vector_int: Vec<Vec<i128>>,
    pub vector_float: Vec<Vec<Decimal>>,
    pub vector_string: Vec<Vec<Vec<char>>>,
    pub vector_boolean: Vec<Vec<bool>>,
    pub vector_char: Vec<Vec<char>>,
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
#[allow(dead_code)] // remove this later. Is here bc Close, Skip, CrossoverPadding
pub enum Gene {
    GeneInt(i128),
    GeneFloat(Decimal),
    GeneBoolean(bool),
    GeneString(Vec<char>),
    GeneChar(char),
    GeneVectorInt(Vec<i128>),
    GeneVectorFloat(Vec<Decimal>),
    GeneVectorBoolean(Vec<bool>),
    GeneVectorString(Vec<Vec<char>>),
    GeneVectorChar(Vec<char>),
    StateFunc(fn(&mut PushState)),
    Close,
    Open(u8),
    Skip,
    Block(Vec<Gene>),
    CrossoverPadding,
}

impl Gene {
    /// Returns the len of a gene. If the gene is a block, returns
    /// the size of the block counting the size of nested blocks.
    pub fn rec_len(&self) -> usize {
        let mut size: usize = 0;
        match self {
            Gene::Block(val) => {
                for el in val.iter() {
                    match el {
                        iblock @ Gene::Block(_) => size += iblock.rec_len() + 1,
                        _ => size += 1,
                    }
                }
            }
            _ => size += 1,
        };
        size
    }

    /// Extracts code at a point in the genome. Recurses into a block
    /// if necessary. Point is based on an int. Pulled straight from HushGP.
    pub fn code_at_point(self, index: usize) -> Option<Gene> {
        if index == 0 {
            return Some(self);
        }
        let mut idx = index;
        match self {
            Gene::Block(val) => {
                for el in val.iter() {
                    idx -= 1;
                    if idx == 0 {
                        return Some(el.clone());
                    }
                    match el {
                        iblock @ Gene::Block(_) => {
                            if let Some(next_depth) = iblock.clone().code_at_point(idx) {
                                return Some(next_depth);
                            }
                            idx -= iblock.rec_len();
                        }
                        _ => continue,
                    }
                }
                None
            }
            val => Some(val),
        }
    }

    /// Insert an element into a block recursively counting along the way.
    /// Depth first. Modifies in-place.
    pub fn with_code_inserted_at_point(&mut self, gene: Gene, idx: usize) {
        if idx > self.rec_len() {
            match self {
                Gene::Block(val) => val.push(gene.clone()),
                _ => {
                    panic!("Error: self must be a block for with_code_inserted_at_point to work!")
                }
            }
        }
        let _ = self.attempt_code_insert(gene, idx);
    }

    /// Attempts to insert an item into a block.
    fn attempt_code_insert(&mut self, gene: Gene, index: usize) -> bool {
        let mut idx = index;
        match self {
            Gene::Block(val) => {
                for (n, el) in val.iter_mut().enumerate() {
                    if idx == 0 {
                        val.insert(n, gene.clone());
                        return true;
                    }
                    match el {
                        iblock @ Gene::Block(_) => {
                            // This line has side effects on iblock if inserts properly.
                            let success = iblock.attempt_code_insert(gene.clone(), idx - 1);
                            if success {
                                return true;
                            }
                            idx -= iblock.rec_len() + 1
                        }
                        _ => (),
                    }
                    idx -= 1;
                }
                if idx == 0 {
                    return true;
                }
                false
            }
            _ => panic!("Error: self must be a block for attempt_code_insert to work!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rec_len_test() {
        let block = Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::Block(vec![Gene::GeneInt(1), Gene::GeneInt(1)]),
        ]);
        assert_eq!(4, block.rec_len());

        let block = Gene::Block(vec![
            Gene::GeneBoolean(true),
            Gene::GeneInt(1),
            Gene::Block(vec![
                Gene::GeneInt(4),
                Gene::GeneFloat(dec!(6.0)),
                Gene::Block(vec![Gene::GeneString(vec!['t'])]),
            ]),
            Gene::GeneInt(10),
            Gene::Block(vec![Gene::GeneBoolean(false)]),
        ]);
        assert_eq!(10, block.rec_len());

        let block = Gene::Block(vec![]);
        assert_eq!(0, block.rec_len());
    }

    #[test]
    fn insert_test() {
        let mut block = Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::Block(vec![Gene::GeneInt(1), Gene::GeneInt(1)]),
        ]);
        let inserted_block = Gene::Block(vec![
            Gene::GeneInt(1),
            Gene::GeneInt(20),
            Gene::Block(vec![Gene::GeneInt(1), Gene::GeneInt(1)]),
        ]);
        block.with_code_inserted_at_point(Gene::GeneInt(20), 1);
        assert_eq!(inserted_block, block);

        let mut block = Gene::Block(vec![
            Gene::GeneBoolean(true),
            Gene::GeneInt(1),
            Gene::Block(vec![
                Gene::GeneInt(4),
                Gene::GeneFloat(dec!(6.0)),
                Gene::Block(vec![Gene::GeneString(vec!['t'])]),
            ]),
            Gene::GeneInt(10),
            Gene::Block(vec![Gene::GeneBoolean(false)]),
        ]);
        let inserted_block = Gene::Block(vec![
            Gene::GeneBoolean(true),
            Gene::GeneInt(1),
            Gene::Block(vec![
                Gene::GeneInt(4),
                Gene::GeneInt(20),
                Gene::GeneFloat(dec!(6.0)),
                Gene::Block(vec![Gene::GeneString(vec!['t'])]),
            ]),
            Gene::GeneInt(10),
            Gene::Block(vec![Gene::GeneBoolean(false)]),
        ]);
        block.with_code_inserted_at_point(Gene::GeneInt(20), 4);
        assert_eq!(inserted_block, block);
    }
}
