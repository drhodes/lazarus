use crate::types::*;

// primitive procedures
// these primitive procedures accept a list of arguments.

fn car(xs: Obj) -> EvalResult<Obj> {
    if xs.list_length()? > 1 {
        Err(format!("car only takes one argument, got: {:?}", xs))
    } else {
        xs.car()?.car()
    }
}
pub fn cdr(xs: Obj) -> EvalResult<Obj> {
    if xs.list_length()? > 1 {
        Err(format!("cdr only takes one argument, got: {:?}", xs))
    } else {
        xs.car()?.cdr()
    }
}

fn list(xs: Obj) -> EvalResult<Obj> {
    Ok(xs)
}

fn is_null(xs: Obj) -> EvalResult<Obj> {
    Ok(Obj::new_bool(xs.car()?.is_null()?, xs.loc.clone()))
}

// fn foldr(f: fn(Obj) -> EvalResult<Obj>, unit: Obj, xs: Obj) {
//     if !xs.is_list()? {
//     }
// }

fn mul(xs: Obj) -> EvalResult<Obj> {
    match xs.list_length()? {
        0 => Ok(Obj::new_int(1, xs.loc.clone())),
        1 => xs.car(),
        _ => {
            let a = xs.car()?.int_val()?;
            let b = xs.cadr()?.int_val()?;
            let c = Obj::new_int(a * b, xs.loc.clone());
            let ys = Obj::cons(c, xs.cddr()?);
            mul(ys)
        }
    }
}

fn add(xs: Obj) -> EvalResult<Obj> {
    match xs.list_length()? {
        0 => Ok(Obj::new_int(0, xs.loc.clone())),
        1 => xs.car(),
        _ => {
            let a = xs.car()?.int_val()?;
            let b = xs.cadr()?.int_val()?;
            let c = Obj::new_int(a + b, xs.loc.clone());
            let ys = Obj::cons(c, xs.cddr()?);
            add(ys)
        }
    }
}

fn sub(xs: Obj) -> EvalResult<Obj> {
    match xs.list_length()? {
        0 => Ok(Obj::new_int(0, xs.loc.clone())),
        1 => xs.car(),
        _ => {
            let a = xs.car()?.int_val()?;
            let b = xs.cadr()?.int_val()?;
            let c = Obj::new_int(a - b, xs.loc.clone());
            let ys = Obj::cons(c, xs.cddr()?);
            add(ys)
        }
    }
}


fn dec(xs: Obj) -> EvalResult<Obj> {
    let x = xs.car()?.int_val()?;
    Ok(Obj::new_int(x - 1, xs.loc.clone()))
}

fn cons(xs: Obj) -> EvalResult<Obj> {
    if xs.list_length()? != 2 {
        Err(format!("cons must take 2 args, got: {:?}", xs))
    } else {
        Ok(Obj::cons(xs.car()?, xs.cadr()?))
    }
}

fn eq(xs: Obj) -> EvalResult<Obj> {
    Ok(Obj::new_bool(xs.car()? == xs.cadr()?, None))
}

fn lt(xs: Obj) -> EvalResult<Obj> {
    // TODO make this work for more than two args.
    let lhs = xs.car()?.as_float()?;
    let rhs = xs.cadr()?.as_float()?;
    Ok(Obj::new_bool(lhs < rhs, None))
}

fn gt(xs: Obj) -> EvalResult<Obj> {
    // TODO make this work for more than two args.
    let lhs = xs.car()?.as_float()?;
    let rhs = xs.cadr()?.as_float()?;
    Ok(Obj::new_bool(lhs > rhs, None))
}

// ------------------------------------------------------------------
impl Env {
    pub fn new(id: usize) -> Env {
        Env {
            id: id,
            frame: mutcell(Frame::new()),
            enclosing: None,
        }
    }

    pub fn the_global_environment() -> Env {
        let mut env = Env::new(0);
        let mut add_obj = |s: &str, obj| {
            env.define_variable(&Symb::new_unknown(s), obj);
        };

        add_obj("true", Obj::new_bool(true, None));
        add_obj("false", Obj::new_bool(false, None));
        add_obj("#t", Obj::new_bool(true, None));
        add_obj("#f", Obj::new_bool(false, None));

        env.add_primitive_func("car", car);
        env.add_primitive_func("cdr", cdr);
        env.add_primitive_func("list", list);
        env.add_primitive_func("null?", is_null);
        env.add_primitive_func("mul", mul);
        env.add_primitive_func("*", mul);
        env.add_primitive_func("+", add);
        env.add_primitive_func("-", sub);
        env.add_primitive_func(">", gt);
        env.add_primitive_func("<", lt);
        env.add_primitive_func("cons", cons);
        env.add_primitive_func("eq?", eq);
        env.add_primitive_func("dec", dec);
        env
    }

    pub fn add_primitive_func(&mut self, funcname: &str, func: fn(Obj) -> EvalResult<Obj>) {
        let proc = Obj::list_from_vec(
            vec![
                Obj::new_symb("primitive".to_owned(), None),
                Obj::new_primitive_func(func, None),
            ],
            None,
        );
        self.define_variable(&Symb::new_unknown(funcname), proc);
    }

    pub fn is_global(&self) -> bool {
        self.enclosing.is_none()
    }

    pub fn define_variable(&mut self, var: &Symb, obj: Obj) {
        self.frame.borrow_mut().insert(var.clone(), obj);
    }

    // Returns the value that is bound to the symbol ⟨var⟩ in the
    // environment ⟨env⟩, or signals an error if the variable is
    // unbound.
    pub fn lookup_variable_value(&self, var: &Symb) -> EvalResult<Obj> {
        match self.frame.borrow().get(var) {
            Some(value) => Ok(value),
            None => {
                if self.is_global() {
                    Err(format!("undefined variable: {}", var.name))
                } else {
                    self.enclosing.as_ref().unwrap().lookup_variable_value(var)
                }
            }
        }
    }

    pub fn set_variable_value(&mut self, var: &Symb, obj: Obj) -> EvalResult<()> {
        let val = self.frame.borrow().get(var);
        match val {
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
        let env = Env::new(0);
        let mut inner = Env::new(1);
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
        let mut outer = Env::new(0);
        let mut mid = Env::new(1);
        let mut inner = Env::new(2);
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
