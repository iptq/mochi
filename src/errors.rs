use std::error::Error as StdError;
use std::fmt;
use std::io;

use codespan::{ByteIndex, CodeMap, Span};
use codespan_reporting::{termcolor::WriteColor, Diagnostic, Label};
use lalrpop_util::ParseError as LalrpopError;

use crate::ParseError;
use crate::Token;
use crate::TypeError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Type(TypeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "Parse error: {}", err),
            Error::Type(err) => write!(f, "Type error: {}", err),
        }
    }
}

impl StdError for Error {}

impl Error {
    pub fn emit(&self, codemap: &CodeMap, buffer: &mut impl WriteColor) -> io::Result<()> {
        let diagnostic = match self {
            Error::Parse(ParseError::BadSymbol(lo)) => {
                let message = format!("Invalid symbol.");
                let label =
                    Label::new_primary(Span::new(ByteIndex(*lo as u32), ByteIndex(*lo as u32)));
                Diagnostic::new_error(message).with_label(label)
            }
            Error::Parse(ParseError::UnrecognizedToken(token, expected)) => {
                let message = format!("Unrecognized token. Expecting: {}", expected.join(","));
                Diagnostic::new_error(message)
            }
            _ => Diagnostic::new_error(format!("failed: {:?}", self)),
        };
        codespan_reporting::emit(buffer, &codemap, &diagnostic)
    }
}

impl From<LalrpopError<usize, Token, ParseError>> for Error {
    fn from(err: LalrpopError<usize, Token, ParseError>) -> Self {
        Error::Parse(ParseError::from(err))
    }
}

impl From<TypeError> for Error {
    fn from(err: TypeError) -> Self {
        Error::Type(err)
    }
}
