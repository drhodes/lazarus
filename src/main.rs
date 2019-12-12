use std::fs;

use lazarus::lexer::Lexer;
use lazarus::parser::Parser;
use lazarus::types::*;
use lazarus::eval::eval;

fn get_parser(s: &str, filename: &str) -> Parser {
    let lexer = Lexer::new(s, filename);
    Parser::new(lexer)
}

fn eval_str(s: &str, filename: &str) -> EvalResult<Obj> {
    let mut env = Env::the_global_environment();
    let mut parser = get_parser(s, filename);
    let parse_results = parser.list().unwrap();
    let obj = parse_results.to_obj();
    eval(obj, &mut env)
}

fn main() { 
    let filename = "test-cases/cycle.scm";
    let prog = fs::read_to_string(filename);
    println!("{:?}", eval_str(&prog.unwrap(), filename));
}
