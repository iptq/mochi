#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

use std::io::{self, Cursor, Read};

mod ast;
mod ir;
mod scanner;
mod types;

use scanner::Scanner;
use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;

fn main() {
    let mut stdin = io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).expect("failed");
    let contents = String::from_utf8(buf).expect("invalid utf-8");

    // let mut scanner = Scanner::new(Cursor::new(contents.clone()));
    // for token in scanner {
    //     println!("token: {:?}", token);
    // }

    let mut scanner = Scanner::new(Cursor::new(contents.clone()));
    let parser = parser::ProgramParser::new();
    let ast = parser.parse(scanner).expect("failed to parse");
    println!("ast: {:?}", ast);
}
