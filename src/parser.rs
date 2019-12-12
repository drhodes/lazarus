use crate::lexer;
use crate::types::*;

pub type ParserResult = Result<Ast, String>;

// ------------------------------------------------------------------
pub struct Parser {
    filename: String,
    toks: Vec<Token>,
    idx: usize,
    ast: Ast,
}

fn err(msg: &str) -> ParserResult {
    Err(msg.to_owned())
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Parser {
        let mut toks = vec![];
        let filename = lexer.filename.clone();

        for span in lexer {
            if let Ok(token) = span {
                if token.tok == Tok::Space {
                    continue;
                }
                toks.push(token);
            } else {
                panic!("failed to lex a string");
            }
        }

        Parser {
            filename,
            toks,
            idx: 0,
            ast: Ast::empty(),
        }
    }

    fn next_token(&mut self) -> Option<&Token> {
        if self.idx >= self.toks.len() {
            None
        } else {
            let i = self.idx;
            self.idx += 1;
            self.toks.get(i)
        }
    }

    fn err(&mut self, cursor: usize, msg: &str) -> ParserResult {
        self.idx = cursor;
        err(msg)
    }

    fn err_plus(&mut self, cursor: usize, msg1: String, msg2: &str) -> ParserResult {
        self.idx = cursor;
        Err(msg1 + "\n " + &self.current_token_pos() + msg2)
    }

    fn current_token_pos(&self) -> String {
        if self.toks.len() == 0 {
            self.filename.clone() + ": end of file"
        } else {
            let pos = format!("char: {}", self.toks[0].start).to_owned();
            self.filename.clone() + ": " + &pos
        }
    }

    // RULES ------------------------------------------------------------------
    pub fn list(&mut self) -> ParserResult {
        let idx = self.idx;

        match (|| {
            self.lparen()?;
            let xs = self.exprs()?;
            self.rparen()?;
            Ok(xs)
        })() as ParserResult
        {
            Ok(mut xs) => {
                // xs is has rule type Exprs, which is zero-or-more
                // expressions, but this is a List production, but
                // that that makes the AST more cumbersome, so flatten
                // it.
                xs.replace_rule(Rule::List);
                return Ok(xs);
            }
            Err(msg) => self.err_plus(idx, msg, "list fails"),
        }
    }

    pub fn expr(&mut self) -> ParserResult {
        //println!("expr");
        let idx = self.idx;

        // this is boiler plate.
        if let Ok(n) = self.float() {
            return Ok(n);
        }
        if let Ok(n) = self.int() {
            return Ok(n);
        }
        if let Ok(n) = self.symbol() {
            return Ok(n);
        }

        let result = self.list();
        // TODO perhaps use a vector of error strings to trace the
        // errors instead of using string append.
        match result {
            Ok(n) => Ok(n),
            Err(msg) => self.err_plus(
                idx,
                msg + &self.current_token_pos(),
                "expr fails to parse expr",
            ),
        }
    }

    // this can't fail.
    fn exprs(&mut self) -> ParserResult {
        //println!("exprs");
        let mut nodes = vec![];
        loop {
            let idx = self.idx;
            if let Ok(n) = self.expr() {
                nodes.push(n);
            } else {
                self.idx = idx;
                return Ok(Ast::node(Rule::Exprs, nodes));
            }
        }
    }

    fn lparen(&mut self) -> ParserResult {
        //println!("lparen");
        let idx = self.idx;
        match self.next_token() {
            Some(tok) => {
                if tok.is_lparen() {
                    Ok(Ast::Leaf(tok.clone()))
                } else {
                    self.err(idx, "lparen got wrong token")
                }
            }
            None => self.err(idx, "done"),
        }
    }

    fn rparen(&mut self) -> ParserResult {
        //println!("rparen");
        let idx = self.idx;
        match self.next_token() {
            Some(token) => {
                if token.is_rparen() {
                    Ok(Ast::Leaf(token.clone()))
                } else {
                    self.err(idx, "rparen got wrong token")
                }
            }
            None => self.err(idx, "done"),
        }
    }

    fn int(&mut self) -> ParserResult {
        //println!("int");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_int() {
                Ok(Ast::Leaf(token.clone()))
            } else {
                self.err(idx, "todo int err msg")
            }
        } else {
            self.err(idx, "todo int err msg 2")
        }
    }

    fn float(&mut self) -> ParserResult {
        //println!("float");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_float() {
                Ok(Ast::Leaf(token.clone()))
            } else {
                self.err(idx, "todo float err msg")
            }
        } else {
            self.err(idx, "todo float err msg 2")
        }
    }

    fn symbol(&mut self) -> ParserResult {
        //println!("symbol");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_symbol() {
                Ok(Ast::Leaf(token.clone()))
            } else {
                self.err(idx, "todo symbol err msg")
            }
        } else {
            self.err(idx, "todo symbol err msg 2")
        }
    }
}

