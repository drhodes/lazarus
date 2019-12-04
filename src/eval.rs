use crate::types::*;
use crate::env::*;
use std::rc::Rc;
use std::cell::RefCell;
// -----------------------------------------------------------------------------
// use indirection to make more flexible.

struct Eval {
    global: Env,
}

impl Ast {
    pub fn is_variable(&self) -> bool {
        self.is_symbol()
    }

    fn is_list(&self) -> bool {
        match self {
            Ast::Node{rule, ..} => rule == &Rule::List,
            _ => false,
        } 
    }
    
    fn car(&self) -> Option<&Ast> {
        if self.is_list() {
            match self {
                Ast::Node{rule, nodes} => {
                    nodes.get(0)
                },
                _ => panic!("impossible.")
            }             
        } else {
            panic!(format!("can't take car of: {:?}", self.name()));
        }
    }
    
    // fn is_pair
    
    // fn is_tagged_list(&self, tag: &str) -> bool {
    //     let Some(node) = self.car() {
    //         match node {
    //             Ast::Node{rule, xs} => {
    //                 if rule != Rule::Expr {
    //                     panic!("")
    //                 }
    //             }
    // }
    
    // fn is_assignment(&self) -> bool {
    //     self.is_tagged_list(&self, "set!")
    // }
}

impl Eval {
    fn new() -> Eval {
        Eval{
            global: Env::new(), //Rc::new(RefCell::new(Env::new())),
        }
    }
    
    fn is_self_evaluating(&self, exp: &Ast) -> bool {
        exp.is_self_evaluating()
    }
    
    
    // fn is_quoted(&self, exp: &Ast) {
    //     ( self.is_list(exp) &&
    //       exp.nodes.len() > 0 &&
    //       exp.nodes[0].has_symbol("quote"))
    // }
    
    // fn eval(&mut self, exp: Ast, mut env: Env) -> Result<Ast, String> {
    //     if self.is_self_evaluating(&exp) {
    //         Ok(exp)
    //     } else if self.is_variable(&exp) {
    //         // ((variable? exp) 
    //         //  (lookup-variable-value exp env))
    //         //self.lookup(&exp, env)
    //     } else {
    //         Err(format!("no eval rule for: {:?}", exp))
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use crate::parser::{Parser};
    use crate::lexer::{Lexer};
    
    fn get_parser(s: &str) -> Parser {
        let lexer = Lexer::new(s, "test.scm");
        Parser::new(lexer)
    }

    
    #[test]
    // this test should fail because the list doesn't have a close paren.
    fn eval_number_1() {
        let mut parser = get_parser("1");
        let results = parser.expr();
        
        match results {
            Err(msg) => {
                panic!(msg)
            },
            Ok(Ast::Leaf(Token{tok, ..})) => {
                match tok {
                    Tok::Int(n) => assert_eq!(n, 1),
                    x => panic!("should have got a 1, got: {:?}", x)
                }
            },
            _ => {
                panic!("should have got a 1, but got: {:?}", results)
            },
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
