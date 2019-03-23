use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

use regex::Regex;
use symbol::Symbol;

use crate::Position;
use crate::Spanned;

#[derive(Debug, Clone)]
pub enum Token {
    KwdClass,
    IntLiteral(String),
    Ident(Symbol),

    Sym1(Symbol),
    Sym2(Symbol),
    Symbol(Symbol),
}

lazy_static! {
    // slow and lazy way, will replace with some efficient RegexSet later
    pub static ref TABLE: Vec<Regex> = vec![
        Regex::new(r"^(([A-Za-z][A-Za-z0-9_]*)|([A-Za-z_][A-Za-z0-9_]+))").unwrap(),
        Regex::new(r"^([0-9]+)").unwrap(),
        Regex::new(r"^((\.\.)|(==)|(!=)|(->))").unwrap(),
        Regex::new(r#"^[=<>\(\)\[\]:\.,_%"']"#).unwrap(),
        Regex::new(r"^([ \t\n]+)").unwrap(),
    ];
}

#[derive(Debug)]
pub enum ScanError {
    BadSymbol(String),
}

pub type ScanOutput = Spanned<usize, Token, ScanError>;

pub struct Scanner<I: Read> {
    input: BufReader<I>,
    queue: VecDeque<ScanOutput>,
    pos: usize,
}

impl<I: Read> Scanner<I> {
    pub fn new(input: I) -> Self {
        Scanner {
            input: BufReader::new(input),
            queue: VecDeque::new(),
            pos: 0,
        }
    }
}

impl<I: Read> Iterator for Scanner<I> {
    type Item = ScanOutput;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(token) => return Some(token),
            None => (),
        }

        let mut line = String::new();
        loop {
            match self.input.read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else if line.trim().len() == 0 {
                        continue;
                    } else if line.ends_with("\\\r\n") || line.ends_with("\\\n") {
                        // read another line
                        continue;
                    } else {
                        break;
                    }
                }
                Err(err) => panic!("error scanning: {}", err),
            }
        }
        line = line.trim().to_owned();

        let mut end = 0;
        'outer: while end < line.len() {
            'inner: for (i, regex) in TABLE.iter().enumerate() {
                // println!("searching: \"{}\"", &line[end..]);
                match regex.find(&line[end..]) {
                    Some(mat) => {
                        let lo = mat.start();
                        let hi = mat.end();
                        let tok =
                            match i {
                                0 => match mat.as_str() {
                                    "class" => Token::KwdClass,
                                    name => Token::Ident(name.into()),
                                },
                                1 => Token::IntLiteral(mat.as_str().to_owned()),
                                2 => Token::Sym2(mat.as_str().into()),
                                3 => Token::Sym1(mat.as_str().into()),
                                4 => {
                                    end += hi;
                                    continue 'outer;
                                }
                                _ => unreachable!("got case {}", i),
                            };
                        // println!("got {:?}", (end + lo, &tok, end + hi));
                        self.queue.push_back(Ok((end + lo, tok, end + hi)));
                        end += hi;
                        continue 'outer;
                    }
                    None => continue 'inner,
                }
            }

            // didn't find anything
            self.queue
                .push_back(Err(ScanError::BadSymbol(line[end..].to_owned())));
            break;
        }
        self.queue.pop_front()
    }
}
