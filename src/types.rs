use std::collections::HashMap;

// struct ParserError {
//     reasons: Vec<String>,
//     filename: String,
//     pos: usize, // this will be line, column
// }

// impl ParserError {
//     new(msg: &str, filename: String, )
// }

pub struct EvalErr {
    msg: String,
    filename: String,
    pos: usize,
}

impl EvalErr {
    pub fn new(msg: &str, filename: String, pos: usize) -> EvalErr {
        EvalErr{msg:msg.to_owned(), filename, pos}
    }
}

pub struct IdNodeMap {
    table: HashMap<NodeId, Ast>,
}

impl IdNodeMap {
    pub fn new() -> IdNodeMap {
        IdNodeMap {
            table: HashMap::new(),
        }
    }
    pub fn lookup(&mut self, id: NodeId) -> Option<&Ast> {
        self.table.get(&id)
    }
    pub fn lookup_mut(&mut self, id: NodeId) -> Option<&mut Ast> {
        self.table.get_mut(&id)
    }
}

impl Symb {
    pub fn new(name: &str, filename: String, pos: usize) -> Symb {
        Symb{name: name.to_owned(), filename, pos }
    }
}

//pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok: Tok,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    Symbol(Symb),
    Float(f64),
    Int(i64),
    LParen,
    RParen,
    Dot,
    Space,
}

// all these token types are ridiculous.
#[derive(Debug, PartialEq, Eq)]
pub enum Rule {
    Expr,
    Exprs,
    List,
    Empty,
    EmptyList,
}

#[derive(Debug)]
pub enum Ast {
    Node {
        rule: Rule,
        nodes: Vec<Ast>,
    },
    Leaf(Token),    
}

pub type NodeId = u64;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symb {
    pub name: String,
    pub filename: String,
    pub pos: usize,
}

#[derive(Debug)]
pub struct Env {
    pub frame: HashMap<Symb, NodeId>,
    /// if enclosing is None, then it is the global environment.
    pub enclosing: Option<Box<Env>>,
} 
