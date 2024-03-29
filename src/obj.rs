use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Obj {
    pub fn new(val: ObjVal, loc: Option<Loc>) -> Obj {
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

    pub fn nil(loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Nil, loc)
    }

    pub fn list_from_vec(xs: Vec<Obj>, loc: Option<Loc>) -> Obj {
        let mut list = Obj::empty_list(loc.clone());
        for x in xs.iter().rev() {
            list = Obj::new(ObjVal::Cons(x.clone(), list), loc.clone());
        }
        list
    }

    pub fn empty_list(loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Nil, loc)
    }

    pub fn cons(x: Obj, y: Obj) -> Obj {
        Obj::new(ObjVal::Cons(x.clone(), y.clone()), x.loc.clone())
    }

    pub fn new_symb(name: String, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Symbol(name), loc)
    }

    pub fn new_bool(b: bool, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Bool(b), loc)
    }

    pub fn from_env(env: Env, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::Env(env), loc)
    }

    pub fn new_primitive_func(f: fn(Obj) -> EvalResult<Obj>, loc: Option<Loc>) -> Obj {
        Obj::new(ObjVal::PrimFunc(f), loc)
    }

    // the Symb type exists and Obj::Symbol exists.
    // Symb is convenient.
    // Obj::Symbol can be stored on the heap.
    // Does Symb need to be ...?

    // for now, just get this working.
    // it will become evident what to do as the system grows.
    pub fn to_symb(&self) -> EvalResult<Symb> {
        if let ObjVal::Symbol(sym) = &*self.val.borrow() {
            Ok(Symb::new(&sym.clone(), "".to_owned(), 0))
        } else {
            //self.pretty();
            Err(format!("Not a symbol!: {:?}", self))
        }
    }

    pub fn int_val(&self) -> EvalResult<i64> {
        if let ObjVal::Int(n) = *self.val.borrow() {
            Ok(n)
        } else {
            Err(format!("looking for an integer value, found: {:?}", self))
        }
    }

    pub fn is_list(&self) -> bool {
        match &*self.val.borrow() {
            ObjVal::Nil => true,
            ObjVal::Cons(_, y) => y.is_list(),
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        if let ObjVal::Float(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }

    pub fn as_float(&self) -> EvalResult<f64> {
        match *self.val.borrow() {
            ObjVal::Float(n) => Ok(n),
            ObjVal::Int(n) => Ok(n as f64),
            _ => Err("may only covert float or int to float".to_string()),
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

    pub fn is_bool(&self) -> bool {
        if let ObjVal::Bool(..) = *self.val.borrow() {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_true(&self) -> bool {
        if let ObjVal::Bool(b) = *self.val.borrow() {
            b
        } else {
            true
        }
    }

    pub fn is_func(&self) -> bool {
        if let ObjVal::PrimFunc(..) = *self.val.borrow() {
            true
        } else {
            false
        }
    }

    pub fn is_cons(&self) -> bool {
        if let ObjVal::Cons(..) = *self.val.borrow() {
            true
        } else {
            false
        }
    }

    pub fn is_variable(&self) -> bool {
        self.is_symbol()
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
        } else if self.is_bool() {
            return "bool";
        } else {
            return "symbol";
        }
    }

    pub fn is_empty_list(&self) -> EvalResult<bool> {
        Ok(ObjVal::Nil == *self.val.borrow())
    }

    pub fn list_length(&self) -> EvalResult<usize> {
        match &*self.val.borrow() {
            ObjVal::Nil => Ok(0),
            ObjVal::Cons(_, y) => Ok(y.list_length()? + 1),
            _ => Err("list_length was not passed a list".to_owned()),
        }
    }

    pub fn car(&self) -> EvalResult<Obj> {
        match &*self.val.borrow() {
            ObjVal::Cons(x, _) => Ok(x.clone()),
            _ => Err("car was not passed a cons cell".to_owned()),
        }
    }

    pub fn cdr(&self) -> EvalResult<Obj> {
        match &*self.val.borrow() {
            ObjVal::Cons(_, y) => Ok(y.clone()),
            _ => Err("object passed to cdr is not the correct type.".to_owned()),
        }
    }

    pub fn cadr(&self) -> EvalResult<Obj> {
        self.cdr()?.car()
    }

    pub fn caddr(&self) -> EvalResult<Obj> {
        self.cdr()?.cdr()?.car()
    }

    pub fn cddr(&self) -> EvalResult<Obj> {
        self.cdr()?.cdr()
    }

    pub fn cdadr(&self) -> EvalResult<Obj> {
        self.cdr()?.car()?.cdr()
    }

    pub fn cdddr(&self) -> EvalResult<Obj> {
        self.cddr()?.cdr()
    }

    pub fn cadddr(&self) -> EvalResult<Obj> {
        self.cdddr()?.car()
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

    pub fn text_of_quotation(&self) -> EvalResult<Obj> {
        self.cadr()
    }

    pub fn is_assignment(&self) -> bool {
        self.is_tagged_list("set!")
    }

    pub fn is_definition(&self) -> bool {
        self.is_tagged_list("define")
    }

    pub fn assignment_variable(&self) -> EvalResult<Symb> {
        self.cadr()?.to_symb()
    }

    pub fn assignment_value(&self) -> EvalResult<Obj> {
        self.caddr()
    }

    pub fn is_if(&self) -> bool {
        self.is_tagged_list("if")
    }
    pub fn if_predicate(&self) -> EvalResult<Obj> {
        self.cadr()
    }

    pub fn if_consequent(&self) -> EvalResult<Obj> {
        self.caddr()
    }

    pub fn if_alternative(&self) -> EvalResult<Obj> {
        if !self.cdddr()?.is_null()? {
            self.cadddr()
        } else {
            Ok(Obj::new_bool(false, None))
        }
    }

    pub fn is_null(&self) -> EvalResult<bool> {
        self.is_empty_list()
    }

    /// Definitions have one of two forms:
    /// | (define ⟨var⟩ ⟨value⟩)
    /// | (define ⟨var⟩ (lambda (⟨param₁⟩ … ⟨paramₙ⟩) ⟨body⟩))

    pub fn definition_variable(&self) -> EvalResult<Obj> {
        if self.cadr()?.is_symbol() {
            self.cadr()
        } else {
            self.caddr()
        }
    }

    fn make_lambda(params: Obj, body: Obj) -> Obj {
        let lambda = Obj::new_symb("lambda".to_string(), None);
        Obj::cons(lambda, Obj::cons(params, body))
    }

    pub fn definition_value(&self) -> EvalResult<Obj> {
        if self.cadr()?.is_symbol() {
            self.caddr()
        } else {
            Ok(Obj::make_lambda(self.cdadr()?, self.cddr()?))
        }
    }

    pub fn is_lambda(&self) -> bool {
        self.is_tagged_list("lambda")
    }

    pub fn lambda_parameters(&self) -> EvalResult<Obj> {
        self.cadr()
    }

    pub fn lambda_body(&self) -> EvalResult<Obj> {
        self.cddr()
    }

    pub fn is_begin(&self) -> bool {
        self.is_tagged_list("begin")
    }

    pub fn begin_actions(&self) -> EvalResult<Obj> {
        self.cdr()
    }

    pub fn is_last_expr(&self) -> EvalResult<bool> {
        self.cdr()?.is_null()
    }

    pub fn first_expr(&self) -> EvalResult<Obj> {
        self.car()
    }
    pub fn rest_expr(&self) -> EvalResult<Obj> {
        self.cdr()
    }

    // cond ------------------------------------------------------------------
    // skip cond for now.
    // pub fn is_cond(&self) -> bool {
    //     self.is_tagged_list("cond")
    // }

    // pub fn cond_clauses(&self) -> EvalResult<Obj> {
    //     self.cdr()
    // }

    // pub fn cond_predicate(&self) -> EvalResult<Obj> {
    //     self.car()
    // }

    // pub fn is_cond_else_clause(&self) -> bool {
    //     self.cond_predicate().string_matches("else")
    // }

    // pub fn cond_actions(&self) -> EvalResult<Obj> {
    //     unimplemented()
    // }

    // pub fn cond_to_if(&self) -> EvalResult<Obj> {
    //     unimplemented()
    // }

    // pub fn expand_clauses(&self) -> EvalResult<Obj> {
    //     unimplemented()
    // }

    // apply helpers -------------------------------------------------------------------------------
    pub fn is_application(&self) -> bool {
        self.is_cons()
    }

    pub fn operator(&self) -> EvalResult<Obj> {
        self.car()
    }

    pub fn operands(&self) -> EvalResult<Obj> {
        self.cdr()
    }
    pub fn has_no_operands(&self) -> EvalResult<bool> {
        self.is_null()
    }
    pub fn first_operand(&self) -> EvalResult<Obj> {
        self.car()
    }
    pub fn rest_operands(&self) -> EvalResult<Obj> {
        self.cdr()
    }

    pub fn is_primitive_procedure(&self) -> bool {
        self.is_tagged_list("primitive")
    }

    pub fn is_compound_procedure(&self) -> bool {
        self.is_tagged_list("procedure")
    }

    pub fn is_primitive_implementation(&self) -> EvalResult<Obj> {
        self.cadr()
    }

    pub fn primitive_apply_to(&self, args: Obj) -> EvalResult<Obj> {
        // println!("self: {:?} ", self);
        // println!("args: {:?} ", args);
        // println!("prim: {:?} ", self.cadr()?);

        if let ObjVal::PrimFunc(f) = &*self.cadr()?.val.borrow() {
            f(args)
        } else {
            Err("Tried to apply something other \
                 than primitive procedure"
                .to_string())
        }
    }

    pub fn body(&self) -> EvalResult<Obj> {
        self.caddr()
    }

    pub fn parameters(&self) -> EvalResult<Obj> {
        self.cadr()
    }

    pub fn environment(&self) -> EvalResult<Env> {
        if !self.is_compound_procedure() {
            return Err(format!(
                "Can only call Obj::environment \
                 on compound procedure, got: {}",
                self.describe_type()
            ));
        }
        if let ObjVal::Env(env) = &*self.cadddr()?.val.borrow() {
            // cloning this.. possible bug, need to see if the values
            // are going to mutate inside this things as expected by
            // apply and eval.
            Ok(env.clone())
        } else {
            panic!(
                "interpeter bug, was expecting env, got: {:?}",
                self.describe_type()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    //use crate::types::*;

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
