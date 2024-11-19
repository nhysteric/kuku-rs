use crate::hash::HashFunc;
use crate::{LocaltionType, TableSizeType};
#[derive(Debug, Clone, PartialEq)]
pub struct LocFunc {
    table_size: TableSizeType,
    h: HashFunc,
}

impl LocFunc {
    pub fn new(table_size: TableSizeType, seed: u128) -> LocFunc {
        LocFunc {
            table_size,
            h: HashFunc::new(seed),
        }
    }
    pub fn hash(&self, item: &[u8; 16]) -> LocaltionType {
        self.h.hash(item) % self.table_size
    }
}
