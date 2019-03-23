#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

use std::io::{self, Read};

mod ast;
mod ir;
mod regex_ext;
mod scanner;
mod types;

use scanner::Scanner;
use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin);

    loop {
        match scanner.next() {
            Some(token) => println!("token: {:?}", token),
            None => break,
        }
    }

    // let parser = parser::ProgramParser::new();
    // let ast = parser.parse(scanner).expect("failed to parse");
}