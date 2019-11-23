#![allow(dead_code)]

use std::io::stdin;
use std::cell::RefCell;
use std::io::{stdout, Write};
use argparse::{ArgumentParser, StoreTrue, Store};
use std::collections::HashMap;
// use file_descriptors::terminal::Character;
// use unicode_reader::CodePoints;
// use std::num::ParseIntError;
// use regex::Regex;
// use crate::types::*;
// use crate::readers::read;


// ################ Lispy: Scheme Interpreter in Python
// ## (c) Peter Norvig, 2010-16; See http://norvig.com/lispy.html

// from __future__ import division
// import math
// import operator as op

// ################ Types

// Symbol = str          # A Lisp Symbol is implemented as a Python str
// List   = list         # A Lisp List is implemented as a Python list
// Number = (int, float) # A Lisp Number is implemented as a Python int or float

// ################ Parsing: parse, tokenize, and read_from_tokens


// def parse(program):
//     "Read a Scheme expression from a string."
//     return read_from_tokens(tokenize(program))

fn tokenize(src: String) -> Vec<String> {
    let s1 = src.replace("(", " ( ");
    let s2 = &s1.replace(")", " ) ");
    let s3: Vec<_> = s2.split_whitespace().map(|s| String::from(s)).collect();
    return s3
}

fn parse(src: String) -> TokenTree{
    let mut tokens = tokenize(src);
    read_from_tokens(&mut tokens)
}

struct EditLoc {
    fname: String,
    line: usize,
    col: usize,
}

#[derive(Debug)]
enum TokenTree {
    ScmNumber(i64),
    ScmFloat(f64),
    Symbol(String),
    Tree(Vec<TokenTree>),
}

impl TokenTree {
    fn abs(&self) -> Result<TokenTree, String> {        
        let msg1 = String::from("Can't take this abs value of a symbol");
        let msg2 = String::from("Can't take this abs value of a list");
        match self {
            TokenTree::ScmNumber(n) => Ok(TokenTree::ScmNumber(n.abs())),
            TokenTree::ScmFloat(n) => Ok(TokenTree::ScmFloat(n.abs())),
            TokenTree::Symbol(_) => Err(msg1),
            TokenTree::Tree(_) => Err(msg2),
        }
    }
}

type Id = u64;

const GLOBAL_POOL: RefCell<Option<Pool>> = RefCell::new(None);

struct Pool { items: HashMap<Id, TokenTree> }

impl Pool {
    fn init() {
        let items = HashMap::new();
        GLOBAL_POOL.replace(Some(Pool{items}));
    }
    
    fn insert(&mut self, id: Id, tt: TokenTree) {
        self.items.insert(id, tt);
    }
    
    fn put(id: Id, tt: TokenTree) -> () {
        // let mut pool = RefCell::new(None);
        // GLOBAL_POOL.swap(&mut pool);
        // pool.
        GLOBAL_POOL.
        //pool.as_ref().unwrap().insert(id, tt);
    }
}



// -------------------------------------------------------------------------------------------------
fn read_from_tokens(tokens: &mut Vec<String>) -> TokenTree {
    // Read an expression from a sequence of tokens.
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading");
    }
    
    let token = tokens.remove(0);
    if token == "(" {
        let mut node = vec!();
        while tokens[0] != ")" {
            node.push(read_from_tokens(tokens));            
        }
        tokens.remove(0);
        return TokenTree::Tree(node)
    } else if ")" == token {
        panic!("Unexpected )");
    } else {
        return atom(token)
    }
}

/// Numbers become numbers; every other token is a symbol.
fn atom(s: String) -> TokenTree {
    if let Ok(num) = s.parse() {
        return TokenTree::ScmNumber(num);
    }
    if let Ok(num) = s.parse() {
        return TokenTree::ScmFloat(num);
    } 
    return TokenTree::Symbol(s);
}

// ################ Environments
//
// def standard_env():
//     "An environment with some Scheme standard procedures."
//     env = Env()
//     env.update(vars(math)) # sin, cos, sqrt, pi, ...
//     env.update({
//         '+':op.add, '-':op.sub, '*':op.mul, '/':op.truediv, 
//         '>':op.gt, '<':op.lt, '>=':op.ge, '<=':op.le, '=':op.eq, 
//         'abs':     abs,
//         'append':  op.add,  
//         'apply':   apply,
//         'begin':   lambda *x: x[-1],
//         'car':     lambda x: x[0],
//         'cdr':     lambda x: x[1:], 
//         'cons':    lambda x,y: [x] + y,
//         'eq?':     op.is_, 
//         'equal?':  op.eq, 
//         'length':  len, 
//         'list':    lambda *x: list(x), 
//         'list?':   lambda x: isinstance(x,list), 
//         'map':     map,
//         'max':     max,
//         'min':     min,
//         'not':     op.not_,
//         'null?':   lambda x: x == [], 
//         'number?': lambda x: isinstance(x, Number),   
//         'procedure?': callable,
//         'round':   round,
//         'symbol?': lambda x: isinstance(x, Symbol),
//     })
//     return env



