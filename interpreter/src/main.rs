use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;

use symbol::Symbol;

use codespan::{CodeMap, FileName};
use codespan_reporting::termcolor::{ColorChoice, StandardStream};
use mochi::{Error, LineParser, Scanner, Type, ast::Line};
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut input = Editor::<()>::new();

    let history_file = dirs::home_dir().map(|mut path| {
        path.push(".mochi_history");
        path
    });
    // silently fail
    if let Some(path) = history_file.as_ref() {
        let _ = input.load_history(&path);
    }

    let parser = LineParser::new();
    let mut termerr = StandardStream::stderr(ColorChoice::Auto);

    let type_environment = HashMap::<Symbol, Type>::new();

    'repl: loop {
        match input.readline("mochi:> ") {
            Ok(line) => {
                input.add_history_entry(line.as_ref());

                let mut codemap = CodeMap::new();
                codemap.add_filemap(FileName::Virtual(Cow::Borrowed("")), line.clone());

                let scanner = Scanner::new(Cursor::new(line));
                let line = match parser.parse(scanner).map_err(Error::from) {
                    Ok(line) => line,
                    Err(err) => {
                        let _ = err.emit(&codemap, &mut termerr);
                        continue 'repl;
                    }
                };

                match line {
                    Line::Expr(expr) => {
                        println!("ast: {:?}", expr);

                        use mochi::TypeCheck;
                        let constraints = expr.constraints();
                        println!("constraints: {:?}", constraints);

                        let substitutions = mochi::unify(constraints, &type_environment);
                        println!("substitutions: {:?}", substitutions);
                    },
                    Line::Decl(decl) => {
                        println!("ast: {:?}", decl);

                        use mochi::TypeCheck;
                        let constraints = decl.constraints();
                        println!("constraints: {:?}", constraints);

                        let substitutions = mochi::unify(constraints, &type_environment);
                        println!("substitutions: {:?}", substitutions);
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                // do nothing for now
                continue 'repl;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
                break;
            }
        }
    }

    if let Some(path) = history_file.as_ref() {
        input.save_history(&path).expect("failed to save history");
    }
}
