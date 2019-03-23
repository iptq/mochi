use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Cursor;

use codespan::{CodeMap, FileName};
use codespan_reporting::termcolor::{ColorChoice, StandardStream};
use mochi::{tast, Error, LineParser, Scanner};
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

    'repl: loop {
        match input.readline("mochi:> ") {
            Ok(line) => {
                input.add_history_entry(line.as_ref());

                let mut codemap = CodeMap::new();
                codemap.add_filemap(FileName::Virtual(Cow::Borrowed("")), line.clone());

                let scanner = Scanner::new(Cursor::new(line));
                let ast = match parser.parse(scanner).map_err(Error::from) {
                    Ok(ast) => ast,
                    Err(err) => {
                        err.emit(&codemap, &mut termerr);
                        continue 'repl;
                    }
                };
                println!("ast: {:?}", ast);
                // let tast = tast::Program::from(ast);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
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
