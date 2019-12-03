use crate::types::*;
use std::cell::Cell;

const __GLOBAL_NONCE: Cell<u64> = Cell::new(0);

fn next_id() -> u64 {
    // increment nonce
    let nonce = __GLOBAL_NONCE.get();
    __GLOBAL_NONCE.set(nonce+1);
    return nonce + 1
}


impl Ast {
    // constructors
    pub fn leaf(tok: Token) -> Ast {
        Ast::Leaf(tok)
    }
    pub fn empty() -> Ast {
        Ast::Node{rule: Rule::Empty, nodes: vec!(), id:next_id() }
    }
    pub fn node(rule: Rule, nodes: Vec<Ast>) -> Ast {
        Ast::Node{rule, nodes, id:next_id()}
    }
    
    // ------------------------------------------------------------------
    pub fn is_self_evaluating(&self) -> bool {
        match &self {
            Ast::Leaf(leaf) => {
                match leaf.tok {
                    Tok::Float(..) => true,
                    Tok::Int(..) => true,
                    _ => false,
                }
            },
            Ast::Node{rule, nodes, id} => {
                match rule {
                    // what is the role of Empty here?
                    // maybe Ast should have another 
                    Rule::Empty => panic!("interpreter error: should not have tried to eval this"),
                    Rule::EmptyList => false, // why is this not true?
                    _ => false,
                }
            }
        }
    }        

    pub fn is_symbol(&self) -> bool {
        match &self {
            Ast::Leaf(token) => {
                match token.tok {
                    Tok::Symbol(..) => true,
                    _ => false,
                }
            },
            _=> false,
        }
    }

    
    pub fn pretty(&self) {
        match &self {
            &Ast::Leaf(tok) => {
                tok.pretty();
            },
            &Ast::Node{rule, nodes, id} => {
                match rule {
                    Rule::Expr => nodes[0].pretty(),
                    Rule::Exprs => {
                        let mut limit = nodes.len();
                        for node in nodes.iter() {
                            node.pretty();
                            if limit > 1 {
                                print!(" "); // done't add space on the last node.
                            }
                            limit -= 1;
                        }
                    },
                    Rule::List => {
                        print!("(");
                        nodes[0].pretty();
                        print!(")");
                    },
                    Rule::Empty => {},
                    Rule::EmptyList => {
                        print!("()");
                    }
                }
            }
        }
    }
}
