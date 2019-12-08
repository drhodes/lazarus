use crate::types::*;

// primitive procedures
fn car(xs: Obj) -> EvalResult<Obj> {
    xs.car()
}
fn cdr(xs: Obj) -> EvalResult<Obj> {
    xs.car()
}
fn list(xs: Obj) -> EvalResult<Obj> {
    Ok(xs)
}

// ------------------------------------------------------------------
impl Env {
    pub fn new() -> Env {
        Env {
            frame: Frame::new(),
            enclosing: None,
        }
    }
    
    pub fn the_global_environment() -> Env {
        let mut env = Env::new();
        let mut add_obj = |s: &str, obj| {
            env.define_variable(&Symb::new_unknown(s), obj);
        };

        add_obj("true", Obj::new_bool(true, None));
        add_obj("false", Obj::new_bool(false, None));
        
        env.add_primitive_func("car", car);
        env.add_primitive_func("cdr", car);
        env.add_primitive_func("list", list);
        env
    }
    
    pub fn add_primitive_func(&mut self, funcname: &str, func: fn(Obj) -> EvalResult<Obj>) {
        let proc = Obj::new_list(
            vec![Obj::new_symb("primitive".to_owned(), None),
                 Obj::new_primitive_func(func, None)],
            None,
        );
        self.define_variable(&Symb::new_unknown(funcname), proc);
        
    }
    
    pub fn extend(&mut self, params: Obj, arguments: Obj) -> EvalResult<()> {
        if params.list_length()? != arguments.list_length()? {
            Err("params and args need to have same length".to_string())
        } else {
            let ps = params.list_items()?;
            let args = arguments.list_items()?;

            for (p, arg) in ps.iter().zip(args.iter()) {
                self.define_variable(&p.to_symb()?, arg.clone());
            }
            Ok(())
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
            Some(..) => {
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
        let _obj3 = Obj::new_int(345345, None);
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
        let _obj3 = Obj::new_int(345345, None);
        let obj2 = obj1.clone();

        outer.define_variable(&sym, obj1);
        mid.enclosing = Some(box outer);
        inner.enclosing = Some(box mid);

        let result: Obj = inner.lookup_variable_value(&sym).unwrap();
        assert_eq!(result, obj2);
    }
}
