#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate target_lexicon;

lalrpop_mod!(parser);

mod ast;
mod codegen;
mod env;
mod scanner;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use lalrpop_util::lalrpop_mod;
use structopt::StructOpt;

use crate::codegen::Codegen;
use crate::parser::ProgramParser;
use crate::scanner::Scanner;

#[derive(StructOpt)]
struct Opt {
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    // read ast from file
    let file = File::open(&opt.file).expect("failed");
    let scanner = Scanner::new(file);
    let parser = ProgramParser::new();
    let ast = parser.parse(scanner).expect("failed");
    println!("ast: {:?}", ast);

    // generate ir from ast
    let mut codegen = Codegen::new();
    for decl in ast.0 {
        use crate::ast::Decl;
        match decl {
            Decl::Func(func) => codegen.compile_func(func),
            _ => (),
        }
    }

    let output_file = opt.file.with_extension("o");
    codegen.finish(&output_file);
}
