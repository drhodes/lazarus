use crate::lexer::*;
use crate::lexer;
use crate::types::*;

type ParserResult = Result<Node, String>;

fn err(msg: &str) -> ParserResult {
    Err(msg.to_owned())
}


// ------------------------------------------------------------------
struct Parser {
    filename: String,
    toks: Vec<Token>,
    idx: usize,
    ast: Node,
}

impl Parser {
    fn new(lexer: lexer::Lexer) -> Parser {
        let mut toks = vec!();
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
 
        Parser{filename, toks, idx:0, ast: Node::empty()}
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
            self.filename.clone() +  ": end of file"
        } else {
            let pos = format!("char: {}", self.toks[0].start).to_owned();
            self.filename.clone() +  ": " + &pos
        }
    }

    // RULES ------------------------------------------------------------------    
    fn list(&mut self) -> ParserResult {
        println!("list");
        let idx = self.idx;

        match (||{
            self.lparen()?;      
            let xs = self.exprs()?;
            self.rparen()?;        
            Ok(xs)
        })() as ParserResult {
            Ok(xs) => Ok(Node::new(Rule::List, vec!(xs))),
            Err(msg) => self.err_plus(idx, msg, "list fails"),
        }
    }
    
    fn expr(&mut self) -> ParserResult {
        println!("expr");
        let idx = self.idx;
        
        if let Ok(n) = self.float() {
            return Ok(Node::new(Rule::Expr, vec!(n)));
        }

        if let Ok(n) = self.int() {
            return Ok(Node::new(Rule::Expr, vec!(n)));
        } 

        if let Ok(n) = self.symbol() {
            return Ok(Node::new(Rule::Expr, vec!(n)));
        }

        let result = self.list();
        match result {
            Ok(n) => Ok(Node::new(Rule::Expr, vec!(n))),
            Err(msg) => self.err_plus(idx, msg + &self.current_token_pos(), "expr fails to parse expr"),
        }
    }

    // this can't fail.
    fn exprs(&mut self) -> ParserResult {
        println!("exprs");
        let mut nodes = vec!();
        loop {
            let idx = self.idx;
            if let Ok(n) = self.expr() {
                nodes.push(n); 
            } else {
                self.idx = idx;
                return Ok(Node::new(Rule::Exprs, nodes));
            }
        }
    }
    
    fn lparen(&mut self) -> ParserResult {
        println!("lparen");
        let idx = self.idx;
        match self.next_token() {
            Some(tok) => {
                if tok.is_lparen() {
                    Ok(Node::token(tok.clone()))
                } else {
                    self.err(idx, "lparen got wrong token")
                }
            }
            None => {
                self.err(idx, "done")
            }
        }
    }

    fn rparen(&mut self) -> ParserResult {
        println!("rparen");
        let idx = self.idx;
        match self.next_token() {
            Some(token) => {
                if token.is_rparen() {
                    Ok(Node::token(token.clone()))
                } else {
                    self.err(idx, "rparen got wrong token")
                }
            }
            None => {
                self.err(idx, "done")
            }
        }
    }
    
    fn int(&mut self) -> ParserResult {
        println!("int");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_int() {
                Ok(Node::token(token.clone()))
            } else {
                self.err(idx, "todo int err msg")
            }
        } else {
            self.err(idx, "todo int err msg 2")
        }
    }
    
    fn float(&mut self) -> ParserResult {
        println!("float");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_float() {
                Ok(Node::token(token.clone()))
            } else {
                self.err(idx, "todo float err msg")
            }
        } else {
            self.err(idx, "todo float err msg 2")
        }
    }
    
    fn symbol(&mut self) -> ParserResult {
        println!("symbol");
        let idx = self.idx;
        if let Some(token) = self.next_token() {
            if token.is_symbol() {
                Ok(Node::token(token.clone()))
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

    fn get_tokens(s: &str) -> Vec<Token> {
        let lexer = Lexer::new(s, "test.scm");
        let mut toks = vec!();
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
        toks
    }

    fn get_parser(s: &str) -> Parser {
        let lexer = Lexer::new(s, "test.scm");
        Parser::new(lexer)
    }

    #[test]
    // this test should fail because the list doesn't have a close paren.
    fn parse_nested_list_2() {
        let mut parser = get_parser("(1.0 2 asdf 3 4");
        let results = parser.list();

        match results {
            Err(msg) => { 
                print!("{:?}", msg);
            },
            Ok(xs) => {
                panic!("this was supposed to fail");
            },
        } 
    }
    
    #[test]
    fn parse_nested_list() {
        let mut parser = get_parser("(1 2.0 three 4 (5 6))");
        let results = parser.list();

        match results {
            Ok(xs) => {
                println!("{:?}", xs);
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
            Ok(xs) => {
                assert_eq!(xs.nodes.len(), 4);
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
