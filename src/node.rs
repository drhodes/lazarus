use crate::types::*;


impl Ast {
    pub fn token(tok: Token) -> Ast {
        Ast::Node{rule: Rule::Token(tok), nodes: vec!() }
    }
    pub fn empty() -> Ast {
        Ast::Node{rule: Rule::Empty, nodes: vec!() }
    }
    pub fn node(rule: Rule, nodes: Vec<Ast>) -> Ast {
        Ast::Node{rule, nodes}
    }
}
