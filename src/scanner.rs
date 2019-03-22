use std::io::Read;

use crate::Position;

pub enum TokenKind {

}

pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

pub struct Scanner<I: Read> {
    input: I,
    pos: usize,
    buf: Vec<u8>,
}

impl<I: Read> Scanner<I> {
    pub fn new(input: I) -> Self {
        Scanner {
            input,
            pos: 0,
            buf: Vec::new(),
        }
    }
}

impl<I: Read> Iterator for Scanner<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
