use crate::types::*;

fn eval_assignment(exp: Obj, env: &mut Env) -> EvalResult<Obj> {
    let var = exp.assignment_variable()?;
    let val = exp.assignment_value()?;
    env.set_variable_value(&var, val.clone())?;
    Ok(val.clone())
}

fn eval_definition(exp: Obj, env: &mut Env) -> EvalResult<Obj> {
    let var = exp.definition_variable()?.to_symb()?;
    let val = eval(exp.definition_value()?, env)?;
    println!("defining_variable: {:?}", var);
    env.define_variable(&var, val.clone());
    Ok(Obj::new_symb("ok".to_owned(), exp.loc.clone()))
}

fn eval_if(exp: Obj, env: &mut Env) -> EvalResult<Obj> {
    if eval(exp.if_predicate()?, env)?.is_true() {
        eval(exp.if_consequent()?, env)
    } else {
        eval(exp.if_alternative()?, env)
    }
}

// drill 
fn make_procedure(parameters: Obj, body: Obj, env: &mut Env) -> Obj {
    Obj::list_from_vec(
        vec![
            Obj::new_symb("procedure".to_owned(), None),
            parameters,
            body,
            // achtung‽ Cloning a whole environment here.
            // need to overhaul env so frames are shared.
            Obj::new_env(env.clone(), None),
        ],
        None,
    )
}

fn eval_sequence(xs: Obj, env: &mut Env) -> EvalResult<Obj> {
    if xs.is_last_expr()? {
        eval(xs.car()?, env)
    } else {
        eval(xs.car()?, env)?;
        eval_sequence(xs.rest_expr()?, env)
    }
}

fn apply(procedure: Obj, arguments: Obj) -> EvalResult<Obj> {
    if procedure.is_primitive_procedure() {
        procedure.primitive_apply_to(arguments)
    } else if procedure.is_compound_procedure() {
        let mut env = procedure.environment()?;
        env.extend(procedure.parameters()?, arguments)?;
        eval_sequence(procedure.body()?, &mut env)
    } else {
        Err(format!("Unknown procedure type: APPLY: {:?}", procedure))
    }
}

fn list_of_values(exps: Obj, env: &mut Env) -> EvalResult<Obj> {
    if exps.has_no_operands()? {
        Ok(Obj::empty_list(exps.loc.clone()))
    } else {
        Ok( Obj::cons( eval(exps.first_operand()?, env)?,
                       list_of_values(exps.rest_operands()?, env)? ))
    }
}

// ________________________________________________________________________________
//                                            _
//                             _____   ____ _| |
//                            / _ \ \ / / _` | |
//                           |  __/\ V / (_| | |
//                            \___| \_/ \__,_|_|

pub fn eval(exp: Obj, env: &mut Env) -> EvalResult<Obj> {
    // self-evaluating?
    if exp.is_self_evaluating() {
        Ok(exp)
    }
    // variable?
    else if exp.is_variable() {
        // TODO think about how to better manage symbols.
        match exp.to_symb() {
            Ok(sym) => env.lookup_variable_value(&sym),
            Err(msg) => Err(msg),
        }
    }
    // quoted?
    else if exp.is_quoted() {
        exp.text_of_quotation()
    }
    // assignment?
    else if exp.is_assignment() {
        eval_assignment(exp, env)
    }
    // definition?
    else if exp.is_definition() {
        eval_definition(exp, env)
    }
    // if?
    else if exp.is_if() {
        eval_if(exp, env)
    }
    // lambda?
    else if exp.is_lambda() {
        Ok(make_procedure(
            exp.lambda_parameters()?,
            exp.lambda_body()?,
            env,
        ))
    }
    // begin?
    else if exp.is_begin() {
        eval_sequence(exp.begin_actions()?, env)
    }
    // application?
    else if exp.is_application() {
        println!("apply env: {:?}", env.frame.borrow().all_names()); 
        apply(
            eval(exp.operator()?, env)?,
            list_of_values(exp.operands()?, env)?,
        )
    }
    // uh oh
    else {
        Err(format!("Unknown expression types: {:?}", exp))
    }
}

