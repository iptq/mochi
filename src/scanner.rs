use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

use regex::Regex;
use symbol::Symbol;

use crate::Position;
use crate::Spanned;

#[derive(Debug, Clone)]
pub enum Token {
    KwdClass,
    KwdFor,
    KwdFunc,
    KwdIn,
    KwdLet,
    KwdMatch,
    KwdUse,

    SymColon,
    SymComma,
    SymDot,
    SymDblDot,
    SymEqual,
    SymParenL,
    SymParenR,
    SymPercent,

    Sym1(Symbol),
    Sym2(Symbol),
    Symbol(Symbol),

    IntLiteral(String),
    Ident(Symbol),

    Indent,
    Dedent,
}

lazy_static! {
    pub static ref WHITESPACE: Regex = Regex::new(r"^([ \t]+)").unwrap();

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
    indents: Vec<usize>,
    pos: usize,
}

impl<I: Read> Scanner<I> {
    pub fn new(input: I) -> Self {
        Scanner {
            input: BufReader::new(input),
            queue: VecDeque::new(),
            indents: vec![0],
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
        let mut off = 0;

        loop {
            match self.input.read_line(&mut line) {
                Ok(n) => {
                    off += n;
                    if n == 0 {
                        break;
                    } else if line.trim().len() == 0 {
                        // count dedents
                        while self.indents.len() > 1 {
                            self.indents.pop();
                            self.queue.push_back(Ok((self.pos, Token::Dedent, self.pos)));
                        }
                        continue;
                    } else if line.ends_with("\\\r\n") || line.ends_with("\\\n") {
                        line = line.trim_end().trim_end_matches('\\').to_owned();
                        // read another line
                        continue;
                    } else {
                        break;
                    }
                }
                Err(err) => panic!("error scanning: {}", err),
            }
        }

        let whitespace = match WHITESPACE.find(&line) {
            Some(mat) => mat.end() - mat.start(),
            None => 0,
        };

        match self.indents.last() {
            Some(n) => {
                if whitespace < *n {
                    let ind = self.indents.binary_search(&whitespace).expect("inconsistent indentation");
                    for i in 0..ind {
                        self.indents.pop();
                        self.queue.push_back(Ok((self.pos, Token::Dedent, self.pos)));
                    }
                } else if whitespace > *n {
                    self.indents.push(whitespace);
                    self.queue.push_back(Ok((self.pos, Token::Indent, self.pos + whitespace)));
                }
            }
            None => unreachable!("indent stack empty"),
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
                        let tok = match i {
                            0 => match mat.as_str() {
                                "class" => Token::KwdClass,
                                "for" => Token::KwdFor,
                                "func" => Token::KwdFunc,
                                "in" => Token::KwdIn,
                                "let" => Token::KwdLet,
                                "match" => Token::KwdMatch,
                                "use" => Token::KwdUse,
                                name => Token::Ident(name.into()),
                            },
                            1 => Token::IntLiteral(mat.as_str().to_owned()),
                            2 => match mat.as_str() {
                                ".." => Token::SymDblDot,
                                sym => Token::Sym2(sym.into()),
                            },
                            3 => match mat.as_str() {
                                ":" => Token::SymColon,
                                "," => Token::SymComma,
                                "." => Token::SymDot,
                                "=" => Token::SymEqual,
                                "(" => Token::SymParenL,
                                ")" => Token::SymParenR,
                                "%" => Token::SymPercent,
                                sym => Token::Sym1(sym.into()),
                            },
                            4 => {
                                end += hi;
                                continue 'outer;
                            }
                            _ => unreachable!("got case {}", i),
                        };
                        self.queue
                            .push_back(Ok((self.pos + end + lo, tok, self.pos + end + hi)));
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
        self.pos += off;
        self.queue.pop_front()
    }
}
