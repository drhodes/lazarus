use crate::types::*;

impl Ast {
    // constructors
    pub fn leaf(tok: Token) -> Ast {
        Ast::Leaf(tok)
    }
    pub fn empty() -> Ast {
        Ast::Node{rule: Rule::Empty, nodes: vec!() }
    }
    pub fn node(rule: Rule, nodes: Vec<Ast>) -> Ast {
        Ast::Node{rule, nodes}
    }

    // ------------------------------------------------------------------
    pub fn pretty(&self) {
        match &self {
            &Ast::Leaf(tok) => {
                tok.pretty();
            },
            &Ast::Node{rule, nodes} => {
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
                
                // for ast in nodes.iter() {
                //     ast.pretty();
                // }
            }
        }
    }
}
