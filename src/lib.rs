#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub mod ast;
mod errors;
mod ir;
mod scanner;
mod typeck;
mod types;

pub use errors::Error;
pub use parser::{LineParser, ProgramParser};
pub use scanner::{ParseError, Scanner, Token};
pub use typeck::{unify, TypeCheck, TypeError};
pub use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;
