use crate::types::*;


impl Node {
    pub fn token(tok: Token) -> Node {
        Node{rule: Rule::Token(tok), nodes: vec!() }
    }
    pub fn empty() -> Node {
        Node{rule: Rule::Empty, nodes: vec!() }
    }
    pub fn new(rule: Rule, nodes: Vec<Node>) -> Node {
        Node{rule, nodes}
    }
}
