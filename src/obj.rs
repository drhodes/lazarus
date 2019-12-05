use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Obj {
    fn new(val: ObjVal, loc: Option<Loc>) -> Obj {
        Obj {
            val: Rc::new(RefCell::new(val)),
            loc,
        }
    }

    pub fn new_int(num: i64, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Int(num), loc)
    }

    pub fn new_float(num: f64, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Float(num), loc)
    }

    pub fn new_list(xs: Vec<Obj>, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::List(xs), loc)
    }

    pub fn new_symb(name: String, loc: Option<Loc>) -> Obj {
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

    pub fn is_self_evaluating(&self) -> bool {
        return self.is_float() || self.is_int();
    }

    pub fn describe_type(&self) -> &str {
        if self.is_int() {
            return "int";
        } else if self.is_float() {
            return "float";
        } else if self.is_list() {
            return "list";
        } else {
            return "symbol";
        }
    }

    pub fn list_items(&self) -> EvalResult<Vec<Obj>> {
        if let ObjVal::List(nodes) = &*self.val.borrow() {
            Ok(nodes.clone())
        } else {
            Err(format!(
                "Can't call list_items method on: {:?}",
                self.describe_type()
            ))
        }
    }

    pub fn is_empty_list(&self) -> EvalResult<bool> {
        if let ObjVal::List(nodes) = &*self.val.borrow() {
            Ok(nodes.is_empty())
        } else {
            Err(format!(
                "Can't call empty_list method on: {:?}",
                self.describe_type()
            ))
        }
    }

    pub fn car(&self) -> EvalResult<Obj> {
        if !self.is_list() {
            Err(format!("Can't call car on {:?}", self.describe_type()))
        } else if self.is_empty_list()? {
            Err(format!("Can't call car on empty list"))
        } else {
            Ok(self.list_items()?[0].clone())
        }
    }

    pub fn string_matches(&self, s: &str) -> bool {
        if let ObjVal::Symbol(sym) = &*self.val.borrow() {
            s == sym
        } else {
            false
        }
    }

    pub fn is_tagged_list(&self, tag: &str) -> bool {
        match self.car() {
            Ok(obj) => obj.string_matches(tag),
            Err(..) => false,
        }
    }

    pub fn is_quoted(&self) -> bool {
        self.is_tagged_list("quote")
    }

    pub fn is_assignment(&self) -> bool {
        self.is_tagged_list("set!")
    }

    pub fn is_definition(&self) -> bool {
        self.is_tagged_list("define")
    }

    /// Definitions have one of two forms:
    /// | (define ⟨var⟩ ⟨value⟩)
    /// | (define ⟨var⟩ (lambda (⟨param₁⟩ … ⟨paramₙ⟩) ⟨body⟩))

    pub fn definition_variable(&self) -> EvalResult<Obj> {
        let items = self.list_items()?;
        if items[1].is_symbol() {
            Ok(items[1].clone())
        } else {
            Ok(items[2].clone())
        }
    }

    // TODO fn make_lambda

    // pub fn definition_value(&self) -> EvalResult<Obj> {
    //     let items = self.list_items()?;

    //     // (if (symbol? (cadr exp))
    //     if items[1].is_symbol() {
    //         //     (caddr exp)
    //         Ok(items[2].clone())
    //     } else {
    //         //     (make-lambda
    //         //      (cdadr exp)   ; formal parameters
    //         //      (cddr exp)))) ; body
    //         Ok(items[2].clone())
    //         //Obj::make_lambda()
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::types::*;

    fn get_parser(s: &str) -> Parser {
        let lexer = Lexer::new(s, "test.scm");
        Parser::new(lexer)
    }

    fn get_obj(s: &str) -> Obj {
        let mut parser = get_parser(s);
        let results = parser.list().unwrap();
        results.to_obj()
    }

    #[test]
    fn is_definition() {
        let objtree = get_obj("(define foo 42)");
        assert!(objtree.is_definition());
    }

    #[test]
    fn definition_variable() {
        let _ = (|| -> EvalResult<()> {
            let objtree = get_obj("(define foo 42)");
            assert!(objtree.definition_variable()?.string_matches("foo"));
            Ok(())
        })();
    }
}
