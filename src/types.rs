use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

impl Symb {
    pub fn new(name: &str, filename: String, pos: usize) -> Symb {
        Symb{name: name.to_owned(), filename, pos }
    }
}

#[derive(Debug)]
pub struct Loc {
    pub filename: String,
    pub start: usize,
    pub end: usize,
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

#[derive(Debug, PartialEq, Eq)]
pub enum Rule {
    //Expr,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symb {
    pub name: String,
    pub filename: String,
    pub pos: usize,
}

#[derive(Debug)]
pub enum ObjVal {
    Symbol(String),
    Float(f64),
    Int(u64),
    List(Vec<Obj>),
}


#[derive(Debug)]
pub struct Obj {
    pub val: Rc<RefCell<ObjVal>>,
    pub loc: Option<Loc>, // experimental
}



#[derive(Debug)]
pub struct Env {
    pub frame: HashMap<Symb, Obj>,
    /// if enclosing is None, then it is the global environment.
    pub enclosing: Option<Box<Env>>,
} 
