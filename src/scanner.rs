use std::collections::VecDeque;
use std::error::Error as StdError;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

use lalrpop_util::ParseError as LalrpopError;
use regex::Regex;
use symbol::Symbol;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;

#[derive(Debug, Clone)]
pub enum Token {
    KwdClass,
    KwdExtern,
    KwdFor,
    KwdFn,
    KwdIf,
    KwdIn,
    KwdLet,
    KwdMatch,
    KwdReturn,
    KwdUse,

    SymArrow,
    SymDblArrow,
    SymDblDot,
    SymDblEqual,
    SymGreaterThanEquals,
    SymLeftShift,
    SymLessThanEquals,
    SymLogicalOr,
    SymLogicalAnd,
    SymNotEqual,
    SymRightShift,

    SymAmpersand,
    SymBang,
    SymCaret,
    SymColon,
    SymComma,
    SymDash,
    SymDot,
    SymEqual,
    SymGreaterThan,
    SymLessThan,
    SymParenL,
    SymParenR,
    SymPercent,
    SymPipe,
    SymPlus,
    SymSemicolon,
    SymSlash,
    SymStar,
    SymTilde,
    SymUnderscore,

    Sym1(Symbol),
    Sym2(Symbol),
    Symbol(Symbol),

    IntLiteral(String),
    StringLiteral(String),
    Ident(Symbol),

    Indent,
    Dedent,
    Sep,
}

lazy_static! {
    pub static ref WHITESPACE: Regex = Regex::new(r"^([ \t]+)").unwrap();

    // slow and lazy way, will replace with some efficient RegexSet later
    pub static ref TABLE: Vec<Regex> = vec![
        // ident
        Regex::new(r"^(([A-Za-z][A-Za-z0-9_]*)|([A-Za-z_][A-Za-z0-9_]+))").unwrap(),

        // int literals
        Regex::new(r"^([0-9]+)").unwrap(),

        // string literals
        // TODO: modal scanning?
        Regex::new(r#"^("[^"]*")"#).unwrap(),

        // 2-char symbols
        Regex::new(r"^((\.\.)|(==)|(!=)|(->)|(=>))").unwrap(),

        // 1-char symbols
        Regex::new(r#"^[=<>\(\)\[\]:\.,_%"'\*+-;]"#).unwrap(),

        // whitespace
        Regex::new(r"^([ \t\n]+)").unwrap(),
    ];
}

#[derive(Debug)]
pub enum ScanError {
    BadSymbol(usize),
    InvalidToken(usize),
    UnrecognizedToken(Option<(usize, Token, usize)>, Vec<String>),
    ExtraToken(usize, Token, usize),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unimplemented"),
        }
    }
}

impl StdError for ScanError {}

impl From<LalrpopError<usize, Token, ScanError>> for ScanError {
    fn from(err: LalrpopError<usize, Token, ScanError>) -> Self {
        match err {
            LalrpopError::InvalidToken { location } => ScanError::InvalidToken(location),
            LalrpopError::UnrecognizedToken { token, expected } => {
                ScanError::UnrecognizedToken(token, expected)
            }
            LalrpopError::ExtraToken { token } => ScanError::ExtraToken(token.0, token.1, token.2),
            LalrpopError::User { error } => error,
        }
    }
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
                        while self.indents.len() > 1 {
                            self.indents.pop();
                            self.queue
                                .push_back(Ok((self.pos, Token::Dedent, self.pos)));
                        }
                        return self.queue.pop_front();
                    } else if line.trim().len() == 0 {
                        break;
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

        if line.trim().len() > 0 {
            let whitespace = match WHITESPACE.find(&line) {
                Some(mat) => mat.end() - mat.start(),
                None => 0,
            };

            match self.indents.last() {
                Some(n) => {
                    if whitespace < *n {
                        let ind = self
                            .indents
                            .binary_search(&whitespace)
                            .expect("inconsistent indentation");
                        for i in ind..self.indents.len() - 1 {
                            self.indents.pop();
                            self.queue
                                .push_back(Ok((self.pos, Token::Dedent, self.pos)));
                        }
                        // add a line just for good measure
                        self.queue.push_back(Ok((self.pos, Token::Sep, self.pos)));
                    } else if whitespace > *n {
                        self.indents.push(whitespace);
                        self.queue
                            .push_back(Ok((self.pos, Token::Indent, self.pos + whitespace)));
                    }
                }
                None => unreachable!("indent stack empty"),
            }
        } else {
            // add a new line
            self.queue.push_back(Ok((self.pos, Token::Sep, self.pos)));
        }

        line = line.trim().to_owned();
        let mut end = 0;
        'outer: while end < line.len() {
            'inner: for (i, regex) in TABLE.iter().enumerate() {
                match regex.find(&line[end..]) {
                    Some(mat) => {
                        let lo = mat.start();
                        let hi = mat.end();
                        let tok = match i {
                            0 => match mat.as_str() {
                                "class" => Token::KwdClass,
                                "extern" => Token::KwdExtern,
                                "for" => Token::KwdFor,
                                "fn" => Token::KwdFn,
                                "if" => Token::KwdIf,
                                "in" => Token::KwdIn,
                                "let" => Token::KwdLet,
                                "match" => Token::KwdMatch,
                                "return" => Token::KwdReturn,
                                "use" => Token::KwdUse,
                                name => Token::Ident(name.into()),
                            },
                            1 => Token::IntLiteral(mat.as_str().to_owned()),
                            2 => Token::StringLiteral(
                                mat.as_str()
                                    .trim_start_matches("\"")
                                    .trim_end_matches("\"")
                                    .to_owned(),
                            ),
                            3 => match mat.as_str() {
                                "->" => Token::SymArrow,
                                "=>" => Token::SymDblArrow,
                                ".." => Token::SymDblDot,
                                "==" => Token::SymDblEqual,
                                ">=" => Token::SymGreaterThanEquals,
                                "<<" => Token::SymLeftShift,
                                "<=" => Token::SymLessThanEquals,
                                "||" => Token::SymLogicalOr,
                                "&&" => Token::SymLogicalAnd,
                                "!=" => Token::SymNotEqual,
                                ">>" => Token::SymRightShift,
                                sym => Token::Sym2(sym.into()),
                            },
                            4 => match mat.as_str() {
                                "&" => Token::SymAmpersand,
                                "!" => Token::SymBang,
                                "^" => Token::SymCaret,
                                ":" => Token::SymColon,
                                "," => Token::SymComma,
                                "-" => Token::SymDash,
                                "." => Token::SymDot,
                                "=" => Token::SymEqual,
                                ">" => Token::SymGreaterThan,
                                "<" => Token::SymLessThan,
                                "(" => Token::SymParenL,
                                ")" => Token::SymParenR,
                                "%" => Token::SymPercent,
                                "|" => Token::SymPipe,
                                "+" => Token::SymPlus,
                                ";" => Token::SymSemicolon,
                                "/" => Token::SymSlash,
                                "*" => Token::SymStar,
                                "~" => Token::SymTilde,
                                "_" => Token::SymUnderscore,
                                sym => Token::Sym1(sym.into()),
                            },
                            5 => {
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
                .push_back(Err(ScanError::BadSymbol(self.pos + end)));
            break;
        }
        self.pos += off;
        if self.indents.len() > 1 {
            self.queue.push_back(Ok((self.pos, Token::Sep, self.pos)));
        }

        if line.trim().len() == 0 {
            self.next()
        } else {
            self.queue.pop_front()
        }
    }
}
