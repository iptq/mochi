#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub mod ast;
mod codegen;
mod errors;
mod env;
mod ir;
mod scanner;
pub mod typeck;
mod types;

pub use codegen::SemanticChecker;
pub use errors::Error;
pub use parser::{LineParser, ProgramParser};
pub use scanner::{ParseError, Scanner, Token};
pub use typeck::{unify, TypeCheck, TypeError};
pub use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;
