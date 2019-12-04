use std::rc::Rc;
use std::cell::RefCell;
use crate::types::*;

impl Obj {
    fn new(val: ObjVal, loc: Option<Loc>) -> Obj {
        Obj{ val: Rc::new(RefCell::new(val)), loc }
    }
    
    pub fn new_int(num: i64, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Int(num), loc)
    }
    
    pub fn new_float(num: f64, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Float(num), loc)
    }

    pub fn new_list(xs: Vec<Obj>, loc:Option<Loc>) -> Obj {
        Obj::new(ObjVal::List(xs), loc)
    }
    
    pub fn new_symb(name: String, loc:Option<Loc>) -> Obj {
        Obj::new(ObjVal::Symbol(name), loc)
    }

    pub fn is_list(&self) -> bool {
        if let ObjVal::List(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_float(&self) -> bool {
        if let ObjVal::Float(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_int(&self) -> bool {
        if let ObjVal::Int(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_symbol(&self) -> bool {
        if let ObjVal::Symbol(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }
}



