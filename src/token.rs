use crate::types::*;

impl Token {
    pub fn pretty(&self) {
        self.tok.pretty()
    }

    pub fn loc(&self) -> Loc {
        Loc::new("nofile".to_owned(), self.start, self.end)
    }

    pub fn to_obj(&self) -> Obj {
        self.tok.to_objval(self.loc())
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

    pub fn to_objval(&self, loc: Loc) -> Obj {
        match &self {
            &Tok::Symbol(symb) => Obj::new_symb(symb.name.clone(), Some(loc)),
            Tok::Float(n) => Obj::new_float(*n, Some(loc)),
            Tok::Int(n) => Obj::new_int(*n, Some(loc)),
            _ => panic!(),
        }
    }
}
