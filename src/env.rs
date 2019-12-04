use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use crate::types::*;

// ------------------------------------------------------------------

impl Env {
    pub fn new() -> Env {
        Env{frame: HashMap::new(), enclosing:None}
    }
    
    fn is_global(&self) -> bool {
        self.enclosing.is_none()
    }
   
    // fn lookup(&self, sym: &Symb) -> Option<NodeId> {
    //     match self.frame.get(sym) {
    //         Some(id) => Some(*id),
    //         None => {
    //             match &self.enclosing {
    //                 None => None,
    //                 Some(env) => env.lookup(sym),
    //             }
    //         }
    //     }
    // }
    
    // think about value of type Node here, should be expression?
    // whatever happens expressions are going to be heavy weight
    // because there will be a record of where each value is created,
    // thereby allowing superduper debugability.
    
    fn define_variable(&mut self, var: &Symb, obj: Obj) {        
        self.frame.insert(var.clone(), obj);
    }
    
    // fn set_variable_value(&mut self, var: &Symb, id: NodeId) -> Result<(), EvalErr> {
    //     match &mut self.frame.get(var) {
    //         Some(_) => {
    //             // found the variable.
    //             self.define_variable(var, id);
    //             Ok(())
    //         },
    //         None => {
    //             // didn't find the variable, look in the enclosing environment
    //             if self.is_global() {
    //                 Err(EvalErr::new("undefined variable: ", "asdf".to_owned(), 666))
    //             } else {
    //                 let env = self.enclosing.as_mut().unwrap();
    //                 env.set_variable_value(var, id)
    //             }
    //         }
    //     }
    // }
}




// ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn env_test_set_variable() {
        let x = Symb::new("x", "env.rs".to_owned(), 42);
        let y = Symb::new("y", "env.rs".to_owned(), 42);

        let mut global = Env::new();
        let mut inner = Env::new();


        
        let xval = make_obj(Ast::leaf(Token::
        
        global.define_variable(&x, 1);
        inner.define_variable(&y, 2);

        inner.enclosing = Some(box global);

        println!("{:?}", inner);
        
        match inner.lookup(&y) {
            Some(id) => assert_eq!(id, 2),
            None => panic!("lookup fails")
        }
        match inner.lookup(&x) {
            Some(id) => assert_eq!(id, 1),
            None => panic!("lookup fails")
        }

        if let Err(msg) = inner.set_variable_value(&x, 3) {
            panic!(msg);
        }
        if let Err(msg) = inner.set_variable_value(&y, 4) {
            panic!(msg);
        }
        
        match inner.lookup(&y) {
            Some(id) => assert_eq!(id, 4),
            None => panic!("lookup fails")
        }
        
        match inner.lookup(&x) {
            Some(id) => assert_eq!(id, 3),
            None => panic!("lookup fails")
        }
        
    }
    
    #[test]
    fn env_test_lookup() {
        let mut global = Env::new();
        let x = Symb::new("x", "env.rs".to_owned(), 42);
        let y = Symb::new("y", "env.rs".to_owned(), 42);
        
        global.define_variable(&x, 1);
        
        let mut inner = Env::new();
        inner.enclosing = Some(box global);
        inner.define_variable(&y, 2);

        match inner.lookup(&y) {
            Some(id) => assert_eq!(id, 2),
            None => panic!("lookup fails")
        }
        match inner.lookup(&x) {
            Some(id) => assert_eq!(id, 1),
            None => panic!("lookup fails")
        }
    }
     */
}

