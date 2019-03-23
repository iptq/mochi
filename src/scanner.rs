use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

use regex::RegexSet;
use symbol::Symbol;

use crate::Position;
use crate::Spanned;

#[derive(Debug, Clone)]
pub enum Token {
    KwdClass,
    Ident(Symbol),
}

lazy_static! {
    static ref TABLE: RegexSet =
        RegexSet::new(&["([A-Za-z][A-Za-z0-9_]*)|([A-Za-z_][A-Za-z0-9_]+)",])
            .expect("failed to compile regexs");
}

#[derive(Debug)]
pub enum ScanError {
    BadLine(String),
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
                        return self.queue.pop_front();
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

        let mut end = 0;
        while end < line.len() {
            let matches = TABLE.matches(&line[end..]);
            if !matches.matched_any() {
                self.queue
                    .push_back(Err(ScanError::BadLine(line[end..].to_owned())));
            } else if matches.matched(0) {
                self.queue.push_back(Ok((0, Token::Ident("".into()), 0)));
            }
        }
        self.queue.pop_front()
    }
}
