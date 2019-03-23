use std::fs::File;
use std::path::PathBuf;

use codespan::CodeMap;
use codespan_reporting::{
    emit as report,
    termcolor::{ColorChoice, StandardStream},
    Diagnostic,
};
use mochi::{Error, ProgramParser, Scanner};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn compile(file: File) -> Result<String, Error> {
    let parser = ProgramParser::new();

    let scanner = Scanner::new(file);
    let ast = match parser.parse(scanner) {
        Ok(ast) => ast,
        Err(err) => return Err(Error::from(err)),
    };

    Ok(format!("ast: {:?}", ast))
}

fn main() {
    let mut termerr = StandardStream::stderr(ColorChoice::Auto);
    let mut codemap = CodeMap::new();
    let opt = Opt::from_args();

    if !opt.input.exists() {
        report(
            &mut termerr,
            &codemap,
            &Diagnostic::new_error("input file doesn't exist"),
        );
    }

    let file = File::open(&opt.input).expect("couldn't open file");
    codemap.add_filemap_from_disk(&opt.input);
    match compile(file) {
        Ok(result) => println!("result: {:?}", result),
        Err(err) => {
            err.emit(&codemap, &mut termerr);
        }
    }
}
