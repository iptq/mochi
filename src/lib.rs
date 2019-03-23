#[macro_use]
extern crate lazy_static;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

mod ast;
mod ir;
mod scanner;
pub mod tast;
mod types;

pub use parser::ProgramParser;
pub use scanner::Scanner;
use types::Type;

type Position = (usize, usize);
type Spanned<Location, Token, Error> = Result<(Location, Token, Location), Error>;
