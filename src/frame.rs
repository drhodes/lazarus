use crate::types::*;
use std::collections::HashMap;

impl Frame {
    pub fn new() -> Frame {
        Frame { symbol_table: HashMap::new() }
    }

    pub fn from_var_vals(mut params: Obj, mut arguments: Obj) -> EvalResult<Frame> {
        if params.list_length()? != arguments.list_length()? {
            Err("Frame got bad constructor params, unmatched lengths".to_string())
        } else {
            let mut frame = Frame::new();
        
            while !params.is_null()? {
                frame.insert(
                    params.car()?.to_symb()?,
                    arguments.car()?
                );
                params = params.cdr()?;
                arguments = arguments.cdr()?;
            }
            Ok(frame)
        }
        
    }
        
    
    pub fn insert(&mut self, sym: Symb, obj: Obj) {
        self.symbol_table.insert(sym, obj);
    }

    // get value stored in frame for a certain symbol
    pub fn get(&self, sym: &Symb) -> Option<Obj> {
        match self.symbol_table.get(sym) {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    pub fn all_names(&self) -> Vec<String> {
        self.symbol_table.keys().map(|s| s.name.clone()).collect()
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
        let obj1cell: Obj = f.get(&sym1).unwrap();
        let obj2cell: Obj = f.get(&sym1).unwrap();
        let newval = ObjVal::Int(43);
        *obj1cell.val.borrow_mut() = newval.clone();
        assert_eq!(*obj2cell.val.borrow(), newval.clone());
    }
}