// class Env(dict):
//     "An environment: a dict of {'var':val} pairs, with an outer Env."
//     def __init__(self, parms=(), args=(), outer=None):
//         self.update(zip(parms, args))
//         self.outer = outer
//     def find(self, var):
//         "Find the innermost Env where var appears."
//         return self if (var in self) else self.outer.find(var)

// global_env = standard_env()

struct Env {    
    table: HashMap<String, Id>,
    outer: Option<Box<Env>>,
}


impl Env {
    fn new(mut parms: Vec<String>, mut args: Vec<Id>, outer: Option<Box<Env>>) -> Env {
        assert_eq!(parms.len() == args.len(), true);
        let mut table = HashMap::new();
        while parms.len() > 0 {
            table.insert(parms.remove(0), args.remove(0));
        }
        Env{table, outer}
    }
   
    fn find(&mut self, var: &String) -> Option<&mut Id> {
        if self.table.contains_key(var) {
            return self.table.get_mut(var);
        } else if self.outer.is_some() {
            return self.find(var);
        } else {
            return None;
        }
    }

    fn standard_env() -> Env {
        let mut env = Env::new(vec!(), vec!(), None);
        env.table.insert(String::from("abs"), 0);
        return env;
    }
    
    // def standard_env():
    //     "An environment with some Scheme standard procedures."
    //     env = Env()
    //     env.update(vars(math)) # sin, cos, sqrt, pi, ...
    //     env.update({
    //         '+':op.add, '-':op.sub, '*':op.mul, '/':op.truediv, 
    //         '>':op.gt, '<':op.lt, '>=':op.ge, '<=':op.le, '=':op.eq, 
    //         'abs':     abs,
    //         'append':  op.add,  
    //         'apply':   apply,
    //         'begin':   lambda *x: x[-1],
    //         'car':     lambda x: x[0],
    //         'cdr':     lambda x: x[1:], 
    //         'cons':    lambda x,y: [x] + y,
    //         'eq?':     op.is_, 
    //         'equal?':  op.eq, 
    //         'length':  len, 
    //         'list':    lambda *x: list(x), 
    //         'list?':   lambda x: isinstance(x,list), 
    //         'map':     map,
    //         'max':     max,
    //         'min':     min,
    //         'not':     op.not_,
    //         'null?':   lambda x: x == [], 
    //         'number?': lambda x: isinstance(x, Number),   
    //         'procedure?': callable,
    //         'round':   round,
    //         'symbol?': lambda x: isinstance(x, Symbol),
    //     })
    //     return env
    
}

// ################ Interaction: A REPL
// def repl(prompt='lis.py> '):
//     "A prompt-read-eval-print loop."
//     while True:
//         val = eval(parse(raw_input(prompt)))
//         if val is not None: 
//             print(lispstr(val))

fn repl() {
    let mut verbose = false;
    let mut name = "World".to_string();

    
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                        "Be verbose");
        ap.refer(&mut name)
            .add_option(&["--name"], Store,
                        "Name for the greeting");
        ap.parse_args_or_exit();
    }

    println!("Welcome to a scheme.");
    println!("Use ctrl-c to exit.");

    let mut input = String::new();
    loop {
        print!("λ ");
        stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Something went wrong with read_line");
        
        let tree = parse(input.clone());
        println!("{:?}", tree);
        
        println!("{}", input.trim());
        if input.trim().len() == 0 {
            continue;
        }
        input.clear();
    }
}

fn main() {
    let src = String::from("(define (square x) (define count 0.0) (define temp 1) (* x x))");
    let tree = parse(src);
    println!("{:?}", tree);
    repl();
}


// def lispstr(exp):
//     "Convert a Python object back into a Lisp-readable string."
//     if isinstance(exp, List):
//         return '(' + ' '.join(map(lispstr, exp)) + ')' 
//     else:
//         return str(exp)

// ################ Procedures

// class Procedure(object):
//     "A user-defined Scheme procedure."
//     def __init__(self, parms, body, env):
//         self.parms, self.body, self.env = parms, body, env
//     def __call__(self, *args): 
//         return eval(self.body, Env(self.parms, args, self.env))


// impl Proc {
//     fn call() {
//     }
// }


// ################ eval

// def eval(x, env=global_env):
//     "Evaluate an expression in an environment."
//     if isinstance(x, Symbol):      # variable reference
//         return env.find(x)[x]
//     elif not isinstance(x, List):  # constant literal
//         return x                
//     elif x[0] == 'quote':          # (quote exp)
//         (_, exp) = x
//         return exp
//     elif x[0] == 'if':             # (if test conseq alt)
//         (_, test, conseq, alt) = x
//         exp = (conseq if eval(test, env) else alt)
//         return eval(exp, env)
//     elif x[0] == 'define':         # (define var exp)
//         (_, var, exp) = x
//         env[var] = eval(exp, env)
//     elif x[0] == 'set!':           # (set! var exp)
//         (_, var, exp) = x
//         env.find(var)[var] = eval(exp, env)
//     elif x[0] == 'lambda':         # (lambda (var...) body)
//         (_, parms, body) = x
//         return Procedure(parms, body, env)
//     else:                          # (proc arg...)
//         proc = eval(x[0], env)
//         args = [eval(exp, env) for exp in x[1:]]
//         return proc(*args)

