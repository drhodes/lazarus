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

    fn is_global(&self) -> bool {
        self.enclosing.is_none()
    }

    fn define_variable(&mut self, var: &Symb, obj: Obj) {
        self.frame.insert(var.clone(), obj);
    }
}

// ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
}
