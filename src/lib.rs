#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

mod ast;
mod errors;
mod ir;
mod scanner;
pub mod tast;
mod types;

pub use errors::Error;
pub use parser::{LineParser, ProgramParser};
pub use scanner::{ParseError, Scanner, Token};
use tast::TypeError;
use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;
