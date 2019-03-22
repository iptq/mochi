use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

use std::io::{self, Read};

mod ast;
mod ir;
mod scanner;
mod types;

use types::Type;

pub type Position = (usize, usize);

fn main() {
    let mut stdin = io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).expect("failed to read");

    let contents = String::from_utf8(buf).expect("invalid utf8");
    let parser = parser::ProgramParser::new();
    let ast = parser.parse(&contents).expect("failed to parse");

    println!("contents: {:?}", ast);
}
