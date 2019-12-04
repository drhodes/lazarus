use crate::env::*;
use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Eval {
    global: Env,
}

impl Ast {
    pub fn is_variable(&self) -> bool {
        self.is_symbol()
    }

    fn is_list(&self) -> bool {
        match self {
            Ast::Node { rule, .. } => rule == &Rule::List,
            _ => false,
        }
    }

    fn list_items(&self) -> &Vec<Ast> {
        if let Ast::Node { rule, nodes } = self {
            &nodes
        } else {
            panic!("Can't get list items of a non list")
        }
    }

    fn is_pair(&self) -> bool {
        match self {
            Ast::Node { rule, nodes } => rule == &Rule::List && nodes.len() >= 2,
            _ => false,
        }
    }

    fn car(&self) -> Option<&Ast> {
        if self.is_list() {
            match self {
                Ast::Node { rule, nodes } => nodes.get(0),
                _ => panic!("impossible."),
            }
        } else {
            panic!(format!("can't take car of: {:?}", self.name()));
        }
    }

    fn first_symbol_string(&self) -> &String {
        if let Some(Ast::Leaf(Token { tok, start, end })) = self.car() {
            if let Tok::Symbol(sym) = tok {
                return &sym.name;
            } else {
                panic!("Not a symbol");
            }
        } else {
            panic!("Not a symbol");
        }
    }

    fn is_quoted(&self) -> bool {
        self.first_symbol_string() == &"quote"
    }

    fn is_tagged_list(&self, tag: &str) -> bool {
        self.first_symbol_string() == tag
    }

    fn is_assignment(&self) -> bool {
        self.is_tagged_list("set!")
    }

    fn is_definition(&self) -> bool {
        self.is_tagged_list("define")
    }

    // fn assignment_variable(&self) -> Symb {
    //     assert!(self.is_assignment());
    // }

    // fn self_evaluate(&self) -> Obj {       
    // }    
}

// fn eval(exp: Ast, mut env: Env) -> Result<Obj, String> {
//     if exp.is_self_evaluating()
//         Ok(exp)
//     } else
//         Err(format!("no eval rule for: {:?}", exp))
//     }
// }

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
    // this test should fail because the list doesn't have a close paren.
    fn eval_number_set() {
        let mut parser = get_parser("(set! foo 42)");
        let results = parser.list();

        match results {
            Err(msg) => panic!(msg),
            Ok(node) => {
                assert!(node.is_assignment());
            }
        }
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

// (define (eval exp env)
//   (cond ((self-evaluating? exp)
//          exp)
//         ((quoted? exp)
//          (text-of-quotation exp))
//         ((assignment? exp)
//          (eval-assignment exp env))
//         ((definition? exp)
//          (eval-definition exp env))
//         ((if? exp)
//          (eval-if exp env))
//         ((lambda? exp)
//          (make-procedure
//           (lambda-parameters exp)
//           (lambda-body exp)
//           env))
//         ((begin? exp)
//          (eval-sequence
//           (begin-actions exp)
//           env))
//         ((cond? exp)
//          (eval (cond->if exp) env))
//         ((application? exp)
//          (apply (eval (operator exp) env)
//                 (list-of-values
//                  (operands exp)
//                  env)))
//         (else
//          (error "Unknown expression
//                  type: EVAL" exp))))