// ------------------------------------------------------------------
// TESTS.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    
    fn get_tokens(s: &str) -> Vec<Token> {
        let lexer = Lexer::new(s, "test.scm");
        let mut toks = vec![];
        for span in lexer {
            if let Ok(token) = span {
                if token.tok == Tok::Space {                    
                    continue;
                }
                println!("token: {:?}", token);
                toks.push(token);
            } else {
                panic!("failed to lex a string");
            }
        }
        println!("tokens: {:?}", toks);
        toks
    }

    fn get_parser(s: &str) -> Parser {
        let lexer = Lexer::new(s, "test.scm");
        println!("lexer: {:?}", get_tokens(s));
        Parser::new(lexer)
    }

    #[test]
    fn parse_to_obj_1() {
        let mut parser = get_parser("(1 2 (3))");
        let results = parser.list();

        match results {
            Err(msg) => panic!(msg),
            Ok(xs) => {
                let obj = xs.to_obj();
                assert!(obj.is_list());
            }
        }
    }

    #[test]
    // this test should fail because the list doesn't have a close paren.
    fn parse_nested_list_2() {
        let mut parser = get_parser("(1.0 2 asdf 3 4");
        let results = parser.list();

        match results {
            Err(_) => {
                //panic!("this was supposed to fail,");
            }
            Ok(_) => {
                panic!("this was supposed to fail");
            }
        }
    }

    #[test]
    fn parse_nested_list() {
        let mut parser = get_parser("(1 2.0 three 4 (5 6))");
        let results = parser.list();

        match results {
            Ok(node) => match &node {
                Ast::Node { rule, nodes:_ } => {
                    println!("{:?}", node);
                    node.pretty();
                    assert_eq!(rule, &Rule::List);
                }
                _ => panic!("This should not be a leaf!"),
            },
            Err(msg) => {
                panic!(msg);
            }
        }
    }

    #[test]
    fn parse_exprs() {
        let mut parser = get_parser("1 2 3 4");
        let results = parser.exprs();

        match results {
            Ok(xs) => match xs {
                Ast::Node { rule:_, nodes } => {
                    assert_eq!(nodes.len(), 4);
                }
                _ => panic!("This should not be a leaf!"),
            },
            Err(msg) => {
                panic!(msg);
            }
        }
    }

    #[test]
    fn parse_list_many() {
        let mut parser = get_parser("(1 2 3 4)");
        if let Err(msg) = parser.list() {
            panic!(msg);
        }
    }

    #[test]
    fn parse_list1() {
        let mut parser = get_parser("( 1 )");
        if let Err(msg) = parser.list() {
            panic!(msg);
        }
    }

    #[test]
    fn parse_expr_int() {
        let mut parser = get_parser("2");
        if let Err(msg) = parser.expr() {
            panic!(msg);
        }
    }

    #[test]
    fn parse_expr_symbol() {
        let mut parser = get_parser("asdf");
        if let Err(msg) = parser.expr() {
            panic!(msg);
        }
    }

    #[test]
    fn parse_unicode_list() {
        let mut parser = get_parser("( ε )");        
        if let Err(msg) = parser.list() {
            panic!(msg);
        }
    }
    
    #[test]
    fn parse_unicode_symbol1() {
        let mut parser = get_parser("asdfε");
        if let Err(msg) = parser.expr() {
            panic!(msg);
        }
    }

    #[test]
    fn parse_unicode_symbol2() {
        let mut parser = get_parser("εasdf");
        if let Err(msg) = parser.expr() {
            panic!(msg);
        }
    }

    
    #[test]
    fn parse_unicode_symbol() {
        let mut parser = get_parser("ε");
        if let Err(msg) = parser.expr() {
            panic!(msg);
        }
    }

    
    #[test]
    fn parse_lparen() {
        let mut parser = get_parser("(");
        let temp = parser.lparen();
        println!("{:?}", temp);
    }

    #[test]
    fn parse_int() {
        let mut parser = get_parser("5");
        let temp = parser.int();
        println!("{:?}", temp);
    }
}
