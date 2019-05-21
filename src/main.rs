#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate target_lexicon;

lalrpop_mod!(parser);

mod ast;
mod codegen;
mod env;
mod mir;
mod scanner;
mod prelude;
mod typeck;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use lalrpop_util::lalrpop_mod;
use structopt::StructOpt;
use symbol::Symbol;

use crate::ast::Type as AstType;
use crate::codegen::Codegen;
use crate::env::Environment;
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

    // create environment
    let mut type_env = Environment::<Symbol, AstType>::new();
    prelude::load_prelude(&mut type_env);
    type_env.push_scope();
    for decl in &ast.0 {
        if let Some((name, ty)) = decl.get_signature() {
            type_env.insert(name, ty);
        }
    }
    println!("type_env: {:?}", type_env);

    // typecheck the ast
    let mut constraints = HashSet::new();
    for decl in &ast.0 {
        typeck::get_constraints_decl(&mut type_env, &mut constraints, decl);
    }
    println!("constraints: {:?}", constraints);

    // generate ir from ast
    let mut codegen = Codegen::new();
    for decl in &ast.0 {
        use crate::ast::Decl;
        match decl {
            Decl::Func(func) => codegen.compile_func(func),
            _ => (),
        }
    }

    let output_file = opt.file.with_extension("o");
    codegen.finish(&output_file);
}
