use std::rc::Rc;
use std::cell::RefCell;
use crate::types::*;

impl Obj {
    fn new(val: ObjVal, loc: Option<Loc>) -> Obj {
        Obj{ val: Rc::new(RefCell::new(val)), loc }
    }
    
    pub fn new_int(num: u64, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Int(num), loc)
    }
}

