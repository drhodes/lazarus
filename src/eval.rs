use crate::types::*;
use crate::env::*;

// -----------------------------------------------------------------------------
// use indirection to make more flexible.

struct Eval {
    node_map: IdNodeMap,
    global: Env,
}

impl Eval {
    fn new() -> Eval {
        Eval{ node_map: IdNodeMap::new(),
              global: Env::new() }
    }
    
    fn lookup(&self, var: &Symb, env: Env) -> Result<&mut Node, String> {
        match env.frame.get(var) {
            Some(id) => match self.node_map.lookup_mut(*id) {
                Some(node) => Ok(node),
                None => Err("Internal error, id not found".to_owned()),
            }
            None => Err(format!("undefined var: {}", var.name))
        } 
    }

    fn is_self_evaluating(&self, exp: &Node) -> bool {
        match &exp.rule {
            Rule::Int | Rule::EmptyList => true,
            _ => false,
        }
    }

    fn is_variable(&self, exp: &Node) -> bool {        
        match exp.rule {
            Rule::Token(_) => true,
            _ => false,
        }
    }

    fn is_list(&self, exp: &Node) -> bool {
        exp.rule == Rule::List
    }

    // fn is_quoted(&self, exp: &Node) {
    //     ( self.is_list(exp) &&
    //       exp.nodes.len() > 0 &&
    //       exp.nodes[0].has_symbol("quote"))
    // }
    
    fn eval(&mut self, exp: Node, mut env: Env) -> Result<Node, String> {
        if self.is_self_evaluating(&exp) {
            Ok(exp)
        } else if self.is_variable(&exp) {
            // ((variable? exp) 
            //  (lookup-variable-value exp env))
            //self.lookup(&exp, env)
        } else {
            Err(format!("no eval rule for: {:?}", exp))
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
