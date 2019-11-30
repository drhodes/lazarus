use regex::Regex;
use crate::types::*;


impl Token {
    pub fn new(tok: Tok, start: usize, end: usize) -> Token {
        Token { tok, start, end }
    }

    pub fn is_float(&self) -> bool {
        match self.tok {
            Tok::Float(_) => true,
            _ => false,
        }
    }
    pub fn is_int(&self) -> bool {
        match self.tok {
            Tok::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_symbol(&self) -> bool {
        match self.tok {
            Tok::Symbol(_) => true,
            _ => false,
        }
    }
    pub fn is_lparen(&self) -> bool {
        match self.tok {
            Tok::LParen => true,
            _ => false,
        }
    }
    pub fn is_rparen(&self) -> bool {
        match self.tok {
            Tok::RParen => true,
            _ => false,
        }
    }
    pub fn is_dot(&self) -> bool {
        match self.tok {
            Tok::Dot => true,
            _ => false,
        }
    }
    pub fn is_space(&self) -> bool {
        match self.tok {
            Tok::Space => true,
            _ => false,
        }
    }
}


#[derive(Debug)]
pub enum LexError {}

pub struct Lexer {
    idx: usize,
    prog: String,
    pub filename: String,
}

impl Lexer {
    pub fn new(input: &str, filename: &str) -> Self {
        Lexer {
            idx: 0,
            prog: input.to_owned(),
            filename: filename.to_owned(),
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexError>; //Spanned<Tok, usize, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.prog.len() {
            return None;
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
                    let tok = Tok::Symbol(Symb::new(m.as_str(), self.filename.clone(), m.start()));
                    self.idx = m.end();
                    return Some(Ok(Token::new(tok, m.start(), m.end())));
                }
            }
            None => {}
        }

        // order matters! must try to parse float before int.
        match float_pat.find_at(&self.prog, self.idx) {
            Some(m) => {
                if m.start() == self.idx {
                    let tok = Tok::Float(m.as_str().parse::<f64>().unwrap());
                    self.idx = m.end();
                    return Some(Ok(Token::new(tok, m.start(), m.end())));
                }
            }
            None => {}
        }

        // try an int
        match int_pat.find_at(&self.prog, self.idx) {
            Some(m) => {
                if m.start() == self.idx {
                    let tok = Tok::Int(m.as_str().parse::<i64>().unwrap());
                    self.idx = m.end();
                    return Some(Ok(Token::new(tok, m.start(), m.end())));
                }
            }
            None => {}
        }

        match space_pat.find_at(&self.prog, self.idx) {
            Some(m) => {
                if m.start() == self.idx {
                    self.idx = m.end();
                    return Some(Ok(Token::new(Tok::Space, m.start(), m.end())));
                }
            }
            None => {}
        }

        if let Some(c) = self.prog.chars().nth(self.idx) {
            self.idx += 1;
            if c == ')' {
                return Some(Ok(Token::new(Tok::RParen, self.idx - 1, self.idx)));
            } else if c == '(' {
                return Some(Ok(Token::new(Tok::LParen, self.idx - 1, self.idx)));
            } else if c == '.' {
                return Some(Ok(Token::new(Tok::Dot, self.idx - 1, self.idx)));
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
        let lexer = Lexer::new("(())", "test.scm");
        let toks: Vec<Result<Token, LexError>> = lexer.collect();
        assert_eq!(4, toks.len());
    }

    #[test]
    fn lex_symbol() {
        let mut lexer = Lexer::new("asdf asdf asdf asdf", "test.scm");
        if let Some(Ok(tok)) = lexer.next() {
            assert_eq!(tok.start, 0);
            //assert_eq!(tok.tok, Tok::Symbol(Symb::new(}));
            assert_eq!(tok.end, 4);
        } else {
            panic!("")
        }
    }

    #[test]
    fn lex_rparen() {
        let mut lexer = Lexer::new(")", "test.scm");
        if let Some(Ok(tok)) = lexer.next() {
            assert_eq!(tok.start, 0);
            assert_eq!(tok.tok, Tok::RParen);
            assert_eq!(tok.end, 1);
        } else {
            panic!("")
        }
    }

    #[test]
    fn lex_float() {
        let mut lexer = Lexer::new("123.123 asdf", "test.scm");
        let float = lexer.next();
        println!("{:?}", float);

        if let Some(Ok(tok)) = float {
            assert_eq!(tok.start, 0);
            assert_eq!(tok.tok, Tok::Float(123.123));
            assert_eq!(tok.end, 7);
        } else {
            panic!("")
        }
    }

    #[test]
    fn lex_int() {
        let mut lexer = Lexer::new("123", "test.scm");
        let int = lexer.next();
        println!("{:?}", int);

        if let Some(Ok(tok)) = int {
            assert_eq!(tok.start, 0);
            assert_eq!(tok.tok, Tok::Int(123));
            assert_eq!(tok.end, 3);
        } else {
            panic!("")
        }
    }

    #[test]
    fn lex_paren() {
        let mut lexer = Lexer::new("(123", "test.scm");
        let paren = lexer.next();
        println!("{:?}", paren);

        if let Some(Ok(tok)) = paren {
            assert_eq!(tok.start, 0);
            assert_eq!(tok.tok, Tok::LParen);
            assert_eq!(tok.end, 1);
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
            }
            None => {}
        }
    }
}
