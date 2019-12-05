use crate::env::*;
use crate::types::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Eval {
    global: Env,
}

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
