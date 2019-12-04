use crate::types::*;
use std::collections::HashMap;
use std::cell::RefMut;
use std::rc::Rc;

impl Frame {
    pub fn new() -> Frame {
        Frame{symbol_table: HashMap::new()}
    }

    pub fn insert(&mut self, sym: Symb, obj: Obj) {
        self.symbol_table.insert(sym, obj);
    }

    pub fn get(&self, sym: &Symb) -> Obj {        
        match self.symbol_table.get(sym) {
            Some(obj) => obj.clone(),
            None => {
                let msg = format!("frame doesn't contain symbol: {:?}", sym);
                // TODO return a EvalError here perhaps.
                panic!(msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn frame_1() {
        let mut f = Frame::new();
        let sym1 = Symb::new("a", "test.scm".to_owned(), 0);
        let obj1 = Obj::new_int(42, None);
        
        f.insert(sym1.clone(), obj1);
        let obj1cell: Obj = f.get(&sym1);
        let obj2cell: Obj = f.get(&sym1);
        let newval = ObjVal::Int(43);
        *obj1cell.val.borrow_mut() = newval.clone();
        assert_eq!(*obj2cell.val.borrow(), newval.clone());
    }
}
