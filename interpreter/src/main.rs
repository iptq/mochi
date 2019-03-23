use std::io::Cursor;
use std::fs::OpenOptions;

use mochi::{tast, LineParser, Scanner};
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

    'repl: loop {
        match input.readline("mochi:> ") {
            Ok(line) => {
                input.add_history_entry(line.as_ref());

                let scanner = Scanner::new(Cursor::new(line));
                let ast = match parser.parse(scanner) {
                    Ok(ast) => ast,
                    Err(err) => {
                        eprintln!("error: {:?}", err);
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
