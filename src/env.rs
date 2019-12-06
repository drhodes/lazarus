use crate::types::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ------------------------------------------------------------------
impl Env {
    pub fn new() -> Env {
        Env {
            frame: Frame::new(),
            enclosing: None,
        }
    }

    pub fn is_global(&self) -> bool {
        self.enclosing.is_none()
    }

    pub fn define_variable(&mut self, var: &Symb, obj: Obj) {
        self.frame.insert(var.clone(), obj);
    }

    // Returns the value that is bound to the symbol ⟨var⟩ in the
    // environment ⟨env⟩, or signals an error if the variable is
    // unbound.
    pub fn lookup_variable_value(&self, var: &Symb) -> EvalResult<Obj> {
        match self.frame.get(var) {
            Some(value) => Ok(value),
            None => {
                if self.is_global() {
                    Err(format!("undefine variable: {:?}", var))
                } else {
                    self.enclosing.as_ref().unwrap().lookup_variable_value(var)
                }
            }
        }
    }

    pub fn set_variable_value(&mut self, var: &Symb, obj: Obj) -> EvalResult<()> {
        match self.frame.get(var) {
            Some(value) => {
                self.define_variable(var, obj);
                Ok(())
            }
            None => {
                if self.is_global() {
                    Err(format!("Unbound varaible: SET! {:?}", var))
                } else {
                    self.enclosing
                        .as_mut()
                        .unwrap()
                        .set_variable_value(var, obj)
                }
            }
        }
    }
}

// ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_define_check() {
        let env = Env::new();
        let mut inner = Env::new();
        let sym = Symb::new("x", "env.rs".to_owned(), 42);
        let obj1 = Obj::new_int(12, None);
        let obj3 = Obj::new_int(345345, None);
        let obj2 = obj1.clone();

        inner.enclosing = Some(box env);
        inner.define_variable(&sym, obj1);

        let result: Obj = inner.lookup_variable_value(&sym).unwrap();
        assert_eq!(result, obj2);
    }

    #[test]
    fn env_define_outer_check() {
        let mut outer = Env::new();
        let mut mid = Env::new();
        let mut inner = Env::new();
        let sym = Symb::new("x", "env.rs".to_owned(), 42);
        let obj1 = Obj::new_int(12, None);
        let obj3 = Obj::new_int(345345, None);
        let obj2 = obj1.clone();

        outer.define_variable(&sym, obj1);
        mid.enclosing = Some(box outer);
        inner.enclosing = Some(box mid);

        let result: Obj = inner.lookup_variable_value(&sym).unwrap();
        assert_eq!(result, obj2);
    }
}
