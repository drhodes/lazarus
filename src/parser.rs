use crate::lexer::*;
use crate::lexer;

type Id = usize;

type ParserResult = Result<Node, String>;

fn err(msg: &str) -> ParserResult {
    Err(msg.to_owned())
}

#[derive(Debug)]
enum Rule {
    Token(Tok),
    Int,
    Expr,
    Exprs,
    List,
    Empty,
    EmptyList,
}

#[derive(Debug)]
struct Node {
    rule: Rule,
    nodes: Vec<Node>,
}

impl Node {
    fn token(tok: lexer::Tok) -> Node {
        Node{rule: Rule::Token(tok), nodes: vec!() }
    }
    fn empty() -> Node {
        Node{rule: Rule::Empty, nodes: vec!() }
    }
    fn new(rule: Rule, nodes: Vec<Node>) -> Node {
        Node{rule, nodes}
    }
}

// ------------------------------------------------------------------
struct Parser {
    toks: Vec<lexer::Tok>,
    idx: usize,
    ast: Node,
}

impl Parser {
    fn new(toks: Vec<lexer::Tok>) -> Parser {
        Parser{toks, idx:0, ast: Node::empty()}
    }

    fn next_token(&mut self) -> Option<&Tok> {
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
    
    fn list(&mut self) -> ParserResult {
        println!("list");
        println!("{:?}", self.toks);
        let idx = self.idx;

        // this nesting is awful.        
        if let Ok(_) = self.lparen() {
            if let Ok(x) = self.exprs() {
                if let Ok(_) = self.rparen() {
                    Ok(Node::new(Rule::List, vec!(x)))
                } else {
                    self.err(idx, "list:rparen")
                }
            } else {
                self.err(idx, "list:expr")
            }
        } else {
            self.err(idx, "list:lparen")
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

        if let Ok(n) = self.list() {
            return Ok(Node::new(Rule::Expr, vec!(n)));
        }

        self.err(idx, "expr fails to parse expr")
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
                if tok == &LParen {
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
            Some(tok) => {
                if tok == &RParen {
                    Ok(Node::token(tok.clone()))
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
        match self.next_token() {
            Some(tok@lexer::Int(_)) => {
                return Ok(Node::token(tok.clone()));
            },
            otherwise =>  {
                self.err(idx, "ok")                
            },
        }
    }
    
    fn float(&mut self) -> ParserResult {
        println!("float");
        let idx = self.idx;
        match self.next_token() {
            Some(tok@lexer::Float(_)) => {
                Ok(Node::token(tok.clone()))
            },
            otherwise => {
                self.err(idx, "ok")                
            },
        }
    }

    fn symbol(&mut self) -> ParserResult {
        println!("symbol");
        let idx = self.idx;
        match self.next_token() {
            Some(tok@lexer::Symbol(_)) => {
                Ok(Node::token(tok.clone()))
            },
            otherwise =>  {
                self.err(idx, "ok")                
            },
        }
    }
}

// ------------------------------------------------------------------
// TESTS.

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tokens(s: &str) -> Vec<Tok> {
        let lexer = Lexer::new(s);
        let mut toks = vec!();
        for span in lexer {
            if let Ok((_, tok, _)) = span {
                if tok == lexer::Space {
                    continue;
                }
                toks.push(tok);
            } else {
                panic!("failed to lex a string");
            }
        }
        toks
    }

    #[test]
    fn parse_nested_list() {
        let mut parser = Parser::new(get_tokens("(1 2 3 4 (5 6))"));
        let results = parser.exprs();

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
        let mut parser = Parser::new(get_tokens("1 2 3 4"));
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
        let mut parser = Parser::new(get_tokens("(1 2 3 4)"));
        if let Err(msg) = parser.list() {
            panic!(msg);
        } 
    }
    
    #[test]
    fn parse_list1() {
        let mut parser = Parser::new(get_tokens("( 1 )"));
        if let Err(msg) = parser.list() {
            panic!(msg);
        } 
    }
    
    #[test]
    fn parse_expr_int() {
        let mut parser = Parser::new(get_tokens("1 2 3"));
        if let Err(msg) = parser.expr() {
            panic!(msg);
        } 
    }

    #[test]
    fn parse_expr_symbol() {
        let mut parser = Parser::new(get_tokens("asdf"));
        if let Err(msg) = parser.expr() {
            panic!(msg);
        } 
    }

    #[test]
    fn parse_lparen() {
        let lexer = Lexer::new("( ( ( ( ) ) ) )");
        let mut toks = vec!();
        for span in lexer {
            if let Ok((_, tok, _)) = span {
                if tok == lexer::Space {
                    continue;
                }
                toks.push(tok);
            } else {
                panic!("failed to lex a string");
            }
        }
        
        let mut parser = Parser::new(toks);
        let temp = parser.lparen();
        println!("{:?}", temp);
    }
    
    #[test]
    fn parse_int() {
        let lexer = Lexer::new("1 2 3");
        let mut toks = vec!();
        for span in lexer {
            if let Ok((_, tok, _)) = span {
                if tok == lexer::Space {
                    continue;
                }
                toks.push(tok);
            } else {
                panic!("failed to lex a string");
            }
        }
        
        let mut parser = Parser::new(toks);
        let temp = parser.int();
        println!("{:?}", temp);
    }
}
