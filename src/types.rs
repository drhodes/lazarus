use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;
use std::fmt;

impl Symb {
    pub fn new(name: &str, filename: String, pos: usize) -> Symb {
        Symb {
            name: name.to_owned(),
            filename,
            pos,
        }
    }

    pub fn new_unknown(name: &str) -> Symb {
        Symb::new(name, "<unknown file>".to_owned(), 0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub filename: String,
    pub start: usize,
    pub end: usize,
}

impl Loc {
    pub fn new(filename: String, start: usize, end: usize) -> Loc {
        Loc {
            filename,
            start,
            end,
        }
    }
}

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
    Exprs,
    List,
    Empty,
    EmptyList,
}

#[derive(Debug)]
pub enum Ast {
    Node { rule: Rule, nodes: Vec<Ast> },
    Leaf(Token),
}

#[derive(Debug, Clone)]
pub struct Symb {
    pub name: String,
    pub filename: String,
    pub pos: usize,
}

impl Hash for Symb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Symb {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Symb {}

#[derive(Clone, PartialEq)]
pub enum ObjVal {
    Symbol(String),
    Float(f64),
    Int(i64),
    List(Vec<Obj>),
    Bool(bool),
    Env(Env),
    PrimFunc(fn(Obj) -> EvalResult<Obj>),
}

impl fmt::Debug for ObjVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjVal::Symbol(s) => write!(f, "'{}", s),
            ObjVal::Float(n) => write!(f, "{}", n),
            ObjVal::Int(n) => write!(f, "{}", n),
            ObjVal::List(v) => write!(f, "{:?}", v),
            ObjVal::Bool(b) => write!(f, "{:?}", b),
            ObjVal::Env(_) => write!(f, "<env>"),
            ObjVal::PrimFunc(_) => write!(f, "<fn>"),
        }
    }
}

#[derive(Clone)]
pub struct Obj {
    pub val: Rc<RefCell<ObjVal>>,
    pub loc: Option<Loc>, // experimental
}

impl fmt::Debug for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.val.borrow())
    }
}



impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Obj {}

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub frame: Frame,
    /// if enclosing is None, then it is the global environment.
    pub enclosing: Option<Box<Env>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub symbol_table: HashMap<Symb, Obj>,
}

pub type EvalResult<T> = Result<T, String>;

pub fn unimplemented_eval<T>() -> EvalResult<T> {
    Err("unimplemented".to_string())
}
