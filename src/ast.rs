use crate::types::*;
//use std::cell::Cell;

impl Ast {
    // constructors
    pub fn leaf(tok: Token) -> Ast {
        Ast::Leaf(tok)
    }
    pub fn empty() -> Ast {
        Ast::Node {
            rule: Rule::Empty,
            nodes: vec![],
        }
    }
    pub fn node(rule: Rule, nodes: Vec<Ast>) -> Ast {
        Ast::Node { rule, nodes }
    }

    // ------------------------------------------------------------------
    pub fn replace_rule(&mut self, new_rule: Rule) {
        match self {
            Ast::Leaf(..) => panic!("this method may not be called on leaf"),
            Ast::Node { rule, .. } => *rule = new_rule,
        }
    }

    pub fn name(&self) -> String {
        match &self {
            Ast::Leaf(..) => "Token".to_owned(),
            Ast::Node { rule, .. } => format!("rule: {:?}", rule),
        }
    }

    pub fn is_self_evaluating(&self) -> bool {
        match &self {
            Ast::Leaf(leaf) => match leaf.tok {
                Tok::Float(..) => true,
                Tok::Int(..) => true,
                _ => false,
            },
            Ast::Node { rule, .. } => {
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

    pub fn to_obj(&self) -> Obj {
        match &self {
            Ast::Leaf(leaf) => leaf.to_obj(),
            Ast::Node { rule: _, nodes } => {
                let mut objs = vec![];
                for node in nodes {
                    objs.push(node.to_obj())
                }
                Obj::list_from_vec(objs, None)
            }
        }
    }

    pub fn is_symbol(&self) -> bool {
        match &self {
            Ast::Leaf(token) => match token.tok {
                Tok::Symbol(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn pretty(&self) {
        match &self {
            &Ast::Leaf(tok) => {
                tok.pretty();
            }
            &Ast::Node { rule, nodes } => {
                match rule {
                    Rule::List => {
                        print!("(");
                        let mut limit = nodes.len();
                        for node in nodes.iter() {
                            node.pretty();
                            if limit > 1 {
                                print!(" "); // done't add space on the last node.
                            }
                            limit -= 1;
                        }
                        print!(")");
                    }
                    Rule::Empty => {}
                    Rule::EmptyList => {
                        print!("()");
                    }
                    Rule::Exprs => panic!("Exprs rule should not make it into AST"),
                }
            }
        }
    }
}
