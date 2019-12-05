use crate::env::*;
use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Eval {
    global: Env,
}

fn eval(exp: Obj, env: Env) -> EvalResult<Obj> {
    if exp.is_self_evaluating() {
        Ok(exp)
    } else if exp.is_variable() {
        // TODO think about how to better manage symbols.
        match exp.to_symb() {
            Ok(sym) => env.lookup_variable_value(&sym),
            Err(msg) => Err(msg),
        }
    } else {
        Err(format!("no eval rule for: {:?}", exp))
    }
}

// TESTS -----------------------------------------------------------------------------

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

    #[test]
    fn eval_number_set() {
        let mut parser = get_parser("(set! foo 42)");
        let results = parser.list().unwrap();
        let objtree = results.to_obj();
        assert!(objtree.is_assignment());
    }

    #[test]
    fn self_evaluating_1() {
        let obj = Obj::new_int(128, None);
        let env = Env::new();
        let result = eval(obj.clone(), env).unwrap();
        assert_eq!(result, obj);
    }

    #[test]
    fn eval_lookup_variable_1() {
        let obj = Obj::new_int(128, None);
        let mut env = Env::new();
        let sym = Symb::new("x", "test-eval.rs".to_owned(), 42);
        env.define_variable(&sym, obj.clone());
        let obj2 = env.lookup_variable_value(&sym).unwrap();
        assert_eq!(obj, obj2);
    }

    #[test]
    fn eval_number_1() {
        let mut parser = get_parser("1");
        let results = parser.expr();
        match results {
            Err(msg) => panic!(msg),
            Ok(Ast::Leaf(Token { tok, .. })) => match tok {
                Tok::Int(n) => assert_eq!(n, 1),
                x => panic!("should have got a 1, got: {:?}", x),
            },
            _ => panic!("should have got a 1, but got: {:?}", results),
        }
    }
}
