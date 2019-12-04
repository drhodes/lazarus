use crate::types::*;
use std::collections::HashMap;

impl Frame {
    pub fn new() -> Frame {
        Frame{symbol_table: HashMap::new()}
    }

    pub fn insert(&mut self, sym: Symb, obj: Obj) {
        self.symbol_table.insert(sym, obj);
    }
}
