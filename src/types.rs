use std::io::stdin;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::collections::HashMap;

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


