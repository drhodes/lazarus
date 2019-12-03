use crate::types::*;
use crate::env::*;

// -----------------------------------------------------------------------------
// use indirection to make more flexible.

struct Eval {
    node_map: IdNodeMap,
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
}
   


impl Eval {
    fn new() -> Eval {
        Eval{ node_map: IdNodeMap::new(),
              global: Env::new() }
    }
    
    // fn lookup(&self, var: &Symb, env: Env) -> Result<&mut Ast, String> {
    //     match env.frame.get(var) {
    //         Some(id) => match self.node_map.lookup_mut(*id) {
    //             Some(node) => Ok(node),
    //             None => Err("Internal error, id not found".to_owned()),
    //         }
    //         None => Err(format!("undefined var: {}", var.name))
    //     } 
    // }

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
