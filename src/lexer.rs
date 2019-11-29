use regex::{Regex};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    Symbol(String),
    Float(f64),
    Int(i64),
    LParen,
    RParen,
    Dot,
    Space,
}
pub use Tok::*;

#[derive(Debug)]
pub enum LexError {}

pub struct Lexer {
    idx: usize,
    prog: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { idx: 0, prog: String::from(input) }
    }
}

impl Iterator for Lexer {
    type Item = Spanned<Tok, usize, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.prog.len() {
            return None
        }
        let symbol_pat = Regex::new(r#"[a-zA-Z]+"#).unwrap();
        let float_pat = Regex::new(r"[-+]?[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?").unwrap();
        let int_pat = Regex::new(r"[-+]?[0-9]+").unwrap();
        let space_pat = Regex::new(r"[\s\n\t]+").unwrap();
        // symbol_pat.
        
        // can we parse a symbol?
        match symbol_pat.find_at(&self.prog, self.idx) {
            Some(m) => {
                if m.start() == self.idx { 
                    let tok = Tok::Symbol(String::from(m.as_str()));
                    self.idx = m.end();
                    return Some(Ok((m.start(), tok, m.end())));
                }
            },
            None => {},
        }

        // order matters! must try to parse float before int.
        match float_pat.find_at(&self.prog, self.idx) {
            Some(m) => {                    
                if m.start() == self.idx {
                    let tok = Tok::Float(m.as_str().parse::<f64>().unwrap());
                    self.idx = m.end();
                    return Some(Ok((m.start(), tok, m.end())));
                }
            },
            None => {},
        }

        // try an int
        match int_pat.find_at(&self.prog, self.idx) {
            Some(m) => {                    
                if m.start() == self.idx {
                    let tok = Tok::Int(m.as_str().parse::<i64>().unwrap());
                    self.idx = m.end();
                    return Some(Ok((m.start(), tok, m.end())));
                }
            },
            None => {},
        }

        match space_pat.find_at(&self.prog, self.idx) {
            Some(m) => {                    
                if m.start() == self.idx {
                    self.idx = m.end();
                    return Some(Ok((m.start(), Tok::Space, m.end())));
                }
            },
            None => {},
        }
        
        if let Some(c) = self.prog.chars().nth(self.idx) {
            self.idx += 1;
            if c == ')' {
                return Some(Ok((self.idx-1, Tok::RParen, self.idx)));
            } else if c == '(' {
                return Some(Ok((self.idx-1, Tok::LParen, self.idx)));
            } else if c == '.' {
                return Some(Ok((self.idx-1, Tok::Dot, self.idx)));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_some() {
        let  lexer = Lexer::new("(())");
        let toks: Vec<Result<(usize, Tok, usize), LexError>> = lexer.collect();
    }
    
    #[test]    
    fn lex_symbol() {
        let mut lexer = Lexer::new("asdf asdf asdf asdf");
        if let Some(Ok((span0, Tok::Symbol(sym), span1))) = lexer.next() {
            assert_eq!(span0, 0);
            assert_eq!(sym, "asdf");
            assert_eq!(span1, 4);
        } else {
            panic!("")
        }
    }

    #[test]    
    fn lex_rparen() {
        let mut lexer = Lexer::new(")");
        if let Some(Ok((span0, tok, span1))) = lexer.next() {
            assert_eq!(span0, 0);
            assert_eq!(tok, Tok::RParen);
            assert_eq!(span1, 1);
        } else {
            panic!("")
        }
    }

    
    #[test]    
    fn lex_float() {
        let mut lexer = Lexer::new("123.123 asdf");
        let float = lexer.next();
        println!("{:?}", float);
            
        if let Some(Ok((span0, Tok::Float(f), span1))) = float {
            assert_eq!(span0, 0);
            assert_eq!(f, 123.123);
            assert_eq!(span1, 7);
        } else {
            panic!("")
        }
    }
    
    #[test]    
    fn lex_int() {
        let mut lexer = Lexer::new("123");
        let int = lexer.next();
        println!("{:?}", int);
        
        if let Some(Ok((span0, Tok::Int(n), span1))) = int {
            assert_eq!(span0, 0);
            assert_eq!(n, 123);
            assert_eq!(span1, 3);
        } else {
            panic!("")
        }
    }

    #[test]    
    fn lex_paren() {
        let mut lexer = Lexer::new("(123");
        let paren = lexer.next();
        println!("{:?}", paren);
        
        if let Some(Ok((span0, tok, span1))) = paren {
            assert_eq!(span0, 0);
            assert_eq!(tok, Tok::LParen);
            assert_eq!(span1, 1);
        } else {
            panic!("")
        }
    }

    #[test]    
    fn float_experiment() {
        // match "123.345 sdfgdsfg".parse::<(f64, usize)>() {
        //     Ok(num) => assert_eq!(num, (123.345, 7)),
        //     Err(_) => panic!("parse fails"),
        // }

        //let symbol_pat = Regex::new(r#"[a-zA-Z]+"#).unwrap();
        let pattern = Regex::new(r"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?").unwrap();
        
        match pattern.find_at("123.123 asdf", 0) {
            Some(m) => {
                assert_eq!(m.as_str().parse::<f64>().unwrap(), 123.123);
            },
            None => {},
        }
    }
}
