use crate::types::*;

impl Token {
    pub fn pretty(&self) {
        self.tok.pretty()
    }
}


impl Tok {
    pub fn pretty(&self) {
        match &self {
            Tok::Symbol(symb) => print!("{}", symb.name),
            Tok::Float(n) => print!("{}", n),
            Tok::Int(n) => print!("{}", n),
            Tok::LParen => print!("("),
            Tok::RParen => print!(")"),
            Tok::Dot => print!("."),
            Tok::Space => print!(" "),
        }
    }
}