// TESTS -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::cdr;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn get_parser(s: &str) -> Parser {
        let lexer = Lexer::new(s, "test.scm");
        Parser::new(lexer)
    }

    fn eval_str(s: &str) -> EvalResult<Obj> {
        let mut env = Env::the_global_environment();
        let mut parser = get_parser(s);
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        println!("obj: {:?}", obj);
        eval(obj, &mut env)
    }

    fn eval_str_env(s: &str) -> (EvalResult<Obj>, Env) {
        let mut env = Env::the_global_environment();
        let mut parser = get_parser(s);
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        println!("obj: {:?}", obj);
        (eval(obj, &mut env), env)
    }

    // ---------------------------------------------------------------------------------------------
    #[test]
    fn test_define_1() {
        let prog = "(begin (define foo (lambda (x) x)) (foo 4))";
        let (result, env) = eval_str_env(prog);
        assert!(env.frame.borrow().all_names().contains(&"foo".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Obj::new_int(4, None));
    }


    
    #[test]
    fn test_define_2() {
        // expect stack overflow
        // let prog = "(begin (define fact (lambda (x) (fact x))) (fact 3) 4)";
        // let (result, env) = eval_str_env(prog);
        // assert!(env.frame.borrow().all_names().contains(&"fact".to_string()));
        // println!("env:    {:?}", env.frame.borrow().all_names()); 
        // println!("result: {:?}", result);
        //assert!(result.is_ok());
        //assert_eq!(result.unwrap(), Obj::new_int(4, None));
    }
    
    #[test]
    fn test_define_3() {
        let prog = "(begin (define temp 42) (define foo (lambda () temp)) (foo))";
        let (result, env) = eval_str_env(prog);
        assert!(env.frame.borrow().all_names().contains(&"foo".to_string()));
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Obj::new_int(42, None));
    }


    #[test]
    fn test_define_4() {
        let prog = r#"
(begin 

(define f1 (lambda (x) x))
(define f2 (lambda (x) (f1 x)))
(define f3 (lambda (x) (f2 x)))
(f3 4)
)

"#;
        let (result, env) = eval_str_env(prog);
        //assert!(env.frame.all_names().contains(&"f1".to_string()));
        println!("env:    {:?}", env.frame.borrow().all_names()); 
        println!("result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Obj::new_int(4, None));
    }

    #[test]
    fn test_define_5() {
        let prog = r#"
(begin 

(define countdown (lambda (n)
 (if (eq? n 0) 1
  (countdown (dec n)))))

(countdown 3)
)

"#;
        let (result, env) = eval_str_env(prog);
        println!("env:    {:?}", env.frame.borrow().all_names()); 
        println!("result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Obj::new_int(1, None));
    }

    
    
    #[test]
    fn test_empty_list() {
        let prog = "(list)";
        let result = eval_str(prog).unwrap();
        assert_eq!( result, Obj::list_from_vec(vec!(), None));
    }

    
    #[test]
    fn test_eval_cdr() {
        let prog = "(cdr (list 1 2 3))";
        let result = eval_str(prog).unwrap();
        assert_eq!(
            result,
            Obj::list_from_vec(vec!(Obj::new_int(2, None), Obj::new_int(3, None)), None)
        );
    }

    #[test]
    fn test_eval_list_car() {
        let prog = "(car (list 1 2 3))";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_int(1, None),);
    }

    #[test]
    fn test_list_of_values() {
        let mut env = Env::the_global_environment();
        let prog = "(list 1 2)";
        let mut parser = get_parser(prog);
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        let result = list_of_values(obj.operands().unwrap(), &mut env).unwrap();
        let inner = Obj::list_from_vec(vec![Obj::new_int(1, None), Obj::new_int(2, None)], None);
        assert_eq!(result, inner);
    }

    #[test]
    fn test_list_list() {
        let prog = "(list (list 1 2))";
        let result = eval_str(prog).unwrap();
        let inner = Obj::list_from_vec(vec![Obj::new_int(1, None), Obj::new_int(2, None)], None);
        let outer = Obj::list_from_vec(vec![inner], None);
        assert_eq!(result, outer);
    }

    #[test]
    fn test_eval_seq_1() {
        let mut env = Env::the_global_environment();
        let seq = Obj::list_from_vec(vec![Obj::new_int(2, None), Obj::new_int(3, None)], None);
        let result = eval_sequence(seq, &mut env).unwrap();
        assert_eq!(result, Obj::new_int(3, None));
    }

    #[test]
    fn test_eval_list_1() {
        let prog = "(list 2 3)";
        let result = eval_str(prog).unwrap();
        assert_eq!(
            result,
            Obj::list_from_vec(vec!(Obj::new_int(2, None), Obj::new_int(3, None),), None)
        );
    }

    #[test]
    fn apply_3() {
        let prog = "(begin (define pair (lambda (x y) (list x y))) (pair 1 2))";
        let result = eval_str(prog).unwrap();
        assert_eq!(
            result,
            Obj::list_from_vec(vec!(Obj::new_int(1, None), Obj::new_int(2, None),), None)
        );
    }

    #[test]
    fn apply_2() {
        let prog = "(begin (define dup (lambda (x) (list x x))) (dup 1))";
        let result = eval_str(prog).unwrap();
        assert_eq!(
            result,
            Obj::list_from_vec(vec!(Obj::new_int(1, None), Obj::new_int(1, None),), None)
        );
    }

    #[test]
    fn apply_1() {
        let result = eval_str("(list 1 2 3)").unwrap();
        println!("result: {:?}", result);
        assert_eq!(
            result,
            Obj::list_from_vec(
                vec!(
                    Obj::new_int(1, None),
                    Obj::new_int(2, None),
                    Obj::new_int(3, None),
                ),
                None
            )
        );
    }

    #[test]
    fn eval_begin_3() {
        let prog = "(begin (define three 5) (set! three 3) 1 2 3 4 three)";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_int(3, None));
        assert_ne!(result, Obj::new_int(1, None));
    }

    #[test]
    fn eval_begin_2() {
        let prog = "(begin (define three 5) (set! three 3) three)";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_int(3, None));
        assert_ne!(result, Obj::new_int(1, None));
    }

    #[test]
    fn eval_begin_1() {
        let prog = "(begin 1 2 3)";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_int(3, None));
        assert_ne!(result, Obj::new_int(1, None));
    }

    #[test]
    fn eval_if_1() {
        let prog = "(if 1 2 3)";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_int(2, None));
        assert_ne!(result, Obj::new_int(3, None));
    }

    #[test]
    fn eval_if_2() {
        let prog = "(if 1 2.0 3)";
        let result = eval_str(prog).unwrap();
        assert_eq!(result, Obj::new_float(2.0, None));
        assert_ne!(result, Obj::new_int(3, None));
    }

    #[test]
    fn eval_definition_1() {
        let mut env = Env::new(0);
        let sym = Symb::new("foo", "test-eval.rs".to_owned(), 0);
        let mut parser = get_parser("(define foo 42)");
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        let _ = eval(obj, &mut env);
        println!("{:?}", env);
        let val = &env.lookup_variable_value(&sym).unwrap();
        assert_eq!(val, &Obj::new_int(42, None));
        assert_ne!(val, &Obj::new_int(43, None));
    }

    #[test]
    fn eval_assign_1() {
        let mut env = Env::new(0);
        let sym = Symb::new("foo", "test-eval.rs".to_owned(), 0);
        env.define_variable(&sym, Obj::new_int(123, None));

        let mut parser = get_parser("(set! foo 42)");
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        let _ = eval(obj, &mut env);
        println!("{:?}", env);

        let val = &env.lookup_variable_value(&sym).unwrap();
        assert_eq!(val, &Obj::new_int(42, None));
        assert_ne!(val, &Obj::new_int(43, None));
    }

    #[test]
    fn eval_assign_2() {
        let mut env1 = Env::new(0);
        let mut env2 = Env::new(1);
        let sym = Symb::new("foo", "test-eval.rs".to_owned(), 0);

        env1.define_variable(&sym, Obj::new_int(123, None));
        env2.enclosing = Some(box env1);

        let mut parser = get_parser("(set! foo 42)");
        let parse_results = parser.list().unwrap();
        let obj = parse_results.to_obj();
        let _ = eval(obj, &mut env2);

        let val = &env2.lookup_variable_value(&sym).unwrap();
        assert_eq!(val, &Obj::new_int(42, None));
        assert_ne!(val, &Obj::new_int(43, None));
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
        let mut env = Env::new(0);
        let result = eval(obj.clone(), &mut env).unwrap();
        assert_eq!(result, obj);
    }

    #[test]
    fn eval_lookup_variable_1() {
        let obj = Obj::new_int(128, None);
        let mut env = Env::new(0);
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
