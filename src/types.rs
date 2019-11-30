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

pub type NodeId = u64;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symb {
    pub name: String,
    pub filename: String,
    pub pos: usize,
}

impl Symb {
    pub fn new(name: &str, filename: String, pos: usize) -> Symb {
        Symb{name: name.to_owned(), filename, pos }
    }
}

pub struct IdNodeMap {
    table: HashMap<NodeId, Node>,
}

impl IdNodeMap {
    pub fn new() -> IdNodeMap {
        IdNodeMap {
            table: HashMap::new(),
        }
    }
}


//pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok: Tok,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug)]
pub enum Rule {
    Token(Token),
    Int,
    Expr,
    Exprs,
    List,
    Empty,
    EmptyList,
}

#[derive(Debug)]
pub struct Node {
    pub rule: Rule,
    pub nodes: Vec<Node>,
}

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
